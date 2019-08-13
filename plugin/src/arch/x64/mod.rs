use syn::parse;
use syn::spanned::Spanned;

mod ast;
mod compiler;
mod parser;
pub mod debug;
pub mod x64data;

use crate::State;
use crate::emit_error_at;
use crate::arch::Arch;
use crate::serialize::{self, Size, Stmt};
use crate::parse_helpers::JumpType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X86Mode {
    Long,
    Protected
}

struct Context<'a, 'b: 'a> {
    pub state: &'a mut State<'b>,
    pub mode: X86Mode,
    pub features: x64data::Features
}

#[derive(Clone, Debug)]
pub struct Archx64 {
    features: x64data::Features
}

impl Default for Archx64 {
    fn default() -> Archx64 {
        Archx64 { features: x64data::Features::all() }
    }
}

impl Arch for Archx64 {
    fn name(&self) -> &str {
        "x64"
    }

    fn set_features(&mut self, features: &[syn::Ident]) {
        let mut new_features = x64data::Features::empty();
        for ident in features {
            new_features |= match x64data::Features::from_str(&ident.to_string()) {
                Some(feature) => feature,
                None => {
                    emit_error_at(ident.span(), format!("Architecture x64 does not support feature '{}'", ident.to_string()));
                    continue;
                }
            }
        }
        self.features = new_features;
    }

    fn handle_static_reloc(&self, stmts: &mut Vec<Stmt>, reloc: JumpType, size: Size) {
        let span = match &reloc {
            JumpType::Global(ident) |
            JumpType::Backward(ident) |
            JumpType::Forward(ident) => ident.span(),
            JumpType::Dynamic(expr) |
            JumpType::Bare(expr) => expr.span(),
        };

        let data = [0, size.in_bytes(), 0]; // no offset, specified size, relative

        stmts.push(Stmt::Const(0, size));
        stmts.push(match reloc {
            JumpType::Global(ident) => Stmt::GlobalJumpTarget(ident, serialize::expr_tuple_of_u8s(span, &data)),
            JumpType::Forward(ident) => Stmt::ForwardJumpTarget(ident, serialize::expr_tuple_of_u8s(span, &data)),
            JumpType::Backward(ident) => Stmt::BackwardJumpTarget(ident, serialize::expr_tuple_of_u8s(span, &data)),
            JumpType::Dynamic(expr) => Stmt::DynamicJumpTarget(serialize::delimited(expr), serialize::expr_tuple_of_u8s(span, &data)),
            JumpType::Bare(_ident) => {
                emit_error_at(span, "Extern relocations in statics are not supported".into());
                return;
            }
        });
    }

    fn compile_instruction(&self, state: &mut State, input: parse::ParseStream) -> parse::Result<()> {
        let mut ctx = Context {
            state,
            mode: X86Mode::Long,
            features: self.features
        };
        let (instruction, args) = parser::parse_instruction(&mut ctx, input)?;
        let span = instruction.span;

        if let Err(Some(e)) = compiler::compile_instruction(&mut ctx, instruction, args) {
            emit_error_at(span, e);
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Archx86 {
    features: x64data::Features
}

impl Default for Archx86 {
    fn default() -> Archx86 {
        Archx86 { features: x64data::Features::all() }
    }
}

impl Arch for Archx86 {
    fn name(&self) -> &str {
        "x86"
    }

    fn set_features(&mut self, features: &[syn::Ident]) {
        let mut new_features = x64data::Features::empty();
        for ident in features {
            new_features |= match x64data::Features::from_str(&ident.to_string()) {
                Some(feature) => feature,
                None => {
                    emit_error_at(ident.span(), format!("Architecture x86 does not support feature '{}'", ident.to_string()));
                    continue;
                }
            }
        }
        self.features = new_features;
    }

    fn handle_static_reloc(&self, stmts: &mut Vec<Stmt>, reloc: JumpType, size: Size) {
        let span = match &reloc {
            JumpType::Global(ident) |
            JumpType::Backward(ident) |
            JumpType::Forward(ident) => ident.span(),
            JumpType::Dynamic(expr) |
            JumpType::Bare(expr) => expr.span(),
        };

        let data = [0, size.in_bytes(), 0]; // no offset, specified size, relative

        stmts.push(Stmt::Const(0, size));
        stmts.push(match reloc {
            JumpType::Global(ident) => Stmt::GlobalJumpTarget(ident, serialize::expr_tuple_of_u8s(span, &data)),
            JumpType::Forward(ident) => Stmt::ForwardJumpTarget(ident, serialize::expr_tuple_of_u8s(span, &data)),
            JumpType::Backward(ident) => Stmt::BackwardJumpTarget(ident, serialize::expr_tuple_of_u8s(span, &data)),
            JumpType::Dynamic(expr) => Stmt::DynamicJumpTarget(serialize::delimited(expr), serialize::expr_tuple_of_u8s(span, &data)),
            JumpType::Bare(_ident) => {
                emit_error_at(span, "Extern relocations in statics are not supported".into());
                return;
            }
        });
    }

    fn compile_instruction(&self, state: &mut State, input: parse::ParseStream) -> parse::Result<()> {
        let mut ctx = Context {
            state,
            mode: X86Mode::Protected,
            features: self.features
        };
        let (instruction, args) = parser::parse_instruction(&mut ctx, input)?;
        let span = instruction.span;

        if let Err(Some(e)) = compiler::compile_instruction(&mut ctx, instruction, args) {
            emit_error_at(span, e);
        }
        Ok(())
    }
}
