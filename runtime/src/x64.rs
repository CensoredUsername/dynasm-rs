use crate::relocations::{Relocation, RelocationSize, RelocationKind};


/// Relocation implementation for the x64 architecture.
#[derive(Debug, Clone)]
pub struct X64Relocation {
    size: RelocationSize,
    offset: u8,
}

impl Relocation for X64Relocation {
    type Encoding = (u8, u8);
    fn from_encoding(encoding: Self::Encoding) -> Self {
        Self {
            offset: encoding.0,
            size: RelocationSize::from_encoding(encoding.1)
        }
    }
    fn encode_from_size(size: RelocationSize) -> Self::Encoding {
        (0, RelocationSize::encode_from_size(size))
    }
    fn start_offset(&self) -> usize {
        0
    }
    fn field_offset(&self) -> usize{
        self.size.size() + self.offset as usize
    }
    fn size(&self) -> usize {
        self.size.size()
    }
    fn write_value(&self, buf: &mut [u8], value: isize) {
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


pub type Assembler = crate::Assembler<X64Relocation>;
pub type AssemblyModifier<'a> = crate::Modifier<'a, X64Relocation>;
pub type UncommittedModifier<'a> = crate::UncommittedModifier<'a>;
