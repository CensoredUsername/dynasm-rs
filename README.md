# A Dynamic assembler written in Rust for Rust.

The purpose of this tool is to ease the creation of programs that require run-time assembling.

It is compatible with stable rustc 1.45 and higher.

[![Build Status](https://travis-ci.org/CensoredUsername/dynasm-rs.svg?branch=master)](https://travis-ci.org/CensoredUsername/dynasm-rs)
[![](https://img.shields.io/crates/v/dynasm.svg)](https://crates.io/crates/dynasm)

`##dynasm-rs` on irc.freenode.net

## Features

- Fully integrated in the rust toolchain, no other tools necessary.
- The assembly is optimized into a series of `Vec.push` and `Vec.extend` statements.
- Errors are almost all diagnosed at compile time in a clear fashion.
- Write the to be generated assembly inline in nasm-like syntax using a simple macro.

## Documentation

[Documentation](https://CensoredUsername.github.com/dynasm-rs/language/index.html).
[Release notes](https://github.com/CensoredUsername/dynasm-rs/blob/master/doc/releasenotes.md).

## Architecture support

- Supports the x64/x86 instruction sets in long and protected mode with every AMD/Intel/VIA extension except for AVX-512.
- Supports the aarch64 instruction set up to ARMv8.4 except for SVE instructions. The development of this assembler backend has been generously sponsored by the awesome folks at [Wasmer](https://github.com/wasmerio/wasmer)!

## Example

```rust
use dynasmrt::{dynasm, DynasmApi, DynasmLabelApi};

use std::{io, slice, mem};
use std::io::Write;

fn main() {
    let mut ops = dynasmrt::x64::Assembler::new().unwrap();
    let string = "Hello World!";

    dynasm!(ops
        ; .arch x64
        ; ->hello:
        ; .bytes string.as_bytes()
    );

    let hello = ops.offset();
    dynasm!(ops
        ; .arch x64
        ; lea rcx, [->hello]
        ; xor edx, edx
        ; mov dl, BYTE string.len() as _
        ; mov rax, QWORD print as _
        ; sub rsp, BYTE 0x28
        ; call rax
        ; add rsp, BYTE 0x28
        ; ret
    );

    let buf = ops.finalize().unwrap();

    let hello_fn: extern "win64" fn() -> bool = unsafe { mem::transmute(buf.ptr(hello)) };

    assert!(hello_fn());
}

pub extern "win64" fn print(buffer: *const u8, length: u64) -> bool {
    io::stdout()
        .write_all(unsafe { slice::from_raw_parts(buffer, length as usize) })
        .is_ok()
}
```

## Background

This project is heavily inspired by [Dynasm](http://luajit.org/dynasm.html)

## Sponsorship

The development of the Aarch64 assembler backend has been sponsored by [Wasmer](https://github.com/wasmerio/wasmer).

## License

Mozilla Public License, v. 2.0, see LICENSE

Copyright 2016 CensoredUsername

## Guaranteed to be working compiler versions

This project used to be a compiler plugin, so for old compilers, here's a list of which version of dynasm was guaranteed to work with which compiler.
As the project has since transitioned to be a proc macro, this is not relevant for modern versions of the compiler.

- `v0.2.0`: `rustc 1.27.0-nightly (ac3c2288f 2018-04-18)`
- `v0.2.1`: `rustc 1.28.0-nightly (a1d4a9503 2018-05-20)`
- `v0.2.3`: `rustc 1.31.0-nightly (96cafc53c 2018-10-09)`
