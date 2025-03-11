//! This module contains several utility functions to manage the state of the caches
//! of the executing processor. On von Neumann architectures (like x86/AMD64), these are no-ops,
//! as these processors ensure synchronization of the instruction and data caches internally.
//! On modified Harvard architectures like ARMv8, these functions are needed to ensure that
//! the data cache and instruction cache stay synchronized.

/// This function should be called before any jit-compiled code is executed, on the thread that will
/// execute this code.
#[inline(always)]
pub fn prepare_for_execution(slice: &[u8]) {
    #![allow(unused_variables)]
    #[cfg(target_arch="aarch64")]
    {
        aarch64::prepare_for_execution()
    }
    #[cfg(any(target_arch="riscv64", target_arch="riscv32"))]
    {
        riscv::enforce_ordering_dcache_icache(slice, true);
    }
}

/// This function should be called after modification of any data that could've been loaded into the
/// instruction cache previously. It will ensure that these modifications will be propagated into
/// the instruction caches
#[inline(always)]
#[allow(unused_variables)]
pub fn synchronize_icache(slice: &[u8]) {
    #[cfg(target_arch="aarch64")]
    {
        aarch64::synchronize_icache(slice);
    }
}

#[cfg(target_arch="aarch64")]
mod aarch64 {
    use std::arch::asm;

    /// return the cache line sizes as reported by the processor as a tuple of (dcache, icache)
    fn get_cacheline_sizes() -> (usize, usize) {
        let ctr_el0: usize;

        // safety: we're just reading a system register (ctr_cl0) that can always be read
        unsafe {
            asm!(
                "mrs {outreg}, ctr_el0",
                outreg = lateout(reg) ctr_el0,
                options(nomem, nostack, preserves_flags)
            );
        }

        (
            4 << ((ctr_el0 >> 16) & 0xF),
            4 << (ctr_el0 & 0xF)
        )
    }

    /// waits for any previous cache operations to complete. According to the Aarch64 manuals
    /// `dsb ish` has bonus functionality where it will also wait for any previous cache maintenance
    /// operations to complete before allowing execution to continue.
    #[inline(always)]
    fn wait_for_cache_ops_complete() {
        // safety: this is purely a memory barrier.
        unsafe {
            asm!(
                "dsb ish",
                options(nostack, preserves_flags)
            );
        }
    }

    /// inform the processor that the dache line containing `addr` should be synchronized back to
    /// the unified memory layer
    #[inline(always)]
    fn flush_dcache_line(addr: usize) {
        // safety: flushing caches is always safe
        unsafe {
            asm!(
                "dc cvau, {address}",
                address = in(reg)addr,
                options(nostack, preserves_flags)
            );
        }
    }

    /// inform the processor that icache line containing `addr` is invalid, and that it should be
    /// re-fetched from unified memory
    #[inline(always)]
    fn invalidate_icache_line(addr: usize) {
        // safety: invalidating caches is always safe
        unsafe {
            asm!(
                "ic ivau, {address}",
                address = in(reg)addr,
                options(nostack, preserves_flags)
            );
        }
    }

    /// inform the current core that the pipeline might contain stale data that should
    /// be re-fetched from the instruction cache
    #[inline(always)]
    fn invalidate_pipeline() {
        // safety: this is just a barrier.
        unsafe {
            asm!(
                "isb",
                options(nostack, preserves_flags)
            );
        }
    }

    /// On Aarch64, after the data has been synchronized from the dcache to the icache
    /// it is necessary to flush the pipelines of the cores that will execute the modified data
    /// as some may already have been loaded into the pipeline.
    #[inline(always)]
    pub fn prepare_for_execution() {
        invalidate_pipeline();
    }

    /// On Aarch64, we first need to flush data from the dcache to unified memory, and then 
    /// inform the icache(s) that their current data might be invalid. This is a no-op if
    /// the slice is zero length.
    pub fn synchronize_icache(slice: &[u8]) {
        if slice.len() == 0 {
            return;
        }

        let start_addr = slice.as_ptr() as usize;
        let end_addr = start_addr + slice.len();

        // query the cache line sizes
        let (dcache_line_size, icache_line_size) = get_cacheline_sizes();

        // dcache cleaning loop
        let mut addr = start_addr & !(dcache_line_size - 1);
        while addr < end_addr {
            flush_dcache_line(addr);
            addr += dcache_line_size;
        }

        // need to wait for dcache cleaning to complete before invalidating the icache
        wait_for_cache_ops_complete();

        // icache invalidation loop
        addr = start_addr & !(icache_line_size - 1);
        while addr < end_addr {
            invalidate_icache_line(addr);
            addr += icache_line_size;
        }

        // wait for that to complete as well
        wait_for_cache_ops_complete();
    }
}

#[cfg(any(target_arch="riscv64", target_arch="riscv32"))]
mod riscv {
    // On risc-v, the story about how we synchronize caches is confused.
    // The data sheet states that we ought to do the following.
    // 1: on the assembling hart, perform a data fence to ensure that
    //    any stores will be visible to other harts
    // 2: on the executing hart, perform a fence.i instruction fence to
    //    ensure that all the observed stores are visible to our instruction
    //    fetches
    //
    // however, this doesn't solve all problems. Namely, the OS might just move our process
    // from the current hart to another hart after the fence.i instruction. So it basically
    // offers no guarantees. for this reason, linux has removed FENCE.I from the user ABI, and
    // instead offered a syscall for managing this.
    // this is `riscv_flush_icache()`, which has options to apply to a single thread, or all threads
    // and over a range of addresses.
    // as there are no other operating systems targetting risc-v right now, this is the only choice
    // we have.
    use std::ffi::{c_void, c_long, c_int};

    #[cfg(unix)]
    extern "C" {
        #[link_name="__riscv_flush_icache"]
        fn riscv_flush_icache(start: *const c_void, end: *const c_void, flags: c_long) -> c_int;
    }

    pub fn enforce_ordering_dcache_icache(slice: &[u8], local: bool) {
        let range = slice.as_ptr_range();
        let start = range.start as *const c_void;
        let end = range.end as *const c_void;
        let mut flags: c_long = 0;
        if local {
            flags |= 1;
        }
        let rv;
        unsafe {
            rv = riscv_flush_icache(start, end, flags);
        }
        assert!(rv == 0, "riscv_flush_icache failed, returned {rv}");
    }
}
