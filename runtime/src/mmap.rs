//! This module implements some wrappers around Mmap/MmapMut to also support a cheap "empty" variant.
// Unfortunately Memmap itself doesn't support a cheap zero-length variant

use std::ops::{Deref, DerefMut};
use std::io;

use memmap2::{Mmap, MmapMut};

use crate::AssemblyOffset;

/// A structure holding a buffer of executable memory. It also derefs to a `&[u8]`.
/// This structure does not allocate when its size is 0.
#[derive(Debug)]
pub struct ExecutableBuffer {
    // length of the buffer that has actually been written to
    length: usize,
    // backing buffer
    buffer: Option<Mmap>
}

/// ExecutableBuffer equivalent that holds a buffer of mutable memory instead of executable memory. It also derefs to a `&mut [u8]`.
/// This structure does not allocate when its size is 0.
#[derive(Debug)]
pub struct MutableBuffer {
    // length of the buffer that has actually been written to
    length: usize,
    // backing buffer
    buffer: Option<MmapMut>
}

impl ExecutableBuffer {
    /// Obtain a pointer into the executable memory from an offset into it.
    /// When an offset returned from `DynasmLabelApi::offset` is used, the resulting pointer
    /// will point to the start of the first instruction after the offset call,
    /// which can then be jumped or called to divert control flow into the executable
    /// buffer. Note that if this buffer is accessed through an Executor, these pointers
    /// will only be valid as long as its lock is held. When no locks are held,
    /// the assembler is free to relocate the executable buffer when it requires
    /// more memory than available.
    pub fn ptr(&self, offset: AssemblyOffset) -> *const u8 {
        cache_management::invalidate_pipeline();
        &self[offset.0] as *const u8
    }

    /// Create a new executable buffer, backed by a buffer of size `size`.
    /// It will start with an initialized length of 0.
    pub fn new(size: usize) -> io::Result<ExecutableBuffer> {
        let buffer = if size == 0 {
            None
        } else {
            Some(MmapMut::map_anon(size)?.make_exec()?)
        };

        Ok(ExecutableBuffer {
            length: 0,
            buffer
        })
    }

    /// Query the backing size of this executable buffer
    pub fn size(&self) -> usize {
        self.buffer.as_ref().map(|b| b.len()).unwrap_or(0) as usize
    }

    /// Change this executable buffer into a mutable buffer.
    pub fn make_mut(self) -> io::Result<MutableBuffer> {
        let buffer = if let Some(map) = self.buffer {
            Some(map.make_mut()?)
        } else {
            None
        };

        Ok(MutableBuffer {
            length: self.length,
            buffer
        })
    }
}

impl MutableBuffer {
    /// Create a new mutable buffer, backed by a buffer of size `size`.
    /// It will start with an initialized length of 0.
    pub fn new(size: usize) -> io::Result<MutableBuffer> {
        let buffer = if size == 0 {
            None
        } else {
            Some(MmapMut::map_anon(size)?)
        };

        Ok(MutableBuffer {
            length: 0,
            buffer
        })
    }

    /// Query the backing size of this mutable buffer
    pub fn size(&self) -> usize {
        self.buffer.as_ref().map(|b| b.len()).unwrap_or(0) as usize
    }

    /// Set the length of the usable part of this mutable buffer. The length
    /// should not be set larger than the allocated size, otherwise methods can panic.
    pub fn set_len(&mut self, length: usize) {
        self.length = length
    }

    /// Change this mutable buffer into an executable buffer.
    pub fn make_exec(self) -> io::Result<ExecutableBuffer> {
        let buffer = if let Some(map) = self.buffer {
            Some(map.make_exec()?)
        } else {
            None
        };

        Ok(ExecutableBuffer {
            length: self.length,
            buffer
        })
    }
}

impl Default for ExecutableBuffer {
    fn default() -> ExecutableBuffer {
        ExecutableBuffer {
            length: 0,
            buffer: None
        }
    }
}

impl Default for MutableBuffer {
    fn default() -> MutableBuffer {
        MutableBuffer {
            length: 0,
            buffer: None
        }
    }
}

impl Deref for ExecutableBuffer {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        if let Some(map) = &self.buffer {
            &map[..self.length]
        } else {
            &[]
        }
    }
}

impl Deref for MutableBuffer {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        if let Some(map) = &self.buffer {
            &map[..self.length]
        } else {
            &[]
        }
    }
}

