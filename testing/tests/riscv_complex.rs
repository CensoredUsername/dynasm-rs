#![allow(unused_imports)]

extern crate dynasmrt;

use dynasmrt::{dynasm, MutPointer};
use dynasmrt::{DynasmApi, DynasmLabelApi};
use dynasmrt::components::LitPool;

// aliases, and dynasm! in item position
macro_rules! my_dynasm {
    ($ops:ident $($t:tt)*) => {
        dynasm!($ops
            ; .arch riscv64
            $($t)*
        )
    }
}

#[test]
fn complex() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    // let d = 3i32;
    let c = 4u32;

    // interesting testcases
    my_dynasm!(ops
        // no args
        ; nop
        // short
        ; c.nop
        // bare immediate
        ; cm.jalt 99
        // expression
        ; cm.jt c
        // registers
        ; add x1, x2, x3
        ; add X(1), X(2), X(3)
        ; fadd.q f1, f2, f3
        // weird registers
        ; add x1, x2, x0
        ; c.add x1, x2
        ; c.addw x8, x9
        ; c.lui x1, 0x1F000
        ; amocas.d x2, x4, [x6]
        ; cm.mva01s s1, s7
        // register list
        ; cm.push {ra, s0 - s6}, -80
        ; cm.push {ra; 7}, -80
        // memory references
        ; ld x6, [x7]
        ; ld x6, [x7, 0]
        ; ld x6, [x7, 512]
        ; amocas.d x2, x4, [x6]
        ; amocas.d x2, x4, [x6, 0]
        // ident arguments
        ; fence iorw, iorw
        ; fence.tso
        ; fadd.q f1, f2, f3, rdn
        ; fli.q f1, min
        ; fli.q f2, inf
        ; fli.q f3, nan
        ; fli.q f4, 1.0
        ; csrrc x1, fflags, x2
        ; csrrc x1, 1, x2

    );

    let buf = ops.finalize();

    println!("Generated assembly:");
    for i in buf.iter() {
        print!("{:02x }", i);
    }
    println!("");
}
