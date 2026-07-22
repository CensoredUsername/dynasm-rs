use syn::parse;
use proc_macro_error3::emit_error;

mod ast;
mod parser;
mod matching;
mod compiler;
mod aarch64data;
mod encoding_helpers;
mod debug;

use crate::State;
use crate::arch::Arch;

#[cfg(feature = "dynasm_opmap")]
pub use debug::create_opmap;
#[cfg(feature = "dynasm_extract")]
pub use debug::extract_opmap;

struct Context<'a, 'b: 'a> {
    pub state: &'a mut State<'b>
}

#[derive(Clone, Debug, Default)]
pub struct ArchAarch64 {

}

impl Arch for ArchAarch64 {
    fn set_features(&mut self, features: &[syn::Ident]) {
        if let Some(feature) = features.first() {
            emit_error!(feature, "Arch aarch64 has no known features");
        }
    }

    fn default_align(&self) -> u8 {
        0
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
                emit_error!(span, e);
                return Ok(())
            }
            Ok(m) => m
        };

        match compiler::compile_instruction(&mut ctx, match_data) {
            Err(None) => return Ok(()),
            Err(Some(e)) => {
                emit_error!(span, e);
                return Ok(())
            }
            Ok(()) => ()
        }

        Ok(())
    }
}
