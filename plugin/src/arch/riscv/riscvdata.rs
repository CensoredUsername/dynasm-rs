//! this file contains the datastructure specification for the RISC-V encoding data
use bitflags::bitflags;
use lazy_static::lazy_static;

use std::collections::{HashMap, hash_map};
use super::ast::RegId;

/// A template contains the information for the static parts of an instruction encoding, as well
/// as its bitsize and length
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Template {
    /// A 16-bit compressed instruction (Needed for the C extension)
    Compressed(u16),
    /// A single 32-bit instruction
    Single(u32),
    // Two 32-bit instructions. Used for load/store reg, offset
    Double(u32, u32),
    // / A long instruction sequence. Used to load big immediates.
    Many(&'static [u32])
}


bitflags! {
    /// Flags indicating what ISA targets an instruction is valid on
    #[derive(Debug, Clone, Copy)]
    pub struct ISAFlags: u8 {
        const RV32 = 0x01;
        const RV64 = 0x02;
    }


    /// Flags specifying what ISA extensions are required for an instruction
    // debug.rs opmap generation relies on the Ex_ prefixes being fixed,
    // so change that if editing this.
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
        const Ex_Zbs = 0x0000_0000_0001_0000;
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
        /// Zicbo: cache block management operations
        const Ex_Zicbom = 0x0000_0000_0100_0000;
        /// Zicbo: cache block prefetch operations
        const Ex_Zicbop = 0x0000_0000_0200_0000;
        /// Zicbo: cache block zero operations
        const Ex_Zicboz = 0x0000_0000_0400_0000;
        /// Zicfilp: control flow integrity landing pad
        const Ex_Zicfilp = 0x0000_0000_0800_0000;
        /// Zicfiss: Shadow stack
        const Ex_Zicfiss = 0x0000_0000_1000_0000;
        /// Zicntr: base counters and timers
        const Ex_Zicntr = 0x0000_0000_2000_0000;
        /// Zicond: conditional operations
        const Ex_Zicond = 0x0000_0000_4000_0000;
        /// Zicsr: control and status registers
        const Ex_Zicsr = 0x0000_0000_8000_0000;
        /// Zifencei: instruction-fetch fence
        const Ex_Zifencei = 0x0000_0001_0000_0000;
        /// Zihintntl: non-temporal hints
        const Ex_Zihintntl = 0x0000_0002_0000_0000;
        /// Zihintpause: pause hint
        const Ex_Zihintpause = 0x0000_0004_0000_0000;
        /// Zimop: may-be-operations
        const Ex_Zimop = 0x0000_0008_0000_0000;
        /// Zk: scalar cryptography
        const Ex_Zk = 0x0000_0010_0000_0000;
        /// Zkn: NIST algorithm suite
        const Ex_Zkn = 0x0000_0020_0000_0000;
        /// Zknd: NIST suite: AES decyrption
        const Ex_Zknd = 0x0000_0040_0000_0000;
        /// Zkne: NIST suite: AES encryption
        const Ex_Zkne = 0x0000_0080_0000_0000;
        /// Zknh: NIST suite: Hash functions
        const Ex_Zknh = 0x0000_0100_0000_0000;
        /// Zks: ShangMi algorithm suite
        const Ex_Zks = 0x0000_0200_0000_0000;
        /// Zksed: ShangMi suite: SM4 block cipher
        const Ex_Zksed = 0x0000_0400_0000_0000;
        /// Zksh: ShangMi suite: SM3 hash functions
        const Ex_Zksh = 0x0000_0800_0000_0000;
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

    /// formats the extension list into the standard RISC-V architecture specification format
    pub fn to_string(&self) -> String {
        let mut item = String::new();
        let mut encountered_z = false;

        // assemble extension sets. only use underscores between Z flags
        for (flag, bits) in self.iter_names() {
            let flag = &flag[3..];

            if encountered_z {
                item.push_str("_");
            }

            item.push_str(flag);

            if flag.starts_with("Z") {
                encountered_z = true;
            }
        }

        item
    }
}

impl Default for ExtensionFlags {
    fn default() -> ExtensionFlags {
        ExtensionFlags::Ex_I
    }
}


/// Matchers. These validate the types of arguments passed to an instruction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Matcher {
    /// A normal register
    X,

    /// A floating point register
    F,

    /// A specific register
    Reg(RegId),

    /// A vector register
    // V,

    /// An indirect reference to a register
    Ref,

    /// An indirect reference with offset. expands args to X, Imm
    RefOffset,

    /// An indirect reference with offset, with the base always being SP
    RefSp,

    /// An pc-relative indirect reference. The register ought to contain an address
    /// generated by AUIPC for the same label
    RefLabel,

    /// An immediate
    Imm,

    /// A jump offset
    Offset,

    /// a random ident
    Ident,

    /// a register list
    Xlist,

    /// a literal
    Lit(Literal)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Literal {
    RTZ
}

impl Literal {
    pub fn as_str(&self) -> &'static str {
        match self {
            Literal::RTZ => "rtz"
        }
    }
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

    /// A 3-bit encoding that allows specifying any of s0-s7 (x5-x7, x18-22), as used in cm.mva01s
    /// It also enforces that this is a different register from the previous one
    Rpops2(u8),

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

    /// Technically an immediate, but the way we encode this depends on the amount of items
    /// in the previous register list. The boolean indicates if the value should be negated.
    SPImm(u8, bool),

    // immediate handling, validation fields

    /// validate that the current arg is an unsigned value that fits in .0 bits, and that the
    /// lower .1 bits are 0
    UImm(u8, u8),

    /// validate that the current arg is an unsigned value that fits in .0 bits, and that the
    /// lower .1 bits are 0
    SImm(u8, u8),

    /// Same as SImm, but handles 64-bit values. No scaling support
    BigImm(u8),

    /// validate that the current arg is an unsigned value that fits in .0 bits, and that the
    /// lower .1 bits are 0. Also check that the immediate is not 0
    UImmNo0(u8, u8),

    /// validate that the current arg is an unsigned value that fits in .0 bits, and that the
    /// lower .1 bits are 0. Also check that the immediate is not 0
    SImmNo0(u8, u8),

    /// validate that the current arg is an unsigned value that fits in .0 bits, and that the
    /// lower .1 bits are 1
    UImmOdd(u8, u8),

    /// Validate that the given argument conforms to .0 >= arg >= .1 
    UImmRange(u32, u32),

    // immediate handling, encoding fields.

    /// Encode a slice of bits from a value .0 = offset, .1 = amount of bits, .2 = offset in value
    BitRange(u8, u8, u8),

    /// Encode a slice of bits from a value .0 = offset, .1 = amount of bits, .2 = offset in value,
    /// but add 1 << (offset - 1) to value before encoding to round it
    RBitRange(u8, u8, u8),

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
    // 32 bits, 12-bit scaled
    HI20 = 4,
    // loads, addi.
    // 12 bits, no scaling
    LO12 = 5,
    // stores
    // 12 bits, no scaling
    LO12S = 6,
    // pc-relative addrgen/load pseudo instructions
    // 32 bits, no scaling
    SPLIT32 = 7,
    // pc-relative store pseudo instructions
    // 32 bits, no scaling
    SPLIT32S = 8,
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
pub fn mnemonics() -> hash_map::Keys<'static, &'static str, &'static [Opdata]> {
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
        const Ex_Zicbom: u64 = ExtensionFlags::Ex_Zicbom.bits();
        const Ex_Zicbop: u64 = ExtensionFlags::Ex_Zicbop.bits();
        const Ex_Zicboz: u64 = ExtensionFlags::Ex_Zicboz.bits();
        const Ex_Zicfilp: u64 = ExtensionFlags::Ex_Zicfilp.bits();
        const Ex_Zicfiss: u64 = ExtensionFlags::Ex_Zicfiss.bits();
        const Ex_Zicntr: u64 = ExtensionFlags::Ex_Zicntr.bits();
        const Ex_Zicond: u64 = ExtensionFlags::Ex_Zicond.bits();
        const Ex_Zicsr: u64 = ExtensionFlags::Ex_Zicsr.bits();
        const Ex_Zifencei: u64 = ExtensionFlags::Ex_Zifencei.bits();
        const Ex_Zihintntl: u64 = ExtensionFlags::Ex_Zihintntl.bits();
        const Ex_Zihintpause: u64 = ExtensionFlags::Ex_Zihintpause.bits();
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

    pub static ref CSR_MAP: HashMap<&'static str, u16> = {
        let mut map = HashMap::new();

        map.insert("fflags", 0x001);
        map.insert("frm", 0x002);
        map.insert("fcsr", 0x003);
        map.insert("vstart", 0x008);
        map.insert("vxsat", 0x009);
        map.insert("vxrm", 0x00A);
        map.insert("vcsr", 0x00F);
        map.insert("ssp", 0x011);
        map.insert("seed", 0x015);
        map.insert("jvt", 0x017);
        map.insert("cycle", 0xC00);
        map.insert("time", 0xC01);
        map.insert("instret", 0xC02);
        map.insert("hpmcounter3", 0xC03);
        map.insert("hpmcounter4", 0xC04);
        map.insert("hpmcounter5", 0xC05);
        map.insert("hpmcounter6", 0xC06);
        map.insert("hpmcounter7", 0xC07);
        map.insert("hpmcounter8", 0xC08);
        map.insert("hpmcounter9", 0xC09);
        map.insert("hpmcounter10", 0xC0A);
        map.insert("hpmcounter11", 0xC0B);
        map.insert("hpmcounter12", 0xC0C);
        map.insert("hpmcounter13", 0xC0D);
        map.insert("hpmcounter14", 0xC0E);
        map.insert("hpmcounter15", 0xC0F);
        map.insert("hpmcounter16", 0xC10);
        map.insert("hpmcounter17", 0xC11);
        map.insert("hpmcounter18", 0xC12);
        map.insert("hpmcounter19", 0xC13);
        map.insert("hpmcounter20", 0xC14);
        map.insert("hpmcounter21", 0xC15);
        map.insert("hpmcounter22", 0xC16);
        map.insert("hpmcounter23", 0xC17);
        map.insert("hpmcounter24", 0xC18);
        map.insert("hpmcounter25", 0xC19);
        map.insert("hpmcounter26", 0xC1A);
        map.insert("hpmcounter27", 0xC1B);
        map.insert("hpmcounter28", 0xC1C);
        map.insert("hpmcounter29", 0xC1D);
        map.insert("hpmcounter30", 0xC1E);
        map.insert("hpmcounter31", 0xC1F);
        map.insert("vl", 0xC20);
        map.insert("vtype", 0xC21);
        map.insert("vlenb", 0xC22);
        map.insert("sstatus", 0x100);
        map.insert("sedeleg", 0x102);
        map.insert("sideleg", 0x103);
        map.insert("sie", 0x104);
        map.insert("stvec", 0x105);
        map.insert("scounteren", 0x106);
        map.insert("senvcfg", 0x10A);
        map.insert("sstateen0", 0x10C);
        map.insert("sstateen1", 0x10D);
        map.insert("sstateen2", 0x10E);
        map.insert("sstateen3", 0x10F);
        map.insert("scountinhibit", 0x120);
        map.insert("sscratch", 0x140);
        map.insert("sepc", 0x141);
        map.insert("scause", 0x142);
        map.insert("stval", 0x143);
        map.insert("sip", 0x144);
        map.insert("stimecmp", 0x14D);
        map.insert("sctrctl", 0x14E);
        map.insert("sctrstatus", 0x14F);
        map.insert("siselect", 0x150);
        map.insert("sireg", 0x151);
        map.insert("sireg2", 0x152);
        map.insert("sireg3", 0x153);
        map.insert("sireg4", 0x155);
        map.insert("sireg5", 0x156);
        map.insert("sireg6", 0x157);
        map.insert("stopei", 0x15C);
        map.insert("sctrdepth", 0x15F);
        map.insert("satp", 0x180);
        map.insert("srmcfg", 0x181);
        map.insert("scontext", 0x5A8);
        map.insert("vsstatus", 0x200);
        map.insert("vsie", 0x204);
        map.insert("vstvec", 0x205);
        map.insert("vsscratch", 0x240);
        map.insert("vsepc", 0x241);
        map.insert("vscause", 0x242);
        map.insert("vstval", 0x243);
        map.insert("vsip", 0x244);
        map.insert("vstimecmp", 0x24D);
        map.insert("vsctrctl", 0x24E);
        map.insert("vsiselect", 0x250);
        map.insert("vsireg", 0x251);
        map.insert("vsireg2", 0x252);
        map.insert("vsireg3", 0x253);
        map.insert("vsireg4", 0x255);
        map.insert("vsireg5", 0x256);
        map.insert("vsireg6", 0x257);
        map.insert("vstopei", 0x25C);
        map.insert("vsatp", 0x280);
        map.insert("hstatus", 0x600);
        map.insert("hedeleg", 0x602);
        map.insert("hideleg", 0x603);
        map.insert("hie", 0x604);
        map.insert("htimedelta", 0x605);
        map.insert("hcounteren", 0x606);
        map.insert("hgeie", 0x607);
        map.insert("hvien", 0x608);
        map.insert("hvictl", 0x609);
        map.insert("henvcfg", 0x60A);
        map.insert("hstateen0", 0x60C);
        map.insert("hstateen1", 0x60D);
        map.insert("hstateen2", 0x60E);
        map.insert("hstateen3", 0x60F);
        map.insert("htval", 0x643);
        map.insert("hip", 0x644);
        map.insert("hvip", 0x645);
        map.insert("hviprio1", 0x646);
        map.insert("hviprio2", 0x647);
        map.insert("htinst", 0x64A);
        map.insert("hgatp", 0x680);
        map.insert("hcontext", 0x6A8);
        map.insert("hgeip", 0xE12);
        map.insert("vstopi", 0xEB0);
        map.insert("scountovf", 0xDA0);
        map.insert("stopi", 0xDB0);
        map.insert("utvt", 0x007);
        map.insert("unxti", 0x045);
        map.insert("uintstatus", 0x046);
        map.insert("uscratchcsw", 0x048);
        map.insert("uscratchcswl", 0x049);
        map.insert("stvt", 0x107);
        map.insert("snxti", 0x145);
        map.insert("sintstatus", 0x146);
        map.insert("sscratchcsw", 0x148);
        map.insert("sscratchcswl", 0x149);
        map.insert("mtvt", 0x307);
        map.insert("mnxti", 0x345);
        map.insert("mintstatus", 0x346);
        map.insert("mscratchcsw", 0x348);
        map.insert("mscratchcswl", 0x349);
        map.insert("mstatus", 0x300);
        map.insert("misa", 0x301);
        map.insert("medeleg", 0x302);
        map.insert("mideleg", 0x303);
        map.insert("mie", 0x304);
        map.insert("mtvec", 0x305);
        map.insert("mcounteren", 0x306);
        map.insert("mvien", 0x308);
        map.insert("mvip", 0x309);
        map.insert("menvcfg", 0x30a);
        map.insert("mstateen0", 0x30C);
        map.insert("mstateen1", 0x30D);
        map.insert("mstateen2", 0x30E);
        map.insert("mstateen3", 0x30F);
        map.insert("mcountinhibit", 0x320);
        map.insert("mscratch", 0x340);
        map.insert("mepc", 0x341);
        map.insert("mcause", 0x342);
        map.insert("mtval", 0x343);
        map.insert("mip", 0x344);
        map.insert("mtinst", 0x34a);
        map.insert("mtval2", 0x34b);
        map.insert("mctrctl", 0x34E);
        map.insert("miselect", 0x350);
        map.insert("mireg", 0x351);
        map.insert("mireg2", 0x352);
        map.insert("mireg3", 0x353);
        map.insert("mireg4", 0x355);
        map.insert("mireg5", 0x356);
        map.insert("mireg6", 0x357);
        map.insert("mtopei", 0x35c);
        map.insert("pmpcfg0", 0x3a0);
        map.insert("pmpcfg1", 0x3a1);
        map.insert("pmpcfg2", 0x3a2);
        map.insert("pmpcfg3", 0x3a3);
        map.insert("pmpcfg4", 0x3a4);
        map.insert("pmpcfg5", 0x3a5);
        map.insert("pmpcfg6", 0x3a6);
        map.insert("pmpcfg7", 0x3a7);
        map.insert("pmpcfg8", 0x3a8);
        map.insert("pmpcfg9", 0x3a9);
        map.insert("pmpcfg10", 0x3aa);
        map.insert("pmpcfg11", 0x3ab);
        map.insert("pmpcfg12", 0x3ac);
        map.insert("pmpcfg13", 0x3ad);
        map.insert("pmpcfg14", 0x3ae);
        map.insert("pmpcfg15", 0x3af);
        map.insert("pmpaddr0", 0x3b0);
        map.insert("pmpaddr1", 0x3b1);
        map.insert("pmpaddr2", 0x3b2);
        map.insert("pmpaddr3", 0x3b3);
        map.insert("pmpaddr4", 0x3b4);
        map.insert("pmpaddr5", 0x3b5);
        map.insert("pmpaddr6", 0x3b6);
        map.insert("pmpaddr7", 0x3b7);
        map.insert("pmpaddr8", 0x3b8);
        map.insert("pmpaddr9", 0x3b9);
        map.insert("pmpaddr10", 0x3ba);
        map.insert("pmpaddr11", 0x3bb);
        map.insert("pmpaddr12", 0x3bc);
        map.insert("pmpaddr13", 0x3bd);
        map.insert("pmpaddr14", 0x3be);
        map.insert("pmpaddr15", 0x3bf);
        map.insert("pmpaddr16", 0x3c0);
        map.insert("pmpaddr17", 0x3c1);
        map.insert("pmpaddr18", 0x3c2);
        map.insert("pmpaddr19", 0x3c3);
        map.insert("pmpaddr20", 0x3c4);
        map.insert("pmpaddr21", 0x3c5);
        map.insert("pmpaddr22", 0x3c6);
        map.insert("pmpaddr23", 0x3c7);
        map.insert("pmpaddr24", 0x3c8);
        map.insert("pmpaddr25", 0x3c9);
        map.insert("pmpaddr26", 0x3ca);
        map.insert("pmpaddr27", 0x3cb);
        map.insert("pmpaddr28", 0x3cc);
        map.insert("pmpaddr29", 0x3cd);
        map.insert("pmpaddr30", 0x3ce);
        map.insert("pmpaddr31", 0x3cf);
        map.insert("pmpaddr32", 0x3d0);
        map.insert("pmpaddr33", 0x3d1);
        map.insert("pmpaddr34", 0x3d2);
        map.insert("pmpaddr35", 0x3d3);
        map.insert("pmpaddr36", 0x3d4);
        map.insert("pmpaddr37", 0x3d5);
        map.insert("pmpaddr38", 0x3d6);
        map.insert("pmpaddr39", 0x3d7);
        map.insert("pmpaddr40", 0x3d8);
        map.insert("pmpaddr41", 0x3d9);
        map.insert("pmpaddr42", 0x3da);
        map.insert("pmpaddr43", 0x3db);
        map.insert("pmpaddr44", 0x3dc);
        map.insert("pmpaddr45", 0x3dd);
        map.insert("pmpaddr46", 0x3de);
        map.insert("pmpaddr47", 0x3df);
        map.insert("pmpaddr48", 0x3e0);
        map.insert("pmpaddr49", 0x3e1);
        map.insert("pmpaddr50", 0x3e2);
        map.insert("pmpaddr51", 0x3e3);
        map.insert("pmpaddr52", 0x3e4);
        map.insert("pmpaddr53", 0x3e5);
        map.insert("pmpaddr54", 0x3e6);
        map.insert("pmpaddr55", 0x3e7);
        map.insert("pmpaddr56", 0x3e8);
        map.insert("pmpaddr57", 0x3e9);
        map.insert("pmpaddr58", 0x3ea);
        map.insert("pmpaddr59", 0x3eb);
        map.insert("pmpaddr60", 0x3ec);
        map.insert("pmpaddr61", 0x3ed);
        map.insert("pmpaddr62", 0x3ee);
        map.insert("pmpaddr63", 0x3ef);
        map.insert("mseccfg", 0x747);
        map.insert("tselect", 0x7a0);
        map.insert("tdata1", 0x7a1);
        map.insert("tdata2", 0x7a2);
        map.insert("tdata3", 0x7a3);
        map.insert("tinfo", 0x7a4);
        map.insert("tcontrol", 0x7a5);
        map.insert("mcontext", 0x7a8);
        map.insert("mscontext", 0x7aa);
        map.insert("dcsr", 0x7b0);
        map.insert("dpc", 0x7b1);
        map.insert("dscratch0", 0x7b2);
        map.insert("dscratch1", 0x7b3);
        map.insert("mcycle", 0xB00);
        map.insert("minstret", 0xB02);
        map.insert("mhpmcounter3", 0xB03);
        map.insert("mhpmcounter4", 0xB04);
        map.insert("mhpmcounter5", 0xB05);
        map.insert("mhpmcounter6", 0xB06);
        map.insert("mhpmcounter7", 0xB07);
        map.insert("mhpmcounter8", 0xB08);
        map.insert("mhpmcounter9", 0xB09);
        map.insert("mhpmcounter10", 0xB0A);
        map.insert("mhpmcounter11", 0xB0B);
        map.insert("mhpmcounter12", 0xB0C);
        map.insert("mhpmcounter13", 0xB0D);
        map.insert("mhpmcounter14", 0xB0E);
        map.insert("mhpmcounter15", 0xB0F);
        map.insert("mhpmcounter16", 0xB10);
        map.insert("mhpmcounter17", 0xB11);
        map.insert("mhpmcounter18", 0xB12);
        map.insert("mhpmcounter19", 0xB13);
        map.insert("mhpmcounter20", 0xB14);
        map.insert("mhpmcounter21", 0xB15);
        map.insert("mhpmcounter22", 0xB16);
        map.insert("mhpmcounter23", 0xB17);
        map.insert("mhpmcounter24", 0xB18);
        map.insert("mhpmcounter25", 0xB19);
        map.insert("mhpmcounter26", 0xB1A);
        map.insert("mhpmcounter27", 0xB1B);
        map.insert("mhpmcounter28", 0xB1C);
        map.insert("mhpmcounter29", 0xB1D);
        map.insert("mhpmcounter30", 0xB1E);
        map.insert("mhpmcounter31", 0xB1F);
        map.insert("mcyclecfg", 0x321);
        map.insert("minstretcfg", 0x322);
        map.insert("mhpmevent3", 0x323);
        map.insert("mhpmevent4", 0x324);
        map.insert("mhpmevent5", 0x325);
        map.insert("mhpmevent6", 0x326);
        map.insert("mhpmevent7", 0x327);
        map.insert("mhpmevent8", 0x328);
        map.insert("mhpmevent9", 0x329);
        map.insert("mhpmevent10", 0x32A);
        map.insert("mhpmevent11", 0x32B);
        map.insert("mhpmevent12", 0x32C);
        map.insert("mhpmevent13", 0x32D);
        map.insert("mhpmevent14", 0x32E);
        map.insert("mhpmevent15", 0x32F);
        map.insert("mhpmevent16", 0x330);
        map.insert("mhpmevent17", 0x331);
        map.insert("mhpmevent18", 0x332);
        map.insert("mhpmevent19", 0x333);
        map.insert("mhpmevent20", 0x334);
        map.insert("mhpmevent21", 0x335);
        map.insert("mhpmevent22", 0x336);
        map.insert("mhpmevent23", 0x337);
        map.insert("mhpmevent24", 0x338);
        map.insert("mhpmevent25", 0x339);
        map.insert("mhpmevent26", 0x33A);
        map.insert("mhpmevent27", 0x33B);
        map.insert("mhpmevent28", 0x33C);
        map.insert("mhpmevent29", 0x33D);
        map.insert("mhpmevent30", 0x33E);
        map.insert("mhpmevent31", 0x33F);
        map.insert("mvendorid", 0xF11);
        map.insert("marchid", 0xF12);
        map.insert("mimpid", 0xF13);
        map.insert("mhartid", 0xF14);
        map.insert("mconfigptr", 0xF15);
        map.insert("mtopi", 0xFB0);

        // 32-bit only csr's
        map.insert("sieh", 0x114);
        map.insert("siph", 0x154);
        map.insert("stimecmph", 0x15D);
        map.insert("vsieh", 0x214);
        map.insert("vsiph", 0x254);
        map.insert("vstimecmph", 0x25D);
        map.insert("hedelegh", 0x612);
        map.insert("htimedeltah", 0x615);
        map.insert("hidelegh", 0x613);
        map.insert("hvienh", 0x618);
        map.insert("henvcfgh", 0x61A);
        map.insert("hviph", 0x655);
        map.insert("hviprio1h", 0x656);
        map.insert("hviprio2h", 0x657);
        map.insert("hstateen0h", 0x61C);
        map.insert("hstateen1h", 0x61D);
        map.insert("hstateen2h", 0x61E);
        map.insert("hstateen3h", 0x61F);
        map.insert("cycleh", 0xC80);
        map.insert("timeh", 0xC81);
        map.insert("instreth", 0xC82);
        map.insert("hpmcounter3h", 0xC83);
        map.insert("hpmcounter4h", 0xC84);
        map.insert("hpmcounter5h", 0xC85);
        map.insert("hpmcounter6h", 0xC86);
        map.insert("hpmcounter7h", 0xC87);
        map.insert("hpmcounter8h", 0xC88);
        map.insert("hpmcounter9h", 0xC89);
        map.insert("hpmcounter10h", 0xC8A);
        map.insert("hpmcounter11h", 0xC8B);
        map.insert("hpmcounter12h", 0xC8C);
        map.insert("hpmcounter13h", 0xC8D);
        map.insert("hpmcounter14h", 0xC8E);
        map.insert("hpmcounter15h", 0xC8F);
        map.insert("hpmcounter16h", 0xC90);
        map.insert("hpmcounter17h", 0xC91);
        map.insert("hpmcounter18h", 0xC92);
        map.insert("hpmcounter19h", 0xC93);
        map.insert("hpmcounter20h", 0xC94);
        map.insert("hpmcounter21h", 0xC95);
        map.insert("hpmcounter22h", 0xC96);
        map.insert("hpmcounter23h", 0xC97);
        map.insert("hpmcounter24h", 0xC98);
        map.insert("hpmcounter25h", 0xC99);
        map.insert("hpmcounter26h", 0xC9A);
        map.insert("hpmcounter27h", 0xC9B);
        map.insert("hpmcounter28h", 0xC9C);
        map.insert("hpmcounter29h", 0xC9D);
        map.insert("hpmcounter30h", 0xC9E);
        map.insert("hpmcounter31h", 0xC9F);
        map.insert("mstatush", 0x310);
        map.insert("midelegh", 0x313);
        map.insert("mieh", 0x314);
        map.insert("mvienh", 0x318);
        map.insert("mviph", 0x319);
        map.insert("menvcfgh", 0x31A);
        map.insert("mstateen0h", 0x31C);
        map.insert("mstateen1h", 0x31D);
        map.insert("mstateen2h", 0x31E);
        map.insert("mstateen3h", 0x31F);
        map.insert("miph", 0x354);
        map.insert("mcyclecfgh", 0x721);
        map.insert("minstretcfgh", 0x722);
        map.insert("mhpmevent3h", 0x723);
        map.insert("mhpmevent4h", 0x724);
        map.insert("mhpmevent5h", 0x725);
        map.insert("mhpmevent6h", 0x726);
        map.insert("mhpmevent7h", 0x727);
        map.insert("mhpmevent8h", 0x728);
        map.insert("mhpmevent9h", 0x729);
        map.insert("mhpmevent10h", 0x72A);
        map.insert("mhpmevent11h", 0x72B);
        map.insert("mhpmevent12h", 0x72C);
        map.insert("mhpmevent13h", 0x72D);
        map.insert("mhpmevent14h", 0x72E);
        map.insert("mhpmevent15h", 0x72F);
        map.insert("mhpmevent16h", 0x730);
        map.insert("mhpmevent17h", 0x731);
        map.insert("mhpmevent18h", 0x732);
        map.insert("mhpmevent19h", 0x733);
        map.insert("mhpmevent20h", 0x734);
        map.insert("mhpmevent21h", 0x735);
        map.insert("mhpmevent22h", 0x736);
        map.insert("mhpmevent23h", 0x737);
        map.insert("mhpmevent24h", 0x738);
        map.insert("mhpmevent25h", 0x739);
        map.insert("mhpmevent26h", 0x73A);
        map.insert("mhpmevent27h", 0x73B);
        map.insert("mhpmevent28h", 0x73C);
        map.insert("mhpmevent29h", 0x73D);
        map.insert("mhpmevent30h", 0x73E);
        map.insert("mhpmevent31h", 0x73F);
        map.insert("mnscratch", 0x740);
        map.insert("mnepc", 0x741);
        map.insert("mncause", 0x742);
        map.insert("mnstatus", 0x744);
        map.insert("mseccfgh", 0x757);
        map.insert("mcycleh", 0xB80);
        map.insert("minstreth", 0xB82);
        map.insert("mhpmcounter3h", 0xB83);
        map.insert("mhpmcounter4h", 0xB84);
        map.insert("mhpmcounter5h", 0xB85);
        map.insert("mhpmcounter6h", 0xB86);
        map.insert("mhpmcounter7h", 0xB87);
        map.insert("mhpmcounter8h", 0xB88);
        map.insert("mhpmcounter9h", 0xB89);
        map.insert("mhpmcounter10h", 0xB8A);
        map.insert("mhpmcounter11h", 0xB8B);
        map.insert("mhpmcounter12h", 0xB8C);
        map.insert("mhpmcounter13h", 0xB8D);
        map.insert("mhpmcounter14h", 0xB8E);
        map.insert("mhpmcounter15h", 0xB8F);
        map.insert("mhpmcounter16h", 0xB90);
        map.insert("mhpmcounter17h", 0xB91);
        map.insert("mhpmcounter18h", 0xB92);
        map.insert("mhpmcounter19h", 0xB93);
        map.insert("mhpmcounter20h", 0xB94);
        map.insert("mhpmcounter21h", 0xB95);
        map.insert("mhpmcounter22h", 0xB96);
        map.insert("mhpmcounter23h", 0xB97);
        map.insert("mhpmcounter24h", 0xB98);
        map.insert("mhpmcounter25h", 0xB99);
        map.insert("mhpmcounter26h", 0xB9A);
        map.insert("mhpmcounter27h", 0xB9B);
        map.insert("mhpmcounter28h", 0xB9C);
        map.insert("mhpmcounter29h", 0xB9D);
        map.insert("mhpmcounter30h", 0xB9E);
        map.insert("mhpmcounter31h", 0xB9F);

        map
    };




    pub static ref FP_IMM_IDENT_MAP: HashMap<&'static str, u8> = {
        let mut map = HashMap::new();

        map.insert("min", 1);
        map.insert("inf", 30);
        map.insert("nan", 31);

        map
    };

    pub static ref FP_IMM_VALUE_MAP: &'static [(f32, u8)] = &[
        (-1.0, 0),
        (1.52587890625e-05, 2),
        (3.0517578125e-05, 3),
        (3.90625e-03, 4),
        (7.8125e-03, 5),
        (0.0625, 6),
        (0.125, 7),
        (0.25, 8),
        (0.3125, 9),
        (0.375, 10),
        (0.4375, 11),
        (0.5, 12),
        (0.625, 13),
        (0.75, 14),
        (0.875, 15),
        (1.0, 16),
        (1.25, 17),
        (1.5, 18),
        (1.75, 19),
        (2.0, 20),
        (2.5, 21),
        (3.0, 22),
        (4.0, 23),
        (8.0, 24),
        (16.0, 25),
        (128.0, 26),
        (256.0, 27),
        (32768.0, 28),
        (65536.0, 29),
    ];
}