impl DerefMut for MutableBuffer {
    fn deref_mut(&mut self) -> &mut [u8] {
        if let Some(map) = &mut self.buffer {
            &mut map[..self.length]
        } else {
            &mut []
        }
    }
}


#[cfg(target_arch="aarch64")]
pub mod cache_management {
    //! This module exports the necessary interfaces to handle instruction cache invalidation that has to happen on the target platform.
    //! The current target platform is aarch64 and therefore this is quite necessary.

    // returns the dcache and icache line sizes in bytes. Both are guaranteed to at least be 4
    fn get_cacheline_sizes() -> (usize, usize) {
        let ctr_el0: usize;
        // safety: just querying a system register for the cache line sizes.
        unsafe {
            asm!(
                "mrs {outreg}, ctr_cl0",
                outreg = lateout(reg) ctr_el0,
                options(nomem, nostack, preserves_flags)
            );
        }
        (
            4 << ((ctr_el0 >> 16) & 0xF),
            4 << (ctr_el0 & 0xF)
        )
    }

    // globals containing the smallest recorded dcache and icache size
    // to work around big.LITTLE shenanigans.
    // they're initialized to the max possible cache line size
    use std::sync::atomic::{AtomicUsize, Ordering};
    static min_dcache_size: AtomicUsize = AtomicUsize::new(0x20000);
    static min_icache_size: AtomicUsize = AtomicUsize::new(0x20000);

    /// Ensures that the cache lines covered by `slice` are invalidated in the instruction cache.
    pub fn invalidate_cache(slice: &[u8]) {
        let start_addr = slice.as_ptr() as usize;
        let end_addr = start_addr + slice.len();

        // figure out the minimum cache line size, accounting for big.LITTLE insanity
        let (dcache_size, icache_size) = get_cacheline_sizes();
        let dcache_size = min_dcache_size.fetch_min(dcache_size, Ordering::Relaxed).min(dcache_size);
        let icache_size = min_icache_size.fetch_min(icache_size, Ordering::Relaxed).min(icache_size);

        // dcache cleaning loop (to update data to point of unification)
        let mut addr = start_addr & !(dcache_size - 1);
        while addr < end_addr {
            // safety: cleaning caches is always safe
            unsafe {
                asm!(
                    "dc cvau, {address}",
                    address = in(reg)addr,
                    options(nostack, preserves_flags)
                );
            }
            addr += dcache_size;
        }
        // await completion of the previous instructions
        // safety: barrier
        unsafe {
            asm!(
                "dsb ish",
                options(nostack, preserves_flags)
            );
        }
        // icache invalidation loop (to make the icache coherent to the point to unification)
        let mut addr = start_addr & !(icache_size - 1);
        while addr < end_addr {
            // safety: invalidating caches is always safe
            unsafe {
                asm!(
                    "ic ivau, {address}",
                    address = in(reg)addr,
                    options(nostack, preserves_flags)
                );
            }
            addr += icache_size;
        }
        // await completion of the previous instructions
        // safety: barrier
        unsafe {
            asm!(
                "dsb ish",
                options(nostack, preserves_flags)
            );
        }
    }

    /// Invalidate any instructions already fetched from the instruction cache before executing possibly altered code.
    pub fn invalidate_pipeline() {
        // safety: this is just a barrier. It can at worst slow performance down.
        unsafe {
            asm!(
                "isb", // flush the pipeline of the current processor
                options(nostack, preserves_flags)
            );
        }
    }
}

#[cfg(not(target_arch="aarch64"))]
pub mod cache_management {
    //! This module exports the necessary interfaces to handle instruction cache invalidation that has to happen on the target platform.
    //! The current target architecture has a coherent instruction cache, data cache and pipeline so these are no-ops.
    //! Cache management is necessary at two points:
    //! first, after data that was already in the instruction cache has been altered in the data cache. The instruction cache needs
    //! to be made coherent again with the data cache. This should happen in the thread that altered the data.
    //! second, before data that has possibly been altered in the instruction cache is executed. It is required to flush the pipeline
    //! of the executing thread to prevent the execution of possibly stale data that was already loaded from the instruction cache.

    /// Ensures that the cache lines covered by `slice` are invalidated in the instruction cache.
    /// This is a no-op on the current platform.
    pub fn invalidate_cache(_slice: &[u8]) {}

    /// Invalidate any instructions already fetched from the instruction cache before executing possibly altered code.
    /// This is a no-op on the current platform.
    pub fn invalidate_pipeline() {}
}