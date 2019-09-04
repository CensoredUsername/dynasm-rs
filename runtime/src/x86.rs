use crate::relocations::{Relocation, RelocationSize, RelocationKind, ImpossibleRelocation};


/// Relocation implementation for the x86 architecture.
#[derive(Debug, Clone)]
pub struct X86Relocation {
    size: RelocationSize,
    kind: RelocationKind,
    offset: u8,
    start_offset: u8,
}

impl Relocation for X86Relocation {
    type Encoding = (u8, u8, u8);
    fn from_encoding(encoding: Self::Encoding) -> Self {
        Self {
            offset: encoding.0,
            size: RelocationSize::from_encoding(encoding.1),
            kind: RelocationKind::from_encoding(encoding.2),
            start_offset: 0,
        }
    }
    fn from_size(size: RelocationSize) -> Self {
        Self {
            size,
            kind: RelocationKind::Relative,
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
        self.kind
    }
    fn page_size() -> usize {
        4096
    }
}


pub type Assembler = crate::Assembler<X86Relocation>;
pub type AssemblyModifier<'a> = crate::Modifier<'a, X86Relocation>;
pub type UncommittedModifier<'a> = crate::UncommittedModifier<'a>;
