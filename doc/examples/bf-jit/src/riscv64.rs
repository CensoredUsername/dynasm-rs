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
            ; .arch riscv64
            ; .feature GC
            ; .alias a_state, a0
            ; .alias a_current, a1
            ; .alias a_begin, a2
            ; .alias a_end, a3
            ; .alias retval, a0
            $($t)*
        );
    }
}

macro_rules! prologue {
    ($ops:ident) => {{
        let start = $ops.offset();
        my_dynasm!($ops
            ; c.addi16sp sp, -48
            ; c.sdsp ra, [sp]
            ; c.sdsp a0, [sp, 8]
            ; c.sdsp a1, [sp, 16]
            ; c.sdsp a2, [sp, 24]
            ; c.sdsp a3, [sp, 32]
        );
        start
    }};
}

macro_rules! epilogue {
    ($ops:ident, $e:expr) => {my_dynasm!($ops
        ; li.12 a0, $e
        ; c.ldsp ra, [sp]
        ; c.addi16sp sp, 48
        ; ret
    );};
}

macro_rules! call_extern {
    ($ops:ident, $addr:ident) => {my_dynasm!($ops
        ; c.sdsp a1, [sp, 16]
        ; ld a4, ->$addr
        ; c.jalr a4
        ; c.mv a4, a0
        ; c.ldsp a0, [sp, 8]
        ; c.ldsp a1, [sp, 16]
        ; c.ldsp a2, [sp, 24]
        ; c.ldsp a3, [sp, 32]
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
        let mut ops = dynasmrt::riscv::Assembler::new().unwrap();
        let mut loops = Vec::new();
        let mut code = multipeek(program.iter().cloned());

        // literal pool
        dynasm!(ops
            ; .align 8
            ; ->getchar:
            ; .u64 State::getchar as _
            ; ->putchar:
            ; .u64 State::putchar as _
        );

        let start = prologue!(ops);

        while let Some(c) = code.next() {
            match c {
                b'<' => {
                    let amount = code.take_while_ref(|x| *x == b'<').count() + 1;
                    my_dynasm!(ops
                        ; li.w a4, (amount % TAPE_SIZE) as u32 as i32
                        ; c.sub a_current, a4
                        ; bgeu a_current, a_begin, >nowrap
                        ;  li.w a4, TAPE_SIZE as u32 as i32
                        ;  c.add a_current, a4
                        ; nowrap:
                    );
                },
                b'>' => {
                    let amount = code.take_while_ref(|x| *x == b'>').count() + 1;
                    my_dynasm!(ops
                        ; li.w a4, (amount % TAPE_SIZE) as u32 as i32
                        ; c.add a_current, a4
                        ; bltu a_current, a_end, >nowrap
                        ;  li.w a4, TAPE_SIZE as u32 as i32
                        ;  c.sub a_current, a4
                        ; nowrap:
                    );
                },
                b'+' => {
                    let amount = code.take_while_ref(|x| *x == b'+').count() + 1;
                    if amount > u8::MAX as usize {
                        return Err("An overflow occurred");
                    }
                    my_dynasm!(ops
                        ; lb a4, [a_current]
                        ; addi a4, a4, amount as i32
                        ; addi a5, a4, -256
                        ; bltz a5, >fine
                        ;  j ->overflow
                        ; fine:
                        ; sb a4, [a_current]
                    );
                },
                b'-' => {
                    let amount = code.take_while_ref(|x| *x == b'-').count() + 1;
                    if amount > u8::MAX as usize {
                        return Err("An overflow occurred");
                    }
                    my_dynasm!(ops
                        ; lb a4, [a_current]
                        ; addi a4, a4, -(amount as i32)
                        ; bgez a4, >fine
                        ;  j ->overflow
                        ; fine:
                        ; sb a4, [a_current]
                    );
                },
                b',' => {
                    my_dynasm!(ops
                        ;; call_extern!(ops, getchar)
                        ; c.beqz a4, >fine
                        ;  j ->io_failure
                        ; fine:
                    );
                },
                b'.' => {
                    my_dynasm!(ops
                        ;; call_extern!(ops, putchar)
                        ; c.beqz a4, >fine
                        ;  j ->io_failure
                        ; fine:
                    );
                },
                b'[' => {
                    let first = code.peek() == Some(&b'-');
                    if first && code.peek() == Some(&b']') {
                        code.next();
                        code.next();
                        my_dynasm!(ops
                            ; sb zero, [a_current]
                        );
                    } else {
                        let backward_label = ops.new_dynamic_label();
                        let forward_label = ops.new_dynamic_label();
                        loops.push((backward_label, forward_label));
                        my_dynasm!(ops
                            ; lb a4, [a_current]
                            ; c.bnez a4, =>backward_label
                            ;  j =>forward_label
                            ;=>backward_label
                        );
                    }
                },
                b']' => {
                    if let Some((backward_label, forward_label)) = loops.pop() {
                        my_dynasm!(ops
                            ; lb a4, [a_current]
                            ; c.beqz a4, =>forward_label
                            ;  j =>backward_label
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
        let f: extern "C" fn(*mut State, *mut u8, *mut u8, *const u8) -> u8 =
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
    unsafe extern "C" fn getchar(state: *mut State, cell: *mut u8) -> u8 {
        let state = &mut *state;
        let err = state.output.flush().is_err();
        (state.input.read_exact(slice::from_raw_parts_mut(cell, 1)).is_err() || err) as u8
    }

    unsafe extern "C" fn putchar(state: *mut State, cell: *mut u8) -> u8 {
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
        println!("Expected 2 argument, got {}", args.len());
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
