// FIXME remove this
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]

use syn::parse;
use proc_macro_error2::emit_error;

use crate::State;
use crate::arch::{Stmt, Jump, Size};
use crate::arch::Arch;

pub mod riscvdata;
pub mod ast;
pub mod parser;
pub mod matching;

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
    pub features: ()
}


#[derive(Clone, Debug, Default)]
pub struct ArchRiscV64I {
    features: ()
}

impl Arch for ArchRiscV64I {
    fn set_features(&mut self, features: &[syn::Ident]) {
        unimplemented!()
    }

    fn handle_static_reloc(&self, stmts: &mut Vec<Stmt>, reloc: Jump, size: Size) {
        unimplemented!()
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

        compile_instruction_inner(&mut ctx, input)?;

        let instruction = parser::parse_instruction(&mut ctx, input)?;

        unimplemented!();

        Ok(())
    }
}


#[derive(Clone, Debug, Default)]
pub struct ArchRiscV64E {
    features: ()
}

impl Arch for ArchRiscV64E {
    fn set_features(&mut self, features: &[syn::Ident]) {
        unimplemented!()
    }

    fn handle_static_reloc(&self, stmts: &mut Vec<Stmt>, reloc: Jump, size: Size) {
        unimplemented!()
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

        let instruction = parser::parse_instruction(&mut ctx, input)?;

        unimplemented!();

        Ok(())
    }
}


#[derive(Clone, Debug, Default)]
pub struct ArchRiscV32I {
    features: ()
}

impl Arch for ArchRiscV32I {
    fn set_features(&mut self, features: &[syn::Ident]) {
        unimplemented!()
    }

    fn handle_static_reloc(&self, stmts: &mut Vec<Stmt>, reloc: Jump, size: Size) {
        unimplemented!()
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

        let instruction = parser::parse_instruction(&mut ctx, input)?;

        unimplemented!();

        Ok(())
    }
}


#[derive(Clone, Debug, Default)]
pub struct ArchRiscV32E {
    features: ()
}

impl Arch for ArchRiscV32E {
    fn set_features(&mut self, features: &[syn::Ident]) {
        unimplemented!()
    }

    fn handle_static_reloc(&self, stmts: &mut Vec<Stmt>, reloc: Jump, size: Size) {
        unimplemented!()
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

        let instruction = parser::parse_instruction(&mut ctx, input)?;

        unimplemented!();

        Ok(())
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

    Ok(())
}