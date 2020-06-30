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
