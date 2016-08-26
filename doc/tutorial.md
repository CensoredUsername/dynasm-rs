% Tutorial

# Introduction

Dynasm-rs is a library and sytnax extension for assembling code at runtime. For the first part of the tutorial we will be examining the following example program that assembles a simple function at runtime:

```
#![feature(plugin)]
#![plugin(dynasm)]

#[macro_use]
extern crate dynasmrt;

use dynasmrt::DynasmApi;

use std::{io, slice, mem};
use std::io::Write;

fn main() {
    let mut ops = dynasmrt::Assembler::new();
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

We will now examine this code snippet piece by piece.

```
#![feature(plugin)]
#![plugin(dynasm)]
```
To use the dynasm! procedural macro, first the dynasm plugin has to be loaded. As plugins are currently unstable, the plugin feature first needs to be enabled. This currently requires a nightly version of rustc.

```
#[macro_use]
extern crate dynasmrt;

use dynasmrt::DynasmApi;
```
We then link to the dynasm runtime crate. Although they are not used here, it also contains various utility macros which we load here.
Furthermore, the `DynasmApi` trait is loaded. This trait defines the interface used by the `dynasm!` procedural macro to produce assembled code.

```
let mut ops = dynasmrt::Assembler::new();
```
Of course, the machine code that will be generated will need to live somewhere. `dynasmrt::Assembler` is a struct that implements the `DynasmApi` trait, provides storage for the generated machine code, handles memory permissions and provides various utilities for dynamically assembling code. It even allows assembling code in one thread while several other threads execute said code. For this example though, we will use it in the most simple usecase, just assembling everything in advance and then executing it.

```
dynasm!(ops
    ; ->hello:
    ; .bytes string.as_bytes()
);
```
The first invocation of the `dynasm!` macro shows of two features of dynasm. The first line defines a global label `hello` which later can be referenced, while the second line contains an assembler directive. Assembler directives allow the assembler to perform tasks that do not involve instruction assembling like, in this case, inserting a string into the executable buffer.

```
let hello = ops.offset();
```
This utility function returns a value indicating the position of the current end of the machine code buffer. It can later be used to obtain a pointer to this position in the generated machine code.


```
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
```
The second invocation of the `dynasm!` macro contains the definition of a small function. It performs the following tasks:

```
; lea rcx, [->hello]
```
First, the address of the global label `->hello` is loaded using the load effective address instruction and a label memory reference.

```
; xor edx, edx
; mov dl, BYTE string.len() as _
```
Then the length of the string is loaded. Here the `BYTE` prefix determines the size of the immediate in the second instruction. the `as _` cast is necessary to coerce the size of the length down to the `i8` type expected of an immediate. Dynasm-rs tries to avoid performing implicit casts as this tends to hide errors.

```
; mov rax, QWORD print as _
; sub rsp, BYTE 0x28
; call rax
; add rsp, BYTE 0x28
```
Here, a call is made from the dynamically assembled code to the rust `print` function. Note the `QWORD` size prefix which is necessary to determine the appropriate form of the `mov` instruction to encode as `dynasm!` does not analyze the immediate expression at runtime. As this example uses the `"win64"` calling convention, the stack pointer needs to be manipulated too. (Note: the `"win64"` calling convention is used as this it is currently impossible to use the `"sysv64"` calling convention on all platforms)

```
; ret
```
And finally the assembled function returns, returning the return value from the `print` function in `rax` back to the caller rust code.

```
let buf = ops.finalize().unwrap();
```
With the assembly completed, we now finalize the `dynasmrt::Assembler`, which will resolve all labels previously used and move the data into a `dynasmrt::ExecutableBuffer`. This struct, which dereferences to a `&[u8]`, wraps a buffer of readable and executable memory.

```
let hello_fn: extern "win64" fn() -> bool = unsafe {
    mem::transmute(buf.ptr(hello))
};
```
We can now get a pointer to the executable memory using the `dynasmrt::ExecutableBuffer::ptr` method, using the value obtained earlier from `ops.offset()`. We can then transmute this pointer into a function.

```
assert!(
    hello_fn()
);
```
And finally we can call this function, asserting that it returns true to ensure that it managed to print the encoded message!

# Advanced usage

Coming soon.
