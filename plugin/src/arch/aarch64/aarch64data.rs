use serialize::Size;
use super::ast::Modifier;

use lazy_static::lazy_static;
use std::collections::HashMap;

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

    // scalar simd reg, generic over size
    V,

    // vector simd regs
    VWild, // matches any vector reg without element specifier
    VSized(Size), // certain data type size, and indeterminate but present lane count, without element.
    VSizedStatic(Size, u8), // certain data type size and lane count, without element. (i.e. V.S4)
    VLanes(Size), // element specifier i.e. V.S4?[lane]
    VLanesStatic(Size, u8), // static element specifier i.e. V.S4?[lane]

    // register lists with .0 items, with the registers of size class .1
    RegList(u8, Size),
    // register list of a specific register size
    RegListSized(u8, Size, u8),
    // register list with lane specifier. element_size, amount
    RegListLanes(u8, Size),

    // jump offsets
    Offset,

    // references
    RefBase,
    RefOffset,
    RefPre,
    RefIndex,

    // a set of allowed modifiers
    Mod(&'static [Modifier]),

    // possible op mnemnonic end (everything after this point uses the default encoding)
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    // commands that advance the argument pointer
    R(u8), // encode a register, or reference base, into a 5-bit bitfield.
    R4(u8), // encode a register in the range 0-15 into a 4-bit bitfield

    // unsigned immediate encodings
    Ubits(u8, u8), // encodes an unsigned immediate starting at bit .0, .1 bits long
    Uscaled(u8, u8, u8), // encodes an unsigned immediate, starting at bit .0, .1 bits long, shifted .2 bits to the right before encoding
    Ulist(u8, &'static [u16]), // encodes an immediate that can only be a limited amount of options
    Urange(u8, u8, u8), // (loc, min, max) asserts the immediate is below or equal to max, encodes the value of (imm-min)
    UlogicalW(u8), // logical immediate encoding, 32-bit
    UlogicalX(u8), // logical immediate encoding, 64-bit
    Usub(u8, u8, u8), // encodes at .0, .1 bits long, .2 - value. Checks if the value is in range.
    Usumdec(u8, u8), // encodes at .0, .1 bits long, the value of the previous arg + the value of the current arg - 1
    Ufields(&'static [u8]), // an immediate, encoded bitwise with the highest bit going into field 0, up to the lowest going into the last bitfield.

    // signed immediate encodings
    Sbits(u8, u8), // encodes a signed immediate starting at bit .0, .1 bits long
    Sscaled(u8, u8, u8), // encodes a signed immediate, starting at bit .0, .1 bits long, shifted .2 bits to the right before encoding
    Sfloat(u8), // 8-bit encoded floating point literal

    // bit slice encodings. These don't advance the current argument. Only the slice argument actually encodes anything
    BUbits(u8), // checks if the pointed value fits in the given amount of bits
    BSbits(u8), // checks if the pointed value fits in the given amount of bits
    BUrange(u8, u8), // check if the pointed value is between min/max
    BUscaled(u8, u8), // check if the pointed value fits in .0 bits, after being shifted .1 bits to the right
    BSscaled(u8, u8), // check if the pointed value fits in .0 bits, after being shifted .1 bits to the right
    BUslice(u8, u8, u8), // encodes at .0, .1 bits long, the bitslice starting at .2 from the current arg
    BSslice(u8, u8, u8), // encodes at .0, .1 bits long, the bitslice starting at .2 from the current arg

    // special immediate encodings
    Special(u8, &'static str),

    // SIMD 128-bit indicator
    Rwidth(u8),

    // Extend/Shift fields
    Rotates(u8), // 2-bits field encoding [LSL, LSR, ASR, ROR]
    ExtendsW(u8), // 3-bits field encoding [UXTB, UXTH, UXTW, UXTX, SXTB, SXTH, SXTW, SXTX]. Additionally, LSL is interpreted as UXTW
    ExtendsX(u8), // 3-bits field encoding [UXTB, UXTH, UXTW, UXTX, SXTB, SXTH, SXTW, SXTX]. Additionally, LSL is interpreted as UXTX

    // Condition encodings.
    Cond(u8),
    CondInv(u8),

    // Mapping of literal -> bitvalue
    LitList(u8, &'static str),

    // special commands
    A, // advances the argument pointer, only needed to skip over an argument.
    C, // moves the argument pointer back.
    Static(u8, u32) // just insert these bits at this location.
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
            static MATCHERS: &'static [Matcher] = {
                #[allow(unused_imports)]
                use self::Matcher::*;
                &[ $(
                    $matcher
                ),* ]
            };
            static COMMANDS: &'static [Command] = {
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
                static DATA: &'static [Opdata] = &[ $(
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

lazy_static! {
    static ref OPMAP: HashMap<&'static str, &'static [Opdata]> = {
        let mut map = HashMap::new();

        use super::ast::Modifier::*;
        use serialize::Size::*;

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
    static ref SPECIAL_IDENT_MAP: HashMap<&'static str, HashMap<&'static str, u32>> = {
        let mut mapmap = HashMap::new();
        mapmap.insert("AT_OPS", {
            let mut map = HashMap::new();
            map.insert("S1E1R",  0b00001111000000);
            map.insert("S1E1W",  0b00001111000001);
            map.insert("S1E0R",  0b00001111000010);
            map.insert("S1E0W",  0b00001111000011);
            map.insert("S1E2R",  0b10001111000000);
            map.insert("S1E2W",  0b10001111000001);
            map.insert("S12E1R", 0b10001111000100);
            map.insert("S12E1W", 0b10001111000101);
            map.insert("S12E0R", 0b10001111000110);
            map.insert("S12E0W", 0b10001111000111);
            map.insert("S1E3R",  0b11001111000000);
            map.insert("S1E3W",  0b11001111000001);
            map.insert("S1E1RP", 0b00001111001000);
            map.insert("S1E1WP", 0b00001111001001);
            map
        });
        mapmap.insert("DC_OPS", {
            let mut map = HashMap::new();
            map.insert("IVAC",  0b00001110110001);
            map.insert("ISW",   0b00001110110010);
            map.insert("CSW",   0b00001111010010);
            map.insert("CISW",  0b00001111110010);
            map.insert("ZVA",   0b01101110100001);
            map.insert("CVAC",  0b01101111010001);
            map.insert("CVAU",  0b01101111011001);
            map.insert("CIVAC", 0b01101111110001);
            map.insert("CVAP",  0b01101111100001);
            map
        });
        mapmap.insert("BARRIER_OPS", {
            let mut map = HashMap::new();
            map.insert("SY",    0b1111);
            map.insert("ST",    0b1110);
            map.insert("LD",    0b1101);
            map.insert("ISH",   0b1011);
            map.insert("ISHST", 0b1010);
            map.insert("ISHLD", 0b1001);
            map.insert("NSH",   0b0111);
            map.insert("NSHST", 0b0110);
            map.insert("NSHLD", 0b0101);
            map.insert("OSH",   0b0011);
            map.insert("OSHST", 0b0010);
            map.insert("OSHLD", 0b0001);
            map
        });
        mapmap.insert("MSR_IMM_OPS", {
            let mut map = HashMap::new();
            map.insert("SPSel",   0b00001000000101);
            map.insert("DAIFSet", 0b01101000000110);
            map.insert("DAIFClr", 0b01101000000111);
            map.insert("UAO",     0b00001000000011);
            map.insert("PAN",     0b00001000000100);
            map.insert("DIT",     0b01101000000010);
            map
        });
        mapmap.insert("CONTROL_REGS", {
            let mut map = HashMap::new();
            map.insert("C0",  0);
            map.insert("C1",  1);
            map.insert("C2",  2);
            map.insert("C3",  3);
            map.insert("C4",  4);
            map.insert("C5",  5);
            map.insert("C6",  6);
            map.insert("C7",  7);
            map.insert("C8",  8);
            map.insert("C9",  9);
            map.insert("C10", 10);
            map.insert("C11", 11);
            map.insert("C12", 12);
            map.insert("C13", 13);
            map.insert("C14", 14);
            map.insert("C15", 15);
            map
        });
        mapmap.insert("TLBI_OPS", {
            let mut map = HashMap::new();
            map.insert("VMALLE1IS",    0b00010000011000);
            map.insert("VAE1IS",       0b00010000011001);
            map.insert("ASIDE1IS",     0b00010000011010);
            map.insert("VAAE1IS",      0b00010000011011);
            map.insert("VALE1IS",      0b00010000011101);
            map.insert("VAALE1IS",     0b00010000011111);
            map.insert("VMALLE1",      0b00010000111000);
            map.insert("VAE1",         0b00010000111001);
            map.insert("ASIDE1",       0b00010000111010);
            map.insert("VAAE1",        0b00010000111011);
            map.insert("VALE1",        0b00010000111101);
            map.insert("VAALE1",       0b00010000111111);
            map.insert("IPAS2E1IS",    0b10010000000001);
            map.insert("IPAS2LE1IS",   0b10010000000101);
            map.insert("ALLE2IS",      0b10010000011000);
            map.insert("VAE2IS",       0b10010000011001);
            map.insert("ALLE1IS",      0b10010000011100);
            map.insert("VALE2IS",      0b10010000011101);
            map.insert("VMALLS12E1IS", 0b10010000011110);
            map.insert("IPAS2E1",      0b10010000100001);
            map.insert("IPAS2LE1",     0b10010000100101);
            map.insert("ALLE2",        0b10010000111000);
            map.insert("VAE2",         0b10010000111001);
            map.insert("ALLE1",        0b10010000111100);
            map.insert("VALE2",        0b10010000111101);
            map.insert("VMALLS12E1",   0b10010000111110);
            map.insert("ALLE3IS",      0b11010000011000);
            map.insert("VAE3IS",       0b11010000011001);
            map.insert("VALE3IS",      0b11010000011101);
            map.insert("ALLE3",        0b11010000111000);
            map.insert("VAE3",         0b11010000111001);
            map.insert("VALE3",        0b11010000111101);
            map.insert("VMALLE1OS",    0b00010000001000);
            map.insert("VAE1OS",       0b00010000001001);
            map.insert("ASIDE1OS",     0b00010000001010);
            map.insert("VAAE1OS",      0b00010000001011);
            map.insert("VALE1OS",      0b00010000001101);
            map.insert("VAALE1OS",     0b00010000001111);
            map.insert("RVAE1IS",      0b00010000010001);
            map.insert("RVAAE1IS",     0b00010000010011);
            map.insert("RVALE1IS",     0b00010000010101);
            map.insert("RVAALE1IS",    0b00010000010111);
            map.insert("RVAE1OS",      0b00010000101001);
            map.insert("RVAAE1OS",     0b00010000101011);
            map.insert("RVALE1OS",     0b00010000101101);
            map.insert("RVAALE1OS",    0b00010000101111);
            map.insert("RVAE1",        0b00010000110001);
            map.insert("RVAAE1",       0b00010000110011);
            map.insert("RVALE1",       0b00010000110101);
            map.insert("RVAALE1",      0b00010000110111);
            map.insert("RIPAS2E1IS",   0b10010000000010);
            map.insert("RIPAS2LE1IS",  0b10010000000110);
            map.insert("ALLE2OS",      0b10010000001000);
            map.insert("VAE2OS",       0b10010000001001);
            map.insert("ALLE1OS",      0b10010000001100);
            map.insert("VALE2OS",      0b10010000001101);
            map.insert("VMALLS12E1OS", 0b10010000001110);
            map.insert("RVAE2IS",      0b10010000010001);
            map.insert("RVALE2IS",     0b10010000010101);
            map.insert("IPAS2E1OS",    0b10010000100000);
            map.insert("RIPAS2E1",     0b10010000100010);
            map.insert("RIPAS2E1OS",   0b10010000100011);
            map.insert("IPAS2LE1OS",   0b10010000100100);
            map.insert("RIPAS2LE1",    0b10010000100110);
            map.insert("RIPAS2LE1OS",  0b10010000100111);
            map.insert("RVAE2OS",      0b10010000101001);
            map.insert("RVALE2OS",     0b10010000101101);
            map.insert("RVAE2",        0b10010000110001);
            map.insert("RVALE2",       0b10010000110101);
            map.insert("ALLE3OS",      0b11010000001000);
            map.insert("VAE3OS",       0b11010000001001);
            map.insert("VALE3OS",      0b11010000001101);
            map.insert("RVAE3IS",      0b11010000010001);
            map.insert("RVALE3IS",     0b11010000010101);
            map.insert("RVAE3OS",      0b11010000101001);
            map.insert("RVALE3OS",     0b11010000101101);
            map.insert("RVAE3",        0b11010000110001);
            map.insert("RVALE3",       0b11010000110101);
            map
        });
        mapmap
    };
}
