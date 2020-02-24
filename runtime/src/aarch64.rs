//! This module implements the relocation model for the aarch64 architecture, as well as aliases for aarch64 Assemblers.

use crate::relocations::{Relocation, RelocationSize, RelocationKind, ImpossibleRelocation, fits_signed_bitfield};
use byteorder::{ByteOrder, LittleEndian};
use std::convert::TryFrom;

/// Relocation implementation for the aarch64 architecture.
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub enum Aarch64Relocation {
    // b, bl 26 bits, dword aligned
    B,
    // b.cond, cbnz, cbz, ldr, ldrsw, prfm: 19 bits, dword aligned
    BCOND,
    // adr split 21 bit, byte aligned
    ADR,
    // adrp split 21 bit, 4096-byte aligned
    ADRP,
    // tbnz, tbz: 14 bits, dword aligned
    TBZ,
    // Anything in directives
    Plain(RelocationSize),
}

impl Aarch64Relocation {
    fn op_mask(&self) -> u32 {
        match self {
            Self::B => 0xFC00_0000,
            Self::BCOND => 0xFF00_001F,
            Self::ADR => 0x9F00_001F,
            Self::ADRP => 0x9F00_001F,
            Self::TBZ => 0xFFF8_001F,
            Self::Plain(_) => 0
        }
    }

    fn encode(&self, value: isize) -> Result<u32, ImpossibleRelocation> {
        let value = i64::try_from(value).map_err(|_| ImpossibleRelocation { } )?;
        Ok(match self {
            Self::B => {
                if value & 3 != 0 || !fits_signed_bitfield(value >> 2, 26) {
                    return Err(ImpossibleRelocation { } );
                }
                let value = (value >> 2) as u32;
                (value & 0x3FF_FFFF)
            },
            Self::BCOND => {
                if value & 3 != 0 || !fits_signed_bitfield(value >> 2, 19) {
                    return Err(ImpossibleRelocation { } );
                }
                let value = (value >> 2) as u32;
                (value & 0x7FFFF) << 5
            },
            Self::ADR => {
                if !fits_signed_bitfield(value, 21) {
                    return Err(ImpossibleRelocation { } );
                }
                let low = (value) as u32;
                let high = (value >> 2) as u32;
                ((high & 0x7FFFF) << 5) | ((low & 3) << 29)
            },
            Self::ADRP => {
                let value = value + 0xFFF;
                if !fits_signed_bitfield(value >> 12, 21) {
                    return Err(ImpossibleRelocation { } );
                }
                let low = (value >> 12) as u32;
                let high = (value >> 14) as u32;
                ((high & 0x7FFFF) << 5) | ((low & 3) << 29)
            },
            Self::TBZ => {
                if value & 3 != 0 || !fits_signed_bitfield(value >> 2, 14) {
                    return Err(ImpossibleRelocation { } );
                }
                let value = (value >> 2) as u32;
                (value & 0x3FFF) << 5
            },
            Self::Plain(_) => return Err(ImpossibleRelocation { } )
        })
    }
}

