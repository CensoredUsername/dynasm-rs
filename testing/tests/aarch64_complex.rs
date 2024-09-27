#![allow(unused_imports)]

extern crate dynasmrt;

use dynasmrt::{dynasm, MutPointer};
use dynasmrt::{DynasmApi, DynasmLabelApi};
use dynasmrt::components::LitPool;

// aliases, and dynasm! in item position
macro_rules! my_dynasm {
    ($ops:ident $($t:tt)*) => {
        dynasm!($ops
            ; .arch x64
            ; .alias test, x1
            $($t)*
        )
    }
}

#[test]
fn complex() {
    let mut ops = dynasmrt::aarch64::Assembler::new().unwrap();
    let d = 3;
    let c = 4;

    let label = ops.new_dynamic_label();
    let litpool_dyn = ops.new_dynamic_label();

    let wide_integer = 0x5600000u64;
    let a_random_float = 3.625f32;
    let bitmask = 0x00FF00FFFF00FF00;
    let logical = 0x03F003F003F003F0;

    let mut litpool = LitPool::new();

    // interesting testcases
    my_dynasm!(ops
        ; .arch aarch64
        ; aligned:
        // no args
        ; nop
        // bare immediate
        ; hlt 0x324
        // arm immediate
        ; hlt #0x123
        // registers
        ; mov w1, w2
        // weird registers
        ; mov w1, wzr
        ; mov w1, wsp
        ; mov test, sp
        // dynamic registers
        ; mov w1, W(2)
        // memory references
        ; ldr x2, [x3]
        ; ldr x2, [x3, 8]
        ; ldr x2, [x3, #16]
        ; ldr x2, [x3, #16]!
        ; ldr x2, [x3], #5
        ; ldr x2, [x3, x4]
        ; ldr x2, [x5, x6, lsl 3]
        ; ldr x2, [x5, x6, lsl d]
        ; ldr x2, [x7, x8, lsl #3]
        ; ldr x2, [x9, w10, uxtw]
        ; ldr x2, [x11, w12, uxtw #3]
        // register lists
        ; ld1 {v1.b8}, [x3]
        ; ld1 {v1.b16, v2.b16}, [x3]
        ; ld1 {v1.h4, v2.h4, v3.h4}, [x3]
        ; ld1 {v1.h8, v2.h8, v3.h8, v4.h8}, [x3]
        ; ld1 {v1.s2 - v4.s2}, [x3]
        ; ld1 {v1.d2 * 4}, [x3]
        // vector lane specificers
        ; mla v1.h4, v2.h4, v2.h4[0]
        ; mla v1.h8, v2.h8, V(c).h[d]
        ; ld4 {v1.b * 4}[0], [x3]
        // "funny" immediates
        ; mov x29, 0x12340000
        ; mov x29, 0x123400000000
        ; mov x29, 0x1234000000000000
        ; mov x29, #wide_integer
        ; mov.logical x30, 0xff00ff00ff00ff00
        ; mov.logical x29, 0x5555555555555555
        ; mov.logical w28, 0xff00ff00
        ; mov.logical w27, 0x33333333
        ; mov.logical x5, #logical
        ; mov.inverted x26, 0xffff1234ffffffff
        ; mov.inverted x25, 0xffffffff5678ffff
        ; mov.inverted w24, 0x9012ffff
        ; movi v1.d2, 0xff00ff00ff00ff00
        ; movi d1, 0x00ff00ffff00ff00
        ; movi d1, #bitmask
        ; fmov s2, 0.1875
        ; fmov d3, 11.0
        ; fmov s4, a_random_float
        // ident args
        ; dsb sy
        // labels
        ; a: // local
        ; -> b: // global
        ; => label // dynamic.
        // jumps
        ; b <a
        ; b ->b
        ; b => label
        ; b.lt <a
        ; adr x1, ->b
        ; adrp x1, <aligned
        ; tbz x2, 1, ->b
        ; ldr x2, <a
        // lit pool fun
        ; adr x1, >litpool + litpool.push_u8(0xCC)
        ; ldrb w1, [x1]
        ; adr x1, >litpool + litpool.push_u8(0xEE)
        ; ldrb w1, [x1]
        ; adr x1, >litpool + litpool.push_u8(0xFF)
        ; ldrb w1, [x1]
        ; adr x1, =>(litpool_dyn) + litpool.push_u8(0x12)
        ; adr x1, =>(litpool_dyn) + litpool.push_u8(0x34)
        ; adr x1, =>(litpool_dyn) + litpool.push_u8(0x56)
        ; ldr w2, >litpool + litpool.push_u64(0x00ff00ffff00ff00)
        ; ldr w2, >litpool + litpool.push_u64(0x1111111111111111)
        ; ldr w2, >litpool + litpool.push_u64(0x2222222222222222)
        ;.align 8, 0xDD
        ;litpool:
        ;=>litpool_dyn
        ;;litpool.emit(&mut ops)
    );


    let buf = ops.finalize().unwrap();

    println!("Generated assembly:");
    for i in buf.iter() {
        print!("{:02x }", i);
    }
    println!("");
}
