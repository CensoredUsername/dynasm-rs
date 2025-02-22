//! this file contains the datastructure specification for the RISC-V encoding data
use bitflags::bitflags;
use lazy_static::lazy_static;

use std::collections::{HashMap, hash_map};

/// A template contains the information for the static parts of an instruction encoding, as well
/// as its bitsize and length
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Template {
    /// A 16-bit compressed instruction (Needed for the C extension)
    Compressed(u16),
    /// A single 32-bit instruction
    Single(u32),
    // / A double 32-bit instruction. This is used to handle 32-bit offset jumps.
    // Double(u32, u32),
    // / A long instruction sequence. Used to load big immediates.
    // Many(&'static [u32])
}


bitflags! {
    /// Flags indicating what ISA targets an instruction is valid on
    #[derive(Debug, Clone, Copy)]
    pub struct ISAFlags: u8 {
        const RV32 = 0x01;
        const RV64 = 0x02;
    }


    /// Flags specifying what ISA extensions are required for an instruction
    #[derive(Debug, Clone, Copy)]
    pub struct ExtensionFlags: u64 {
        /// A: atomics
        const Ex_A = 0x0000_0000_0000_0001;
        /// C: compressed
        const Ex_C = 0x0000_0000_0000_0002;
        /// D: double fp support
        const Ex_D = 0x0000_0000_0000_0004;
        /// F: single fp support
        const Ex_F = 0x0000_0000_0000_0008;
        /// I: integer base instruction set
        const Ex_I = 0x0000_0000_0000_0010;
        /// M: multiply/divide
        const Ex_M = 0x0000_0000_0000_0020;
        /// Q: quad fp support
        const Ex_Q = 0x0000_0000_0000_0040;
        /// Zabha: byte and halfword atomics
        const Ex_Zabha = 0x0000_0000_0000_0080;
        /// Zacas: atomic compare and swap
        const Ex_Zacas = 0x0000_0000_0000_0100;
        /// Zawrs: wait-on-reservation-set
        const Ex_Zawrs = 0x0000_0000_0000_0200;
        /// Zba: bit manipulation address generation
        const Ex_Zba = 0x0000_0000_0000_0400;
        /// Zbb: basic bit manipulation
        const Ex_Zbb = 0x0000_0000_0000_0800;
        /// Zbc: carry-less multiplication
        const Ex_Zbc = 0x0000_0000_0000_1000;
        /// Zbkb: bit manipulation for cryptography
        const Ex_Zbkb = 0x0000_0000_0000_2000;
        /// Zbkc: carry-less multiplication for cryptography
        const Ex_Zbkc = 0x0000_0000_0000_4000;
        /// Zbkx: crossbar permutations
        const Ex_Zbkx = 0x0000_0000_0000_8000;
        /// Zbs: single-bit instructions
        const Ex_Zbs = 0x0000_0000_0001_000;
        /// Zcb: simple code-size saving instructions
        const Ex_Zcb = 0x0000_0000_0002_0000;
        /// Zcmop: compressed may-be-operations
        const Ex_Zcmop = 0x0000_0000_0004_0000;
        /// Zcmp: compressed instruction sequences
        const Ex_Zcmp = 0x0000_0000_0008_0000;
        /// Zcmt: compressed table jump instructions
        const Ex_Zcmt = 0x0000_0000_0010_0000;
        /// Zfa: additional floating point instructions
        const Ex_Zfa = 0x0000_0000_0020_0000;
        /// Zfbfmin: Scalar convert to/from BF16
        const Ex_Zfbfmin = 0x0000_0000_0040_0000;
        /// Zfh: half-width fp support
        const Ex_Zfh = 0x0000_0000_0080_0000;
        /// Zicbo: cache block operations
        const Ex_Zicbo = 0x0000_0000_0100_0000;
        /// Zicfilp: control flow integrity landing pad
        const Ex_Zicfilp = 0x0000_0000_0200_0000;
        /// Zicfiss: Shadow stack
        const Ex_Zicfiss = 0x0000_0000_0400_0000;
        /// Zicntr: base counters and timers
        const Ex_Zicntr = 0x0000_0000_0800_0000;
        /// Zicond: conditional operations
        const Ex_Zicond = 0x0000_0000_1000_0000;
        /// Zicsr: control and status registers
        const Ex_Zicsr = 0x0000_0000_2000_0000;
        /// Zifencei: instruction-fetch fence
        const Ex_Zifencei = 0x0000_0000_4000_0000;
        /// Zihintntl: non-temporal hints
        const Ex_Zihintntl = 0x0000_0000_8000_0000;
        /// Zimop: may-be-operations
        const Ex_Zimop = 0x0000_0001_0000_0000;
        /// Zk: scalar cryptography
        const Ex_Zk = 0x0000_0002_0000_0000;
        /// Zkn: NIST algorithm suite
        const Ex_Zkn = 0x0000_0004_0000_0000;
        /// Zknd: NIST suite: AES decyrption
        const Ex_Zknd = 0x0000_0008_0000_0000;
        /// Zkne: NIST suite: AES encryption
        const Ex_Zkne = 0x0000_0010_0000_0000;
        /// Zknh: NIST suite: Hash functions
        const Ex_Zknh = 0x0000_0020_0000_0000;
        /// Zks: ShangMi algorithm suite
        const Ex_Zks = 0x0000_0040_0000_0000;
        /// Zksed: ShangMi suite: SM4 block cipher
        const Ex_Zksed = 0x0000_0080_0000_0000;
        /// Zksh: ShangMi suite: SM3 hash functions
        const Ex_Zksh = 0x0000_0100_0000_0000;
    }
}


