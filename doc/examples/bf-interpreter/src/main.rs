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
