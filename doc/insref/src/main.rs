#![feature(plugin)]
#![plugin(dynasm)]

use std::io::{self, Write};

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    stdout.write_all(dynasm_opmap!().as_bytes()).unwrap();
}
