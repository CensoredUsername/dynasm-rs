% Tutorial

# Introduction

Dynasm-rs is a library and syntax extension for assembling code at runtime. For the first part of the tutorial we will be examining the following example program that assembles a simple function at runtime for the x64 instruction set:

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

We will now examine this code snippet piece by piece.


```rust
use dynasmrt::{dynasm, DynasmApi, DynasmLabelApi};
```
We then link to the dynasm runtime crate. We import the `dynasmrt::dynasm!` macro which will handle all assembling.
Furthermore, the `DynasmApi` and `DynasmLabelApi` traits are loaded. These traits define the interfaces used by the `dynasm!` procedural macro to produce assembled code.

```rust
let mut ops = dynasmrt::x64::Assembler::new();
```
Of course, the machine code that will be generated will need to live somewhere. `dynasmrt::x64::Assembler` is a struct that implements the `DynasmApi` and `DynasmLabelApi` traits, provides storage for the generated machine code, handles memory permissions and provides various utilities for dynamically assembling code. It even allows assembling code in one thread while several other threads execute said code. For this example though, we will use it in the most simple use case, just assembling everything in advance and then executing it.

```rust
dynasm!(ops
    ; .arch x64
    ; ->hello:
    ; .bytes string.as_bytes()
);
```
The first invocation of the `dynasm!` macro shows of three features of dynasm. `.arch x64` is a directive that specifies the current assembling architecture. By default this is set to the compiler target architecture, but we repeat it here to show it clearly. The second line defines a global label `hello` which later can be referenced, and the third line contains another assembler directive. Assembler directives allow the assembler to perform tasks that do not involve instruction assembling like, in this case, inserting a string into the executable buffer.

```rust
let hello = ops.offset();
```
This utility function returns a value indicating the position of the current end of the machine code buffer. It can later be used to obtain a pointer to this position in the generated machine code.


```rust
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
```
The second invocation of the `dynasm!` macro contains the definition of a small function. It performs the following tasks:

```rust
; lea rcx, [->hello]
```
First, the address of the global label `->hello` is loaded using the load effective address instruction and a label memory reference.

```rust
; xor edx, edx
; mov dl, BYTE string.len() as _
```
Then the length of the string is loaded. Here the `BYTE` prefix determines the size of the immediate in the second instruction. the `as _` cast is necessary to coerce the size of the length down to the `i8` type expected of an immediate. Dynasm-rs tries to avoid performing implicit casts as this tends to hide errors.

```rust
; mov rax, QWORD print as _
; sub rsp, BYTE 0x28
; call rax
; add rsp, BYTE 0x28
```
Here, a call is made from the dynamically assembled code to the rust `print` function. Note the `QWORD` size prefix which is necessary to determine the appropriate form of the `mov` instruction to encode as `dynasm!` does not analyze the immediate expression at runtime. As this example uses the `"win64"` calling convention, the stack pointer needs to be manipulated too. (Note: the `"win64"` calling convention is used as this it is currently impossible to use the `"sysv64"` calling convention on all platforms)

```rust
; ret
```
And finally the assembled function returns, returning the return value from the `print` function in `rax` back to the caller rust code.

```rust
let buf = ops.finalize().unwrap();
```
With the assembly completed, we now finalize the `dynasmrt::x64::Assembler`, which will resolve all labels previously used and move the data into a `dynasmrt::ExecutableBuffer`. This struct, which dereferences to a `&[u8]`, wraps a buffer of readable and executable memory.

```rust
let hello_fn: extern "win64" fn() -> bool = unsafe { mem::transmute(buf.ptr(hello)) };
```
We can now get a pointer to the executable memory using the `dynasmrt::ExecutableBuffer::ptr` method, using the value obtained earlier from `ops.offset()`. We can then transmute this pointer into a function.

```rust
assert!(hello_fn());
```
And finally we can call this function, asserting that it returns true to confirm that it managed to print the encoded message!

For the people interested in the behind-the-scenes, here's what the `dynasm!` macros expand to:

```rust
fn main() {
    let mut ops = dynasmrt::x64::Assembler::new().unwrap();
    let string = "Hello World!";

    {
        ops.global_label("hello");
        ops.extend(string.as_bytes());
    };

    let hello = ops.offset();
    {
        ops.extend(b"H\x8d\r\x00\x00\x00\x00");
        ops.global_reloc("hello", 0isize, 4u8, 0u8, (4u8,));
        ops.extend(b"1\xd2\xb2");
        ops.push_i8(string.len() as _);
        ops.extend(b"H\xb8");
        ops.push_i64(print as _);
        ops.extend(b"H\x83\xec");
        ops.push_i8(0x28);
        ops.extend(b"\xff\xd0H\x83\xc4");
        ops.push_i8(0x28);
        ops.extend(b"\xc3");
    };

    let buf = ops.finalize().unwrap();

    let hello_fn: extern "win64" fn() -> bool = unsafe { mem::transmute(buf.ptr(hello)) };

    assert!(hello_fn());
}
```
As you can see, the encoding has been determined fully at compile time, and the assembly has been reduced to a series of push and extend calls.

# Advanced usage

To demonstrate some of the more advanced usage, we'll show how to rewrite a rust brainfsck interpreter to a jit compiler. The starting point is the following interpreter that can also be found [here](https://github.com/CensoredUsername/dynasm-rs/tree/master/doc/examples/bf-interpreter):

```rust
use std::io::{Read, BufRead, Write, stdin, stdout, BufReader, BufWriter};
use std::env;
use std::fs::File;

const TAPE_SIZE: usize = 30000;

struct Interpreter<'a> {
    pub input: Box<dyn BufRead + 'a>,
    pub output: Box<dyn Write + 'a>,
    pub loops: Vec<usize>,
    pub tape: [u8; TAPE_SIZE],
    pub tape_index: usize,
    pub pos: usize
}

impl<'a> Interpreter<'a> {
    fn new(input: Box<dyn BufRead + 'a>, output: Box<dyn Write + 'a>) -> Interpreter<'a> {
        Interpreter {
            input: input,
            output: output,
            loops: Vec::new(),
            tape: [0; TAPE_SIZE],
            tape_index: 0,
            pos: 0
        }
    }

    fn run(&mut self, program: &[u8]) -> Result<(), &'static str> {
        while let Some(&c) = program.get(self.pos) {
            self.pos += 1;

            match c {
                b'<' => {
                    let amount = count_leading_chars(&program[self.pos..], b'<');
                    self.pos += amount;

                    self.tape_index = self.tape_index.wrapping_sub(amount + 1);
                    while self.tape_index >= TAPE_SIZE {
                        self.tape_index = self.tape_index.wrapping_add(TAPE_SIZE);
                    }
                },
                b'>' => {
                    let amount = count_leading_chars(&program[self.pos..], b'>');
                    self.pos += amount;

                    self.tape_index += amount + 1;
                    while self.tape_index >= TAPE_SIZE {
                        self.tape_index -= TAPE_SIZE;
                    }
                },
                b'+' => {
                    let amount = count_leading_chars(&program[self.pos..], b'+');
                    self.pos += amount;
                    if let Some(a) = self.tape[self.tape_index].checked_add(amount as u8 + 1) {
                        self.tape[self.tape_index] = a;
                    } else {
                        return Err("An overflow occurred");
                    }
                },
                b'-' => {
                    let amount = count_leading_chars(&program[self.pos..], b'-');
                    self.pos += amount;
                    if let Some(a) = self.tape[self.tape_index].checked_sub(amount as u8 + 1) {
                        self.tape[self.tape_index] = a;
                    } else {
                        return Err("An overflow occurred");
                    }
                },
                b',' => {
                    let err = self.output.flush().is_err();
                    if self.input.read_exact(&mut self.tape[self.tape_index..self.tape_index + 1]).is_err() || err {
                        return Err("IO error");
                    }
                },
                b'.' => {
                    if self.output.write_all(&self.tape[self.tape_index..self.tape_index + 1]).is_err() {
                        return Err("IO error");
                    }
                },
                b'[' => {
                    if self.tape[self.tape_index] == 0 {
                        let mut nesting = 1;
                        let amount = program[self.pos..].iter().take_while(|x| match **x {
                            b'[' => {nesting += 1; true},
                            b']' => {nesting -= 1; nesting != 0},
                            _ => true
                        }).count() + 1;
                        if nesting != 0 {
                            return Err("[ without matching ]");
                        }
                        self.pos += amount;
                    } else {
                        self.loops.push(self.pos);
                    }
                },
                b']' => {
                    if self.tape[self.tape_index] == 0 {
                        self.loops.pop();
                    } else if let Some(&loc) = self.loops.last() {
                        self.pos = loc;
                    } else {
                        return Err("] without matching [");
                    }
                },
                _ => ()
            }
        }

        if self.loops.len() != 0 {
            return Err("[ without matching ]");
        }
        Ok(())
    }
}

fn count_leading_chars(program: &[u8], c: u8) -> usize {
    program.iter().take_while(|x| **x == c).count()
}

fn main() {
    let mut args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Expected 1 argument, got {}", args.len());
        return;
    }
    let path = args.pop().unwrap();

    let mut f = if let Ok(f) = File::open(&path) { f } else {
        println!("Could not open file {}", path);
        return;
    };

    let mut buf = Vec::new();
    if let Err(_) = f.read_to_end(&mut buf) {
        println!("Failed to read from file");
        return;
    }

    let mut interp = Interpreter::new(
        Box::new(BufReader::new(stdin())), 
        Box::new(BufWriter::new(stdout()))
    );
    if let Err(e) = interp.run(&buf) {
        println!("{}", e);
    }
}
```

