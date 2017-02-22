use syntax::ext::base::ExtCtxt;
use syntax::parse::parser::Parser;
use syntax::parse::PResult;

mod parser;
mod compiler;
pub mod debug;
pub mod x64data;

use ::State;
use serialize::Ident;
use arch::Arch;

#[derive(Clone, Debug)]
pub struct Archx64 {
    features: x64data::features::Features
}

impl Default for Archx64 {
    fn default() -> Archx64 {
        Archx64 { features: x64data::features::X64_IMPLICIT }
    }
}

impl Arch for Archx64 {
    fn name(&self) -> &str {
        "x64"
    }

    fn set_features(&mut self, ecx: &ExtCtxt, features: &[Ident]) {
        for &Ident {span, ref node} in features {
            self.features |= match &*node.name.as_str() {
                "test" => x64data::features::X64_IMPLICIT,
                e => {
                    ecx.span_err(span, &format!("Architecture x64 does not support feature '{}'", e));
                    continue;
                }
            }
        }
    }

    fn compile_instruction<'a>(&self, state: &mut State, ecx: &mut ExtCtxt, parser: &mut Parser<'a>) -> PResult<'a, ()> {
        let instruction = parser::parse_instruction(state, ecx, parser)?;
        let span = instruction.2;

        if let Err(Some(e)) = compiler::compile_instruction(state, ecx, instruction) {
            ecx.span_err(span, &e);
        }
        Ok(())
    }
}