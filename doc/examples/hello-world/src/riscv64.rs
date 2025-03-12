use dynasmrt::{dynasm, DynasmApi, DynasmLabelApi};

use std::{io, slice, mem};
use std::io::Write;

fn main() {
    let mut ops = dynasmrt::riscv::Assembler::new().unwrap();
    let string = "Hello World!";

    dynasm!(ops
        ; .arch riscv64
        ; ->hello:
        ; .bytes string.as_bytes()
        ; .align 8
        ; ->print:
        ; .u64 print as _
    );

    let hello = ops.offset();
    dynasm!(ops
        ; .arch riscv64
        ; .feature IC
        ; c.addi16sp sp, -16
        ; c.sdsp ra, [sp, 0]
        ; ld t1, ->print
        ; la a0, ->hello
        ; li.12 a1, string.len() as i32
        ; c.jalr t1
        ; c.ldsp ra, [sp, 0]
        ; c.addi16sp sp, 16
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
