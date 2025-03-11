//! Runtime support for the 32-bit and 64-bit RISC-V architecture assembling targets.
//!
//! The riscv instruction sets feature 16-bit and 32-bit width instructions. It features relocations
//! up to 20 bits in size in a single instruction, or 32 bits in size using sequences of two
//! instructions.
//!
//! The core relocation behaviour for these architecture is provided by the [`RiscvRelocation`] type.
//!
//! Next to that, this module contains the following:
//!
//! ## Type aliases
//!
//! Several specialized type aliases of the generic [`Assembler`] are provided as these are by far the most common usecase.
//!
//! ## Enums
//!
//! There are enumerations of every RISC-V register family. 
//! These enums implement the [`Register`] trait and their discriminant values match their numeric encoding in dynamic register literals.
//!
//! *Note: The presence of some registers listed here is purely what is encodable. Check the relevant architecture documentation to find what is architecturally valid.*
//!
//! ## Functions
//!
//! This module contains handlers for error conditions in the case where a dynamically selected register is invalid, or a dynamically encoded immediate is out of range.
//! These panic with a friendly error message if any of these conditions happen at runtime.

use crate::relocations::{Relocation, RelocationSize, RelocationKind, ImpossibleRelocation, fits_signed_bitfield};
use byteorder::{ByteOrder, LittleEndian};
use std::convert::TryFrom;
use crate::Register;

/// Relocation implementation for the RV32 and RV64 architectures.
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub enum RiscvRelocation {
    // beq, beqz, bge, bgeu, bgez, bgt, bgtu, bgtz, ble, bleu, blez, blt, bltu, bltz, bne, bnez
    // 12 bits, 2-bit scaled
    B,
    // j, jal
    // 20 bits, 2-bit scaled
    J,
    // c.beqz, c.bnez
    // 9 bits, 2-bit scaled
    BC,
    // c.j, c.jal
    // 12 bits, 2-bit scaled
    JC,
    // auipc
    // 32 bits, 12-bit scaled
    HI20,
    // loads, addi.
    // 12 bits, no scaling
    LO12,
    // stores
    // 12 bits, no scaling
    LO12S,
    // pc-relative addrgen/load pseudo instructions
    // 32 bits, no scaling
    SPLIT32,
    // pc-relative store pseudo instructions
    // 32 bits, no scaling
    SPLIT32S,
    // Anything in directives
    Plain(RelocationSize),
}

impl RiscvRelocation {
    fn bitsize(&self) -> (u8, u8) {
        match self {
            Self::B => (12, 1),
            Self::J => (20, 1),
            Self::BC => (9, 1),
            Self::JC => (12, 1),

            Self::HI20 => (32, 0),
            Self::LO12 => (32, 0),
            Self::LO12S => (32, 0),

            Self::SPLIT32 => (32, 0),
            Self::SPLIT32S => (32, 0),
            Self::Plain(s) => ((s.size() * 8) as u8, 0)
        }
    }
}

