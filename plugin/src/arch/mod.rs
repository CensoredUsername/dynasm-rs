use syntax::ext::base::ExtCtxt;
use syntax::parse::parser::Parser;
use syntax::parse::PResult;

use ::State;

pub mod x64;

#[derive(Debug, Clone, Copy)]
pub enum Arch {
    X64,
    X86,
    Unknown
}

impl Arch {
    pub fn from_str(s: &str) -> Option<Arch> {
        match s {
            "x64" => Some(Arch::X64),
            "x86" => Some(Arch::X86),
            _ => None
        }
    }
}

impl<'a> State<'a> {
    pub fn compile_instruction<'b>(&mut self, ecx: &mut ExtCtxt, parser: &mut Parser<'b>) -> PResult<'b, ()> {
        match self.crate_data.current_arch {
            Arch::X64 => x64::compile_instruction(self, ecx, parser),
            _ => {
                ecx.span_err(ecx.call_site(),
                             "Current assembling architecture is undefined. Define it using a .arch directive"
                );
                Ok(())
            }
        }
    }
}

#[cfg(target_arch="x86_64")]
pub const CURRENT_ARCH: Arch = Arch::X64;
#[cfg(target_arch="x86")]
pub const CURRENT_ARCH: Arch = Arch::X86;
#[cfg(not(any(target_arch="x86", target_arch="x86_64")))]
pub const CURRENT_ARCH: Arch = Arch::Unknown;