## Basics

To kickstart this process, we'll `use` the `dynasm!` macro as well as the `DynasmApi` and `DynasmLabelApi` traits:

```diffnew
+ use dynasmrt::{dynasm, DynasmApi, DynasmLabelApi};
```

Next, we'll define our own custom dynasm macro which first defines the current architecture as well as several aliases we'd like to use when writing the rest of the code.

```diffnew
+ macro_rules! my_dynasm {
+     ($ops:ident $($t:tt)*) => {
+         dynasm!($ops
+             ; .arch x64
+             ; .alias a_state, rcx
+             ; .alias a_current, rdx
+             ; .alias a_begin, r8
+             ; .alias a_end, r9
+             ; .alias retval, rax
+             $($t)*
+         )
+     }
+ }
```

We can now define several utility macros to handle common operations in the code.

```diffnew
+ macro_rules! prologue {
+     ($ops:ident) => {{
+         let start = $ops.offset();
+         my_dynasm!($ops
+             ; sub rsp, 0x28
+             ; mov [rsp + 0x30], rcx
+             ; mov [rsp + 0x40], r8
+             ; mov [rsp + 0x48], r9
+         );
+         start
+     }};
+ }
+ 
+ macro_rules! epilogue {
+     ($ops:ident, $e:expr) => {my_dynasm!($ops
+         ; mov retval, $e
+         ; add rsp, 0x28
+         ; ret
+     );};
+ }
+ 
+ macro_rules! call_extern {
+     ($ops:ident, $addr:expr) => {my_dynasm!($ops
+         ; mov [rsp + 0x38], rdx
+         ; mov rax, QWORD $addr as _
+         ; call rax
+         ; mov rcx, [rsp + 0x30]
+         ; mov rdx, [rsp + 0x38]
+         ; mov r8,  [rsp + 0x40]
+         ; mov r9,  [rsp + 0x48]
+     );};
+ }
```

## State

Looking at the state of the interpreter, it contains a lot of fields that the jit compiler doesn't need. We therefore reduce the state and we define a second struct which will hold the compiled machine code:

```diffold
- struct Interpreter<'a> {
-     pub input: Box<BufRead + 'a>,
-     pub output: Box<Write + 'a>,
-     pub loops: Vec<usize>,
-     pub tape: [u8; TAPE_SIZE],
-     pub tape_index: usize,
-     pub pos: usize
- }
```
```diffnew
+ struct State<'a> {
+     pub input: Box<BufRead + 'a>,
+     pub output: Box<Write + 'a>,
+     tape: [u8; TAPE_SIZE]
+ }
+ 
+ struct Program {
+     code: dynasmrt::ExecutableBuffer,
+     start: dynasmrt::AssemblyOffset,
+ }
```

## Compiler

With the state defined, we can now start adapting the `Interpreter::run` method into `Program::compile`, starting with initalization code. We initialize an assembler and a `Vec` to hold the loops stack. As we will only parse through the code once, replace the slice indexing by an iterator that allows us to look ahead, courtesy of the `itertools` crate. We get the starting offset and emit the function prologue and finally we start iterating through the program:

