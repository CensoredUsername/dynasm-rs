use syntax::ext::base::ExtCtxt;
use syntax::parse::parser::Parser;
use syntax::parse::PResult;

// the parser for x86 and x64 is shared, with an argument determining which rules to follow
// the only real difference is in which registers are considered valid. 
use super::x64::parser;
//mod compiler;
//pub mod debug;
//pub mod x64data;

use ::State;
use serialize::Ident;
use arch::Arch;

#[derive(Clone, Debug)]
pub struct Archx86 {
    // features: x64data::features::Features
}

impl Default for Archx86 {
    fn default() -> Archx86 {
        Archx86 {} // features: x64data::features::X64_IMPLICIT }
    }
}

impl Arch for Archx86 {
    fn name(&self) -> &str {
        "x86"
    }

    fn set_features(&mut self, _ecx: &ExtCtxt, _features: &[Ident]) {
        unimplemented!();
    }

    fn compile_instruction<'a>(&self, state: &mut State, ecx: &mut ExtCtxt, parser: &mut Parser<'a>) -> PResult<'a, ()> {
        let options = parser::ParserOptions {
            x86mode: true
        };
        let instruction = parser::parse_instruction(state, &options, ecx, parser)?;
        let _span = instruction.2;


        //if let Err(Some(e)) = compiler::compile_instruction(state, ecx, instruction) {
        //    ecx.span_err(span, &e);
        //}
        Ok(())
    }
}
