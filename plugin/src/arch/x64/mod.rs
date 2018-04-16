use syntax::ext::base::ExtCtxt;
use syntax::parse::parser::Parser;
use syntax::parse::PResult;

pub(super) mod parser;
mod compiler;
pub mod debug;
pub mod x64data;

use ::State;
use serialize::Ident;
use arch::Arch;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X86Mode {
    Long,
    Protected
}

pub struct Context<'a, 'b: 'a> {
    pub state: &'a mut State<'b>,
    pub mode: X86Mode
}

#[derive(Clone, Debug)]
pub struct Archx64 {
    features: x64data::Features
}

impl Default for Archx64 {
    fn default() -> Archx64 {
        Archx64 { features: x64data::Features::X64_IMPLICIT }
    }
}

impl Arch for Archx64 {
    fn name(&self) -> &str {
        "x64"
    }

    fn set_features(&mut self, ecx: &ExtCtxt, features: &[Ident]) {
        for &Ident {span, ref node} in features {
            self.features |= match &*node.name.as_str() {
                "test" => x64data::Features::X64_IMPLICIT,
                e => {
                    ecx.span_err(span, &format!("Architecture x64 does not support feature '{}'", e));
                    continue;
                }
            }
        }
    }

    fn compile_instruction<'a>(&self, state: &mut State, ecx: &mut ExtCtxt, parser: &mut Parser<'a>) -> PResult<'a, ()> {
        let mut ctx = Context {
            state: state,
            mode: X86Mode::Long
        };
        let instruction = parser::parse_instruction(&mut ctx, ecx, parser)?;
        let span = instruction.2;

        if let Err(Some(e)) = compiler::compile_instruction(&mut ctx, ecx, instruction) {
            ecx.span_err(span, &e);
        }
        Ok(())
    }
}

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
        let mut ctx = Context {
            state: state,
            mode: X86Mode::Protected
        };
        let instruction = parser::parse_instruction(&mut ctx, ecx, parser)?;
        let span = instruction.2;


        if let Err(Some(e)) = compiler::compile_instruction(&mut ctx, ecx, instruction) {
            ecx.span_err(span, &e);
        }
        Ok(())
    }
}