```diffold
- impl<'a> Interpreter<'a> {
-     fn new(input: Box<BufRead + 'a>, output: Box<Write + 'a>) -> Interpreter<'a> {
-         Interpreter {
-             input: input,
-             output: output,
-             loops: Vec::new(),
-             tape: [0; TAPE_SIZE],
-             tape_index: 0,
-             pos: 0
-         }
-     }
- 
-     fn run(&mut self, program: &[u8]) -> Result<(), &'static str> {
-         while let Some(&c) = program.get(self.pos) {
-             self.pos += 1;
```
```diffnew
+ impl Program {
+     fn compile(program: &[u8]) -> Result<Program, &'static str> {
+         let mut ops = dynasmrt::x64::Assembler::new().unwrap();
+         let mut loops = Vec::new();
+         let mut code = multipeek(program.iter().cloned());
+ 
+         let start = prologue!(ops);
+ 
+         while let Some(c) = code.next() {
```

### Tape movement

We can now replace the the tape movement by an optimized assembly version. Note the local label used to implement tape wraparound:

```diffold
- let amount = count_leading_chars(&program[self.pos..], b'<');
- self.pos += amount;
- 
- self.tape_index = self.tape_index.wrapping_sub(amount + 1);
- while self.tape_index >= TAPE_SIZE {
-     self.tape_index = self.tape_index.wrapping_add(TAPE_SIZE);
- }
```
```diffnew
+ let amount = code.take_while_ref(|x| *x == b'<').count() + 1;
+ my_dynasm!(ops
+     ; sub a_current, (amount % TAPE_SIZE) as _
+     ; cmp a_current, a_begin
+     ; jae >wrap
+     ; add a_current, TAPE_SIZE as _
+     ;wrap:
+ );
```

```diffold
- let amount = count_leading_chars(&program[self.pos..], b'>');
- self.pos += amount;
- self.tape_index += amount + 1;
- while self.tape_index >= TAPE_SIZE {
-     self.tape_index -= TAPE_SIZE;
- }
```
```diffnew
+ let amount = code.take_while_ref(|x| *x == b'>').count() + 1;
+ my_dynasm!(ops
+     ; add a_current, (amount % TAPE_SIZE) as _
+     ; cmp a_current, a_end
+     ; jb >wrap
+     ; sub a_current, TAPE_SIZE as _
+     ;wrap:
+ );
```
### Arithmetric

The `+` and `-` instructions have by far the most simple implementations. Note that when an overflow occurs at runtime, we jump to the `->overflow:` global label (Note: overflow is somewhat ill-defined in brainfsck, but in the spirit of rust I decided that everything that can be checked should be checked):

```diffold
- let amount = count_leading_chars(&program[self.pos..], b'+');
- self.pos += amount;
- if let Some(a) = self.tape[self.tape_index].checked_add(amount as u8 + 1) {
-     self.tape[self.tape_index] = a;
- } else {
-     return Err("An overflow occurred");
- }
```
```diffnew
+ let amount = code.take_while_ref(|x| *x == b'+').count() + 1;
+ if amount > u8::MAX as usize {
+     return Err("An overflow occurred");
+ }
+ my_dynasm!(ops
+     ; add BYTE [a_current], amount as _
+     ; jo ->overflow
+ );
```

```diffold
- let amount = count_leading_chars(&program[self.pos..], b'-');
- self.pos += amount;
- if let Some(a) = self.tape[self.tape_index].checked_sub(amount as u8 + 1) {
-     self.tape[self.tape_index] = a;
- } else {
-     return Err("An overflow occurred");
- }
```
```diffnew
+ let amount = code.take_while_ref(|x| *x == b'-').count() + 1;
+ if amount > u8::MAX as usize {
+     return Err("An overflow occurred");
+ }
+ my_dynasm!(ops
+     ; sub BYTE [a_current], amount as _
+     ; jo ->overflow
+ );
```

### I/O

As the input and output fields of state are implemented as trait objects, we need to handle the virtual call to them in rust code. Therefore, we first define the following wrapper methods on `State`:

```diffnew
impl<'a> State<'a> {
    unsafe extern "win64" fn getchar(state: *mut State, cell: *mut u8) -> u8 {
        let state = &mut *state;
        let err = state.output.flush().is_err();
        (state.input.read_exact(slice::from_raw_parts_mut(cell, 1)).is_err() || err) as u8
    }

    unsafe extern "win64" fn putchar(state: *mut State, cell: *mut u8) -> u8 {
        let state = &mut *state;
        state.output.write_all(slice::from_raw_parts(cell, 1)).is_err() as u8
    }
```

We can then simply call these methods directly from the compiled code. If the I/O functions failed, we jump to the `->io_failure` global label:

