# A [Dynasm](http://luajit.org/dynasm.html)-like tool written in Rust for Rust.

The purpose of this tool is to ease the creation of programs that require run-time assembling.

It is currently pre-alpha, a lot of testing is still necessary

## Features

- Fully integrated in the rust toolchain, no other tools necessary.
- The assembly is optimized into a series of Vec<u8>.push statements for high performance.
- Errors are fully diagnosed at compile time in a clear fashion.
- Write the to be generated assembly inline in nasm-like syntax using a simple macro:

```rust
    let ops = dynasmrt::Assembler::new();
    let d = 1;
    let c = -5;
    dynasm!(ops
        ;     jmp >test
        ;     mov DWORD [rax], 1
        ;     mov rax, QWORD -1
        ;     mov BYTE [rax + rax + rcx], 1
        ; test:
        ;     mov BYTE [9*r15], 1
        ;     fs imul sp, WORD [r8 * 2 + rcx + 0x77], 0x77
        ;     mov QWORD [rax * 2 + rbx + c + d], 1
    );
    ops.finalize().unwrap()
```

## Limitations

- Currently only supports x64 long mode (with several extensions)
- No documentation

## License

Mozilla Public License, v. 2.0, see LICENSE

Copyright 2016 CensoredUsername
