use syn::parse;

mod ast;
mod parser;
mod matching;
mod compiler;
mod aarch64data;
mod encoding_helpers;

use crate::State;
use crate::common::{Size, Stmt, JumpType, emit_error_at};
use crate::arch::Arch;
use self::aarch64data::Relocation;


struct Context<'a, 'b: 'a> {
    pub state: &'a mut State<'b>
}

#[derive(Clone, Debug)]
pub struct ArchAarch64 {

}

impl Default for ArchAarch64 {
    fn default() -> ArchAarch64 {
        ArchAarch64 { }
    }
}

impl Arch for ArchAarch64 {
    fn name(&self) -> &str {
        "aarch64"
    }

    fn set_features(&mut self, features: &[syn::Ident]) {
        if let Some(feature) = features.first() {
            emit_error_at(feature.span(), "Arch aarch64 has no known features".into());
        }
    }

    fn handle_static_reloc(&self, stmts: &mut Vec<Stmt>, reloc: JumpType, size: Size) {
        let span = reloc.span();

        let relocation = match size {
            Size::DWORD => Relocation::LITERAL32,
            Size::QWORD => Relocation::LITERAL64,
            _ => {
                emit_error_at(span, "Relocation of unsupported size for the current target architecture".into());
                return;
            }
        };
        let data = [relocation.to_id()];

        stmts.push(Stmt::Const(0, size));
        stmts.push(reloc.encode(&data));
    }

    fn compile_instruction(&self, state: &mut State, input: parse::ParseStream) -> parse::Result<()> {
        let mut ctx = Context {
            state
        };

        let (instruction, args) = parser::parse_instruction(&mut ctx, input)?;
        let span = instruction.span;

        let match_data = match matching::match_instruction(&mut ctx, &instruction, args) {
            Err(None) => return Ok(()),
            Err(Some(e)) => {
                emit_error_at(span, e);
                return Ok(())
            }
            Ok(m) => m
        };

        println!("{:?}", instruction);
        println!("{:?}", match_data);

        match compiler::compile_instruction(&mut ctx, match_data) {
            Err(None) => return Ok(()),
            Err(Some(e)) => {
                emit_error_at(span, e);
                return Ok(())
            }
            Ok(()) => ()
        }

        Ok(())
    }
}