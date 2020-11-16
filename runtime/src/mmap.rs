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

    // some hacks to execute instructions not wrapped by the core::arch::arm
    #[repr(align(4))]
    struct Align4<T> {
        inner: T
    }

    #[link_section = ".text"]
    static GET_CTR_CL0: Align4<[u8; 8]> = Align4 {
        inner: [
            0x20, 0x00, 0x3b, 0xd5, // mrs x0, ctr_cl0
            0xc0, 0x03, 0x5f, 0xd6, // ret
        ]
    };

    #[link_section = ".text"]
    static INVALIDATE_CACHELINE: Align4<[u8; 8]> = Align4 {
        inner: [
            0x20, 0x75, 0x0b, 0xd5, // ic ivau
            0xc0, 0x03, 0x5f, 0xd6, // ret
        ]
    };

    #[link_section = ".text"]
    static INVALIDATE_PIPELINE: Align4<[u8; 12]> = Align4 { 
        inner: [
            0x9f, 0x3b, 0x03, 0xd5, // dsb ish
            0xdf, 0x3f, 0x03, 0xd5, // isb sy
            0xc0, 0x03, 0x5f, 0xd6, // ret
        ]
    };

    fn get_cacheline_size() -> usize {
        let get_cacheline_size_internal: extern "aarch64" fn() -> usize = unsafe { std::mem::transmute(GET_CTR_CL0.inner.as_ptr()) };
        4 << (get_cacheline_size_internal() & 0xF)
    }

    /// Ensures that the cache lines covered by `slice` are invalidated in the instruction cache, by successive calls to `ic ivau
    pub fn invalidate_icache_lines(slice: &[u8]) {
        let invalidate_cacheline: extern "aarch64" fn(usize) -> () = unsafe {std::mem::transmute(INVALIDATE_CACHELINE.inner.as_ptr()) };

        // icache lines are guaranteed at least an instruction in size so for small values we can just emit a single instruction.
        // relocations generate a lot of these so we handle them specially here.
        if slice.len() <= 4 {
            invalidate_cacheline((slice.as_ptr() as usize) & !3);
            return;
        }


        // have to flush between these addresses
        let start_addr = slice.as_ptr() as usize;
        let end_addr = start_addr + slice.len();

        // there is no nice API for figuring out the minimum cache line size on aarch64 BIG.little processors.
        // therefore, this ugly kludge
        let mut line_size = get_cacheline_size();
        let mut current_line_size = 0xFFFFFFFF; // max cacheline size is 4 << 15

        while current_line_size > line_size {
            current_line_size = line_size;

            let mut addr = start_addr & !(line_size - 1);

            while addr < end_addr {
                invalidate_cacheline(addr);
                addr += line_size;
            }

            // recheck the line size. The loop will run again if it became smaller.
            line_size = get_cacheline_size();
        }
    }

    /// Ensures the instruction pipeline is brought fully up to date with any previous writes and cache invalidations. Equivalent to `dsb ish, isb sy`.
    pub fn invalidate_pipeline() {
        let invalidate_pipeline_internal: extern "aarch64" fn() -> () = unsafe { std::mem::transmute(INVALIDATE_PIPELINE.inner.as_ptr()) };
        invalidate_pipeline_internal();
    }
}

#[cfg(not(target_arch="aarch64"))]
pub mod cache_management {
    //! This module exports the necessary interfaces to handle instruction cache invalidation that has to happen on the target platform.
    //! The current target architecture has a coherent instruction cache, data cache and pipeline so these are no-ops.

    /// Ensures that the cache lines covered by `slice` are invalidated in the instruction cache. This is a no-op on the current platform.
    pub fn invalidate_icache_lines(_slice: &[u8]) {}
    /// Ensures the instruction pipeline is brought fully up to date with any previous writes and cache invalidations. This is a no-op on the current platform.
    pub fn invalidate_pipeline() {}
}