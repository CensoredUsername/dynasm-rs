// FIXME remove this
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]

use std::collections::HashSet;

use syn::parse;
use proc_macro_error2::emit_error;

pub mod riscvdata;
pub mod ast;
pub mod parser;
pub mod matching;
pub mod compiler;
pub mod debug;

use crate::State;
use crate::arch::{Stmt, Jump, Size};
use crate::arch::Arch;

#[cfg(feature = "dynasm_opmap")]
pub use debug::create_opmap;
#[cfg(feature = "dynasm_extract")]
pub use debug::extract_opmap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiscVTarget {
    RV32I,
    RV32E,
    RV64I,
    RV64E
}

impl RiscVTarget {
    pub fn is_embedded(&self) -> bool {
        match self {
            RiscVTarget::RV32I |
            RiscVTarget::RV64I => false,
            RiscVTarget::RV32E |
            RiscVTarget::RV64E => true,
        }
    }

    pub fn is_64_bit(&self) -> bool {
        match self {
            RiscVTarget::RV32I |
            RiscVTarget::RV32E => false,
            RiscVTarget::RV64I |
            RiscVTarget::RV64E => true,
        }
    }

    pub fn is_32_bit(&self) -> bool {
        !self.is_64_bit()
    }
}


struct Context<'a, 'b: 'a>  {
    pub state: &'a mut State <'b>,
    pub target: RiscVTarget,
    pub features: riscvdata::ExtensionFlags
}


#[derive(Clone, Debug, Default)]
pub struct ArchRiscV64I {
    features: riscvdata::ExtensionFlags
}

impl Arch for ArchRiscV64I {
    fn set_features(&mut self, features: &[syn::Ident]) {
        self.features = parse_features(features);
    }

    fn handle_static_reloc(&self, stmts: &mut Vec<Stmt>, reloc: Jump, size: Size) {
        todo!()
    }

    fn default_align(&self) -> u8 {
        0
    }

    fn compile_instruction(&self, state: &mut State, input: parse::ParseStream) -> parse::Result<()> {
        let mut ctx = Context {
            state,
            target: RiscVTarget::RV64I,
            features: self.features
        };

        compile_instruction_inner(&mut ctx, input)
    }
}


#[derive(Clone, Debug, Default)]
pub struct ArchRiscV64E {
    features: riscvdata::ExtensionFlags
}

impl Arch for ArchRiscV64E {
    fn set_features(&mut self, features: &[syn::Ident]) {
        self.features = parse_features(features);
    }

    fn handle_static_reloc(&self, stmts: &mut Vec<Stmt>, reloc: Jump, size: Size) {
        todo!()
    }

    fn default_align(&self) -> u8 {
        0
    }

    fn compile_instruction(&self, state: &mut State, input: parse::ParseStream) -> parse::Result<()> {
        let mut ctx = Context {
            state,
            target: RiscVTarget::RV64E,
            features: self.features
        };

        compile_instruction_inner(&mut ctx, input)
    }
}


#[derive(Clone, Debug, Default)]
pub struct ArchRiscV32I {
    features: riscvdata::ExtensionFlags
}

impl Arch for ArchRiscV32I {
    fn set_features(&mut self, features: &[syn::Ident]) {
        self.features = parse_features(features);
    }

    fn handle_static_reloc(&self, stmts: &mut Vec<Stmt>, reloc: Jump, size: Size) {
        todo!()
    }

    fn default_align(&self) -> u8 {
        0
    }

    fn compile_instruction(&self, state: &mut State, input: parse::ParseStream) -> parse::Result<()> {
        let mut ctx = Context {
            state,
            target: RiscVTarget::RV32I,
            features: self.features
        };

        compile_instruction_inner(&mut ctx, input)
    }
}


#[derive(Clone, Debug, Default)]
pub struct ArchRiscV32E {
    features: riscvdata::ExtensionFlags
}

impl Arch for ArchRiscV32E {
    fn set_features(&mut self, features: &[syn::Ident]) {
        self.features = parse_features(features);
    }

    fn handle_static_reloc(&self, stmts: &mut Vec<Stmt>, reloc: Jump, size: Size) {
        todo!()
    }

    fn default_align(&self) -> u8 {
        0
    }

    fn compile_instruction(&self, state: &mut State, input: parse::ParseStream) -> parse::Result<()> {
        let mut ctx = Context {
            state,
            target: RiscVTarget::RV32E,
            features: self.features
        };

        compile_instruction_inner(&mut ctx, input)
    }
}

fn compile_instruction_inner(ctx: &mut Context, input: parse::ParseStream) -> parse::Result<()> {
    let instruction = parser::parse_instruction(ctx, input)?;
    let span = instruction.span;

    let match_data = match matching::match_instruction(ctx, instruction) {
        Err(None) => return Ok(()),
        Err(Some(e)) => {
            emit_error!(span, e);
            return Ok(())
        }
        Ok(m) => m
    };

    match compiler::compile_instruction(ctx, match_data) {
        Err(None) => return Ok(()),
        Err(Some(e)) => {
            emit_error!(span, e);
            return Ok(())
        }
        Ok(()) => ()
    }

    Ok(())
}

