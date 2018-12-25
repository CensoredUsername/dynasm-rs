use syn::parse;

use ::State;
use ::emit_error_at;

use std::fmt::Debug;

pub mod x64;

pub(crate) trait Arch : Debug + Send {
    fn name(&self) -> &str;
    fn set_features(&mut self, features: &[syn::Ident]);
    fn compile_instruction(&self, state: &mut State, input: parse::ParseStream) -> parse::Result<()>;
}

#[derive(Clone, Debug)]
pub struct DummyArch {
    name: &'static str
}

impl DummyArch {
    fn new(name: &'static str) -> DummyArch {
        DummyArch { name }
    }
}

impl Arch for DummyArch {
    fn name(&self) -> &str {
        self.name
    }

    fn set_features(&mut self, features: &[syn::Ident]) {
        if let Some(feature) = features.first() {
            emit_error_at(feature.span(), "Cannot set features when the assembling architecture is undefined. Define it using a .arch directive".into());
        }
    }

    fn compile_instruction(&self, _state: &mut State, input: parse::ParseStream) -> parse::Result<()> {
        emit_error_at(input.cursor().span(), "Current assembling architecture is undefined. Define it using a .arch directive".into());
        Ok(())
    }
}

pub(crate) fn from_str(s: &str) -> Option<Box<Arch>> {
    match s {
        "x64" => Some(Box::new(x64::Archx64::default())),
        "x86" => Some(Box::new(x64::Archx86::default())),
        "unknown" => Some(Box::new(DummyArch::new("unknown"))),
        _ => None
    }
}

#[cfg(target_arch="x86_64")]
pub const CURRENT_ARCH: &str = "x64";
#[cfg(target_arch="x86")]
pub const CURRENT_ARCH: &str = "x86";
#[cfg(not(any(target_arch="x86", target_arch="x86_64")))]
pub const CURRENT_ARCH: &str = "unknown";
