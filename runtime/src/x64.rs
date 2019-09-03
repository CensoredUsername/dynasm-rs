use crate::relocations::X64Relocation;

pub type Assembler = crate::Assembler<X64Relocation>;
pub type AssemblyModifier<'a> = crate::Modifier<'a, X64Relocation>;
pub type UncommittedModifier<'a> = crate::UncommittedModifier<'a>;
