extern crate itertools;

// we generate this list directly from dynasm's internals
#[allow(plugin_as_library)]
extern crate dynasm;

use dynasm::debug;
use dynasm::x64data;

use std::io::{self, Write};
use itertools::Itertools;

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    stdout.write_all(b"% Instruction Reference\n\n").unwrap();

    let mut mnemnonics: Vec<_> = x64data::mnemnonics().cloned().collect();
    mnemnonics.sort();

    for mnemnonic in mnemnonics {
        let data = x64data::get_mnemnonic_data(mnemnonic).unwrap();
        let mut formats = data.into_iter()
                              .map(|x| debug::format_opdata(mnemnonic, x))
                              .flatten()
                              .map(|x| x.replace(">>> ", ""))
                              .collect::<Vec<_>>();
        formats.sort();

        stdout.write_all(b"### ").unwrap();
        stdout.write_all(mnemnonic.as_bytes()).unwrap();
        stdout.write_all(b"\n```\n").unwrap();

        stdout.write_all(formats.join("\n").as_bytes()).unwrap();
        stdout.write_all(b"\n```\n").unwrap();
    }
}