impl Relocation for RiscvRelocation {
    type Encoding = (u8,);
    fn from_encoding(encoding: Self::Encoding) -> Self {
        match encoding.0 {
            0 => Self::B,
            1 => Self::J,
            2 => Self::BC,
            3 => Self::JC,
            4 => Self::HI20,
            5 => Self::LO12,
            6 => Self::LO12S,
            7 => Self::SPLIT32,
            8 => Self::SPLIT32S,
            x  => Self::Plain(RelocationSize::from_encoding(x - 8))
        }
    }
    fn from_size(size: RelocationSize) -> Self {
        Self::Plain(size)
    }
    fn size(&self) -> usize {
        match self {
            Self::BC
            | Self::JC => 2,
            Self::B
            | Self::J
            | Self::HI20
            | Self::LO12
            | Self::LO12S => 4,
            Self::SPLIT32
            | Self::SPLIT32S => 8,
            Self::Plain(s) => s.size(),
        }
    }
    fn write_value(&self, buf: &mut [u8], value: isize) -> Result<(), ImpossibleRelocation> {
        // determine if the value fits
        let value = i64::try_from(value).map_err(|_| ImpossibleRelocation { } )?;

        let (bits, scaling) = self.bitsize();
        let mask = (1i64 << scaling) - 1;
        // special case: the 32-bit AUIPC-based offsets don't actually
        // range from -0x8000_0000 to 0x7FFF_FFFF on RV64 due to how
        // sign extension interacts between them, they range from
        // -0x8000_0800 to 0x7FFF_F7FF. But on RV32 they do span
        // from -0x8000_0000 to 0x7FFF_FFFF.
        // neither of these limits will ever occur in practical code,
        // so for sanity's sake we just clamp to between -0x8000_0000 and
        // 0x7FFF_F7FF
        match self {
            Self::HI20
            | Self::LO12
            | Self::LO12S
            | Self::SPLIT32
            | Self::SPLIT32S => {
                if value < -0x8000_0800 || value > 0x7FFF_F7FF {
                    return Err(ImpossibleRelocation { } );  
                }
            },
            _ => {
                if !fits_signed_bitfield(value, bits) || (value & mask) != 0 {
                    return Err(ImpossibleRelocation { } );
                }
            }
        }

        // we never encode any bit above the 31st so cast now
        let val_cast = value as u32;

        match self {
            Self::Plain(s) => s.write_value(buf, value as isize)?,
            Self::B => {
                let mut instr = LittleEndian::read_u32(buf);
                instr &= 0x01FF_F07F;

                instr |= ((val_cast >> 12) & 0x1) << 31;
                instr |= ((val_cast >> 5) & 0x3F) << 25;
                instr |= ((val_cast >> 1) & 0xF) << 8;
                instr |= ((val_cast >> 11) & 0x1) << 7;

                LittleEndian::write_u32(buf, instr);
            },
            Self::J => {
                let mut instr = LittleEndian::read_u32(buf);
                instr &= 0x0000_0FFF;

                instr |= ((val_cast >> 20) & 0x1) << 31;
                instr |= ((val_cast >> 1) & 0x3FF) << 21;
                instr |= ((val_cast >> 11) & 0x1) << 20;
                instr |= ((val_cast >> 12) & 0xFF) << 12;

                LittleEndian::write_u32(buf, instr);
            },
            Self::BC => {
                let mut instr = LittleEndian::read_u16(buf);
                instr &= 0xE383;

                instr |= (((val_cast >> 8) & 0x1) as u16) << 12;
                instr |= (((val_cast >> 3) & 0x3) as u16) << 10;
                instr |= (((val_cast >> 6) & 0x3) as u16) << 5;
                instr |= (((val_cast >> 1) & 0x3) as u16) << 3;
                instr |= (((val_cast >> 5) & 0x1) as u16) << 2;

                LittleEndian::write_u16(buf, instr);
            },
            Self::JC => {
                let mut instr = LittleEndian::read_u16(buf);
                instr &= 0xE003;

                instr |= (((val_cast >> 11) & 0x1) as u16) << 12;
                instr |= (((val_cast >> 4) & 0x1) as u16) << 11;
                instr |= (((val_cast >> 8) & 0x3) as u16) << 9;
                instr |= (((val_cast >> 10) & 0x1) as u16) << 8;
                instr |= (((val_cast >> 6) & 0x1) as u16) << 7;
                instr |= (((val_cast >> 7) & 0x1) as u16) << 6;
                instr |= (((val_cast >> 1) & 0x7) as u16) << 3;
                instr |= (((val_cast >> 5) & 0x1) as u16) << 2;

                LittleEndian::write_u16(buf, instr);
            },
            Self::HI20 => {
                let mut instr = LittleEndian::read_u32(buf);
                instr &= 0x0000_0FFF;

                let val_round: u32 = val_cast.wrapping_add(0x800);
                instr |= val_round & 0xFFFF_F000;

                LittleEndian::write_u32(buf, instr);
            },
            Self::LO12 => {
                let mut instr = LittleEndian::read_u32(buf);
                instr &= 0x000F_FFFF;

                instr |= (val_cast & 0xFFF) << 20;

                LittleEndian::write_u32(buf, instr);
            },
            Self::LO12S => {
                let mut instr = LittleEndian::read_u32(buf);
                instr &= 0x01FF_F07F;

                instr |= (val_cast & 0x1F) << 7;
                instr |= ((val_cast >> 5) & 0x7F) << 25;

                LittleEndian::write_u32(buf, instr);
            },
            Self::SPLIT32 => {
                let mut instr1 = LittleEndian::read_u32(&buf[..4]);
                let mut instr2 = LittleEndian::read_u32(&buf[4..]);
                instr1 &= 0x0000_0FFF;
                instr2 &= 0x000F_FFFF;

                let val_round: u32 = val_cast.wrapping_add(0x800);
                instr1 |= val_round & 0xFFFF_F000;
                instr2 |= (val_cast & 0xFFF) << 20;

                LittleEndian::write_u32(&mut buf[..4], instr1);
                LittleEndian::write_u32(&mut buf[4..], instr2);
            },
            Self::SPLIT32S => {
                let mut instr1 = LittleEndian::read_u32(&buf[..4]);
                let mut instr2 = LittleEndian::read_u32(&buf[4..]);
                instr1 &= 0x0000_0FFF;
                instr2 &= 0x01FF_F07F;

                let val_round: u32 = val_cast.wrapping_add(0x800);
                instr1 |= val_round & 0xFFFF_F000;
                instr2 |= (val_cast & 0x1F) << 7;
                instr2 |= ((val_cast >> 5) & 0x7F) << 25;

                LittleEndian::write_u32(&mut buf[..4], instr1);
                LittleEndian::write_u32(&mut buf[4..], instr2);
            },
        };

        Ok(())
    }
    fn read_value(&self, buf: &[u8]) -> isize {
        let bits;
        let mut unpacked;

        match self {
            Self::Plain(s) => {
                return s.read_value(buf)
            },
            Self::B => {
                bits = 12;
                let instr = LittleEndian::read_u32(buf);

                unpacked = ((instr >> 31) & 0x1) << 12;
                unpacked |= ((instr >> 25) & 0x3F) << 5;
                unpacked |= ((instr >> 8) & 0xF) << 1;
                unpacked |= ((instr >> 7) & 0x1) << 11;
            },
            Self::J => {
                bits = 20;
                let instr = LittleEndian::read_u32(buf);

                unpacked = ((instr >> 31) & 0x1) << 20;
                unpacked |= ((instr >> 21) & 0x3FF) << 1;
                unpacked |= ((instr >> 20) & 0x1) << 11;
                unpacked |= ((instr >> 12) & 0xFF) << 12;
            },
            Self::BC => {
                bits = 9;
                let instr = u32::from(LittleEndian::read_u16(buf));

                unpacked = ((instr >> 12) & 0x1) << 8;
                unpacked |= ((instr >> 10) & 0x3) << 3;
                unpacked |= ((instr >> 5) & 0x3) << 6;
                unpacked |= ((instr >> 3) & 0x3) << 1;
                unpacked |= ((instr >> 2) & 0x1) << 5;
            },
            Self::JC => {
                bits = 12;
                let instr = u32::from(LittleEndian::read_u16(buf));

                unpacked = ((instr >> 12) & 0x1) << 11;
                unpacked |= ((instr >> 11) & 0x1) << 4;
                unpacked |= ((instr >> 9) & 0x3) << 8;
                unpacked |= ((instr >> 8) & 0x1) << 10;
                unpacked |= ((instr >> 7) & 0x1) << 6;
                unpacked |= ((instr >> 6) & 0x1) << 7;
                unpacked |= ((instr >> 3) & 0x7) << 1;
                unpacked |= ((instr >> 2) & 0x1) << 5;
            },
            Self::HI20 => {
                bits = 32;
                let instr = LittleEndian::read_u32(buf);

                unpacked = ((instr >> 12) & 0xFFFFF) << 12;
                // There's a problem here. We don't know the lower
                // bits of the value that is being read, but they do matter
                // if this thing would get relocated. luckily, riscv only does
                // relative relocations so this should never happen, and we
                // should be fine with just returning the value without adjustment
            },
            Self::LO12 => {
                bits = 12;
                let instr = LittleEndian::read_u32(buf);

                unpacked = (instr >> 20) & 0xFFF;
            },
            Self::LO12S => {
                bits = 12;
                let instr = LittleEndian::read_u32(buf);

                unpacked = (instr >> 7) & 0x1F;
                unpacked |= ((instr >> 25) & 0x7F) << 5;
            },
            Self::SPLIT32 => {
                bits = 32;
                let instr1 = LittleEndian::read_u32(&buf[..4]);
                let instr2 = LittleEndian::read_u32(&buf[4..]);

                unpacked = ((instr1 >> 12) & 0xFFFFF) << 12;
                let mut lower: u32 = (instr2 >> 20) & 0xFFF;

                // sign extend the lower part and then add them
                lower = (lower ^ 0x800).wrapping_sub(0x800);
                unpacked = unpacked.wrapping_add(lower)

            },
            Self::SPLIT32S => {
                bits = 32;
                let instr1 = LittleEndian::read_u32(&buf[..4]);
                let instr2 = LittleEndian::read_u32(&buf[4..]);

                unpacked = ((instr1 >> 12) & 0xFFFFF) << 12;
                let mut lower: u32 = (instr2 >> 7) & 0x1F;
                lower |= ((instr2 >> 25) & 0x7F) << 5;

                // sign extend the lower part and then add them
                lower = (lower ^ 0x800).wrapping_sub(0x800);
                unpacked = unpacked.wrapping_add(lower)
            },
        }

        // sign extension
        let offset = 1u64 << (bits - 1);
        let value: u64 = (unpacked as u64 ^ offset).wrapping_sub(offset);

        value as i64 as isize
    }
    fn kind(&self) -> RelocationKind {
        RelocationKind::Relative
    }
    fn page_size() -> usize {
        4096
    }
}

