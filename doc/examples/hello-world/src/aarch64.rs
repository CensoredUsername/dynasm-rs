use dynasmrt::{dynasm, DynasmApi, DynasmLabelApi};

use std::{io, slice, mem};
use std::io::Write;

fn main() {
    let mut ops = dynasmrt::aarch64::Assembler::new().unwrap();
    let string = "Hello World!";

    dynasm!(ops
        ; .arch aarch64
        ; ->hello:
        ; .bytes string.as_bytes()
        ; .align 4
        ; ->print:
        ; .qword print as _
    );

    let hello = ops.offset();
    dynasm!(ops
        ; .arch aarch64
        ; adr x0, ->hello
        ; movz x1, string.len() as u32
        ; ldr x9, ->print
        ; str x30, [sp, #-16]!
        ; blr x9
        ; ldr x30, [sp], #16
        ; ret
    );

    let buf = ops.finalize().unwrap();

    let hello_fn: extern "C" fn() -> bool = unsafe { mem::transmute(buf.ptr(hello)) };

    assert!(hello_fn());
}

pub extern "C" fn print(buffer: *const u8, length: u64) -> bool {
    io::stdout()
        .write_all(unsafe { slice::from_raw_parts(buffer, length as usize) })
        .is_ok()
}