impl ISAFlags {
    const fn make(bits: u8) -> ISAFlags {
        ISAFlags::from_bits_truncate(bits)
    }
}


impl ExtensionFlags {
    const fn make(bits: u64) -> ExtensionFlags {
        ExtensionFlags::from_bits_truncate(bits)
    }
}


/// Matchers. These validate the types of arguments passed to an instruction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Matcher {
    /// A normal register
    X,

    /// A floating point register
    F,

    /// A vector register
    // V,

    /// An indirect reference to a register
    Ref,

    /// An indirect reference with offset. expands args to X, Imm
    RefOffset,

    /// An immediate
    Imm,

    /// A jump offset
    Offset,

    /// a random ident
    Ident,

    /// a register list
    Xlist
}


/// Encoding commands. They specify how arguments should be checked / encoded.
///
/// Unless otherwise stated, the first argument indicates the bottom-most bit of the affected bitfield
#[derive(Debug, Clone)]
pub enum Command {
    // Meta commands

    /// Repeat the same argument again, as it needs to be encoded twice
    Repeat,

    /// go to the next argument, if not done implicitly
    Next,

    // register fields

    /// A normal 5-bit register encoding. Argument specifies
    R(u8),

    /// A normal 5-bit register encoding, but the register must be even
    Reven(u8),

    /// A 5-bit register encoding that cannot be x0
    Rno0(u8),

    /// A 5-bit register encoding that cannot be x0 or x2
    Rno02(u8),

    /// A 3-bit encoding for "popular" registers (x8-x15
    Rpop(u8),

    /// A 3-bit encoding that allows specifying any of s0-s7 (x5-x7, x18-22), as used in cm.mva01s
    Rpops(u8),

    /// A register list, as used in cm.pop. values 3-15 encode ra, s0-s11
    Rlist(u8),

    // weird fields

    /// Rounding mode. 3-bit encoding indicating (RNE, RTZ, RDN, RUP, RMM, _, _, DYN). DYN default
    RoundingMode(u8),

    /// fence specification. 4-bit field encoding each of the letters iorw
    /// (input, output, read, write) to bits 3-0
    FenceSpec(u8),

    /// 12-bit field encoding CSRs. Used to provide support for encoding actual names
    Csr(u8),

    /// weird floating point immediate instruction
    FloatingPointImmediate(u8),


    // immediate handling, validation fields

    /// validate that the current arg is an unsigned value that fits in .0 bits, and that the
    /// lower .1 bits are 0
    UImm(u8, u8),

    /// validate that the current arg is an unsigned value that fits in .0 bits, and that the
    /// lower .1 bits are 0
    SImm(u8, u8),

    /// validate that the current arg is an unsigned value that fits in .0 bits, and that the
    /// lower .1 bits are 0. The value can also not be 0.
    UImmNo0(u8, u8),

    /// validate that the current arg is an unsigned value that fits in .0 bits, and that the
    /// lower .1 bits are 0. The value can also not be 0.
    SImmNo0(u8, u8),

    // weird ones

    /// Validate that the given argument conforms to .0 >= arg >= .1 
    UImmRange(u32, u32),

    /// validate that the current arg is an unsigned value that fits in .0 bits, and that the
    /// lower .1 bits are 1
    UImmOdd(u8, u8),

    /// validate that the current arg is a negative value, which, if negated, fits in .0 bits, and
    /// that the lower .1 bits of its negated representation are 0.
    NImm(u8, u8),

