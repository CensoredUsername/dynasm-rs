#![feature(proc_macro_hygiene)]
#![allow(unused_imports)]

#[macro_use]
extern crate dynasmrt;
extern crate dynasm;

fn main() {
    println!("Please execute: cargo test --no-fail-fast")
}
