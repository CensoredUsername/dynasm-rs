// FIXME remove this
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]

use syn::parse;

use crate::State;
use crate::arch::{Stmt, Jump, Size};
use crate::arch::Arch;

pub mod ast;
pub mod parser;

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

        validate_embedded_registers(&instruction)?;

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

        validate_embedded_registers(&instruction)?;

        unimplemented!();

        Ok(())
    }
}

/// Returns an error if any static register above 15 is used as those are not supported
/// on embedded RISC-V target
fn validate_embedded_registers(instruction: &ast::ParsedInstruction) -> parse::Result<()> {
    for arg in instruction.args.iter() {
        let (register, span) = match arg {
            ast::ParsedArg::Register { reg, span } => (reg, span),
            ast::ParsedArg::Reference { base, span, .. } => (base, span),
            _ => continue
        };

        if let Some(code) = register.code() {
            if code >= 16 {
                return Err(parse::Error::new(*span, "Registers above x15 are not supported on embedded RISC-V"));
            }
        }
    }

    Ok(())
}
