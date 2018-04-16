use syntax::ext::base::ExtCtxt;
use syntax::parse::parser::Parser;
use syntax::parse::PResult;

use std::fmt::Debug;

use serialize::Ident;
use ::State;

pub mod x64;

pub trait Arch : Debug + Send {
    fn name(&self) -> &str;
    fn set_features(&mut self, ecx: &ExtCtxt, features: &[Ident]);
    fn compile_instruction<'a>(&self, state: &mut State, ecx: &mut ExtCtxt, parser: &mut Parser<'a>) -> PResult<'a, ()>;
}

#[derive(Clone, Debug)]
pub struct DummyArch {
    name: &'static str
}

impl DummyArch {
    fn new(name: &'static str) -> DummyArch {
        DummyArch { name: name }
    }
}

impl Arch for DummyArch {
    fn name(&self) -> &str {
        self.name
    }

    fn set_features(&mut self, ecx: &ExtCtxt, features: &[Ident]) {
        if let Some(feature) = features.first() {
            ecx.span_err(feature.span,
                "Cannot set features when the assembling architecture is undefined. Define it using a .arch directive"
            );
        }
    }

    fn compile_instruction<'a>(&self, _state: &mut State, ecx: &mut ExtCtxt, parser: &mut Parser<'a>) -> PResult<'a, ()> {
        ecx.span_err(parser.span,
            "Current assembling architecture is undefined. Define it using a .arch directive"
        );
        Ok(())
    }
}

pub fn from_str(s: &str) -> Option<Box<Arch>> {
    match s {
        "x64" => Some(Box::new(x64::Archx64::default())),
        "x86" => Some(Box::new(x64::Archx86::default())),
        "unknown" => Some(Box::new(DummyArch::new("unknown"))),
        _ => None
    }
}

#[cfg(target_arch="x86_64")]
pub const CURRENT_ARCH: &'static str = "x64";
#[cfg(target_arch="x86")]
pub const CURRENT_ARCH: &'static str = "x86";
#[cfg(not(any(target_arch="x86", target_arch="x86_64")))]
pub const CURRENT_ARCH: &'static str = "unknown";