/// A RISC-V Assembler. This is aliased here for backwards compatability.
pub type Assembler = crate::Assembler<RiscvRelocation>;
/// A RISC-V AssemblyModifier. This is aliased here for backwards compatability.
pub type AssemblyModifier<'a> = crate::Modifier<'a, RiscvRelocation>;
/// A RISC-V UncommittedModifier. This is aliased here for backwards compatability.
pub type UncommittedModifier<'a> = crate::UncommittedModifier<'a>;

// these should explicitly never be inlined, as this is the slow path.
// that's also why these aren't made generic.

/// Handler for `u32` out-of-range riscv64 & riscv32 immediates.
#[inline(never)]
pub fn immediate_out_of_range_unsigned_32(immediate: u32) -> ! {
    panic!("Cannot assemble this RISC-V instruction. Immediate {immediate} is out of range.")
}

/// Handler for `i32` out-of-range riscv64 & riscv32 immediates.
#[inline(never)]
pub fn immediate_out_of_range_signed_32(immediate: i32) -> ! {
    panic!("Cannot assemble this RISC-V instruction. Immediate {immediate} is out of range.")
}
/// Handler for `u64` out-of-range riscv64 & riscv32 immediates.
#[inline(never)]
pub fn immediate_out_of_range_unsigned_64(immediate: u64) -> ! {
    panic!("Cannot assemble this RISC-V instruction. Immediate {immediate} is out of range.")
}

