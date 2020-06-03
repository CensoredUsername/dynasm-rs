//! This module implements the relocation model for the x64 architecture, as well as aliases for x64 Assemblers.

use crate::relocations::{Relocation, RelocationSize, RelocationKind, ImpossibleRelocation};
use crate::Register;

use std::hash::Hash;


/// Relocation implementation for the x64 architecture.
#[derive(Debug, Clone)]
pub struct X64Relocation {
    size: RelocationSize,
    offset: u8,
    start_offset: u8
}

impl Relocation for X64Relocation {
    type Encoding = (u8, u8);
    fn from_encoding(encoding: Self::Encoding) -> Self {
        Self {
            offset: encoding.0,
            size: RelocationSize::from_encoding(encoding.1),
            start_offset: 0,
        }
    }
    fn from_size(size: RelocationSize) -> Self {
        Self {
            size,
            offset: 0,
            start_offset: size as u8,
        }
    }
    fn start_offset(&self) -> usize {
        self.start_offset as usize
    }
    fn field_offset(&self) -> usize{
        self.size.size() + self.offset as usize
    }
    fn size(&self) -> usize {
        self.size.size()
    }
    fn write_value(&self, buf: &mut [u8], value: isize) -> Result<(), ImpossibleRelocation> {
        self.size.write_value(buf, value)
    }
    fn read_value(&self, buf: &[u8]) -> isize {
        self.size.read_value(buf)
    }
    fn kind(&self) -> RelocationKind {
        RelocationKind::Relative
    }
    fn page_size() -> usize {
        4096
    }
}

/// An x64 Assembler. This is aliased here for backwards compatability.
pub type Assembler = crate::Assembler<X64Relocation>;
/// An x64 AssemblyModifier. This is aliased here for backwards compatability.
pub type AssemblyModifier<'a> = crate::Modifier<'a, X64Relocation>;
/// An x64 UncommittedModifier. This is aliased here for backwards compatability.
pub type UncommittedModifier<'a> = crate::UncommittedModifier<'a>;

/// 8-byte general purpose "quad-word" registers.
///
/// RIP does not appear here as it is addressed differently in dynasm.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rq {
    RAX = 0x0, RCX = 0x1, RDX = 0x2, RBX = 0x3,
    RSP = 0x4, RBP = 0x5, RSI = 0x6, RDI = 0x7,
    R8  = 0x8, R9  = 0x9, R10 = 0xA, R11 = 0xB,
    R12 = 0xC, R13 = 0xD, R14 = 0xE, R15 = 0xF,
}
reg_impls!(Rq);


/// 16-byte SSE registers.
///
/// Note that XMM8-SMM15 are X86_64 specific, so we don't inherit this from the enum of the same
/// name in the X86 backend.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rx {
    XMM0  = 0x0, XMM1  = 0x1, XMM2  = 0x2, XMM3  = 0x3,
    XMM4  = 0x4, XMM5  = 0x5, XMM6  = 0x6, XMM7  = 0x7,
    XMM8  = 0x8, XMM9  = 0x9, XMM10 = 0xA, XMM11 = 0xB,
    XMM12 = 0xC, XMM13 = 0xD, XMM14 = 0xE, XMM15 = 0xF,
}
reg_impls!(Rx);

/// 8-byte control registers.
///
/// Note that 32-bit x86 can only address CR0-7, hence this enum is duplicated here.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RC {
    CR0  = 0x0, CR1  = 0x1, CR2  = 0x2, CR3  = 0x3,
    CR4  = 0x4, CR5  = 0x5, CR6  = 0x6, CR7  = 0x7,
    CR8  = 0x8, CR9  = 0x9, CR10 = 0xA, CR11 = 0xB,
    CR12 = 0xC, CR13 = 0xD, CR14 = 0xE, CR15 = 0xF,
}
reg_impls!(RC);

// The other register families are the same as 32-bit X86.
pub use crate::x86::{Rh, Rf, Rm, Rs, RD, RB};

#[cfg(test)]
mod tests {
    use super::Rq::*;
    use crate::Register;

    #[test]
    fn reg_code() {
        assert_eq!(RAX.code(), 0);
    }

    #[test]
    fn reg_code_from() {
        assert_eq!(u8::from(R11), 11);
    }
}
