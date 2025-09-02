#![allow(unused_imports)]

use dynasmrt::dynasm;
use dynasmrt::DynasmApi;

// basic dynamic register usage
#[test]
fn bugreport_1() {
    let mut ops = dynasmrt::x64::Assembler::new().unwrap();
    dynasm!(ops
       ; .arch x64
       ; int 3
       ; mov Rq(8), rdi
       ; add Rq(8), 1
       ; mov rax, Rq(8)
       ; ret
    );
    let buf = ops.finalize().unwrap();
    let hex: Vec<String> = buf.iter().map(|x| format!("0x{:02X}", *x)).collect();
    let hex: String = hex.join(", ");
    assert_eq!(hex, "0xCD, 0x03, 0x49, 0x89, 0xF8, 0x49, 0x83, 0xC0, 0x01, 0x4C, 0x89, 0xC0, 0xC3", "bugreport_1");
}

// ensure RBP/RSP can be used as dynamic base register by always emitting the full SIB byte and a displacement
#[test]
fn bugreport_2() {
    let mut ops = dynasmrt::x64::Assembler::new().unwrap();
    dynasm!(ops
       ; .arch x64
       ; inc [rsp]
       ; inc [Rq(4)]
       ; inc [Rq(4) + 1]
       ; inc [4 * rdx + Rq(4) + 1]
       ; inc [rbp]
       ; inc [Rq(5)]
       ; inc [Rq(5) + 1]
       ; inc [4 * rdx + Rq(5) + 1]
       ; inc [r12]
       ; inc [Rq(12)]
       ; inc [Rq(12) + 1]
       ; inc [4 * rdx + Rq(12) + 1]
       ; inc [r13]
       ; inc [Rq(13)]
       ; inc [Rq(13) + 1]
       ; inc [4 * rdx + Rq(13) + 1]
       ; inc [rcx]
       ; inc [Rq(1)]
       ; inc [Rq(1) + 1]
       ; inc [4 * rdx + Rq(1) + 1]
    );
    let buf = ops.finalize().unwrap();
    let hex: Vec<String> = buf.iter().map(|x| format!("0x{:02X}", *x)).collect();
    let hex: String = hex.join(", ");
    assert_eq!(hex, "0xFE, 0x04, 0x24, 0x40, 0xFE, 0x44, 0x24, 0x00, 0x40, 0xFE, 0x44, 0x24, 0x01, 0x40, 0xFE, 0x44, 0x94, 0x01, 0xFE, 0x45, 0x00, 0x40, 0xFE, 0x44, 0x25, 0x00, 0x40, 0xFE, 0x44, 0x25, 0x01, 0x40, 0xFE, 0x44, 0x95, 0x01, 0x41, 0xFE, 0x04, 0x24, 0x41, 0xFE, 0x44, 0x24, 0x00, 0x41, 0xFE, 0x44, 0x24, 0x01, 0x41, 0xFE, 0x44, 0x94, 0x01, 0x41, 0xFE, 0x45, 0x00, 0x41, 0xFE, 0x44, 0x25, 0x00, 0x41, 0xFE, 0x44, 0x25, 0x01, 0x41, 0xFE, 0x44, 0x95, 0x01, 0xFE, 0x01, 0x40, 0xFE, 0x44, 0x21, 0x00, 0x40, 0xFE, 0x44, 0x21, 0x01, 0x40, 0xFE, 0x44, 0x91, 0x01", "bugreport_2");
}

// ensure dynamic registers work correctly with VEX ops
#[test]
fn bugreport_3() {
    let mut ops = dynasmrt::x64::Assembler::new().unwrap();
    dynasm!(ops
       ; .arch x64
       ; vaddsd Rx(1), Rx(2), Rx(3)
       ; vaddsd Rx(10), Rx(9), Rx(11)
    );
    let buf = ops.finalize().unwrap();
    let hex: Vec<String> = buf.iter().map(|x| format!("0x{:02X}", *x)).collect();
    let hex: String = hex.join(", ");
    assert_eq!(hex, "0xC4, 0xE1, 0x6B, 0x58, 0xCB, 0xC4, 0x41, 0x33, 0x58, 0xD3", "bugreport_3");
}

// overflow in logical immediate encoding
#[test]
fn bugreport_4() {
    let mut ops = dynasmrt::VecAssembler::<dynasmrt::aarch64::Aarch64Relocation>::new(0);
    dynasm!(ops
        ; .arch aarch64
        ; and w0, w0, 255
    );
}

// Precedence issue around typemapped operands due to proc_macro2::Delimiter::None being broken.
#[test]
fn bugreport_5() {
    #[allow(dead_code)]
    struct Test {
        a: u32,
        b: u32
    }

    let mut ops = dynasmrt::x64::Assembler::new().unwrap();
    dynasm!(ops
       ; .arch x64
       ; mov rbx => Test[2 + 1].b, rax
    );

    let buf = ops.finalize().unwrap();
    let hex: Vec<String> = buf.iter().map(|x| format!("0x{:02X}", *x)).collect();
    let hex: String = hex.join(", ");
    assert_eq!(hex, "0x48, 0x89, 0x83, 0x1C, 0x00, 0x00, 0x00", "bugreport_5");
}

// Bad sizing of constant immediates in x64 mode
#[test]
fn bugreport_6() {
    let mut ops = dynasmrt::x64::Assembler::new().unwrap();
    dynasm!(ops
       ; .arch x64
       ; lea rax, [rbx + 1]
       ; lea rax, [rbx + 0x80]
       ; lea rax, [rbx - 1]
       ; lea rax, [rbx + -1]
       ; lea rax, [rbx - 0x81]
    );

    let buf = ops.finalize().unwrap();
    let hex: Vec<String> = buf.iter().map(|x| format!("0x{:02X}", *x)).collect();
    let hex: String = hex.join(", ");
    assert_eq!(hex, "0x48, 0x8D, 0x43, 0x01, 0x48, 0x8D, 0x83, 0x80, 0x00, 0x00, 0x00, 0x48, 0x8D, 0x43, 0xFF, 0x48, 0x8D, 0x43, 0xFF, 0x48, 0x8D, 0x83, 0x7F, 0xFF, 0xFF, 0xFF", "bugreport_6");
}


#[test]
fn rustc_does_not_properly_respect_macro_expr_grouping_for_precedence() {
    // the issue here is that code for emitting dynamic registers ends up emitting code like this
    // let _dyn_reg: u8 = #expr.into();
    // now unfortunately rustc doesn't properly handle delimitation of macro operands when reassembling tokenstreams
    // so in this test case, this could accidentally get emitted as
    // let _dyn_reg: u8 = Test(1) + Test(2).into(); which is a compilation error over the intended
    // let _dyn_reg: u8 = (Test(1) + Test(2)).into();
    // we ought to guard against this properly, so that's what this test is for.
    let mut ops = dynasmrt::SimpleAssembler::new();

    #[derive(Clone, Copy)]
    struct Test(u8);

    impl std::convert::From<Test> for u8 {
        fn from(value: Test) -> u8 {
            value.0
        }
    }

    impl std::ops::Add<Test> for Test {
        type Output = Test;
        fn add(self, rhs: Test) -> Test {
            Test(self.0 + rhs.0)
        }
    }

    dynasm!(ops
        ; .arch x64
        ; add Rq(Test(1) + Test(2)), rax
        ; .arch aarch64
        ; add x5, X(Test(1) + Test(2)), x4
        ; .arch riscv64
        ; add x27, X(Test(1) + Test(2)), x13
    );


}
