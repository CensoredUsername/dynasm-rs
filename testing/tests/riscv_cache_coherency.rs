// This file contains test cases designed to validate proper assembler cache invalidation
// this is needed because aarch64's modified harvard architecture has an incoherent instruction and
// data cache. Therefore, it is needed to explicitly command the cache hierarchy to flush the dcache
// to the coherent layers, invalidate the icache, and ensure no stale data is left in the
// instruction pipeline. Testcases in this file are designed to break if this isn't handled properly
#![allow(unused_imports)]

extern crate dynasmrt;

use dynasmrt::dynasm;
use dynasmrt::{DynasmApi, DynasmLabelApi};

#[cfg(target_arch="riscv64")]
#[test]
fn test_cache_coherency_same_core() {
    let mut ops = dynasmrt::riscv::Assembler::new().unwrap();
    let reader = ops.reader();

    // write some code
    let start = ops.offset();
    dynasm!(ops
        ; .arch riscv64
        ; li a0, 0xABCD
        ; ret
    );
    let end = ops.offset();

    ops.commit().unwrap();

    // execute it once
    {
        let buf = reader.lock();
        let callable: extern "C" fn() -> u32 = unsafe { std::mem::transmute(buf.ptr(start)) };
        assert_eq!(callable(), 0xABCD);
        drop(buf);
    }

    // change the code back and forth to see if errors happen
    for _ in 0 .. 10000 {
        // change the code
        ops.alter(|modifier| {
            modifier.goto(start);

            dynasm!(modifier
                ; .arch riscv64
                ; li a0, 0xCDEF
                ; ret
            );
            modifier.check_exact(end).unwrap();

        }).unwrap();

        // execute it
        {
            let buf = reader.lock();
            let callable: extern "C" fn() -> u32 = unsafe { std::mem::transmute(buf.ptr(start)) };
            assert_eq!(callable(), 0xCDEF);
            drop(buf);
        }

        // change the code again
        ops.alter(|modifier| {
            modifier.goto(start);

            dynasm!(modifier
                ; .arch riscv64
                ; li a0, 0xABCD
                ; ret
            );
            modifier.check_exact(end).unwrap();

        }).unwrap();

        // execute it again
        {
            let buf = reader.lock();
            let callable: extern "C" fn() -> u32 = unsafe { std::mem::transmute(buf.ptr(start)) };
            assert_eq!(callable(), 0xABCD);
            drop(buf);
        }
    }
}

#[cfg(target_arch="riscv64")]
#[test]
fn test_cache_coherency_other_cores() {
    // spawn a bunch of threads, and have them all racing to execute some assembly
    // then modify the assembly, and see if we execute stale data
    let thread_count = 3;

    use std::sync::atomic::{AtomicU32, AtomicBool, Ordering};

    // the code we'll generate tries to read one of these atomics with acquire ordering,
    // and always expects to read 0x12345678. At first it tries to read the first one, and
    // then we update it to read the second one, at which point we also change the second one
    // to hold the expected value, and invalidate the first one. If stale code is read
    // it will read the first value instead, which at that point should be updated to be invalid
    let first_value = AtomicU32::new(0x12345678);
    let second_value = AtomicU32::new(0xDEADC0DE);
    let rejoin_threads = AtomicBool::new(false);

    let mut ops = dynasmrt::riscv::Assembler::new().unwrap();

    // write some code;
    dynasm!(ops
        ; .arch riscv64
        ; .align 8
        ; -> first_addr:
        ; .u64 first_value.as_ptr() as *mut u8 as _
        ; -> second_addr:
        ; .u64 second_value.as_ptr() as *mut u8 as _
    );
    let start = ops.offset();
    dynasm!(ops
        ; .arch riscv64
        ; la t0, ->first_addr
        ; la t1, ->second_addr
    );
    let edit = ops.offset();
    dynasm!(ops
        ; .arch riscv64
        ; ld t2, [t0]
        ; lwu a0, [t2]
        ; ret
    );
    let end = ops.offset();

    ops.commit().unwrap();

    std::thread::scope(|scope| {

        // start our racing threads
        let mut handles = Vec::new();
        for _ in 0 .. thread_count {

            // these get moved to each threads
            let reader = ops.reader();
            let rejoin_threads_borrow = &rejoin_threads;

            handles.push(scope.spawn(move || {

                let mut bad_results = 0usize;
                while !rejoin_threads_borrow.load(Ordering::Acquire) {

                    let buf = reader.lock();
                    let callable: extern "C" fn() -> u32 = unsafe { std::mem::transmute(buf.ptr(start)) };

                    let value = callable();
                    if value != 0x12345678 {
                        assert_eq!(value, 0xDEADC0DE, "something worse is broken");
                        bad_results += 1;
                    }
                }

                bad_results
            }));
        }

        // wait a bit
        std::thread::sleep(std::time::Duration::from_millis(10));

        // change the code back and forth to see if errors happen
        for _ in 0 .. 100 {
            ops.alter(|modifier| {
                modifier.goto(edit);

                dynasm!(modifier
                    ; .arch riscv64
                    ; ld t2, [t1]
                    ; lwu a0, [t2]
                    ; ret
                );
                modifier.check_exact(end).unwrap();

                // also change the values. ordering is relaxed as the lock of the assembler
                // guarantees that these values will be visible.
                first_value.store(0xDEADC0DE, Ordering::Relaxed);
                second_value.store(0x12345678, Ordering::Relaxed);

            }).unwrap();

            // wait a bit more
            std::thread::sleep(std::time::Duration::from_millis(10));

            // change it back
            ops.alter(|modifier| {
                modifier.goto(edit);

                dynasm!(modifier
                    ; .arch riscv64
                    ; ld t2, [t0]
                    ; lwu a0, [t2]
                    ; ret
                );
                modifier.check_exact(end).unwrap();

                // also change the values. ordering is relaxed as the lock of the assembler
                // guarantees that these values will be visible.
                first_value.store(0x12345678, Ordering::Relaxed);
                second_value.store(0xDEADC0DE, Ordering::Relaxed);

            }).unwrap();

            // wait a bit more
            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        // join our threads
        rejoin_threads.store(true, Ordering::Release);

        let errors: usize = handles.into_iter().map(|handle| handle.join().unwrap()).sum();

        assert_eq!(errors, 0, "racing threads read the wrong value");

    });
}