```diffold
- let err = self.output.flush().is_err();
- if self.input.read_exact(&mut self.tape[self.tape_index..self.tape_index + 1]).is_err() || err {
-     return Err("IO error");
- }
```
```diffnew
+ my_dynasm!(ops
+     ;; call_extern!(ops, State::getchar)
+     ; cmp al, 0
+     ; jnz ->io_failure
+ );
```

```diffold
- if self.output.write_all(&self.tape[self.tape_index..self.tape_index + 1]).is_err() {
-     return Err("IO error");
- }
```
```diffnew
+ my_dynasm!(ops
+     ;; call_extern!(ops, State::putchar)
+     ; cmp al, 0
+     ; jnz ->io_failure
+ );
```

### Loops

The `[` and `]` commands have the most complex implementation. When a `[` is encountered, we need to declare two dynamic labels. One to jump to after the `]` when the current tape value is 0 and another to jump from the `]` to just after the `[` when the current tape value is not 0. Additionally, we special case the sequence `[-]` which is often used to set the current tape value to 0 and emit optimized machine code for it.

```diffold
- if self.tape[self.tape_index] == 0 {
-     let mut nesting = 1;
-     let amount = program[self.pos..].iter().take_while(|x| match **x {
-         b'[' => {nesting += 1; true},
-         b']' => {nesting -= 1; nesting != 0},
-         _ => true
-     }).count() + 1;
-     if nesting != 0 {
-         return Err("[ without matching ]");
-     }
-     self.pos += amount;
- } else {
-     self.loops.push(self.pos);
- }
```
```diffnew
+ let first = code.peek() == Some(&b'-');
+ if first && code.peek() == Some(&b']') {
+     code.next();
+     code.next();
+     my_dynasm!(ops
+         ; mov BYTE [a_current], 0
+     );
+ } else {
+     let backward_label = ops.new_dynamic_label();
+     let forward_label  = ops.new_dynamic_label();
+     loops.push((backward_label, forward_label));
+     my_dynasm!(ops
+         ; cmp BYTE [a_current], 0
+         ; jz =>forward_label
+         ;=>backward_label
+     );
+ }
```

```diffold
- if self.tape[self.tape_index] == 0 {
-     self.loops.pop();
- } else if let Some(&loc) = self.loops.last() {
-     self.pos = loc;
- } else {
-     return Err("] without matching [");
- }
```
```diffnew
+ if let Some((backward_label, forward_label)) = loops.pop() {
+     my_dynasm!(ops
+         ; cmp BYTE [a_current], 0
+         ; jnz =>backward_label
+         ;=>forward_label
+     );
+ } else {
+     return Err("] without matching [");
+ },
```

### Epilogue

With the end of the parsing reached, we must now handle the return and possible error conditions. This is done by returning 0 if the execution was successful, or an error code when an error happened at runtime.

```diffnew
+ my_dynasm!(ops
+     ;; epilogue!(ops, 0)
+     ;->overflow:
+     ;; epilogue!(ops, 1)
+     ;->io_failure:
+     ;; epilogue!(ops, 2)
+ );
```

Now we can finalize the assembler and construct a Program from the resulting buffer:

```diffold
- Ok(())
```
```diffnew
+ let code = ops.finalize().unwrap();
+ Ok(Program {
+     code: code,
+     start: start
+ })
```

## Calling the compiled code

First, we add an initializer method to `State`:

```diffnew
+ fn new(input: Box<BufRead + 'a>, output: Box<Write + 'a>) -> State<'a> {
+     State {
+         input: input,
+         output: output,
+         tape: [0; TAPE_SIZE]
+     }
+ }
```

Then, we add a `run` method on `Program`. This method does the following:

- Transmute a pointer to the start of our compiled data to a function. This is the one `unsafe` block always needed when using Dynasm-rs, but it is probably the most dangerous one you'll ever find.
- Create the input arguments to the function from a `State`. Since the `"win64"` calling convention is used these arguments will end up in the registers `rcx`, `rdx`, `r8` and `r9`.
- Run the function.
- Return a `Result` based on the error code returned by the function.