fn parse_features(features: &[syn::Ident]) -> riscvdata::ExtensionFlags {
    // always enable the base extension
    let mut extension_flags = riscvdata::ExtensionFlags::default();

    for feature in features {
        let mut s = feature.to_string();
        s.make_ascii_lowercase();
        let span = feature.span();



        // riscv architecture specs are written like this: ABCDEFZlong_Zmore, which needs to be split
        // into A, B, C, D, E, F, Zlong, Zmore
        let mut split = HashSet::new();
        let mut long_extension = None;
        for (i, c) in s.char_indices() {
            match (long_extension, c) {
                (Some(x), '_') => {
                    split.insert(&s[x .. i]);
                    long_extension = None;
                },
                (Some(_), c) => (),
                (None, 'z') => long_extension = Some(i),
                (None, c) => {
                    split.insert(&s[i .. i + c.len_utf8()]);
                },
            }
        }
        if let Some(x) = long_extension {
            split.insert(&s[x .. ]);
        }

        // expand g to imafdzicsr_zifencei
        if split.remove(&"g") {
            split.insert("m");
            split.insert("a");
            split.insert("f");
            split.insert("d");
            split.insert("zicsr");
            split.insert("zifencei");
        }

        // many extensions
        for s in split.into_iter() {
            let flag = match s {
                "a" => riscvdata::ExtensionFlags::Ex_A,
                "c" => riscvdata::ExtensionFlags::Ex_C,
                "d" => riscvdata::ExtensionFlags::Ex_D,
                "f" => riscvdata::ExtensionFlags::Ex_F,
                "i" => riscvdata::ExtensionFlags::Ex_I,
                "m" => riscvdata::ExtensionFlags::Ex_M,
                "q" => riscvdata::ExtensionFlags::Ex_Q,
                "zabha" => riscvdata::ExtensionFlags::Ex_Zabha,
                "zacas" => riscvdata::ExtensionFlags::Ex_Zacas,
                "zawrs" => riscvdata::ExtensionFlags::Ex_Zawrs,
                "zba" => riscvdata::ExtensionFlags::Ex_Zba,
                "zbb" => riscvdata::ExtensionFlags::Ex_Zbb,
                "zbc" => riscvdata::ExtensionFlags::Ex_Zbc,
                "zbkb" => riscvdata::ExtensionFlags::Ex_Zbkb,
                "zbkc" => riscvdata::ExtensionFlags::Ex_Zbkc,
                "zbkx" => riscvdata::ExtensionFlags::Ex_Zbkx,
                "zbs" => riscvdata::ExtensionFlags::Ex_Zbs,
                "zcb" => riscvdata::ExtensionFlags::Ex_Zcb,
                "zcmop" => riscvdata::ExtensionFlags::Ex_Zcmop,
                "zcmp" => riscvdata::ExtensionFlags::Ex_Zcmp,
                "zcmt" => riscvdata::ExtensionFlags::Ex_Zcmt,
                "zfa" => riscvdata::ExtensionFlags::Ex_Zfa,
                "zfbfmin" => riscvdata::ExtensionFlags::Ex_Zfbfmin,
                "zfh" => riscvdata::ExtensionFlags::Ex_Zfh,
                "zicbom" => riscvdata::ExtensionFlags::Ex_Zicbom,
                "zicbop" => riscvdata::ExtensionFlags::Ex_Zicbop,
                "zicboz" => riscvdata::ExtensionFlags::Ex_Zicboz,
                "zicfilp" => riscvdata::ExtensionFlags::Ex_Zicfilp,
                "zicfiss" => riscvdata::ExtensionFlags::Ex_Zicfiss,
                "zicntr" => riscvdata::ExtensionFlags::Ex_Zicntr,
                "zicond" => riscvdata::ExtensionFlags::Ex_Zicond,
                "zicsr" => riscvdata::ExtensionFlags::Ex_Zicsr,
                "zifencei" => riscvdata::ExtensionFlags::Ex_Zifencei,
                "zihintntl" => riscvdata::ExtensionFlags::Ex_Zihintntl,
                "zihintpause" => riscvdata::ExtensionFlags::Ex_Zihintpause,
                "zimop" => riscvdata::ExtensionFlags::Ex_Zimop,
                "zk" => riscvdata::ExtensionFlags::Ex_Zk,
                "zkn" => riscvdata::ExtensionFlags::Ex_Zkn,
                "zknd" => riscvdata::ExtensionFlags::Ex_Zknd,
                "zkne" => riscvdata::ExtensionFlags::Ex_Zkne,
                "zknh" => riscvdata::ExtensionFlags::Ex_Zknh,
                "zks" => riscvdata::ExtensionFlags::Ex_Zks,
                "zksed" => riscvdata::ExtensionFlags::Ex_Zksed,
                "zksh" => riscvdata::ExtensionFlags::Ex_Zksh,
                x => {
                    emit_error!(span, "Unknown risc-v extension '{}'", x);
                    continue;
                }
            };

            extension_flags.insert(flag);
        }
    }

    // TODO
    // in the future we might want to handle extension dependencies / requirements here
    // but as of right now the docs are very vague about the exact way this should be handled.
    // so for now we just allow any combinations and do not add implicit dependencies

    extension_flags
}
