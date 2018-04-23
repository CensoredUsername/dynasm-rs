use syntax::ext::base::ExtCtxt;
use syntax::parse::parser::Parser;
use syntax::parse::PResult;

mod ast;
mod compiler;
mod parser;
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

    fn set_features(&mut self, ecx: &ExtCtxt, features: &[Ident]) {
        let mut new_features = x64data::Features::empty();
        for &Ident {span, ref node} in features {
            new_features |= match x64data::Features::from_str(&*node.name.as_str()) {
                Some(feature) => feature,
                None => {
                    ecx.span_err(span, &format!("Architecture x64 does not support feature '{}'", &*node.name.as_str()));
                    continue;
                }
            }
        }
        self.features = new_features;
    }

    fn compile_instruction<'a>(&self, state: &mut State, ecx: &mut ExtCtxt, parser: &mut Parser<'a>) -> PResult<'a, ()> {
        let mut ctx = Context {
            state: state,
            mode: X86Mode::Long,
            features: self.features
        };
        let (instruction, args) = parser::parse_instruction(&mut ctx, ecx, parser)?;
        let span = instruction.span;

        if let Err(Some(e)) = compiler::compile_instruction(&mut ctx, ecx, instruction, args) {
            ecx.span_err(span, &e);
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

    fn set_features(&mut self, ecx: &ExtCtxt, features: &[Ident]) {
        let mut new_features = x64data::Features::empty();
        for &Ident {span, ref node} in features {
            new_features |= match x64data::Features::from_str(&*node.name.as_str()) {
                Some(feature) => feature,
                None => {
                    ecx.span_err(span, &format!("Architecture x86 does not support feature '{}'", &*node.name.as_str()));
                    continue;
                }
            }
        }
        self.features = new_features;
    }

    fn compile_instruction<'a>(&self, state: &mut State, ecx: &mut ExtCtxt, parser: &mut Parser<'a>) -> PResult<'a, ()> {
        let mut ctx = Context {
            state: state,
            mode: X86Mode::Protected,
            features: self.features
        };
        let (instruction, args) = parser::parse_instruction(&mut ctx, ecx, parser)?;
        let span = instruction.span;

        if let Err(Some(e)) = compiler::compile_instruction(&mut ctx, ecx, instruction, args) {
            ecx.span_err(span, &e);
        }
        Ok(())
    }
}
