#![feature(plugin)]
#![plugin(dynasm)]

#[macro_use]
extern crate dynasmrt;
use dynasmrt::DynasmApi;

extern crate itertools;
use itertools::Itertools;

use std::io::{Read, BufRead, Write, stdin, stdout, BufReader, BufWriter};
use std::env;
use std::fs::File;
use std::slice;
use std::mem;

const TAPE_SIZE: usize = 30000;

dynasm!(ops
    ; .alias a_state, rcx
    ; .alias a_current, rdx
    ; .alias a_begin, r8
    ; .alias a_end, r9
    ; .alias retval, rax
);

macro_rules! prologue {
    ($ops:ident) => {{
        let start = $ops.offset();
        dynasm!($ops
            ; sub rsp, 0x28
            ; mov [rsp + 0x30], rcx
            ; mov [rsp + 0x40], r8
            ; mov [rsp + 0x48], r9
        );
        start
    }};
}

macro_rules! epilogue {
    ($ops:ident, $e:expr) => {dynasm!($ops
        ; mov retval, $e
        ; add rsp, 0x28
        ; ret
    );};
}

macro_rules! call_extern {
    ($ops:ident) => {dynasm!($ops
        ; mov [rsp + 0x38], rdx
        ; call rax
        ; mov rcx, [rsp + 0x30]
        ; mov rdx, [rsp + 0x38]
        ; mov r8,  [rsp + 0x40]
        ; mov r9,  [rsp + 0x48]
    );};
    ($ops:ident, $addr:expr) => {dynasm!($ops
        ; mov [rsp + 0x38], rdx
        ; mov rax, QWORD $addr as _
        ; call rax
        ; mov rcx, [rsp + 0x30]
        ; mov rdx, [rsp + 0x38]
        ; mov r8,  [rsp + 0x40]
        ; mov r9,  [rsp + 0x48]
    );};
    ($ops:ident, $addr:expr, $a1:expr) => {dynasm!($ops
        ; mov [rsp + 0x38], rdx
        ; mov rax, QWORD $addr as _
        ; mov rcx, $a1
        ; call rax
        ; mov rcx, [rsp + 0x30]
        ; mov rdx, [rsp + 0x38]
        ; mov r8,  [rsp + 0x40]
        ; mov r9,  [rsp + 0x48]
    );};
    ($ops:ident, $addr:expr, $a1:expr, $a2:expr) => {dynasm!($ops
        ; mov [rsp + 0x38], rdx
        ; mov rax, QWORD $addr as _
        ; mov rcx, $a1
        ; mov rdx, $a2
        ; call rax
        ; mov rcx, [rsp + 0x30]
        ; mov rdx, [rsp + 0x38]
        ; mov r8,  [rsp + 0x40]
        ; mov r9,  [rsp + 0x48]
    );};
}

struct BfProgram {
    code: dynasmrt::ExecutableBuffer,
    start: dynasmrt::AssemblyOffset,
}

struct BfState<'a> {
    pub input: Box<BufRead + 'a>,
    pub output: Box<Write + 'a>,
    tape: [u8; TAPE_SIZE]
}

impl BfProgram {
    fn run(self, state: &mut BfState) -> Result<(), ()> {
        let f: extern "win64" fn(*mut BfState, *mut u8, *mut u8, *const u8) -> u8 = unsafe {
            mem::transmute(self.code.ptr(self.start))
        };
        let start = state.tape.as_mut_ptr();
        let end = unsafe { start.offset(TAPE_SIZE as isize) };
        if f(state, start, start, end) == 0 {
            Ok(())
        } else {
            Err(())
        }
    }

    fn compile(program: &[u8]) -> Result<BfProgram, &'static str> {
        let mut ops = dynasmrt::Assembler::new();
        let mut loops = Vec::new();
        let mut code = program.iter().cloned().multipeek();

        let start = prologue!(ops);

        while let Some(c) = code.next() {
            match c {
                b'<' => {
                    let amount = code.take_while_ref(|x| *x == b'<').count() + 1;
                    dynasm!(ops
                        ; sub a_current, (amount % TAPE_SIZE) as _
                        ; cmp a_current, a_begin
                        ; jae >wrap
                        ; add a_current, TAPE_SIZE as _
                        ;wrap:
                    );
                },
                b'>' => {
                    let amount = code.take_while_ref(|x| *x == b'>').count() + 1;
                    dynasm!(ops
                        ; add a_current, (amount % TAPE_SIZE) as _
                        ; cmp a_current, a_end
                        ; jb >wrap
                        ; sub a_current, TAPE_SIZE as _
                        ;wrap:
                    );
                },
                b'+' => {
                    let amount = code.take_while_ref(|x| *x == b'+').count() + 1;
                    dynasm!(ops
                        ; add BYTE [a_current], amount as _
                        ; jo ->error
                    );
                },
                b'-' => {
                    let amount = code.take_while_ref(|x| *x == b'-').count() + 1;
                    dynasm!(ops
                        ; sub BYTE [a_current], amount as _
                        ; jo ->error
                    );
                },
                b',' => {
                    call_extern!(ops, BfState::getchar); // we're actually passing arguments, it's just that they're already in the right regs
                },
                b'.' => {
                    call_extern!(ops, BfState::putchar); // same here.
                },
                b'[' => {
                    let first = code.peek() == Some(&b'-');
                    if first && code.peek() == Some(&b']') {
                        dynasm!(ops
                            ; mov BYTE [a_current], 0
                        );
                    } else {
                        let backward_label = ops.new_dynamic_label();
                        let forward_label  = ops.new_dynamic_label();
                        loops.push((backward_label, forward_label));
                        dynasm!(ops
                            ; cmp BYTE [a_current], 0
                            ; jz =>forward_label
                            ;=>backward_label
                        );
                    }
                },
                b']' => if let Some((backward_label, forward_label)) = loops.pop() {
                    dynasm!(ops
                        ; cmp BYTE [a_current], 0
                        ; jnz =>backward_label
                        ;=>forward_label
                    );
                } else {
                    return Err("] without matching [");
                },
                _ => ()
            }
        }
        if loops.len() != 0 {
            return Err("[ without matching ]");
        }

        epilogue!(ops, 0);

        dynasm!(ops
            ;->error:
        );
        epilogue!(ops, 1);

        let code = ops.finalize().unwrap();
        Ok(BfProgram {
            code: code,
            start: start
        })
    }
}

impl<'a> BfState<'a> {
    fn new(input: Box<BufRead + 'a>, output: Box<Write + 'a>) -> BfState<'a> {
        BfState {
            input: input,
            output: output,
            tape: [0; TAPE_SIZE]
        }
    }

    unsafe extern "win64" fn getchar(state: *mut BfState, cell: *mut u8) -> u8 {
        let state = &mut *state;
        if state.output.flush().is_err() {
            return 1;
        }
        state.input.read_exact(slice::from_raw_parts_mut(cell, 1)).is_err() as u8
    }

    unsafe extern "win64" fn putchar(state: *mut BfState, cell: *mut u8) -> u8 {
        let state = &mut *state;
        state.output.write_all(slice::from_raw_parts(cell, 1)).is_err() as u8
    }
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

    let mut state = BfState::new(
        Box::new(BufReader::new(stdin())), 
        Box::new(BufWriter::new(stdout()))
    );
    let program = BfProgram::compile(&buf).expect("An overflow occurred");
    program.run(&mut state).unwrap();
}

