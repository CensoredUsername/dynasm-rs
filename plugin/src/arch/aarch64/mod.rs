use syn::parse;

mod ast;
mod parser;
mod matching;
mod aarch64data;

use ::State;
use ::emit_error_at;
use arch::Arch;


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

    fn compile_instruction(&self, state: &mut State, input: parse::ParseStream) -> parse::Result<()> {
        let mut ctx = Context {
            state
        };

        let (instruction, args) = parser::parse_instruction(&mut ctx, input)?;
        let span = instruction.span;

        let (opdata, args) = match matching::match_instruction(&mut ctx, &instruction, args) {
            Err(None) => return Ok(()),
            Err(Some(e)) => {
                emit_error_at(span, e);
                return Ok(())
            }
            Ok(m) => m
        };

        println!("{:?}", instruction);
        println!("{:?}", opdata);
        println!("{:?}", args);

        Ok(())
    }
}