/// Handler for `i64` out-of-range riscv64 & riscv32 immediates.
#[inline(never)]
pub fn immediate_out_of_range_signed_64(immediate: i64) -> ! {
    panic!("Cannot assemble this RISC-V instruction. Immediate {immediate} is out of range.")
}

/// Handler for invalid riscv64 & riscv32 registers.
#[inline(never)]
pub fn invalid_register(register: u8) -> ! {
    panic!("Cannot assemble this RISC-V instruction. Register x{register} cannot be encoded.")
}


/// 4 or 8-byte general purpopse registers, where X0 is the zero register
/// When using the RV32/64E profile, only the first 16 registers are valid
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RX {
    X0 = 0x00, X1 = 0x01, X2 = 0x02, X3 = 0x03,
    X4 = 0x04, X5 = 0x05, X6 = 0x06, X7 = 0x07,
    X8 = 0x08, X9 = 0x09, X10= 0x0A, X11= 0x0B,
    X12= 0x0C, X13= 0x0D, X14= 0x0E, X15= 0x0F,
    X16= 0x10, X17= 0x11, X18= 0x12, X19= 0x13,
    X20= 0x14, X21= 0x15, X22= 0x16, X23= 0x17,
    X24= 0x18, X25= 0x19, X26= 0x1A, X27= 0x1B,
    X28= 0x1C, X29= 0x1D, X30= 0x1E, X31= 0x1F,
}
reg_impls!(RX);

/// 4, 8 or 16-byte floating point registers
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RF {
    F0 = 0x00, F1 = 0x01, F2 = 0x02, F3 = 0x03,
    F4 = 0x04, F5 = 0x05, F6 = 0x06, F7 = 0x07,
    F8 = 0x08, F9 = 0x09, F10= 0x0A, F11= 0x0B,
    F12= 0x0C, F13= 0x0D, F14= 0x0E, F15= 0x0F,
    F16= 0x10, F17= 0x11, F18= 0x12, F19= 0x13,
    F20= 0x14, F21= 0x15, F22= 0x16, F23= 0x17,
    F24= 0x18, F25= 0x19, F26= 0x1A, F27= 0x1B,
    F28= 0x1C, F29= 0x1D, F30= 0x1E, F31= 0x1F,
}
reg_impls!(RF);


#[cfg(test)]
mod tests {
    use super::RX::*;
    use crate::Register;

    #[test]
    fn reg_code() {
        assert_eq!(X2.code(), 2);
    }

    #[test]
    fn reg_code_from() {
        assert_eq!(u8::from(X24), 0x18);
    }
}