```diffnew
+ fn run(self, state: &mut State) -> Result<(), &'static str> {
+     let f: extern "win64" fn(*mut State, *mut u8, *mut u8, *const u8) -> u8 = unsafe {
+         mem::transmute(self.code.ptr(self.start))
+     };
+     let start = state.tape.as_mut_ptr();
+     let end = unsafe { start.offset(TAPE_SIZE as isize) };
+     let res = f(state, start, start, end);
+     if res == 0 {
+         Ok(())
+     } else if res == 1 {
+         Err("An overflow occurred")
+     } else if res == 2 {
+         Err("IO error")
+     } else {
+         panic!("Unknown error code");
+     }
+ }
```

And finally, we can edit the `main` function to use the JIT:

```diffold
- let mut interp = Interpreter::new(
-     Box::new(BufReader::new(stdin())), 
-     Box::new(BufWriter::new(stdout()))
- );
- if let Err(e) = interp.run(&buf) {
-     println!("{}", e);
- }
```
```diffnew
+ let mut state = State::new(
+     Box::new(BufReader::new(stdin())), 
+     Box::new(BufWriter::new(stdout()))
+ );
+ let program = match Program::compile(&buf) {
+     Ok(p) => p,
+     Err(e) => {
+         println!("{}", e);
+         return;
+     }
+ };
+ if let Err(e) = program.run(&mut state) {
+     println!("{}", e);
+     return;
+ }
```

## Result

