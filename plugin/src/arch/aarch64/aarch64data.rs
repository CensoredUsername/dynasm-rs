use crate::common::Size;
use super::ast::Modifier;

use lazy_static::lazy_static;
use std::collections::{HashMap, hash_map};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Matcher {
    // a literal "."
    Dot,

    // a specific literal (basically just an ident)
    Lit(&'static str),

    // immediate literal
    LitInt(u32),

    // float literal
    LitFloat(f32),

    // a random ident
    Ident,

    // a condition code literal
    Cond,

    // immediate
    Imm,

    // Wregisters, XRegisters, etc. match any static register in their family except for SP
    W,
    X,

    // same but addressing the stack pointer instead of the zero register. match any static register in their family except for ZR
    WSP,
    XSP,

    // scalar simd regs
    B,
    H,
    S,
    D,
    Q,

    // vector simd regs
    /// vector register with elements of the specified size. Accepts a lane count of either 64 or 128 total bits
    V(Size),
    /// vector register with elements of the specifized size, with the specified lane count
    VStatic(Size, u8),
    /// vector register with element specifier, with the element of the specified size. The lane count is unchecked.
    VElement(Size),
    /// vector register with element specifier, with the element of the specified size and the element index set to the provided value
    VElementStatic(Size, u8),
    /// vector register with elements of the specified size, with the specified lane count, with an element specifier
    VStaticElement(Size, u8),

    // register list with .0 items, with the elements of size .1
    RegList(u8, Size),
    // register list with .0 items, with the elements of size .1 and a lane count of .2
    RegListStatic(u8, Size, u8),
    // register list with element specifier. It has .0 items with a size of .1
    RegListElement(u8, Size),

    // jump offsets
    Offset,

    // references
    RefBase,
    RefOffset,
    RefPre,
    RefIndex,

    // a single modifier
    LitMod(Modifier),

    // a set of allowed modifiers
    Mod(&'static [Modifier]),

    // possible op mnemnonic end (everything after this point uses the default encoding)
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    // commands that advance the argument pointer
    R(u8), // encode a register, or reference base, into a 5-bit bitfield.
    REven(u8), // same as R, but requires that the register is even.
    RNoZr(u8), // same as R, but does not allow register 31.
    R4(u8), // encode a register in the range 0-15 into a 4-bit bitfield
    RNext, // encode that this register should be the previous register, plus one

    // unsigned immediate encodings
    Ubits(u8, u8), // encodes an unsigned immediate starting at bit .0, .1 bits long
    Uscaled(u8, u8, u8), // encodes an unsigned immediate, starting at bit .0, .1 bits long, shifted .2 bits to the right before encoding
    Ulist(u8, &'static [u16]), // encodes an immediate that can only be a limited amount of options
    Urange(u8, u8, u8), // (loc, min, max) asserts the immediate is below or equal to max, encodes the value of (imm-min)
    Usub(u8, u8, u8), // encodes at .0, .1 bits long, .2 - value. Checks if the value is in the range 1 ..= value
    Unegmod(u8, u8), // encodes at .0, .1 bits long, -value % (1 << .1). Checks if the value is in the range 0 .. value
    Usumdec(u8, u8), // encodes at .0, .1 bits long, the value of the previous arg + the value of the current arg - 1
    Ufields(&'static [u8]), // an immediate, encoded bitwise with the highest bit going into field 0, up to the lowest going into the last bitfield.

    // signed immediate encodings
    Sbits(u8, u8), // encodes a signed immediate starting at bit .0, .1 bits long
    Sscaled(u8, u8, u8), // encodes a signed immediate, starting at bit .0, .1 bits long, shifted .2 bits to the right before encoding

    // bit slice encodings. These don't advance the current argument. Only the slice argument actually encodes anything
    BUbits(u8), // checks if the pointed value fits in the given amount of bits
    BUsum(u8), // checks that the pointed value fits between 1 and (1 << .0) - prev
    BSscaled(u8, u8),
    BUrange(u8, u8), // check if the pointed value is between min/max
    Uslice(u8, u8, u8), // encodes at .0, .1 bits long, the bitslice starting at .2 from the current arg
    Sslice(u8, u8, u8), // encodes at .0, .1 bits long, the bitslice starting at .2 from the current arg

    // special immediate encodings
    Special(u8, SpecialComm),

    // SIMD 128-bit indicator
    Rwidth(u8),

    // Extend/Shift fields
    Rotates(u8), // 2-bits field encoding [LSL, LSR, ASR, ROR]
    ExtendsW(u8), // 3-bits field encoding [UXTB, UXTH, UXTW, UXTX, SXTB, SXTH, SXTW, SXTX]. Additionally, LSL is interpreted as UXTW
    ExtendsX(u8), // 3-bits field encoding [UXTB, UXTH, UXTW, UXTX, SXTB, SXTH, SXTW, SXTX]. Additionally, LSL is interpreted as UXTX

    // Condition encodings.
    /// Normal condition code 4-bit encoding
    Cond(u8),
    /// Condition 4-bit encoding, but the last bit is inverted. No AL/NV allowed
    CondInv(u8),

    // Mapping of literal -> bitvalue
    LitList(u8, &'static str),

    // Offsets
    Offset(Relocation),

    // special commands
    A, // advances the argument pointer, only needed to skip over an argument.
    C, // moves the argument pointer back.
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum SpecialComm {
    INVERTED_WIDE_IMMEDIATE_W,
    INVERTED_WIDE_IMMEDIATE_X,
    WIDE_IMMEDIATE_W,
    WIDE_IMMEDIATE_X,
    STRETCHED_IMMEDIATE,
    LOGICAL_IMMEDIATE_W,
    LOGICAL_IMMEDIATE_X,
    FLOAT_IMMEDIATE,
    SPLIT_FLOAT_IMMEDIATE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Relocation {
    // b, bl 26 bits, dword aligned
    B = 0,
    // b.cond, cbnz, cbz, ldr, ldrsw, prfm: 19 bits, dword aligned
    BCOND = 1,
    // adr split 21 bit, byte aligned
    ADR = 2,
    // adrp split 21 bit, 4096-byte aligned
    ADRP = 3,
    // tbnz, tbz: 14 bits, dword aligned
    TBZ = 4,
    // 32-bit literal
    LITERAL32 = 5,
    // 64-bit literal
    LITERAL64 = 6,
}

impl Relocation {
    pub fn to_id(&self) -> u8 {
        *self as u8
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Opdata {
    /// The base template for the encoding.
    pub base: u32,
    /// A set of matchers capable of matching the instruction encoding that this instruction represents.
    pub matchers: &'static [Matcher],
    /// A sequence of encoder commands that check the matched instruction on validity and whose output gets orred together with the original template at runtime.
    pub commands: &'static [Command]
}

macro_rules! SingleOp {
    ( $base:expr, [ $( $matcher:expr ),* ], [ $( $command:expr ),* ] ) => {
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
                &[ $(
                    $command
                ),* ]
            };
            Opdata {
                base: $base,
                matchers: MATCHERS,
                commands: COMMANDS,
            }
        }
    }
}

macro_rules! Ops {
    ( $bind:ident; $( $name:tt $(| $more:tt)* = [ $( $base:tt = [ $( $matcher:expr ),* ] => [ $( $command:expr ),* ] ; )+ ] )* ) => {
        {
            $({
                const DATA: &'static [Opdata] = &[ $(
                    SingleOp!( $base, [ $( $matcher ),* ], [ $( $command ),* ] )
                ),+ ];
                $bind.insert($name, DATA);
                $(
                    $bind.insert($more, DATA);
                )*
            })*
        }
    }
}

pub fn get_mnemonic_data(name: &str) -> Option<&'static [Opdata]> {
    OPMAP.get(&name).cloned()
}

#[allow(dead_code)]
pub fn mnemnonics() -> hash_map::Keys<'static, &'static str, &'static [Opdata]> {
    OPMAP.keys()
}

lazy_static! {
    static ref OPMAP: HashMap<&'static str, &'static [Opdata]> = {
        let mut map = HashMap::new();

        use super::ast::Modifier::*;
        use crate::common::Size::*;
        use self::SpecialComm::*;
        use self::Relocation::*;

        const EXTENDS: &'static [super::ast::Modifier] = &[UXTB, UXTH, UXTW, UXTX, SXTB, SXTH, SXTW, SXTX, LSL];
        const EXTENDS_W: &'static [super::ast::Modifier] = &[UXTB, UXTH, UXTW, SXTB, SXTH, SXTW];
        const EXTENDS_X: &'static [super::ast::Modifier] = &[UXTX, SXTX, LSL];
        const SHIFTS: &'static [super::ast::Modifier] = &[LSL, LSR, ASR];
        const ROTATES: &'static [super::ast::Modifier] = &[LSL, LSR, ASR, ROR];

        include!("opmap.rs");

        map
    };

    /// A map of existing condition codes and their normal encoding
    pub static ref COND_MAP: HashMap<&'static str, u8> = {
        let mut a = HashMap::new();

        a.insert("eq", 0);
        a.insert("ne", 1);
        a.insert("cs", 2);
        a.insert("hs", 2);
        a.insert("cc", 3);
        a.insert("lo", 3);
        a.insert("mi", 4);
        a.insert("pl", 5);
        a.insert("vs", 6);
        a.insert("vc", 7);
        a.insert("hi", 8);
        a.insert("ls", 9);
        a.insert("ge", 10);
        a.insert("lt", 11);
        a.insert("gt", 12);
        a.insert("le", 13);
        a.insert("al", 14);
        a.insert("nv", 15);

        a
    };

    // special ident maps
    pub static ref SPECIAL_IDENT_MAP: HashMap<&'static str, HashMap<&'static str, u32>> = {
        let mut mapmap = HashMap::new();
        mapmap.insert("AT_OPS", {
            let mut map = HashMap::new();
            map.insert("s1e1r",  0b00001111000000);
            map.insert("s1e1w",  0b00001111000001);
            map.insert("s1e0r",  0b00001111000010);
            map.insert("s1e0w",  0b00001111000011);
            map.insert("s1e2r",  0b10001111000000);
            map.insert("s1e2w",  0b10001111000001);
            map.insert("s12e1r", 0b10001111000100);
            map.insert("s12e1w", 0b10001111000101);
            map.insert("s12e0r", 0b10001111000110);
            map.insert("s12e0w", 0b10001111000111);
            map.insert("s1e3r",  0b11001111000000);
            map.insert("s1e3w",  0b11001111000001);
            map.insert("s1e1rp", 0b00001111001000);
            map.insert("s1e1wp", 0b00001111001001);
            map
        });
        mapmap.insert("IC_OPS", {
            let mut map = HashMap::new();
            map.insert("ialluis", 0b00001110001000);
            map.insert("iallu",   0b00001110101000);
            map
        });
        mapmap.insert("DC_OPS", {
            let mut map = HashMap::new();
            map.insert("ivac",  0b00001110110001);
            map.insert("isw",   0b00001110110010);
            map.insert("csw",   0b00001111010010);
            map.insert("cisw",  0b00001111110010);
            map.insert("zva",   0b01101110100001);
            map.insert("cvac",  0b01101111010001);
            map.insert("cvau",  0b01101111011001);
            map.insert("civac", 0b01101111110001);
            map.insert("cvap",  0b01101111100001);
            map
        });
        mapmap.insert("BARRIER_OPS", {
            let mut map = HashMap::new();
            map.insert("sy",    0b1111);
            map.insert("st",    0b1110);
            map.insert("ld",    0b1101);
            map.insert("ish",   0b1011);
            map.insert("ishst", 0b1010);
            map.insert("ishld", 0b1001);
            map.insert("nsh",   0b0111);
            map.insert("nshst", 0b0110);
            map.insert("nshld", 0b0101);
            map.insert("osh",   0b0011);
            map.insert("oshst", 0b0010);
            map.insert("oshld", 0b0001);
            map
        });
        mapmap.insert("MSR_IMM_OPS", {
            let mut map = HashMap::new();
            map.insert("spsel",   0b00001000000101);
            map.insert("daifset", 0b01101000000110);
            map.insert("daifclr", 0b01101000000111);
            map.insert("uao",     0b00001000000011);
            map.insert("pan",     0b00001000000100);
            map.insert("dit",     0b01101000000010);
            map
        });
        mapmap.insert("CONTROL_REGS", {
            let mut map = HashMap::new();
            map.insert("c0",  0);
            map.insert("c1",  1);
            map.insert("c2",  2);
            map.insert("c3",  3);
            map.insert("c4",  4);
            map.insert("c5",  5);
            map.insert("c6",  6);
            map.insert("c7",  7);
            map.insert("c8",  8);
            map.insert("c9",  9);
            map.insert("c10", 10);
            map.insert("c11", 11);
            map.insert("c12", 12);
            map.insert("c13", 13);
            map.insert("c14", 14);
            map.insert("c15", 15);
            map
        });
        mapmap.insert("TLBI_OPS", {
            let mut map = HashMap::new();
            map.insert("vmalle1is",    0b00010000011000);
            map.insert("vae1is",       0b00010000011001);
            map.insert("aside1is",     0b00010000011010);
            map.insert("vaae1is",      0b00010000011011);
            map.insert("vale1is",      0b00010000011101);
            map.insert("vaale1is",     0b00010000011111);
            map.insert("vmalle1",      0b00010000111000);
            map.insert("vae1",         0b00010000111001);
            map.insert("aside1",       0b00010000111010);
            map.insert("vaae1",        0b00010000111011);
            map.insert("vale1",        0b00010000111101);
            map.insert("vaale1",       0b00010000111111);
            map.insert("ipas2e1is",    0b10010000000001);
            map.insert("ipas2le1is",   0b10010000000101);
            map.insert("alle2is",      0b10010000011000);
            map.insert("vae2is",       0b10010000011001);
            map.insert("alle1is",      0b10010000011100);
            map.insert("vale2is",      0b10010000011101);
            map.insert("vmalls12e1is", 0b10010000011110);
            map.insert("ipas2e1",      0b10010000100001);
            map.insert("ipas2le1",     0b10010000100101);
            map.insert("alle2",        0b10010000111000);
            map.insert("vae2",         0b10010000111001);
            map.insert("alle1",        0b10010000111100);
            map.insert("vale2",        0b10010000111101);
            map.insert("vmalls12e1",   0b10010000111110);
            map.insert("alle3is",      0b11010000011000);
            map.insert("vae3is",       0b11010000011001);
            map.insert("vale3is",      0b11010000011101);
            map.insert("alle3",        0b11010000111000);
            map.insert("vae3",         0b11010000111001);
            map.insert("vale3",        0b11010000111101);
            map.insert("vmalle1os",    0b00010000001000);
            map.insert("vae1os",       0b00010000001001);
            map.insert("aside1os",     0b00010000001010);
            map.insert("vaae1os",      0b00010000001011);
            map.insert("vale1os",      0b00010000001101);
            map.insert("vaale1os",     0b00010000001111);
            map.insert("rvae1is",      0b00010000010001);
            map.insert("rvaae1is",     0b00010000010011);
            map.insert("rvale1is",     0b00010000010101);
            map.insert("rvaale1is",    0b00010000010111);
            map.insert("rvae1os",      0b00010000101001);
            map.insert("rvaae1os",     0b00010000101011);
            map.insert("rvale1os",     0b00010000101101);
            map.insert("rvaale1os",    0b00010000101111);
            map.insert("rvae1",        0b00010000110001);
            map.insert("rvaae1",       0b00010000110011);
            map.insert("rvale1",       0b00010000110101);
            map.insert("rvaale1",      0b00010000110111);
            map.insert("ripas2e1is",   0b10010000000010);
            map.insert("ripas2le1is",  0b10010000000110);
            map.insert("alle2os",      0b10010000001000);
            map.insert("vae2os",       0b10010000001001);
            map.insert("alle1os",      0b10010000001100);
            map.insert("vale2os",      0b10010000001101);
            map.insert("vmalls12e1os", 0b10010000001110);
            map.insert("rvae2is",      0b10010000010001);
            map.insert("rvale2is",     0b10010000010101);
            map.insert("ipas2e1os",    0b10010000100000);
            map.insert("ripas2e1",     0b10010000100010);
            map.insert("ripas2e1os",   0b10010000100011);
            map.insert("ipas2le1os",   0b10010000100100);
            map.insert("ripas2le1",    0b10010000100110);
            map.insert("ripas2le1os",  0b10010000100111);
            map.insert("rvae2os",      0b10010000101001);
            map.insert("rvale2os",     0b10010000101101);
            map.insert("rvae2",        0b10010000110001);
            map.insert("rvale2",       0b10010000110101);
            map.insert("alle3os",      0b11010000001000);
            map.insert("vae3os",       0b11010000001001);
            map.insert("vale3os",      0b11010000001101);
            map.insert("rvae3is",      0b11010000010001);
            map.insert("rvale3is",     0b11010000010101);
            map.insert("rvae3os",      0b11010000101001);
            map.insert("rvale3os",     0b11010000101101);
            map.insert("rvae3",        0b11010000110001);
            map.insert("rvale3",       0b11010000110101);
            map
        });
        mapmap
    };
}
