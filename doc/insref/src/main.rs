use std::io::{self, Write};

fn main() {
    let mut args = std::env::args();
    args.next().unwrap();

    let opmap = match args.next().expect("Architecture name").as_str() {
        "x64" => dynasm::dynasm_opmap!(x64),
        "aarch64" => dynasm::dynasm_opmap!(aarch64),
        x => panic!("Unknown opmap format '{}'", x)
    };

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    stdout.write_all(opmap.as_bytes()).unwrap();
}