With these changes, adding the necessary `use` statements and removing unused functions, you should end up with the following code (you can also find this example [here](https://github.com/CensoredUsername/dynasm-rs/tree/master/doc/examples/bf-jit)):

```rust
use dynasmrt::{dynasm, DynasmApi, DynasmLabelApi};

use itertools::Itertools;
use itertools::multipeek;

use std::io::{Read, BufRead, Write, stdin, stdout, BufReader, BufWriter};
use std::env;
use std::fs::File;
use std::slice;
use std::mem;
use std::u8;

const TAPE_SIZE: usize = 30000;

macro_rules! my_dynasm {
    ($ops:ident $($t:tt)*) => {
        dynasm!($ops
            ; .arch x64
            ; .alias a_state, rcx
            ; .alias a_current, rdx
            ; .alias a_begin, r8
            ; .alias a_end, r9
            ; .alias retval, rax
            $($t)*
        )
    }
}

macro_rules! prologue {
    ($ops:ident) => {{
        let start = $ops.offset();
        my_dynasm!($ops
            ; sub rsp, 0x28
            ; mov [rsp + 0x30], rcx
            ; mov [rsp + 0x40], r8
            ; mov [rsp + 0x48], r9
        );
        start
    }};
}

macro_rules! epilogue {
    ($ops:ident, $e:expr) => {my_dynasm!($ops
        ; mov retval, $e
        ; add rsp, 0x28
        ; ret
    );};
}

macro_rules! call_extern {
    ($ops:ident, $addr:expr) => {my_dynasm!($ops
        ; mov [rsp + 0x38], rdx
        ; mov rax, QWORD $addr as _
        ; call rax
        ; mov rcx, [rsp + 0x30]
        ; mov rdx, [rsp + 0x38]
        ; mov r8,  [rsp + 0x40]
        ; mov r9,  [rsp + 0x48]
    );};
}

struct State<'a> {
    pub input: Box<dyn BufRead + 'a>,
    pub output: Box<dyn Write + 'a>,
    tape: [u8; TAPE_SIZE],
}

struct Program {
    code: dynasmrt::ExecutableBuffer,
    start: dynasmrt::AssemblyOffset,
}


impl Program {
    fn compile(program: &[u8]) -> Result<Program, &'static str> {
        let mut ops = dynasmrt::x64::Assembler::new().unwrap();
        let mut loops = Vec::new();
        let mut code = multipeek(program.iter().cloned());

        let start = prologue!(ops);

        while let Some(c) = code.next() {
            match c {
                b'<' => {
                    let amount = code.take_while_ref(|x| *x == b'<').count() + 1;
                    my_dynasm!(ops
                        ; sub a_current, (amount % TAPE_SIZE) as _
                        ; cmp a_current, a_begin
                        ; jae >wrap
                        ; add a_current, TAPE_SIZE as _
                        ;wrap:
                    );
                }
                b'>' => {
                    let amount = code.take_while_ref(|x| *x == b'>').count() + 1;
                    my_dynasm!(ops
                        ; add a_current, (amount % TAPE_SIZE) as _
                        ; cmp a_current, a_end
                        ; jb >wrap
                        ; sub a_current, TAPE_SIZE as _
                        ;wrap:
                    );
                },
                b'+' => {
                    let amount = code.take_while_ref(|x| *x == b'+').count() + 1;
                    if amount > u8::MAX as usize {
                        return Err("An overflow occurred");
                    }
                    my_dynasm!(ops
                        ; add BYTE [a_current], amount as _
                        ; jo ->overflow
                    );
                },
                b'-' => {
                    let amount = code.take_while_ref(|x| *x == b'-').count() + 1;
                    if amount > u8::MAX as usize {
                        return Err("An overflow occurred");
                    }
                    my_dynasm!(ops
                        ; sub BYTE [a_current], amount as _
                        ; jo ->overflow
                    );
                },
                b',' => {
                    my_dynasm!(ops
                        ;; call_extern!(ops, State::getchar)
                        ; cmp al, 0
                        ; jnz ->io_failure
                    );
                },
                b'.' => {
                    my_dynasm!(ops
                        ;; call_extern!(ops, State::putchar)
                        ; cmp al, 0
                        ; jnz ->io_failure
                    );
                },
                b'[' => {
                    let first = code.peek() == Some(&b'-');
                    if first && code.peek() == Some(&b']') {
                        code.next();
                        code.next();
                        my_dynasm!(ops
                            ; mov BYTE [a_current], 0
                        );
                    } else {
                        let backward_label = ops.new_dynamic_label();
                        let forward_label = ops.new_dynamic_label();
                        loops.push((backward_label, forward_label));
                        my_dynasm!(ops
                            ; cmp BYTE [a_current], 0
                            ; jz =>forward_label
                            ;=>backward_label
                        );
                    }
                },
                b']' => {
                    if let Some((backward_label, forward_label)) = loops.pop() {
                        my_dynasm!(ops
                            ; cmp BYTE [a_current], 0
                            ; jnz =>backward_label
                            ;=>forward_label
                        );
                    } else {
                        return Err("] without matching [");
                    }
                },
                _ => (),
            }
        }
        if loops.len() != 0 {
            return Err("[ without matching ]");
        }

        my_dynasm!(ops
            ;; epilogue!(ops, 0)
            ;->overflow:
            ;; epilogue!(ops, 1)
            ;->io_failure:
            ;; epilogue!(ops, 2)
        );

        let code = ops.finalize().unwrap();
        Ok(Program {
            code: code,
            start: start,
        })
    }

    fn run(self, state: &mut State) -> Result<(), &'static str> {
        let f: extern "win64" fn(*mut State, *mut u8, *mut u8, *const u8) -> u8 =
            unsafe { mem::transmute(self.code.ptr(self.start)) };
        let start = state.tape.as_mut_ptr();
        let end = unsafe { start.offset(TAPE_SIZE as isize) };
        let res = f(state, start, start, end);
        if res == 0 {
            Ok(())
        } else if res == 1 {
            Err("An overflow occurred")
        } else if res == 2 {
            Err("IO error")
        } else {
            panic!("Unknown error code");
        }
    }
}

impl<'a> State<'a> {
    unsafe extern "win64" fn getchar(state: *mut State, cell: *mut u8) -> u8 {
        let state = &mut *state;
        let err = state.output.flush().is_err();
        (state.input.read_exact(slice::from_raw_parts_mut(cell, 1)).is_err() || err) as u8
    }

    unsafe extern "win64" fn putchar(state: *mut State, cell: *mut u8) -> u8 {
        let state = &mut *state;
        state.output.write_all(slice::from_raw_parts(cell, 1)).is_err() as u8
    }

    fn new(input: Box<dyn BufRead + 'a>, output: Box<dyn Write + 'a>) -> State<'a> {
        State {
            input: input,
            output: output,
            tape: [0; TAPE_SIZE],
        }
    }
}


fn main() {
    let mut args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Expected 1 argument, got {}", args.len());
        return;
    }
    let path = args.pop().unwrap();

    let mut f = if let Ok(f) = File::open(&path) {
        f
    } else {
        println!("Could not open file {}", path);
        return;
    };

    let mut buf = Vec::new();
    if let Err(_) = f.read_to_end(&mut buf) {
        println!("Failed to read from file");
        return;
    }

    let mut state = State::new(Box::new(BufReader::new(stdin())),
                               Box::new(BufWriter::new(stdout())));
    let program = match Program::compile(&buf) {
        Ok(p) => p,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    if let Err(e) = program.run(&mut state) {
        println!("{}", e);
        return;
    }
}
```
