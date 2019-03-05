#![feature(proc_macro_hygiene)]
#![allow(unused_imports)]

extern crate dynasmrt;
extern crate dynasm;

use dynasm::dynasm;
use dynasmrt::DynasmApi;

#[test]
fn bugreport_1() {
     let mut ops = dynasmrt::x64::Assembler::new().unwrap();
     dynasm!(ops
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