    // immediate handling, encoding fields.

    /// Encode a slice of bits from a value .0 = offset, .1 = amount of bits, .2 = offset in value
    BitRange(u8, u8, u8),

    /// Same as Bitrange, but negate the value before encoding
    NBitRange(u8, u8, u8),

    /// Encode at offset .0, bits from the argument specified by .1. so if it is [3, 5, 4]
    /// then at .0 we encode arg.bits[3], at .0+1 arg.bits[5], at .0+2 arg.bits[4]
    Bits(u8, &'static [u8]),

    /// some kind of offset for a jump.
    Offset(Relocation),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Relocation {
    // beq, beqz, bge, bgeu, bgez, bgt, bgtu, bgtz, ble, bleu, blez, blt, bltu, bltz, bne, bnez
    // 12 bits, 2-bit scaled
    B = 0,
    // j, jal
    // 20 bits, 2-bit scaled
    J = 1,
    // c.beqz, c.bnez
    // 9 bits, 2-bit scaled
    BC = 2,
    // c.j, c.jal
    // 12 bits, 2-bit scaled
    JC = 3,
    // auipc
    // 32-bits, 12-bit scaled
    AUIPC = 4,
    // jalr
    // 12-bits, no scaling
    JALR = 5,
}

impl Relocation {
    pub fn to_id(self) -> u8 {
        self as u8
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Opdata {
    /// The base template for the encoding.
    pub template: Template,
    /// What ISA targets this op is valid for
    pub isa_flags: ISAFlags,
    /// What (combination) of extensions is required for this instruction
    pub ext_flags: &'static [ExtensionFlags],
    /// A set of matchers capable of matching the instruction encoding that this instruction represents.
    pub matchers: &'static [Matcher],
    /// A sequence of encoder commands that check the matched instruction on validity and whose output gets orred together with the original template at runtime.
    pub commands: &'static [Command],
}

macro_rules! SingleOp {
    ( $template:expr, $isa:expr, [ $( $matcher:expr ),* ], [ $( $command:expr ),* ], [ $( $extension:expr ),* ] ) => {
        {
            const MATCHERS: &'static [Matcher] = {
                #[allow(unused_imports)]
                use self::Matcher::*;
                &[ $(
                    $matcher
                ),* ]
            };
            const COMMANDS: &'static [Command] = {
                #[allow(unused_imports)]
                use self::Command::*;
                #[allow(unused_imports)]
                use self::Relocation::*;
                &[ $(
                    $command
                ),* ]
            };
            const EXTENSIONS: &'static [ExtensionFlags] = {
                #[allow(unused_imports)]
                &[ $(
                    ExtensionFlags::make($extension)
                ),* ]
            };

            use self::Template::*;
            Opdata {
                template: $template,
                isa_flags: ISAFlags::make($isa),
                ext_flags: EXTENSIONS,
                matchers: MATCHERS,
                commands: COMMANDS,
            }
        }
    }
}

macro_rules! Ops {
    ( $( $name:tt = [ $( $template:expr , $isa:expr , [ $( $matcher:expr ),* ] => [ $( $command:expr ),* ] , [ $( $extension:expr ),* ] ; )+ ] , )* ) => {
        [ $(
            (
                $name,
                &[ $(
                    SingleOp!( $template, $isa, [ $( $matcher ),* ], [ $( $command ),* ], [ $( $extension ),* ] )
                ),+ ] as &[_]
            )
        ),* ]
    }
}

pub fn get_mnemonic_data(name: &str) -> Option<&'static [Opdata]> {
    OPMAP.get(&name).cloned()
}

#[allow(dead_code)]
pub fn mnemnonics() -> hash_map::Keys<'static, &'static str, &'static [Opdata]> {
    OPMAP.keys()
}


