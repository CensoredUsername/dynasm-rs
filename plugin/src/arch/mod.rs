use syn::parse;
use proc_macro_error2::emit_error;

use crate::common::{Size, Stmt, Jump};
use crate::State;

use std::fmt::Debug;

pub mod x64;
pub mod aarch64;
pub mod riscv;

pub(crate) trait Arch : Debug + Send {
    /// When the .features directive is used for an architecture, this architecture method will be
    /// called with the list of features as argument
    fn set_features(&mut self, features: &[syn::Ident]);
    /// When a data directive (.u32, .i64) is used with a jump in it, this needs to be emitted
    /// in a way that the target runtime understands it. This architecture method handles this.
    fn handle_static_reloc(&self, stmts: &mut Vec<Stmt>, reloc: Jump, size: Size);
    /// The default byte to pad with for alignment for this architecture.
    fn default_align(&self) -> u8;
    /// The core of the architecture. This function parses a single instruction, storing the to be
    /// emitted code in the passed `state` parameter. 
    fn compile_instruction(&self, state: &mut State, input: parse::ParseStream) -> parse::Result<()>;
}

#[derive(Clone, Debug)]
pub struct DummyArch {}

impl DummyArch {
    fn new() -> DummyArch {
        DummyArch{}
    }
}

impl Arch for DummyArch {
    fn set_features(&mut self, features: &[syn::Ident]) {
        if let Some(feature) = features.first() {
            emit_error!(feature, "Cannot set features when the assembling architecture is undefined. Define it using a .arch directive");
        }
    }

    fn handle_static_reloc(&self, _stmts: &mut Vec<Stmt>, reloc: Jump, _size: Size) {
        let span = reloc.span();
        emit_error!(span, "Current assembling architecture is undefined. Define it using a .arch directive");
    }

    fn default_align(&self) -> u8 {
        0
    }

    fn compile_instruction(&self, _state: &mut State, input: parse::ParseStream) -> parse::Result<()> {
        emit_error!(input.cursor().span(), "Current assembling architecture is undefined. Define it using a .arch directive");
        Ok(())
    }
}

pub(crate) fn from_str(s: &str) -> Option<Box<dyn Arch>> {
    match s {
        "x64" => Some(Box::new(x64::Archx64::default())),
        "x86" => Some(Box::new(x64::Archx86::default())),
        "aarch64" => Some(Box::new(aarch64::ArchAarch64::default())),
        "riscv64i" => Some(Box::new(riscv::ArchRiscV64I::default())),
        "riscv64e" => Some(Box::new(riscv::ArchRiscV64E::default())),
        "riscv32i" => Some(Box::new(riscv::ArchRiscV32I::default())),
        "riscv32e" => Some(Box::new(riscv::ArchRiscV32E::default())),
        "unknown" => Some(Box::new(DummyArch::new())),
        _ => None
    }
}

#[cfg(target_arch="x86_64")]
pub const CURRENT_ARCH: &str = "x64";
#[cfg(target_arch="x86")]
pub const CURRENT_ARCH: &str = "x86";
#[cfg(target_arch="aarch64")]
pub const CURRENT_ARCH: &str = "aarch64";
// TODO: there seems to be no good way to detect riscv64i from riscv64e. You're probably not running
// rustc on an embedded targets so assume the i variant.
#[cfg(target_arch="riscv64")]
pub const CURRENT_ARCH: &str = "riscv64i";
#[cfg(target_arch="riscv32")]
pub const CURRENT_ARCH: &str = "riscv32i";
#[cfg(not(any(
    target_arch="x86",
    target_arch="x86_64",
    target_arch="aarch64",
    target_arch="riscv64",
    target_arch="riscv32"
)))]
pub const CURRENT_ARCH: &str = "unknown";
