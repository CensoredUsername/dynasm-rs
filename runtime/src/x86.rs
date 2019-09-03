use crate::relocations::X86Relocation;

pub type Assembler = crate::Assembler<X86Relocation>;
pub type AssemblyModifier<'a> = crate::Modifier<'a, X86Relocation>;
pub type UncommittedModifier<'a> = crate::UncommittedModifier<'a>;
