#![feature(plugin)]
#![plugin(dynasm)]

#[macro_use]
extern crate dynasmrt;

fn main() {
    println!("Please execute: cargo test")
}

#[cfg(test)]
mod tests {
    use dynasmrt;
    use dynasmrt::DynasmApi;
    include!("enctest1.rs");

    // #[test]
    // fn adc0() {
    //     let mut ops = dynasmrt::x64::Assembler::new();
    //     dynasm!(ops
    //             ; adc WORD [rax], 10
    //     );
    //     let buf = ops.finalize().unwrap();
    //     let hex: Vec<String> = buf.iter().map(|x| format!("0x{:02X}", *x)).collect();
    //     let hex: String = hex.join(", ");
    //     assert_eq!(hex, "0x12, 0x10", "adc dl, [rax]");
    // }
}