lazy_static!{
    static ref OPMAP: HashMap<&'static str, &'static [Opdata]> = {
        #![allow(non_upper_case_globals)]

        // we need to reimport these as we can't use bitflags in const
        const RV32: u8 = ISAFlags::RV32.bits();
        const RV64: u8 = ISAFlags::RV64.bits();

        // yes these too...
        const Ex_A: u64 = ExtensionFlags::Ex_A.bits();
        const Ex_C: u64 = ExtensionFlags::Ex_C.bits();
        const Ex_D: u64 = ExtensionFlags::Ex_D.bits();
        const Ex_F: u64 = ExtensionFlags::Ex_F.bits();
        const Ex_I: u64 = ExtensionFlags::Ex_I.bits();
        const Ex_M: u64 = ExtensionFlags::Ex_M.bits();
        const Ex_Q: u64 = ExtensionFlags::Ex_Q.bits();
        const Ex_Zabha: u64 = ExtensionFlags::Ex_Zabha.bits();
        const Ex_Zacas: u64 = ExtensionFlags::Ex_Zacas.bits();
        const Ex_Zawrs: u64 = ExtensionFlags::Ex_Zawrs.bits();
        const Ex_Zba: u64 = ExtensionFlags::Ex_Zba.bits();
        const Ex_Zbb: u64 = ExtensionFlags::Ex_Zbb.bits();
        const Ex_Zbc: u64 = ExtensionFlags::Ex_Zbc.bits();
        const Ex_Zbkb: u64 = ExtensionFlags::Ex_Zbkb.bits();
        const Ex_Zbkc: u64 = ExtensionFlags::Ex_Zbkc.bits();
        const Ex_Zbkx: u64 = ExtensionFlags::Ex_Zbkx.bits();
        const Ex_Zbs: u64 = ExtensionFlags::Ex_Zbs.bits();
        const Ex_Zcb: u64 = ExtensionFlags::Ex_Zcb.bits();
        const Ex_Zcmop: u64 = ExtensionFlags::Ex_Zcmop.bits();
        const Ex_Zcmp: u64 = ExtensionFlags::Ex_Zcmp.bits();
        const Ex_Zcmt: u64 = ExtensionFlags::Ex_Zcmt.bits();
        const Ex_Zfa: u64 = ExtensionFlags::Ex_Zfa.bits();
        const Ex_Zfbfmin: u64 = ExtensionFlags::Ex_Zfbfmin.bits();
        const Ex_Zfh: u64 = ExtensionFlags::Ex_Zfh.bits();
        const Ex_Zicbo: u64 = ExtensionFlags::Ex_Zicbo.bits();
        const Ex_Zicfilp: u64 = ExtensionFlags::Ex_Zicfilp.bits();
        const Ex_Zicfiss: u64 = ExtensionFlags::Ex_Zicfiss.bits();
        const Ex_Zicntr: u64 = ExtensionFlags::Ex_Zicntr.bits();
        const Ex_Zicond: u64 = ExtensionFlags::Ex_Zicond.bits();
        const Ex_Zicsr: u64 = ExtensionFlags::Ex_Zicsr.bits();
        const Ex_Zifencei: u64 = ExtensionFlags::Ex_Zifencei.bits();
        const Ex_Zihintntl: u64 = ExtensionFlags::Ex_Zihintntl.bits();
        const Ex_Zimop: u64 = ExtensionFlags::Ex_Zimop.bits();
        const Ex_Zk: u64 = ExtensionFlags::Ex_Zk.bits();
        const Ex_Zkn: u64 = ExtensionFlags::Ex_Zkn.bits();
        const Ex_Zknd: u64 = ExtensionFlags::Ex_Zknd.bits();
        const Ex_Zkne: u64 = ExtensionFlags::Ex_Zkne.bits();
        const Ex_Zknh: u64 = ExtensionFlags::Ex_Zknh.bits();
        const Ex_Zks: u64 = ExtensionFlags::Ex_Zks.bits();
        const Ex_Zksed: u64 = ExtensionFlags::Ex_Zksed.bits();
        const Ex_Zksh: u64 = ExtensionFlags::Ex_Zksh.bits();


        static MAP: &[(&str, &[Opdata])] = &include!("opmap.rs");
        MAP.iter().cloned().collect()
    };

    pub static ref ROUNDMODE_MAP: HashMap<&'static str, u8> = {
        let mut map = HashMap::new();

        map.insert("rne", 0);
        map.insert("rtz", 1);
        map.insert("rdn", 2);
        map.insert("rup", 3);
        map.insert("rmm", 4);
        map.insert("dyn", 7);

        map
    };

    pub static ref FENCESPEC_MAP: HashMap<&'static str, u8> = {
        let mut map = HashMap::new();

        map.insert("w", 1);
        map.insert("r", 2);
        map.insert("rw", 3);
        map.insert("o", 4);
        map.insert("ow", 5);
        map.insert("or", 6);
        map.insert("orw", 7);
        map.insert("i", 8);
        map.insert("iw", 9);
        map.insert("ir", 10);
        map.insert("irw", 11);
        map.insert("io", 12);
        map.insert("iow", 13);
        map.insert("ior", 14);
        map.insert("iorw", 15);

        map
    };
}
