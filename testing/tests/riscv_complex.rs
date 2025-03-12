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
            ; .feature GCQZcmt_Zcmp_Zacas_Zfa
            $($t)*
        )
    }
}

#[test]
fn complex() {
    let mut ops = dynasmrt::riscv::Assembler::new().unwrap();
    // let d = 3i32;
    let c = 4u32;

    // interesting testcases
    my_dynasm!(ops
        ; test:
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
        // all the branches
        ; beqz x31, <test
        ; j <test
        ; c.beqz x8, <test
        ; c.j <test
        ; auipc x1, <test
        ; jalr ra, x1, <test + 4
        ; lw x2, [x1, <test + 8]
        ; sw x2, [x1, <test + 12]
        ; lw x1, <test
        ; sw x1, <test, x2
        ; la x1, <test
        ; call <test
        ; jump <test, x2
        ; tail <test
        // load immediates
        ; li.32 x4, 0x7FFFF_FFF
        ; li.43 x4, 0x3FFF_FFFF_FFF
        ; li.54 x4, 0x1FFF_FFFF_FFFF_FF
        ; li x4, 0x7FFF_FFFF_FFFF_FFFF
    );

    let buf = ops.finalize().unwrap();

    println!("Generated assembly:");
    for i in buf.iter() {
        print!("{:02x }", i);
    }
    println!("");
}