impl Relocation for Aarch64Relocation {
    type Encoding = (u8,);
    fn from_encoding(encoding: Self::Encoding) -> Self {
        match encoding.0 {
            0 => Self::B,
            1 => Self::BCOND,
            2 => Self::ADR,
            3 => Self::ADRP,
            4 => Self::TBZ,
            x  => Self::Plain(RelocationSize::from_encoding(x - 4))
        }
    }
    fn from_size(size: RelocationSize) -> Self {
        Self::Plain(size)
    }
    fn size(&self) -> usize {
        match self {
            Self::Plain(s) => s.size(),
            _ => RelocationSize::DWord.size(),
        }
    }
    fn write_value(&self, buf: &mut [u8], value: isize) -> Result<(), ImpossibleRelocation> {
        if let Self::Plain(s) = self {
            return s.write_value(buf, value);
        };

        let mask = self.op_mask();
        let template = LittleEndian::read_u32(buf) & mask;

        let packed = self.encode(value)?;

        LittleEndian::write_u32(buf, template | packed);
        Ok(())
    }
    fn read_value(&self, buf: &[u8]) -> isize {
        if let Self::Plain(s) = self {
            return s.read_value(buf);
        };

        let mask = !self.op_mask();
        let value = LittleEndian::read_u32(buf);
        let unpacked = match self {
            Self::B => u64::from(
                value & mask
            ) << 2,
            Self::BCOND => u64::from(
                (value & mask) >> 5
            ) << 2,
            Self::ADR  => u64::from(
                (((value >> 5 ) & 0x7FFFF) << 2) |
                ((value >> 29) & 3 )
            ),
            Self::ADRP => u64::from(
                (((value >> 5 ) & 0x7FFFF) << 2) |
                ((value >> 29) & 3 )
            ) << 12,
            Self::TBZ => u64::from(
                (value & mask) >> 5
            ) << 2,
            Self::Plain(_) => unreachable!()
        };

        // Sign extend.
        let bits = match self {
            Self::B => 26,
            Self::BCOND => 19,
            Self::ADR => 21,
            Self::ADRP => 33,
            Self::TBZ => 14,
            Self::Plain(_) => unreachable!()
        };
        let offset = 1u64 << (bits - 1);
        let value: u64 = (unpacked ^ offset) - offset;

        value as i64 as isize
    }
    fn kind(&self) -> RelocationKind {
        RelocationKind::Relative
    }
    fn page_size() -> usize {
        4096
    }
}

/// An aarch64 Assembler. This is aliased here for backwards compatability.
pub type Assembler = crate::Assembler<Aarch64Relocation>;
/// An aarch64 AssemblyModifier. This is aliased here for backwards compatability.
pub type AssemblyModifier<'a> = crate::Modifier<'a, Aarch64Relocation>;
/// An aarch64 UncommittedModifier. This is aliased here for backwards compatability.
pub type UncommittedModifier<'a> = crate::UncommittedModifier<'a>;


/// Helper function for validating that a given value can be encoded as a 32-bit logical immediate
pub fn encode_logical_immediate_32bit(value: u32) -> Option<u16> {
    let transitions = value ^ value.rotate_right(1);
    let element_size = (64u32).checked_div(transitions.count_ones())?;

    // confirm that the elements are identical
    if value != value.rotate_left(element_size) {
        return None;
    }

    let element = value & (1u32 << element_size).wrapping_sub(1);
    let ones = element.count_ones();
    let imms = (!((element_size << 1) - 1) & 0x3F) | (ones - 1);

    let immr = if (element & 1) != 0 {
        ones - (!element).trailing_zeros()
    } else {
        element_size - element.trailing_zeros()
    };

    Some(((immr as u16) << 6) | (imms as u16))
}

/// Helper function for validating that a given value can be encoded as a 64-bit logical immediate
pub fn encode_logical_immediate_64bit(value: u64) -> Option<u16> {
    let transitions = value ^ value.rotate_right(1);
    let element_size = (128u32).checked_div(transitions.count_ones())?;

    // confirm that the elements are identical
    if value != value.rotate_left(element_size) {
        return None;
    }

    let element = value & (1u64 << element_size).wrapping_sub(1);
    let ones = element.count_ones();
    let imms = (!((element_size << 1) - 1) & 0x7F) | (ones - 1);

    let immr = if (element & 1) != 0 {
        ones - (!element).trailing_zeros()
    } else {
        element_size - element.trailing_zeros()
    };

    let n = imms & 0x40 == 0;
    let imms = imms & 0x3F;

    Some(((n as u16) << 12) | ((immr as u16) << 6) | (imms as u16))
}

/// Helper function for validating that a given value can be encoded as a floating point immediate
pub fn encode_floating_point_immediate(value: f32) -> Option<u8> {
    // floating point ARM immediates are encoded as
    // abcdefgh => aBbbbbbc defgh000 00000000 00000000
    // where B = !b
    // which means we can just slice out "a" and "bcdefgh" and assume the rest was correct

    let bits = value.to_bits();

    let check = (bits >> 25) & 0x3F;
    if (check == 0b10_0000 || check == 0b01_1111) && (bits & 0x7_FFFF) == 0 {
        Some((((bits >> 24) & 0x80) | ((bits >> 19) & 0x7F)) as u8)
    } else {
        None
    }
}