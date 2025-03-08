//! RISC-V registers are simple. The registers contain no size information, this is purely encoded
//! in the target architecture and instruction
//! we currently have three known register families.
//! * General purpose registers, either denoted as x0-x31 or by specific names
//! * floating point registers, denoted as f0-f31 or by specific names
//! * vector registers, denoted as v0-v31
use proc_macro2::Span;
use crate::common::Jump;
use super::riscvdata::Opdata;


/// A generic register reference. Can be either a static RegId or a dynamic register from a family
#[derive(Debug, Clone)]
pub enum Register {
    Static(RegId),
    Dynamic(RegFamily, syn::Expr)
}

/// Unique identifiers for a specific register
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegId {
    // regular registers
    X0 = 0x00, X1 = 0x01, X2 = 0x02, X3 = 0x03, // zero, ra, sp, gp
    X4 = 0x04, X5 = 0x05, X6 = 0x06, X7 = 0x07, // tp, t0, t1, t2
    X8 = 0x08, X9 = 0x09, X10= 0x0A, X11= 0x0B, // s0, s1, a0, a1
    X12= 0x0C, X13= 0x0D, X14= 0x0E, X15= 0x0F, // a2, a3, a4, a5
    X16= 0x10, X17= 0x11, X18= 0x12, X19= 0x13, // a6, a7, s2, s3
    X20= 0x14, X21= 0x15, X22= 0x16, X23= 0x17, // s4, s5, s6, s7
    X24= 0x18, X25= 0x19, X26= 0x1A, X27= 0x1B, // s8, s9, s10,s11
    X28= 0x1C, X29= 0x1D, X30= 0x1E, X31= 0x1F, // t3, t4, t5, t6

    // floating point registers
    F0 = 0x20, F1 = 0x21, F2 = 0x22, F3 = 0x23,
    F4 = 0x24, F5 = 0x25, F6 = 0x26, F7 = 0x27,
    F8 = 0x28, F9 = 0x29, F10= 0x2A, F11= 0x2B,
    F12= 0x2C, F13= 0x2D, F14= 0x2E, F15= 0x2F,
    F16= 0x30, F17= 0x31, F18= 0x32, F19= 0x33,
    F20= 0x34, F21= 0x35, F22= 0x36, F23= 0x37,
    F24= 0x38, F25= 0x39, F26= 0x3A, F27= 0x3B,
    F28= 0x3C, F29= 0x3D, F30= 0x3E, F31= 0x3F,

    // vector registers
    V0 = 0x40, V1 = 0x41, V2 = 0x42, V3 = 0x43,
    V4 = 0x44, V5 = 0x45, V6 = 0x46, V7 = 0x47,
    V8 = 0x48, V9 = 0x49, V10= 0x4A, V11= 0x4B,
    V12= 0x4C, V13= 0x4D, V14= 0x4E, V15= 0x4F,
    V16= 0x50, V17= 0x51, V18= 0x52, V19= 0x53,
    V20= 0x54, V21= 0x55, V22= 0x56, V23= 0x57,
    V24= 0x58, V25= 0x59, V26= 0x5A, V27= 0x5B,
    V28= 0x5C, V29= 0x5D, V30= 0x5E, V31= 0x5F,
}

/// Register families
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegFamily {
    INTEGER = 0,
    FP = 1,
    VECTOR = 2,
}

impl RegId {
    /// Encode this RegId in a 5-bit value
    pub fn code(self) -> u8 {
        self as u8 & 0x1F
    }

    /// Returns the family of this Regid
    pub fn family(self) -> RegFamily {
        match self as u8 >> 5 {
            0 => RegFamily::INTEGER,
            1 => RegFamily::FP,
            2 => RegFamily::VECTOR,
            _ => unreachable!(),
        }
    }

    /// Returns this register as a string
    pub fn to_string(self) -> String {
        match self.family() {
            RegFamily::INTEGER => format!("x{}", self.code()),
            RegFamily::FP => format!("f{}", self.code()),
            RegFamily::VECTOR => format!("v{}", self.code()),
        }
    }
}

impl Register {
    /// Get the 5-bit code for this Register, if statically known.
    pub fn code(&self) -> Option<u8> {
        match self {
            Register::Static(code) => Some(code.code()),
            Register::Dynamic(_, _) => None
        }
    }

    /// Returns the family that this Register is of
    pub fn family(&self) -> RegFamily {
        match self {
            Register::Static(code) => code.family(),
            Register::Dynamic(family, _) => *family
        }
    }

    /// Returns true if this Register is dynamic
    pub fn is_dynamic(&self) -> bool {
        match self {
            Register::Static(_) => false,
            Register::Dynamic(_, _) => true
        }
    }

    /// Returns Some(RegId) if this Register is static
    pub fn as_id(&self) -> Option<RegId> {
        match self {
            Register::Static(id) => Some(*id),
            Register::Dynamic(_, _) => None
        }
    }
}


#[derive(Debug)]
pub enum RegListCount {
    Static(u8),
    Dynamic(syn::Expr),
    Single(Register),
    Double(Register, Register)
}


/// A RISC-V parsed instruction.
/// These are fairly simple. the format is "op" [ . "opext" ]* [ arg [ , arg ]* ]
/// where arg is
/// * an immediate (arbitrary expression)
/// * a label (in normal dynasm-rs style)
/// * a register (one of the above)
/// * a memory reference `expr? ( intreg ) `
/// * a register list {ra [, s0 [- s_n]]}
/// this last one is somewhat problematic, as just parsing the expr will normally swallow
/// the register reference as a call expression.
#[derive(Debug)]
pub enum RawArg {
    // An immediate, or potentially an identifier
    Immediate {
        value: syn::Expr
    },
    // A label
    JumpTarget {
        jump: Jump
    },
    // A register
    Register {
        span: Span,
        reg: Register
    },
    // A memory reference
    Reference {
        span: Span,
        offset: Option<syn::Expr>,
        base: Register,
    },
    // A register list. These only happen with a single family of instructions in the Zcmp extension
    RegisterList {
        span: Span,
        first: Register, // this should always be ra
        count: RegListCount
    },
}

/// The result of parsing a single instruction
#[derive(Debug)]
pub struct ParsedInstruction {
    pub name: String,
    pub span: Span,
    pub args: Vec<RawArg>
}

#[derive(Debug)]
pub enum RegListFlat {
    Static(u8),
    Dynamic(syn::Expr)
}

#[derive(Debug)]
pub enum FlatArg {
    Immediate {
        value: syn::Expr
    },
    JumpTarget {
        jump: Jump
    },
    Register {
        span: Span,
        reg: Register
    },
    RegisterList {
        span: Span,
        count: RegListFlat
    },
    Default
}

/// The result of finding a match for an instruction
#[derive(Debug)]
pub struct MatchData {
    pub data: &'static Opdata,
    pub args: Vec<FlatArg>
}
