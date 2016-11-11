use syntax::ext::base::ExtCtxt;
use syntax::parse::parser::Parser;
use syntax::parse::PResult;

mod parser;
mod compiler;
pub mod debug;
pub mod x64data;

use ::State;

pub fn compile_instruction<'b>(state: &mut State, ecx: &mut ExtCtxt, parser: &mut Parser<'b>) -> PResult<'b, ()> {
    let instruction = parser::parse_instruction(state, ecx, parser)?;
    let span = instruction.2;

    if let Err(Some(e)) = compiler::compile_instruction(state, ecx, instruction) {
        ecx.span_err(span, &e);
    }
    Ok(())
}
