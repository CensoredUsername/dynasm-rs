# A Dynamic assembler written in Rust for Rust.

The purpose of this tool is to ease the creation of programs that require run-time assembling.

It is currently in alpha, meaning that while everything should work, a lot of features need to be tested

[![Build Status](https://travis-ci.org/CensoredUsername/dynasm-rs.svg?branch=master)](https://travis-ci.org/CensoredUsername/dynasm-rs)
[![](https://img.shields.io/crates/v/dynasm.svg)](https://crates.io/crates/dynasm)

`#dynasm-rs` on irc.mozilla.org

## Features

- Fully integrated in the rust toolchain, no other tools necessary.
- The assembly is optimized into a series of `Vec.push` and `Vec.extend` statements.
- Errors are almost all diagnosed at compile time in a clear fashion.
- Write the to be generated assembly inline in nasm-like syntax using a simple macro.

## Documentation

Documentation can be found [here](https://CensoredUsername.github.com/dynasm-rs/language/index.html).

## Limitations

- Currently only supports the x64 and x86 instruction sets in long and protected mode with every AMD/Intel/VIA extension except for AVX-512.

## Example

```rust
#![feature(plugin)]
#![plugin(dynasm)]

#[macro_use]
extern crate dynasmrt;

use dynasmrt::{DynasmApi, DynasmLabelApi};

use std::{io, slice, mem};
use std::io::Write;

fn main() {
    let mut ops = dynasmrt::x64::Assembler::new().unwrap();
    let string = "Hello World!";

    dynasm!(ops
        ; ->hello:
        ; .bytes string.as_bytes()
    );

    let hello = ops.offset();
    dynasm!(ops
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

    let hello_fn: extern "win64" fn() -> bool = unsafe {
        mem::transmute(buf.ptr(hello))
    };

    assert!(
        hello_fn()
    );
}

pub extern "win64" fn print(buffer: *const u8, length: u64) -> bool {
    io::stdout().write_all(unsafe {
        slice::from_raw_parts(buffer, length as usize)
    }).is_ok()
}
```

## Background

This project is heavily inspired by [Dynasm](http://luajit.org/dynasm.html)

## License

Mozilla Public License, v. 2.0, see LICENSE

Copyright 2016 CensoredUsername

## Guaranteed to be working compiler versions

As this project is a compiler plugin, sometimes new compiler versions tend to break it.
Here a list is kept of what versions of the compiler are guaranteed to work with specific
dynasm-rs versions.

- `v0.2.0`: `rustc 1.27.0-nightly (ac3c2288f 2018-04-18)`
- `v0.2.1`: `rustc 1.28.0-nightly (a1d4a9503 2018-05-20)`
- `v0.2.3`: `rustc 1.31.0-nightly (96cafc53c 2018-10-09)`
