use std::collections::{HashMap, hash_map};

use compiler::Opdata;


macro_rules! constify {
    ($t:ty, $e:expr) => { {const C: &'static $t = &$e; C} }
}

macro_rules! OpInner {
    ($fmt:expr, $ops:expr, $reg:expr)          => { Opdata {args: $fmt, ops: constify!([u8], $ops), reg: $reg, flags: flags::make_flag( 0) }  };
    ($fmt:expr, $ops:expr, $reg:expr, $f:expr) => { Opdata {args: $fmt, ops: constify!([u8], $ops), reg: $reg, flags: flags::make_flag($f) }  };
}

macro_rules! Ops {
    ( $bind:ident; $( $name:tt $(| $more:tt)* = [ $( $( $e:expr ),+ ; )+ ] )* ) => { 
        lazy_static! {
            static ref $bind: HashMap<&'static str, &'static [Opdata]> = {
                let mut map = HashMap::new();
                const X: u8 = 0xFF;
                $({
                    const DATA: &'static [Opdata] = &[$( OpInner!($( $e ),*) ,)+];
                    map.insert($name, DATA);
                    $(
                        map.insert($more, DATA);
                    )*
                })+
                map
            };
        }
    };
}

pub fn get_mnemnonic_data(name: &str) -> Option<&'static [Opdata]> {
    return OPMAP.get(&name).map(|x| *x);
}
#[macro_use]
pub mod flags {
    bitflags! {
        pub flags Flags: u32 {
            const VEX_OP    = 0x0000_0001, // this instruction requires a VEX prefix to be encoded
            const XOP_OP    = 0x0000_0002, // this instruction requires a XOP prefix to be encoded

            // note: the first 4 in this block are mutually exclusive
            const AUTO_SIZE = 0x0000_0004, // 16 bit -> OPSIZE , 32-bit -> None   , 64-bit -> REX.W/VEX.W/XOP.W
            const AUTO_NO32 = 0x0000_0008, // 16 bit -> OPSIZE , 32-bit -> illegal, 64-bit -> None
            const AUTO_REXW = 0x0000_0010, // 16 bit -> illegal, 32-bit -> None   , 64-bit -> REX.W/VEX.W/XOP.W
            const AUTO_VEXL = 0x0000_0020, // 128bit -> None   , 256bit -> VEX.L
            const WORD_SIZE = 0x0000_0040, // implies opsize prefix
            const WITH_REXW = 0x0000_0080, // implies REX.W/VEX.W/XOP.W
            const WITH_VEXL = 0x0000_0100, // implies VEX.L/XOP.L

            const PREF_66   = WORD_SIZE.bits,// mandatory prefix (same as WORD_SIZE)
            const PREF_67   = 0x0000_0200, // mandatory prefix (same as SMALL_ADDRESS)
            const PREF_F0   = 0x0000_0400, // mandatory prefix (same as LOCK)
            const PREF_F2   = 0x0000_0800, // mandatory prefix (REPNE)
            const PREF_F3   = 0x0000_1000, // mandatory prefix (REP)

            const LOCK      = 0x0000_2000, // user lock prefix is valid with this instruction
            const REP       = 0x0000_4000, // user rep prefix is valid with this instruction
            const REPE      = 0x0000_8000,

            const SHORT_ARG = 0x0001_0000, // a register argument is encoded in the last byte of the opcode
            const ENC_MR    = 0x0002_0000, //  select alternate arg encoding
            const ENC_VM    = 0x0004_0000, //  select alternate arg encoding
        }
    }
    // workaround until bitflags can be used in const
    pub const fn flag_bits(flag: Flags) -> u32 {
        flag.bits
    }
    pub const fn make_flag(bits: u32) -> Flags {
        Flags {bits: bits}
    }
}

pub fn mnemnonics() -> hash_map::Keys<'static, &'static str, &'static [Opdata]> {
    OPMAP.keys()
}

// workaround until bitflags can be used in const
const VEX_OP   : u32 = flags::flag_bits(flags::VEX_OP);
const XOP_OP   : u32 = flags::flag_bits(flags::XOP_OP);
const SHORT_ARG: u32 = flags::flag_bits(flags::SHORT_ARG);
const AUTO_SIZE: u32 = flags::flag_bits(flags::AUTO_SIZE);
const AUTO_NO32: u32 = flags::flag_bits(flags::AUTO_NO32);
const AUTO_REXW: u32 = flags::flag_bits(flags::AUTO_REXW);
const AUTO_VEXL: u32 = flags::flag_bits(flags::AUTO_VEXL);
const WORD_SIZE: u32 = flags::flag_bits(flags::WORD_SIZE);
const WITH_REXW: u32 = flags::flag_bits(flags::WITH_REXW);
const WITH_VEXL: u32 = flags::flag_bits(flags::WITH_VEXL);
const PREF_66  : u32 = flags::flag_bits(flags::PREF_66);
const PREF_67  : u32 = flags::flag_bits(flags::PREF_67);
const PREF_F0  : u32 = flags::flag_bits(flags::PREF_F0);
const PREF_F2  : u32 = flags::flag_bits(flags::PREF_F2);
const PREF_F3  : u32 = flags::flag_bits(flags::PREF_F3);
const LOCK     : u32 = flags::flag_bits(flags::LOCK);
const REP      : u32 = flags::flag_bits(flags::REP);
const REPE     : u32 = flags::flag_bits(flags::REPE);
const ENC_MR   : u32 = flags::flag_bits(flags::ENC_MR);
const ENC_VM   : u32 = flags::flag_bits(flags::ENC_VM);

Ops!(OPMAP;
// general purpose instructions according to AMD's AMD64 Arch Programmer's Manual Vol. 3
  "adc"         = [ b"A*i*",     [0x15            ], X, AUTO_SIZE;
                    b"Abib",     [0x14            ], X;
                    b"v*i*",     [0x81            ], 2, AUTO_SIZE | LOCK;
                    b"v*ib",     [0x83            ], 2, AUTO_SIZE | LOCK;
                    b"vbib",     [0x80            ], 2,             LOCK;
                    b"v*r*",     [0x11            ], X, AUTO_SIZE | LOCK;
                    b"vbrb",     [0x10            ], X,             LOCK;
                    b"r*v*",     [0x13            ], X, AUTO_SIZE;
                    b"rbvb",     [0x12            ], X;
] "add"         = [ b"A*i*",     [0x05            ], X, AUTO_SIZE;
                    b"Abib",     [0x04            ], X;
                    b"v*i*",     [0x81            ], 0, AUTO_SIZE | LOCK;
                    b"v*ib",     [0x83            ], 0, AUTO_SIZE | LOCK;
                    b"vbib",     [0x80            ], 0,             LOCK;
                    b"v*r*",     [0x01            ], X, AUTO_SIZE | LOCK;
                    b"vbrb",     [0x00            ], X,             LOCK;
                    b"r*v*",     [0x03            ], X, AUTO_SIZE;
                    b"rbvb",     [0x02            ], X;
] "and"         = [ b"A*i*",     [0x25            ], X, AUTO_SIZE;
                    b"Abib",     [0x24            ], X;
                    b"v*i*",     [0x81            ], 4, AUTO_SIZE | LOCK;
                    b"v*ib",     [0x83            ], 4, AUTO_SIZE | LOCK;
                    b"vbib",     [0x80            ], 4,             LOCK;
                    b"v*r*",     [0x21            ], X, AUTO_SIZE | LOCK;
                    b"vbrb",     [0x20            ], X,             LOCK;
                    b"r*v*",     [0x23            ], X, AUTO_SIZE;
                    b"rbvb",     [0x22            ], X;
] "andn"        = [ b"r*r*v*",   [   2, 0xF2      ], X, AUTO_REXW | VEX_OP;
] "bextr"       = [ b"r*v*r*",   [   2, 0xF7      ], X, AUTO_REXW | VEX_OP;
                    b"r*v*id",   [  10, 0x10      ], X, AUTO_REXW | XOP_OP;
] "blcfill"     = [ b"r*v*",     [   9, 0x01      ], 1, AUTO_REXW | XOP_OP | ENC_VM;
] "blci"        = [ b"r*v*",     [   9, 0x02      ], 6, AUTO_REXW | XOP_OP | ENC_VM;
] "blcic"       = [ b"r*v*",     [   9, 0x01      ], 5, AUTO_REXW | XOP_OP | ENC_VM;
] "blcmsk"      = [ b"r*v*",     [   9, 0x02      ], 1, AUTO_REXW | XOP_OP | ENC_VM;
] "blcs"        = [ b"r*v*",     [   9, 0x01      ], 3, AUTO_REXW | XOP_OP | ENC_VM;
] "blsfill"     = [ b"r*v*",     [   9, 0x01      ], 2, AUTO_REXW | XOP_OP | ENC_VM;
] "blsi"        = [ b"r*v*",     [   2, 0xF3      ], 3, AUTO_REXW | VEX_OP | ENC_VM;
] "blsic"       = [ b"r*v*",     [   9, 0x01      ], 6, AUTO_REXW | XOP_OP | ENC_VM;
] "blsmsk"      = [ b"r*v*",     [   2, 0xF3      ], 2, AUTO_REXW | VEX_OP | ENC_VM;
] "blsr"        = [ b"r*v*",     [   2, 0xF3      ], 1, AUTO_REXW | VEX_OP | ENC_VM;
] "bsf"         = [ b"r*v*",     [0x0F, 0xBC      ], X, AUTO_SIZE;
] "bsr"         = [ b"r*v*",     [0x0F, 0xBD      ], X, AUTO_SIZE;
] "bswap"       = [ b"r*",       [0x0F, 0xC8      ], 0, AUTO_REXW;
] "bt"          = [ b"v*r*",     [0x0F, 0xA3      ], X, AUTO_SIZE;
                    b"v*ib",     [0x0F, 0xBA      ], 4, AUTO_SIZE;
] "btc"         = [ b"v*r*",     [0x0F, 0xBB      ], X, AUTO_SIZE | LOCK;
                    b"v*ib",     [0x0F, 0xBA      ], 7, AUTO_SIZE | LOCK;
] "btr"         = [ b"v*r*",     [0x0F, 0xB3      ], X, AUTO_SIZE | LOCK;
                    b"v*ib",     [0x0F, 0xBA      ], 6, AUTO_SIZE | LOCK;
] "bts"         = [ b"v*r*",     [0x0F, 0xAB      ], X, AUTO_SIZE | LOCK;
                    b"v*ib",     [0x0F, 0xBA      ], 5, AUTO_SIZE | LOCK;
] "bzhi"        = [ b"r*v*r*",   [   2, 0xF5      ], X, AUTO_REXW | VEX_OP;
] "call"        = [ b"o*",       [0xE8            ], X, AUTO_SIZE;
                    b"r*",       [0xFF            ], 2, AUTO_NO32;
] "cbw"         = [ b"",         [0x98            ], X, WORD_SIZE;
] "cwde"        = [ b"",         [0x98            ], X;
] "cdqe"        = [ b"",         [0x98            ], X, WITH_REXW;
] "cwd"         = [ b"",         [0x99            ], X, WORD_SIZE;
] "cdq"         = [ b"",         [0x99            ], X;
] "cqo"         = [ b"",         [0x99            ], X, WITH_REXW;
] "clc"         = [ b"",         [0xF8            ], X;
] "cld"         = [ b"",         [0xFC            ], X;
] "clflush"     = [ b"mb",       [0x0F, 0xAE      ], 7;
] "cmc"         = [ b"",         [0xF5            ], X;
] "cmovo"       = [ b"r*v*",     [0x0F, 0x40      ], X, AUTO_SIZE;
] "cmovno"      = [ b"r*v*",     [0x0F, 0x41      ], X, AUTO_SIZE;
] "cmovb"       |
  "cmovc"       |
  "cmovnae"     = [ b"r*v*",     [0x0F, 0x42      ], X, AUTO_SIZE;
] "cmovnb"      |
  "cmovnc"      |
  "cmovae"      = [ b"r*v*",     [0x0F, 0x43      ], X, AUTO_SIZE;
] "cmovz"       |
  "cmove"       = [ b"r*v*",     [0x0F, 0x44      ], X, AUTO_SIZE;
] "cmovnz"      |
  "cmovne"      = [ b"r*v*",     [0x0F, 0x45      ], X, AUTO_SIZE;
] "cmovbe"      |
  "cmovna"      = [ b"r*v*",     [0x0F, 0x46      ], X, AUTO_SIZE;
] "cmovnbe"     |
  "cmova"       = [ b"r*v*",     [0x0F, 0x47      ], X, AUTO_SIZE;
] "cmovs"       = [ b"r*v*",     [0x0F, 0x48      ], X, AUTO_SIZE;
] "cmovns"      = [ b"r*v*",     [0x0F, 0x49      ], X, AUTO_SIZE;
] "cmovp"       |
  "cmovpe"      = [ b"r*v*",     [0x0F, 0x4A      ], X, AUTO_SIZE;
] "cmovnp"      |
  "cmovpo"      = [ b"r*v*",     [0x0F, 0x4B      ], X, AUTO_SIZE;
] "cmovl"       |
  "cmovnge"     = [ b"r*v*",     [0x0F, 0x4C      ], X, AUTO_SIZE;
] "cmovnl"      |
  "cmovge"      = [ b"r*v*",     [0x0F, 0x4D      ], X, AUTO_SIZE;
] "cmovle"      |
  "cmovng"      = [ b"r*v*",     [0x0F, 0x4E      ], X, AUTO_SIZE;
] "cmovnle"     |
  "cmovg"       = [ b"r*v*",     [0x0F, 0x4F      ], X, AUTO_SIZE;
] "cmp"         = [ b"A*i*",     [0x3C            ], X, AUTO_SIZE;
                    b"Abib",     [0x3D            ], X;
                    b"v*i*",     [0x81            ], 7, AUTO_SIZE;
                    b"v*ib",     [0x83            ], 7, AUTO_SIZE;
                    b"vbib",     [0x80            ], 7;
                    b"v*r*",     [0x39            ], X, AUTO_SIZE;
                    b"vbrb",     [0x38            ], X;
                    b"r*v*",     [0x3B            ], X, AUTO_SIZE;
                    b"rbvb",     [0x3A            ], X;
] "cmpsb"       = [ b"",         [0xA6            ], X,             REPE;
] "cmpsw"       = [ b"",         [0xA7            ], X, WORD_SIZE | REPE;
] "cmpsd"       = [ b"",         [0xA7            ], X,             REPE;
                    b"yowoib",   [0x0F, 0xC2      ], X, PREF_F2;
] "cmpsq"       = [ b"",         [0xA7            ], X, WITH_REXW | REP;
] "cmpxchg"     = [ b"v*r*",     [0x0F, 0xB1      ], X, AUTO_SIZE | LOCK;
                    b"vbrb",     [0x0F, 0xB0      ], X,             LOCK;
] "cmpxchg8b"   = [ b"mq",       [0x0F, 0xC7      ], 1,             LOCK;
] "cmpxchg16b"  = [ b"mo",       [0x0F, 0xC7      ], 1, WITH_REXW | LOCK;
] "cpuid"       = [ b"",         [0x0F, 0xA2      ], X;
] "crc32"       = [ b"r*vb",     [0x0F, 0x38, 0xF0], X, AUTO_REXW | PREF_F2; // unique size encoding scheme
                    b"rdvw",     [0x0F, 0x38, 0xF1], X, WORD_SIZE | PREF_F2; // also odd default
                    b"r*v*",     [0x0F, 0x38, 0xF1], X, AUTO_REXW | PREF_F2;
] "dec"         = [ b"v*",       [0xFF            ], 1, AUTO_SIZE | LOCK;
                    b"vb",       [0xFE            ], 1,             LOCK;
] "div"         = [ b"v*",       [0xF7            ], 6, AUTO_SIZE;
                    b"vb",       [0xF6            ], 6;
] "enter"       = [ b"iwib",     [0xC8            ], X;
] "idiv"        = [ b"v*",       [0xF7            ], 7, AUTO_SIZE;
                    b"vb",       [0xF6            ], 7;
] "imul"        = [ b"v*",       [0xF7            ], 5, AUTO_SIZE;
                    b"vb",       [0xF6            ], 5;
                    b"r*v*",     [0x0F, 0xAF      ], X, AUTO_SIZE;
                    b"r*v*i*",   [0x69            ], X, AUTO_SIZE;
                    b"r*v*ib",   [0x68            ], X, AUTO_SIZE;
] "in"          = [ b"Abib",     [0xE4            ], X;
                    b"Awib",     [0xE5            ], X, WORD_SIZE;
                    b"Adib",     [0xE5            ], X;
                    b"AbCw",     [0xEC            ], X;
                    b"AwCw",     [0xED            ], X, WORD_SIZE;
                    b"AdCw",     [0xED            ], X;
] "inc"         = [ b"v*",       [0xFF            ], 0, AUTO_SIZE | LOCK;
                    b"vb",       [0xFE            ], 0,             LOCK;
] "insb"        = [ b"",         [0x6C            ], X;
] "insw"        = [ b"",         [0x6D            ], X, WORD_SIZE;
] "insd"        = [ b"",         [0x6D            ], X;
] "int"         = [ b"ib",       [0xCD            ], X;
] "jo"          = [ b"o*",       [0x0F, 0x80      ], X, AUTO_SIZE;
                    b"ob",       [0x70            ], X;
] "jno"         = [ b"o*",       [0x0F, 0x81      ], X, AUTO_SIZE;
                    b"ob",       [0x71            ], X;
] "jb"          |
  "jc"          |
  "jnae"        = [ b"o*",       [0x0F, 0x82      ], X, AUTO_SIZE;
                    b"ob",       [0x72            ], X;
] "jnb"         |
  "jnc"         |
  "jae"         = [ b"o*",       [0x0F, 0x83      ], X, AUTO_SIZE;
                    b"ob",       [0x73            ], X;
] "jz"          |
  "je"          = [ b"o*",       [0x0F, 0x84      ], X, AUTO_SIZE;
                    b"ob",       [0x74            ], X;
] "jnz"         |
  "jne"         = [ b"o*",       [0x0F, 0x85      ], X, AUTO_SIZE;
                    b"ob",       [0x75            ], X;
] "jbe"         |
  "jna"         = [ b"o*",       [0x0F, 0x86      ], X, AUTO_SIZE;
                    b"ob",       [0x76            ], X;
] "jnbe"        |
  "ja"          = [ b"o*",       [0x0F, 0x87      ], X, AUTO_SIZE;
                    b"ob",       [0x77            ], X;
] "js"          = [ b"o*",       [0x0F, 0x88      ], X, AUTO_SIZE;
                    b"ob",       [0x78            ], X;
] "jns"         = [ b"o*",       [0x0F, 0x89      ], X, AUTO_SIZE;
                    b"ob",       [0x79            ], X;
] "jp"          |
  "jpe"         = [ b"o*",       [0x0F, 0x8A      ], X, AUTO_SIZE;
                    b"ob",       [0x7A            ], X;
] "jnp"         |
  "jpo"         = [ b"o*",       [0x0F, 0x8B      ], X, AUTO_SIZE;
                    b"ob",       [0x7B            ], X;
] "jl"          |
  "jnge"        = [ b"o*",       [0x0F, 0x8C      ], X, AUTO_SIZE;
                    b"ob",       [0x7C            ], X;
] "jnl"         |
  "jge"         = [ b"o*",       [0x0F, 0x8D      ], X, AUTO_SIZE;
                    b"ob",       [0x7D            ], X;
] "jle"         |
  "jng"         = [ b"o*",       [0x0F, 0x8E      ], X, AUTO_SIZE;
                    b"ob",       [0x7E            ], X;
] "jnle"        |
  "jg"          = [ b"o*",       [0x0F, 0x8F      ], X, AUTO_SIZE;
                    b"ob",       [0x7F            ], X;
] "jecxz"       = [ b"ob",       [0xE3            ], X, PREF_67;
] "jrcxz"       = [ b"ob",       [0xE3            ], X;
] "jmp"         = [ b"o*",       [0xE9            ], X, AUTO_SIZE;
                    b"ob",       [0xEB            ], X;
                    b"v*",       [0xFF            ], 4, AUTO_NO32 ;
] "lahf"        = [ b"",         [0x9F            ], X;
] "lfs"         = [ b"r*m!",     [0x0F, 0xB4      ], X, AUTO_SIZE;
] "lgs"         = [ b"r*m!",     [0x0F, 0xB5      ], X, AUTO_SIZE;
] "lss"         = [ b"r*m!",     [0x0F, 0xB2      ], X, AUTO_SIZE;
] "lea"         = [ b"r*m!",     [0x8D            ], X, AUTO_SIZE;
] "leave"       = [ b"",         [0xC9            ], X;
] "lfence"      = [ b"",         [0x0F, 0xAE, 0xE8], X;
] "llwpcb"      = [ b"r*",       [   9, 0x12      ], 0, AUTO_REXW | XOP_OP;
] "lodsb"       = [ b"",         [0xAC            ], X;
] "lodsw"       = [ b"",         [0xAD            ], X, WORD_SIZE;
] "lodsd"       = [ b"",         [0xAD            ], X;
] "lodsq"       = [ b"",         [0xAD            ], X, WITH_REXW;
] "loop"        = [ b"ob",       [0xE2            ], X;
] "loope"       |
  "loopz"       = [ b"ob",       [0xE1            ], X;
] "loopne"      |
  "loopnz"      = [ b"ob",       [0xE0            ], X;
] "lwpins"      = [ b"r*vdid",   [  10, 0x12      ], 0, AUTO_REXW | XOP_OP | ENC_VM;
] "lwpval"      = [ b"r*vdid",   [  10, 0x12      ], 1, AUTO_REXW | XOP_OP | ENC_VM;
] "lzcnt"       = [ b"r*v*",     [0x0F, 0xBD      ], X, AUTO_SIZE | PREF_F3;
] "mfence"      = [ b"",         [0x0F, 0xAE, 0xF0], X;
] "mov"         = [ b"v*r*",     [0x89            ], X, AUTO_SIZE;
                    b"vbrb",     [0x88            ], X;
                    b"r*v*",     [0x8B            ], X, AUTO_SIZE;
                    b"rbvb",     [0x8A            ], X;
                    b"r*sw",     [0x8C            ], X, AUTO_SIZE;
                    b"mwsw",     [0x8C            ], X;
                    b"swmw",     [0x8C            ], X;
                    b"swrw",     [0x8C            ], X;
                    b"rbib",     [0xB0            ], X,             SHORT_ARG;
                    b"rwiw",     [0xB8            ], X, WORD_SIZE | SHORT_ARG;
                    b"rdid",     [0xB8            ], X,             SHORT_ARG;
                    b"v*i*",     [0xC7            ], 0, AUTO_SIZE;
                    b"rqiq",     [0xB8            ], X, WITH_REXW | SHORT_ARG;
                    b"vbib",     [0xC6            ], 0;
                    b"cdrd",     [0x0F, 0x22      ], X; // can only match in 32 bit mode due to "cd"
                    b"cqrq",     [0x0F, 0x22      ], X; // doesn't need a prefix to be encoded, as it's 64 bit natural in 64 bit mode
                    b"rdcd",     [0x0F, 0x20      ], X;
                    b"rqcq",     [0x0F, 0x20      ], X;
                    b"Wdrd",     [0x0F, 0x22      ], 0, PREF_F0; // note: technically CR8 should actually be encoded, but the encoding is 0.
                    b"Wqrq",     [0x0F, 0x22      ], 0, PREF_F0;
                    b"rdWd",     [0x0F, 0x22      ], 0, PREF_F0;
                    b"rqWq",     [0x0F, 0x22      ], 0, PREF_F0;
                    b"ddrd",     [0x0F, 0x23      ], X; // 32 bit mode only
                    b"dqrq",     [0x0F, 0x23      ], X;
                    b"rddd",     [0x0F, 0x21      ], X;
                    b"rqdq",     [0x0F, 0x21      ], X;
] "movabs"      = [ b"Abib",     [0xA0            ], X; // special syntax for 64-bit disp only mov
                    b"Awiw",     [0xA1            ], X, WORD_SIZE;
                    b"Adid",     [0xA1            ], X;
                    b"Aqiq",     [0xA1            ], X, WITH_REXW;
                    b"ibAb",     [0xA2            ], X;
                    b"iwAw",     [0xA3            ], X, WORD_SIZE;
                    b"idAd",     [0xA3            ], X;
                    b"iqAq",     [0xA3            ], X, WITH_REXW;
] "movbe"       = [ b"r*m*",     [0x0F, 0x38, 0xF0], X, AUTO_SIZE;
                    b"m*r*",     [0x0F, 0x38, 0xF1], X, AUTO_SIZE;
] "movd"        = [ b"yov*",     [0x0F, 0x6E      ], X, AUTO_REXW | PREF_66;
                    b"v*yo",     [0x0F, 0x7E      ], X, AUTO_REXW | PREF_66;
                    b"xqv*",     [0x0F, 0x6E      ], X, AUTO_REXW;
                    b"v*xq",     [0x0F, 0x7E      ], X, AUTO_REXW;
] "movmskpd"    = [ b"r?yo",     [0x0F, 0x50      ], X, PREF_66;
] "movmskps"    = [ b"r?yo",     [0x0F, 0x50      ], X;
] "movnti"      = [ b"m*r*",     [0x0F, 0xC3      ], X, AUTO_REXW;
] "movsb"       = [ b"",         [0xA4            ], X;
] "movsw"       = [ b"",         [0xA5            ], X, WORD_SIZE;
] "movsd"       = [ b"",         [0xA5            ], X;
                    b"yoyo",     [0x0F, 0x10      ], X, PREF_F2;
                    b"yomq",     [0x0F, 0x10      ], X, PREF_F2;
                    b"mqyo",     [0x0F, 0x11      ], X, PREF_F2;
] "movsq"       = [ b"",         [0xA5            ], X, WITH_REXW;
] "movsx"       = [ b"r*vw",     [0x0F, 0xBF      ], X, AUTO_REXW; // currently this defaults to a certain memory size
                    b"r*vb",     [0x0F, 0xBE      ], X, AUTO_SIZE;
] "movsxd"      = [ b"rqvd",     [0x63            ], X, WITH_REXW;
] "movzx"       = [ b"r*vw",     [0x0F, 0xB7      ], X, AUTO_REXW; // currently this defaults to a certain memory size
                    b"r*vb",     [0x0F, 0xB6      ], X, AUTO_SIZE;
] "mul"         = [ b"v*",       [0xF7            ], 4, AUTO_SIZE;
                    b"vb",       [0xF6            ], 4;
] "mulx"        = [ b"r*r*v*",   [   2, 0xF6      ], X, AUTO_REXW | VEX_OP | PREF_F2;
] "neg"         = [ b"v*",       [0xF7            ], 3, AUTO_SIZE | LOCK;
                    b"vb",       [0xF6            ], 3,             LOCK;
] "nop"         = [ b"",         [0x90            ], X;
                    b"v*",       [0x0F, 0x1F      ], 0, AUTO_SIZE;
] "not"         = [ b"v*",       [0xF7            ], 2, AUTO_SIZE | LOCK;
                    b"vb",       [0xF6            ], 2,             LOCK;
] "or"          = [ b"A*i*",     [0x0D            ], X, AUTO_SIZE;
                    b"Abib",     [0x0C            ], X;
                    b"v*i*",     [0x81            ], 1, AUTO_SIZE | LOCK;
                    b"v*ib",     [0x83            ], 1, AUTO_SIZE | LOCK;
                    b"vbib",     [0x80            ], 1,             LOCK;
                    b"v*r*",     [0x09            ], X, AUTO_SIZE | LOCK;
                    b"vbrb",     [0x08            ], X,             LOCK;
                    b"r*v*",     [0x0B            ], X, AUTO_SIZE;
                    b"rbvb",     [0x0A            ], X;
] "out"         = [ b"ibAb",     [0xE6            ], X;
                    b"ibAw",     [0xE7            ], X;
                    b"ibAd",     [0xE7            ], X;
                    b"CwAb",     [0xEE            ], X;
                    b"CwAw",     [0xEF            ], X, WORD_SIZE;
                    b"CwAd",     [0xEF            ], X;
] "outsb"       = [ b"",         [0x6E            ], X,             REP;
] "outsw"       = [ b"",         [0x6F            ], X, WORD_SIZE | REP;
] "outsd"       = [ b"",         [0x6F            ], X,             REP;
] "pause"       = [ b"",         [0xF3, 0x90      ], X;
] "pdep"        = [ b"r*r*v*",   [   2, 0xF5      ], X, AUTO_REXW | VEX_OP | PREF_F2;
] "pext"        = [ b"r*r*v*",   [   2, 0xF5      ], X, AUTO_REXW | VEX_OP | PREF_F3;
] "pop"         = [ b"r*",       [0x58            ], X, AUTO_NO32 | SHORT_ARG;
                    b"v*",       [0x8F            ], 0, AUTO_NO32 ;
                    b"Uw",       [0x0F, 0xA1      ], X;
                    b"Vw",       [0x0F, 0xA9      ], X;
] "popcnt"      = [ b"r*v*",     [0x0F, 0xB8      ], X, AUTO_SIZE | PREF_F3;
] "popf"        = [ b"",         [0x9D            ], X, PREF_66;
] "popfq"       = [ b"",         [0x9D            ], X;
] "prefetch"    = [ b"mb",       [0x0F, 0x0D      ], 0;
] "prefetchw"   = [ b"mb",       [0x0F, 0x0D      ], 1;
] "prefetchnta" = [ b"mb",       [0x0F, 0x18      ], 0;
] "prefetcht0"  = [ b"mb",       [0x0F, 0x18      ], 1;
] "prefetcht1"  = [ b"mb",       [0x0F, 0x18      ], 2;
] "prefetcht2"  = [ b"mb",       [0x0F, 0x18      ], 3;
] "push"        = [ b"r*",       [0x50            ], X, AUTO_NO32 | SHORT_ARG;
                    b"v*",       [0xFF            ], 6, AUTO_NO32 ;
                    b"iq",       [0x68            ], X;
                    b"iw",       [0x68            ], X, WORD_SIZE;
                    b"ib",       [0x6A            ], X;
                    b"Uw",       [0x0F, 0xA0      ], X;
                    b"Vw",       [0x0F, 0xA8      ], X;
] "pushf"       = [ b"",         [0x9C            ], X, PREF_66;
] "pushfq"      = [ b"",         [0x9C            ], X;
] "rcl"         = [ b"v*Bb",     [0xD3            ], 2, AUTO_SIZE; // shift by one forms not supported as immediates are only resolved at runtime
                    b"vbBb",     [0xD2            ], 2;
                    b"v*ib",     [0xC1            ], 2, AUTO_SIZE;
                    b"vbib",     [0xC0            ], 2;
] "rcr"         = [ b"v*Bb",     [0xD3            ], 3, AUTO_SIZE;
                    b"vbBb",     [0xD2            ], 3;
                    b"v*ib",     [0xC1            ], 3, AUTO_SIZE;
                    b"vbib",     [0xC0            ], 3;
] "rdfsbase"    = [ b"r*",       [0x0F, 0xAE      ], 0, AUTO_REXW | PREF_F3;
] "rdgsbase"    = [ b"r*",       [0x0F, 0xAE      ], 1, AUTO_REXW | PREF_F3;
] "rdrand"      = [ b"r*",       [0x0F, 0xC7      ], 6, AUTO_SIZE;
] "ret"         = [ b"",         [0xC3            ], X;
                    b"iw",       [0xC2            ], X;
] "rol"         = [ b"v*Bb",     [0xD3            ], 0, AUTO_SIZE;
                    b"vbBb",     [0xD2            ], 0;
                    b"v*ib",     [0xC1            ], 0, AUTO_SIZE;
                    b"vbib",     [0xC0            ], 0;
] "ror"         = [ b"v*Bb",     [0xD3            ], 1, AUTO_SIZE;
                    b"vbBb",     [0xD2            ], 1;
                    b"v*ib",     [0xC1            ], 1, AUTO_SIZE;
                    b"vbib",     [0xC0            ], 1;
] "rorx"        = [ b"r*v*ib",   [   3, 0xF0      ], X, AUTO_REXW | VEX_OP | PREF_F2;
] "sahf"        = [ b"",         [0x9E            ], X;
] "sal"         |
  "shl"         = [ b"v*Bb",     [0xD3            ], 4, AUTO_SIZE;
                    b"vbBb",     [0xD2            ], 4;
                    b"v*ib",     [0xC1            ], 4, AUTO_SIZE;
                    b"vbib",     [0xC0            ], 4;
] "sar"         = [ b"v*Bb",     [0xD3            ], 7, AUTO_SIZE;
                    b"vbBb",     [0xD2            ], 7;
                    b"v*ib",     [0xC1            ], 7, AUTO_SIZE;
                    b"vbib",     [0xC0            ], 7;
] "sarx"        = [ b"r*v*r*",   [   2, 0xF7      ], X, AUTO_REXW | VEX_OP | PREF_F3;
] "sbb"         = [ b"A*i*",     [0x1D            ], X, AUTO_SIZE;
                    b"Abib",     [0x1C            ], X;
                    b"v*i*",     [0x81            ], 3, AUTO_SIZE | LOCK;
                    b"v*ib",     [0x83            ], 3, AUTO_SIZE | LOCK;
                    b"vbib",     [0x80            ], 3,             LOCK;
                    b"v*r*",     [0x19            ], X, AUTO_SIZE | LOCK;
                    b"vbrb",     [0x18            ], X,             LOCK;
                    b"r*v*",     [0x1B            ], X, AUTO_SIZE;
                    b"rbvb",     [0x1A            ], X;
] "scasb"       = [ b"",         [0xAE            ], X,             REPE;
] "scasw"       = [ b"",         [0xAF            ], X, WORD_SIZE | REPE;
] "scasd"       = [ b"",         [0xAF            ], X,             REPE;
] "scasq"       = [ b"",         [0xAF            ], X, WITH_REXW | REPE;
] "seto"        = [ b"vb",       [0x0F, 0x90      ], 0;
] "setno"       = [ b"vb",       [0x0F, 0x91      ], 0;
] "setb"        |
  "setc"        |
  "setnae"      = [ b"vb",       [0x0F, 0x92      ], 0;
] "setnb"       |
  "setnc"       |
  "setae"       = [ b"vb",       [0x0F, 0x93      ], 0;
] "setz"        |
  "sete"        = [ b"vb",       [0x0F, 0x94      ], 0;
] "setnz"       |
  "setne"       = [ b"vb",       [0x0F, 0x95      ], 0;
] "setbe"       |
  "setna"       = [ b"vb",       [0x0F, 0x96      ], 0;
] "setnbe"      |
  "seta"        = [ b"vb",       [0x0F, 0x97      ], 0;
] "sets"        = [ b"vb",       [0x0F, 0x98      ], 0;
] "setns"       = [ b"vb",       [0x0F, 0x99      ], 0;
] "setp"        |
  "setpe"       = [ b"vb",       [0x0F, 0x9A      ], 0;
] "setnp"       |
  "setpo"       = [ b"vb",       [0x0F, 0x9B      ], 0;
] "setl"        |
  "setnge"      = [ b"vb",       [0x0F, 0x9C      ], 0;
] "setnl"       |
  "setge"       = [ b"vb",       [0x0F, 0x9D      ], 0;
] "setle"       |
  "setng"       = [ b"vb",       [0x0F, 0x9E      ], 0;
] "setnle"      |
  "setg"        = [ b"vb",       [0x0F, 0x9F      ], 0;
] "sfence"      = [ b"",         [0x0F, 0xAE, 0xF8], X;
] "shld"        = [ b"v*r*Bb",   [0x0F, 0xA5      ], X, AUTO_SIZE;
                    b"v*r*ib",   [0x0F, 0xA4      ], X, AUTO_SIZE;
] "shlx"        = [ b"r*v*r*",   [   2, 0xF7      ], X, AUTO_REXW | VEX_OP | PREF_66;
] "shr"         = [ b"v*Bb",     [0xD3            ], 5, AUTO_SIZE;
                    b"vbBb",     [0xD2            ], 5;
                    b"v*ib",     [0xC1            ], 5, AUTO_SIZE;
                    b"vbib",     [0xC0            ], 5;
] "shrd"        = [ b"v*r*Bb",   [0x0F, 0xAD      ], X, AUTO_SIZE;
                    b"v*r*ib",   [0x0F, 0xAC      ], X, AUTO_SIZE;
] "shrx"        = [ b"r*v*r*",   [   2, 0xF7      ], X, AUTO_REXW | VEX_OP | PREF_F2;
] "slwpcb"      = [ b"r*",       [   9, 0x12      ], 1, AUTO_REXW | XOP_OP;
] "stc"         = [ b"",         [0xF9            ], X;
] "std"         = [ b"",         [0xFD            ], X;
] "stosb"       = [ b"",         [0xAA            ], X,             REP;
] "stosw"       = [ b"",         [0xAB            ], X, WORD_SIZE | REP;
] "stosd"       = [ b"",         [0xAB            ], X,             REP;
] "stosq"       = [ b"",         [0xAB            ], X, WITH_REXW | REP;
] "sub"         = [ b"A*i*",     [0x2D            ], X, AUTO_SIZE;
                    b"Abib",     [0x2C            ], X;
                    b"v*i*",     [0x81            ], 5, AUTO_SIZE | LOCK;
                    b"v*ib",     [0x83            ], 5, AUTO_SIZE | LOCK;
                    b"vbib",     [0x80            ], 5,             LOCK;
                    b"v*r*",     [0x29            ], X, AUTO_SIZE | LOCK;
                    b"vbrb",     [0x28            ], X,             LOCK;
                    b"r*v*",     [0x2B            ], X, AUTO_SIZE;
                    b"rbvb",     [0x2A            ], X;
] "t1mskc"      = [ b"r*v*",     [   9, 0x01      ], 7, AUTO_REXW | XOP_OP | ENC_VM;
] "test"        = [ b"A*i*",     [0xA9            ], X, AUTO_SIZE;
                    b"Abib",     [0xA8            ], X;
                    b"v*i*",     [0xF7            ], 0, AUTO_SIZE;
                    b"vbib",     [0xF6            ], 0;
                    b"v*r*",     [0x85            ], X, AUTO_SIZE;
                    b"vbrb",     [0x84            ], X;
] "tzcnt"       = [ b"r*v*",     [0x0F, 0xBC      ], X, AUTO_SIZE | PREF_F3;
] "tzmsk"       = [ b"r*v*",     [   9, 0x01      ], 4, AUTO_REXW | XOP_OP  | ENC_VM;
] "wrfsbase"    = [ b"r*",       [0x0F, 0xAE      ], 2, AUTO_REXW | PREF_F3;
] "wrgsbase"    = [ b"r*",       [0x0F, 0xAE      ], 3, AUTO_REXW | PREF_F3;
] "xadd"        = [ b"v*r*",     [0x0F, 0xC1      ], X, AUTO_SIZE | LOCK;
                    b"vbrb",     [0x0F, 0xC0      ], X,             LOCK;
] "xchg"        = [ b"A*r*",     [0x90            ], X, AUTO_SIZE | SHORT_ARG;
                    b"r*A*",     [0x90            ], X, AUTO_SIZE | SHORT_ARG;
                    b"v*r*",     [0x87            ], X, AUTO_SIZE | LOCK;
                    b"r*v*",     [0x87            ], X, AUTO_SIZE | LOCK;
                    b"vbrb",     [0x86            ], X,             LOCK;
                    b"rbvb",     [0x86            ], X,             LOCK;
] "xlatb"       = [ b"",         [0xD7            ], X;
] "xor"         = [ b"A*i*",     [0x35            ], X, AUTO_SIZE;
                    b"Abib",     [0x34            ], X;
                    b"v*i*",     [0x81            ], 6, AUTO_SIZE | LOCK;
                    b"v*ib",     [0x83            ], 6, AUTO_SIZE | LOCK;
                    b"vbib",     [0x80            ], 6,             LOCK;
                    b"v*r*",     [0x31            ], X, AUTO_SIZE | LOCK;
                    b"vbrb",     [0x30            ], X,             LOCK;
                    b"r*v*",     [0x33            ], X, AUTO_SIZE;
                    b"rbvb",     [0x32            ], X;
]
// System instructions
  "clgi"        = [ b"",         [0x0F, 0x01, 0xDD], X;
] "cli"         = [ b"",         [0xFA            ], X;
] "clts"        = [ b"",         [0x0F, 0x06      ], X;
] "hlt"         = [ b"",         [0xF4            ], X;
] "int3"        = [ b"",         [0xCC            ], X;
] "invd"        = [ b"",         [0x0F, 0x08      ], X;
] "invlpg"      = [ b"mb",       [0x0F, 0x01      ], 7;
] "invlpga"     = [ b"AqBd",     [0x0F, 0x01, 0xDF], X;
] "iret"        = [ b"",         [0xCF            ], X, WORD_SIZE;
] "iretd"       = [ b"",         [0xCF            ], X;
] "iretq"       = [ b"",         [0xCF            ], X, WITH_REXW;
] "lar"         = [ b"r*vw",     [0x0F, 0x02      ], X, AUTO_SIZE;
] "lgdt"        = [ b"m!",       [0x0F, 0x01      ], 2;
] "lidt"        = [ b"m!",       [0x0F, 0x01      ], 3;
] "lldt"        = [ b"vw",       [0x0F, 0x00      ], 2;
] "lmsw"        = [ b"vw",       [0x0F, 0x01      ], 6;
] "lsl"         = [ b"r*vw",     [0x0F, 0x03      ], X, AUTO_SIZE;
] "ltr"         = [ b"vw",       [0x0F, 0x00      ], 3;
] "monitor"     = [ b"",         [0x0F, 0x01, 0xC8], X;
] "monitorx"    = [ b"",         [0x0F, 0x01, 0xFA], X;
] "mwait"       = [ b"",         [0x0F, 0x01, 0xC9], X;
] "mwaitx"      = [ b"",         [0x0F, 0x01, 0xFB], X;
] "rdmsr"       = [ b"",         [0x0F, 0x32      ], X;
] "rdpmc"       = [ b"",         [0x0F, 0x33      ], X;
] "rdtsc"       = [ b"",         [0x0F, 0x31      ], X;
] "rdtscp"      = [ b"",         [0x0F, 0x01, 0xF9], X;
] "rsm"         = [ b"",         [0x0F, 0xAA      ], X;
] "sgdt"        = [ b"m!",       [0x0F, 0x01      ], 0;
] "sidt"        = [ b"m!",       [0x0F, 0x01      ], 1;
] "skinit"      = [ b"Ad",       [0x0F, 0x01, 0xDE], X;
] "sldt"        = [ b"r*",       [0x0F, 0x00      ], 0, AUTO_SIZE;
                    b"mw",       [0x0F, 0x00      ], 0;
] "smsw"        = [ b"r*",       [0x0F, 0x01      ], 4, AUTO_SIZE;
                    b"mw",       [0x0F, 0x01      ], 4;
] "sti"         = [ b"",         [0xFB            ], X;
] "stgi"        = [ b"",         [0x0F, 0x01, 0xDC], X;
] "str"         = [ b"r*",       [0x0F, 0x00      ], 1, AUTO_SIZE;
                    b"mw",       [0x0F, 0x00      ], 1;
] "swapgs"      = [ b"",         [0x0F, 0x01, 0xF8], X;
] "syscall"     = [ b"",         [0x0F, 0x05      ], X;
] "sysenter"    = [ b"",         [0x0F, 0x34      ], X;
] "sysexit"     = [ b"",         [0x0F, 0x35      ], X;
] "sysret"      = [ b"",         [0x0F, 0x07      ], X;
] "ud2"         = [ b"",         [0x0F, 0x0B      ], X;
] "verr"        = [ b"vw",       [0x0F, 0x00      ], 4;
] "verw"        = [ b"vw",       [0x0F, 0x00      ], 5;
] "vmload"      = [ b"Aq",       [0x0F, 0x01, 0xDA], X;
] "vmmcall"     = [ b"",         [0x0F, 0x01, 0xD9], X;
] "vmrun"       = [ b"Aq",       [0x0F, 0x01, 0xD8], X;
] "vmsave"      = [ b"Aq",       [0x0F, 0x01, 0xDB], X;
] "wbinvd"      = [ b"",         [0x0F, 0x09      ], X;
] "wrmsr"       = [ b"",         [0x0F, 0x30      ], X;
]
// x87 FPU instruction set, d   ta taken from amd's programmer manual vol. 5
  "f2xm1"       = [ b"",         [0xD9, 0xF0      ], X;
] "fabs"        = [ b"",         [0xD9, 0xE1      ], X;
] "fadd"        = [ b"Xpfp",     [0xD8, 0xC0      ], X, SHORT_ARG;
                    b"fpXp",     [0xDC, 0xC0      ], X, SHORT_ARG;
                    b"md",       [0xD8            ], 0;
                    b"mq",       [0xDC            ], 0;
] "faddp"       = [ b"",         [0xDE, 0xC1      ], X;
                    b"fpXp",     [0xDE, 0xC0      ], X, SHORT_ARG;
] "fiadd"       = [ b"mw",       [0xDE            ], 0;
                    b"md",       [0xDA            ], 0;
] "fbld"        = [ b"mp",       [0xDF            ], 4;
] "fbstp"       = [ b"mp",       [0xDF            ], 6;
] "fchs"        = [ b"",         [0xD9, 0xE0      ], X;
] "fclex"       = [ b"",         [0x9B, 0xDB, 0xE2], X; // this is actually ;wait ;fnclex
] "fnclex"      = [ b"",         [0xDB, 0xE2      ], X;
] "fcmovb"      = [ b"Xpfp",     [0xDA, 0xC0      ], X, SHORT_ARG;
] "fcmovbe"     = [ b"Xpfp",     [0xDA, 0xD0      ], X, SHORT_ARG;
] "fcmove"      = [ b"Xpfp",     [0xDA, 0xC8      ], X, SHORT_ARG;
] "fcmovnb"     = [ b"Xpfp",     [0xDB, 0xC0      ], X, SHORT_ARG;
] "fcmovnbe"    = [ b"Xpfp",     [0xDB, 0xD0      ], X, SHORT_ARG;
] "fcmovne"     = [ b"Xpfp",     [0xDB, 0xC8      ], X, SHORT_ARG;
] "fcmovnu"     = [ b"Xpfp",     [0xDB, 0xD8      ], X, SHORT_ARG;
] "fcmovu"      = [ b"Xpfp",     [0xDA, 0xD8      ], X, SHORT_ARG;
] "fcom"        = [ b"",         [0xD8, 0xD1      ], X;
                    b"fp",       [0xD8, 0xD0      ], X, SHORT_ARG;
                    b"md",       [0xD8            ], 2;
                    b"mq",       [0xDC            ], 2;
] "fcomp"       = [ b"",         [0xD8, 0xD9      ], X;
                    b"fp",       [0xD8, 0xD8      ], X, SHORT_ARG;
                    b"md",       [0xD8            ], 3;
                    b"mq",       [0xDC            ], 3;
] "fcompp"      = [ b"",         [0xDE, 0xD9      ], X;
] "fcomi"       = [ b"Xpfp",     [0xDB, 0xF0      ], X, SHORT_ARG;
] "fcomip"      = [ b"fpXp",     [0xDF, 0xF0      ], X, SHORT_ARG;
] "fcos"        = [ b"",         [0xD9, 0xFF      ], X;
] "fdecstp"     = [ b"",         [0xD9, 0xF6      ], X;
] "fdiv"        = [ b"Xpfp",     [0xD8, 0xF0      ], X, SHORT_ARG;
                    b"fpXp",     [0xDC, 0xF8      ], X, SHORT_ARG;
                    b"md",       [0xD8            ], 6;
                    b"mq",       [0xDC            ], 6;
] "fdivp"       = [ b"",         [0xDE, 0xF9      ], X;
                    b"fpXp",     [0xDE, 0xF8      ], X, SHORT_ARG;
] "fidiv"       = [ b"mw",       [0xDE            ], 6;
                    b"md",       [0xDA            ], 6;
] "fdivr"       = [ b"Xpfp",     [0xD8, 0xF8      ], X, SHORT_ARG;
                    b"fpXp",     [0xDC, 0xF0      ], X, SHORT_ARG;
                    b"md",       [0xD8            ], 7;
                    b"mq",       [0xDC            ], 7;
] "fdivrp"      = [ b"",         [0xDE, 0xF1      ], X;
                    b"fpXp",     [0xDE, 0xF0      ], X, SHORT_ARG;
] "fidivr"      = [ b"mw",       [0xDE            ], 7;
                    b"md",       [0xDA            ], 7;
] "ffree"       = [ b"fp",       [0xDD, 0xC0      ], X, SHORT_ARG;
] "ficom"       = [ b"mw",       [0xDE            ], 2;
                    b"md",       [0xDA            ], 2;
] "ficomp"      = [ b"mw",       [0xDE            ], 3;
                    b"md",       [0xDA            ], 3;
] "fild"        = [ b"mw",       [0xDF            ], 0;
                    b"md",       [0xDB            ], 0;
                    b"mq",       [0xDF            ], 5;
] "fincstp"     = [ b"",         [0xD9, 0xF7      ], X;
] "finit"       = [ b"",         [0x9B, 0xDB, 0xE3], X; // this is actually ;wait ;fninit
] "fninit"      = [ b"",         [0xDB, 0xE3      ], X;
] "fist"        = [ b"mw",       [0xDF            ], 2;
                    b"md",       [0xDB            ], 2;
                    b"mw",       [0xDF            ], 3;
                    b"md",       [0xDB            ], 3;
                    b"mq",       [0xDF            ], 7;
] "fisttp"      = [ b"mw",       [0xDF            ], 1;
                    b"md",       [0xDB            ], 1;
                    b"mq",       [0xDD            ], 1;
] "fld"         = [ b"fp",       [0xD9, 0xC0      ], X, SHORT_ARG;
                    b"md",       [0xD9            ], 0;
                    b"mq",       [0xDD            ], 0;
                    b"mp",       [0xDB            ], 5;
] "fld1"        = [ b"",         [0xD9, 0xE8      ], X;
] "fldcw"       = [ b"mw",       [0xD9            ], 5;
] "fldenv"      = [ b"m!",       [0xD9            ], 4;
] "fldenvw"     = [ b"m!",       [0xD9            ], 4, WORD_SIZE;
] "fldl2e"      = [ b"",         [0xD9, 0xEA      ], X;
] "fldl2t"      = [ b"",         [0xD9, 0xE9      ], X;
] "fldlg2"      = [ b"",         [0xD9, 0xEC      ], X;
] "fldln2"      = [ b"",         [0xD9, 0xED      ], X;
] "fldpi"       = [ b"",         [0xD9, 0xEB      ], X;
] "fldz"        = [ b"",         [0xD9, 0xEE      ], X;
] "fmul"        = [ b"Xpfp",     [0xD8, 0xC8      ], X, SHORT_ARG;
                    b"fpXp",     [0xDC, 0xC8      ], X, SHORT_ARG;
                    b"md",       [0xD8            ], 1;
                    b"mq",       [0xDC            ], 1;
] "fmulp"       = [ b"",         [0xDE, 0xC9      ], X;
                    b"fpXp",     [0xDE, 0xC8      ], X, SHORT_ARG;
] "fimul"       = [ b"mw",       [0xDE            ], 1;
                    b"md",       [0xDA            ], 1;
] "fnop"        = [ b"",         [0xD9, 0xD0      ], X;
] "fpatan"      = [ b"",         [0xD9, 0xF3      ], X;
] "fprem"       = [ b"",         [0xD9, 0xF8      ], X;
] "fprem1"      = [ b"",         [0xD9, 0xF5      ], X;
] "fptan"       = [ b"",         [0xD9, 0xF2      ], X;
] "frndint"     = [ b"",         [0xD9, 0xFC      ], X;
] "frstor"      = [ b"m!",       [0xDD            ], 4;
] "frstorw"     = [ b"m!",       [0xDD            ], 4, WORD_SIZE;
] "fsave"       = [ b"m!",       [0x9B, 0xDD      ], 6; // note: this is actually ; wait; fnsavew
] "fsavew"      = [ b"m!",       [0x9B, 0x66, 0xDD], 6; // note: this is actually ; wait; OPSIZE fnsave
] "fnsave"      = [ b"m!",       [0xDD            ], 6;
] "fnsavew"     = [ b"m!",       [0xDD            ], 6, WORD_SIZE;
] "fscale"      = [ b"",         [0xD9, 0xFD      ], X;
] "fsin"        = [ b"",         [0xD9, 0xFE      ], X;
] "fsincos"     = [ b"",         [0xD9, 0xFB      ], X;
] "fsqrt"       = [ b"",         [0xD9, 0xFA      ], X;
] "fst"         = [ b"fp",       [0xDD, 0xD0      ], X, SHORT_ARG;
                    b"md",       [0xD9            ], 2;
                    b"mq",       [0xDD            ], 2;
] "fstp"        = [ b"fp",       [0xDD, 0xD8      ], X, SHORT_ARG;
                    b"md",       [0xD9            ], 3;
                    b"mq",       [0xDD            ], 3;
                    b"mp",       [0xDB            ], 7;
] "fstcw"       = [ b"mw",       [0x9B, 0xD9      ], 7; // note: this is actually ; wait; fnstcw
] "fnstcw"      = [ b"mw",       [0xD9            ], 7;
] "fstenv"      = [ b"m!",       [0x9B, 0xD9      ], 6; // note: this is actually ; wait; fnstenv
] "fstenvw"     = [ b"m!",       [0x9B, 0x66, 0xD9], 6; // note: this is actually ; wait; OPSIZE fnsten
] "fnstenv"     = [ b"m!",       [0xD9            ], 6;
] "fnstenvw"    = [ b"m!",       [0xD9            ], 6, WORD_SIZE;
] "fstsw"       = [ b"Aw",       [0x9B, 0xDF, 0xE0], X; // note: this is actually ; wait; fnstsw
                    b"mw",       [0x9B, 0xDD      ], 7;
] "fnstsw"      = [ b"Aw",       [0xDF, 0xE0      ], X;
                    b"mw",       [0xDD            ], 7;
] "fsub"        = [ b"Xpfp",     [0xD8, 0xE0      ], X, SHORT_ARG;
                    b"fpXp",     [0xDC, 0xE8      ], X, SHORT_ARG;
                    b"md",       [0xD8            ], 4;
                    b"mq",       [0xDC            ], 4;
] "fsubp"       = [ b"",         [0xDE, 0xE9      ], X;
                    b"fpXp",     [0xDE, 0xE8      ], X, SHORT_ARG;
] "fisub"       = [ b"mw",       [0xDE            ], 4;
                    b"md",       [0xDA            ], 4;
] "fsubr"       = [ b"Xpfp",     [0xD8, 0xE8      ], X, SHORT_ARG;
                    b"fpXp",     [0xDC, 0xE0      ], X, SHORT_ARG;
                    b"md",       [0xD8            ], 5;
                    b"mq",       [0xDC            ], 5;
] "fsubrp"      = [ b"",         [0xDE, 0xE1      ], X;
                    b"fpXp",     [0xDE, 0xE0      ], X, SHORT_ARG;
] "fisubr"      = [ b"mw",       [0xDE            ], 5;
                    b"md",       [0xDA            ], 5;
] "ftst"        = [ b"",         [0xD9, 0xE4      ], X;
] "fucom"       = [ b"",         [0xDD, 0xE1      ], X;
                    b"fp",       [0xDD, 0xE0      ], X, SHORT_ARG;
] "fucomp"      = [ b"",         [0xDD, 0xE9      ], X;
                    b"fp",       [0xDD, 0xE8      ], X, SHORT_ARG;
] "fucompp"     = [ b"",         [0xDA, 0xE9      ], X;
] "fucomi"      = [ b"Xpfp",     [0xDB, 0xE8      ], X, SHORT_ARG;
                    b"fpXp",     [0xDF, 0xE8      ], X, SHORT_ARG;
] "fwait"       |
  "wait"        = [ b"",         [0x9B            ], X;
] "fxam"        = [ b"",         [0xD9, 0xE5      ], X;
] "fxch"        = [ b"",         [0xD9, 0xC9      ], X;
                    b"fp",       [0xD9, 0xC8      ], X, SHORT_ARG;
] "fxrstor"     = [ b"m!",       [0x0F, 0xAE      ], 1;
] "fxsave"      = [ b"m!",       [0x0F, 0xAE      ], 0;
] "fxtract"     = [ b"",         [0xD9, 0xF4      ], X;
] "fyl2x"       = [ b"",         [0xD9, 0xF1      ], X;
] "fyl2xp1"     = [ b"",         [0xD9, 0xF9      ], X;
]
// MMX instruction (also vol.   5) (note that 3DNow! instructions aren't supported)
        
  "cvtpd2pi"    = [ b"xqwo",     [0x0F, 0x2D      ], X, PREF_66;
] "cvtpi2pd"    = [ b"youq",     [0x0F, 0x2A      ], X, PREF_66;
] "cvtpi2ps"    = [ b"youq",     [0x0F, 0x2A      ], X;
] "cvtps2pi"    = [ b"xqwo",     [0x0F, 0x2D      ], X;
] "cvttpd2pi"   = [ b"xqwo",     [0x0F, 0x2C      ], X, PREF_66;
] "cvttps2pi"   = [ b"xqyo",     [0x0F, 0x2C      ], X;
                    b"xqmq",     [0x0F, 0x2C      ], X;
] "emms"        = [ b"",         [0x0F, 0x77      ], X;
] "maskmovq"    = [ b"xqxq",     [0x0F, 0xF7      ], X;
] "movdq2q"     = [ b"xqyo",     [0x0F, 0xD6      ], X, PREF_F2;
] "movntq"      = [ b"mqxq",     [0x0F, 0xE7      ], X;
] "movq"        = [ b"xquq",     [0x0F, 0x6F      ], X;
                    b"uqxq",     [0x0F, 0x7F      ], X;
                    b"yoyo",     [0x0F, 0x7E      ], X, PREF_F3;
                    b"yomq",     [0x0F, 0x7E      ], X, PREF_F3;
                    b"mqyo",     [0x0F, 0xD6      ], X, PREF_66;
] "movq2dq"     = [ b"yoxq",     [0x0F, 0xD6      ], X, PREF_F3;
] "packssdw"    = [ b"xquq",     [0x0F, 0x6B      ], X;
                    b"yowo",     [0x0F, 0x6B      ], X, PREF_66;
] "packsswb"    = [ b"xquq",     [0x0F, 0x63      ], X;
                    b"yowo",     [0x0F, 0x63      ], X, PREF_66;
] "packuswb"    = [ b"xquq",     [0x0F, 0x67      ], X;
                    b"yowo",     [0x0F, 0x67      ], X, PREF_66;
] "paddb"       = [ b"xquq",     [0x0F, 0xFC      ], X;
                    b"yowo",     [0x0F, 0xFC      ], X, PREF_66;
] "paddd"       = [ b"xquq",     [0x0F, 0xFE      ], X;
                    b"yowo",     [0x0F, 0xFE      ], X, PREF_66;
] "paddq"       = [ b"xquq",     [0x0F, 0xD4      ], X;
                    b"yowo",     [0x0F, 0xD4      ], X, PREF_66;
] "paddsb"      = [ b"xquq",     [0x0F, 0xEC      ], X;
                    b"yowo",     [0x0F, 0xEC      ], X, PREF_66;
] "paddsw"      = [ b"xquq",     [0x0F, 0xED      ], X;
                    b"yowo",     [0x0F, 0xED      ], X, PREF_66;
] "paddusb"     = [ b"xquq",     [0x0F, 0xDC      ], X;
                    b"yowo",     [0x0F, 0xDC      ], X, PREF_66;
] "paddusw"     = [ b"xquq",     [0x0F, 0xDD      ], X;
                    b"yowo",     [0x0F, 0xDD      ], X, PREF_66;
] "paddw"       = [ b"xquq",     [0x0F, 0xFD      ], X;
                    b"yowo",     [0x0F, 0xFD      ], X, PREF_66;
] "pand"        = [ b"xquq",     [0x0F, 0xDB      ], X;
                    b"yowo",     [0x0F, 0xDB      ], X, PREF_66;
] "pandn"       = [ b"xquq",     [0x0F, 0xDF      ], X;
                    b"yowo",     [0x0F, 0xDF      ], X, PREF_66;
] "pavgb"       = [ b"xquq",     [0x0F, 0xE0      ], X;
                    b"yowo",     [0x0F, 0xE0      ], X, PREF_66;
] "pavgw"       = [ b"xquq",     [0x0F, 0xE3      ], X;
                    b"yowo",     [0x0F, 0xE3      ], X, PREF_66;
] "pcmpeqb"     = [ b"xquq",     [0x0F, 0x74      ], X;
                    b"yowo",     [0x0F, 0x74      ], X, PREF_66;
] "pcmpeqd"     = [ b"xquq",     [0x0F, 0x76      ], X;
                    b"yowo",     [0x0F, 0x76      ], X, PREF_66;
] "pcmpeqw"     = [ b"xquq",     [0x0F, 0x75      ], X;
                    b"yowo",     [0x0F, 0x75      ], X, PREF_66;
] "pcmpgtb"     = [ b"xquq",     [0x0F, 0x64      ], X;
                    b"yowo",     [0x0F, 0x64      ], X, PREF_66;
] "pcmpgtd"     = [ b"xquq",     [0x0F, 0x66      ], X;
                    b"yowo",     [0x0F, 0x66      ], X, PREF_66;
] "pcmpgtw"     = [ b"xquq",     [0x0F, 0x65      ], X;
                    b"yowo",     [0x0F, 0x65      ], X, PREF_66;
] "pextrw"      = [ b"rdxqib",   [0x0F, 0xC5      ], X;
                    b"r?yoib",   [0x0F, 0xC5      ], X, PREF_66;
                    b"mwyoib",   [0x0F, 0x3A, 0x15], X, PREF_66;
] "pinsrw"      = [ b"xqrdib",   [0x0F, 0xC4      ], X;
                    b"xqmwib",   [0x0F, 0xC4      ], X;
                    b"yordib",   [0x0F, 0xC4      ], X, PREF_66;
                    b"yomwib",   [0x0F, 0xC4      ], X, PREF_66;
] "pmaddwd"     = [ b"xquq",     [0x0F, 0xF5      ], X;
                    b"yowo",     [0x0F, 0xF5      ], X, PREF_66;
] "pmaxsw"      = [ b"xquq",     [0x0F, 0xEE      ], X;
                    b"yowo",     [0x0F, 0xEE      ], X, PREF_66;
] "pmaxub"      = [ b"xquq",     [0x0F, 0xDE      ], X;
                    b"yowo",     [0x0F, 0xDE      ], X, PREF_66;
] "pminsw"      = [ b"xquq",     [0x0F, 0xEA      ], X;
                    b"yowo",     [0x0F, 0xEA      ], X, PREF_66;
] "pminub"      = [ b"xquq",     [0x0F, 0xDA      ], X;
                    b"yowo",     [0x0F, 0xDA      ], X, PREF_66;
] "pmovmskb"    = [ b"rdxq",     [0x0F, 0xD7      ], X;
                    b"rdyo",     [0x0F, 0xD7      ], X, PREF_66;
] "pmulhuw"     = [ b"xquq",     [0x0F, 0xE4      ], X;
                    b"yowo",     [0x0F, 0xE4      ], X, PREF_66;
] "pmulhw"      = [ b"xquq",     [0x0F, 0xE5      ], X;
                    b"yowo",     [0x0F, 0xE5      ], X, PREF_66;
] "pmullw"      = [ b"xquq",     [0x0F, 0xD5      ], X;
                    b"yowo",     [0x0F, 0xD5      ], X, PREF_66;
] "pmuludq"     = [ b"xquq",     [0x0F, 0xF4      ], X;
                    b"yowo",     [0x0F, 0xF4      ], X, PREF_66;
] "por"         = [ b"xquq",     [0x0F, 0xEB      ], X;
                    b"yowo",     [0x0F, 0xEB      ], X, PREF_66;
] "psadbw"      = [ b"xquq",     [0x0F, 0xF6      ], X;
                    b"yowo",     [0x0F, 0xF6      ], X, PREF_66;
] "pshufw"      = [ b"xquqib",   [0x0F, 0x70      ], X;
                    b"yowoib",   [0x0F, 0x70      ], X, PREF_F3;
] "pslld"       = [ b"xquq",     [0x0F, 0xF2      ], X;
                    b"xqib",     [0x0F, 0x72      ], 6;
                    b"yowo",     [0x0F, 0xF2      ], X, PREF_66;
                    b"yoib",     [0x0F, 0x72      ], 6, PREF_66;
] "psllq"       = [ b"xquq",     [0x0F, 0xF3      ], X;
                    b"xqib",     [0x0F, 0x73      ], 6;
                    b"yowo",     [0x0F, 0xF3      ], X, PREF_66;
                    b"yoib",     [0x0F, 0x73      ], 6, PREF_66;
] "psllw"       = [ b"xquq",     [0x0F, 0xF1      ], X;
                    b"xqib",     [0x0F, 0x71      ], 6;
                    b"yowo",     [0x0F, 0xF1      ], X, PREF_66;
                    b"yoib",     [0x0F, 0x71      ], 6, PREF_66;
] "psrad"       = [ b"xquq",     [0x0F, 0xE2      ], X;
                    b"xqib",     [0x0F, 0x72      ], 4;
                    b"yowo",     [0x0F, 0xE2      ], X, PREF_66;
                    b"yoib",     [0x0F, 0x72      ], 4, PREF_66;
] "psraw"       = [ b"xquq",     [0x0F, 0xE1      ], X;
                    b"xqib",     [0x0F, 0x71      ], 4;
                    b"yowo",     [0x0F, 0xE1      ], X, PREF_66;
                    b"yoib",     [0x0F, 0x71      ], 4, PREF_66;
] "psrld"       = [ b"xquq",     [0x0F, 0xD2      ], X;
                    b"xqib",     [0x0F, 0x72      ], 2;
                    b"yowo",     [0x0F, 0xD2      ], X, PREF_66;
                    b"yoib",     [0x0F, 0x72      ], 2, PREF_66;
] "psrlq"       = [ b"xquq",     [0x0F, 0xD3      ], X;
                    b"xqib",     [0x0F, 0x73      ], 2;
                    b"yowo",     [0x0F, 0xD3      ], X, PREF_66;
                    b"yoib",     [0x0F, 0x73      ], 2, PREF_66;
] "psrlw"       = [ b"xquq",     [0x0F, 0xD1      ], X;
                    b"xqib",     [0x0F, 0x71      ], 2;
                    b"yowo",     [0x0F, 0xD1      ], X, PREF_66;
                    b"yoib",     [0x0F, 0x71      ], 2, PREF_66;
] "psubb"       = [ b"xquq",     [0x0F, 0xF8      ], X;
                    b"yowo",     [0x0F, 0xF8      ], X, PREF_66;
] "psubd"       = [ b"xquq",     [0x0F, 0xFA      ], X;
                    b"yowo",     [0x0F, 0xFA      ], X, PREF_66;
] "psubq"       = [ b"xquq",     [0x0F, 0xFB      ], X;
                    b"yowo",     [0x0F, 0xFB      ], X, PREF_66;
] "psubsb"      = [ b"xquq",     [0x0F, 0xE8      ], X;
                    b"yowo",     [0x0F, 0xE8      ], X, PREF_66;
] "psubsw"      = [ b"xquq",     [0x0F, 0xE9      ], X;
                    b"yowo",     [0x0F, 0xE9      ], X, PREF_66;
] "psubusb"     = [ b"xquq",     [0x0F, 0xD8      ], X;
                    b"yowo",     [0x0F, 0xD8      ], X, PREF_66;
] "psubusw"     = [ b"xquq",     [0x0F, 0xD9      ], X;
                    b"yowo",     [0x0F, 0xD9      ], X, PREF_66;
] "psubw"       = [ b"xquq",     [0x0F, 0xF9      ], X;
                    b"yowo",     [0x0F, 0xF9      ], X, PREF_66;
] "punpckhbw"   = [ b"xquq",     [0x0F, 0x68      ], X;
                    b"yowo",     [0x0F, 0x68      ], X, PREF_66;
] "punpckhdq"   = [ b"xquq",     [0x0F, 0x6A      ], X;
                    b"yowo",     [0x0F, 0x6A      ], X, PREF_66;
] "punpckhwd"   = [ b"xquq",     [0x0F, 0x69      ], X;
                    b"yowo",     [0x0F, 0x69      ], X, PREF_66;
] "punpcklbw"   = [ b"xquq",     [0x0F, 0x60      ], X;
                    b"yowo",     [0x0F, 0x60      ], X, PREF_66;
] "punpckldq"   = [ b"xquq",     [0x0F, 0x62      ], X;
                    b"yowo",     [0x0F, 0x62      ], X, PREF_66;
] "punpcklwd"   = [ b"xquq",     [0x0F, 0x61      ], X;
                    b"yowo",     [0x0F, 0x61      ], X, PREF_66;
] "pxor"        = [ b"xquq",     [0x0F, 0xEF      ], X;
                    b"yowo",     [0x0F, 0xEF      ], X, PREF_66;
]
// SSE instructions (vol. 4)

  "addpd"       = [ b"yowo",     [0x0F, 0x58      ], X, PREF_66;
] "vaddpd"      = [ b"y*y*w*",   [   1, 0x58      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "addps"       = [ b"yowo",     [0x0F, 0x58      ], X;
] "vaddps"      = [ b"y*y*w*",   [   1, 0x58      ], X,           AUTO_VEXL | VEX_OP;
] "addsd"       = [ b"yoyo",     [0x0F, 0x58      ], X, PREF_F2;
                    b"yomq",     [0x0F, 0x58      ], X, PREF_F2;
] "vaddsd"      = [ b"yoyoyo",   [   1, 0x58      ], X, PREF_F2             | VEX_OP;
                    b"yoyomq",   [   1, 0x58      ], X, PREF_F2             | VEX_OP;
] "addss"       = [ b"yoyo",     [0x0F, 0x58      ], X, PREF_F3;
                    b"yomd",     [0x0F, 0x58      ], X, PREF_F3;
] "vaddss"      = [ b"yoyoyo",   [   1, 0x58      ], X, PREF_F3             | VEX_OP;
                    b"yoyomd",   [   1, 0x58      ], X, PREF_F3             | VEX_OP;
] "addsubpd"    = [ b"yowo",     [0x0F, 0xD0      ], X, PREF_66;
] "vaddsubpd"   = [ b"y*y*w*",   [   1, 0xD0      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "addsubps"    = [ b"yowo",     [0x0F, 0xD0      ], X, PREF_F2;
] "vaddsubps"   = [ b"y*y*w*",   [   1, 0xD0      ], X, PREF_F2 | AUTO_VEXL | VEX_OP;
] "aesdec"      = [ b"yowo",     [0x0F, 0x38, 0xDE], X, PREF_66;
] "vaesdec"     = [ b"yoyowo",   [   2, 0xDE      ], X, PREF_66             | VEX_OP;
] "aesdeclast"  = [ b"yowo",     [0x0F, 0x38, 0xDF], X, PREF_66;
] "vaesdeclast" = [ b"yoyowo",   [   2, 0xDF      ], X, PREF_66             | VEX_OP;
] "aesenc"      = [ b"yowo",     [0x0F, 0x38, 0xDC], X, PREF_66;
] "vaesenc"     = [ b"yoyowo",   [   2, 0xDC      ], X, PREF_66             | VEX_OP;
] "aesenclast"  = [ b"yowo",     [0x0F, 0x38, 0xDD], X, PREF_66;
] "vaesenclast" = [ b"yoyowo",   [   2, 0xDD      ], X, PREF_66             | VEX_OP;
] "aesimc"      = [ b"yowo",     [0x0F, 0x38, 0xDB], X, PREF_66;
] "vaesimc"     = [ b"yowo",     [   2, 0xDB      ], X, PREF_66             | VEX_OP;
] "aeskeygenassist"
                = [ b"yowoib",   [0x0F, 0x3A, 0xDF], X, PREF_66;
] "vaeskeygenassist"
                = [ b"yowoib",   [   3, 0xDF      ], X, PREF_66             | VEX_OP;
] "andnpd"      = [ b"yowo",     [0x0F, 0x55      ], X, PREF_66;
] "vandnpd"     = [ b"y*y*w*",   [   1, 0x55      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "andnps"      = [ b"yowo",     [0x0F, 0x55      ], X;
] "vandnps"     = [ b"y*y*w*",   [   1, 0x55      ], X,           AUTO_VEXL | VEX_OP;
] "andpd"       = [ b"yowo",     [0x0F, 0x54      ], X, PREF_66;
] "vandpd"      = [ b"y*y*w*",   [   1, 0x54      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "andps"       = [ b"yowo",     [0x0F, 0x54      ], X;
] "vandps"      = [ b"y*y*w*",   [   1, 0x54      ], X,           AUTO_VEXL | VEX_OP;
] "blendpd"     = [ b"yowoib",   [0x0F, 0x3A, 0x0D], X, PREF_66;
] "vblendpd"    = [ b"y*y*w*ib", [   3, 0x0D      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "blendps"     = [ b"yowoib",   [0x0F, 0x3A, 0x0C], X, PREF_66;
] "vblendps"    = [ b"y*y*w*ib", [   3, 0x0C      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "blendvpd"    = [ b"yowo",     [0x0F, 0x38, 0x15], X, PREF_66;
] "vblendvpd"   = [ b"y*y*w*y*", [   3, 0x4B      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "blendvps"    = [ b"yowo",     [0x0F, 0x38, 0x14], X, PREF_66;
] "vblendvps"   = [ b"y*y*w*y*", [   3, 0x4A      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "cmppd"       = [ b"yowoib",   [0x0F, 0xC2      ], X, PREF_66;
] "vcmppd"      = [ b"y*y*w*ib", [   1, 0xC2      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "cmpps"       = [ b"yowoib",   [0x0F, 0xC2      ], X;
] "vcmpps"      = [ b"y*y*w*ib", [   1, 0xC2      ], X,           AUTO_VEXL | VEX_OP;
] // cmpsd is found in generic instructions
  "vcmpsd"      = [ b"y*y*w*ib", [   1, 0xC2      ], X, PREF_F2 | AUTO_VEXL | VEX_OP;
] "cmpss"       = [ b"yowoib",   [0x0F, 0xC2      ], X, PREF_F3;
] "vcmpss"      = [ b"y*y*w*ib", [   1, 0xC2      ], X, PREF_F3 | AUTO_VEXL | VEX_OP;
] "comisd"      = [ b"yoyo",     [0x0F, 0x2F      ], X, PREF_66;
                    b"yomq",     [0x0F, 0x2F      ], X, PREF_66;
] "vcomisd"     = [ b"yoyo",     [   1, 0x2F      ], X, PREF_66             | VEX_OP;
                    b"yomq",     [   1, 0x2F      ], X, PREF_66             | VEX_OP;
] "comiss"      = [ b"yoyo",     [0x0F, 0x2F      ], X;
                    b"yomd",     [0x0F, 0x2F      ], X;
] "vcomiss"     = [ b"yoyo",     [   1, 0x2F      ], X,                       VEX_OP;
                    b"yomd",     [   1, 0x2F      ], X,                       VEX_OP;
]

  "cvtdq2pd"    = [ b"yoyo",     [0x0F, 0xE6      ], X, PREF_F3;
                    b"yomq",     [0x0F, 0xE6      ], X, PREF_F3;
] "vcvtdq2pd"   = [ b"y*y*",     [   1, 0xE6      ], X, PREF_F3 | AUTO_VEXL | VEX_OP;
                    b"yomq",     [   1, 0xE6      ], X, PREF_F3             | VEX_OP;
                    b"yhmo",     [   1, 0xE6      ], X, PREF_F3 | WITH_VEXL | VEX_OP; // intel/amd disagree over this memory ops size
] "cvtdq2ps"    = [ b"yowo",     [0x0F, 0x5B      ], X;
] "vcvtdq2ps"   = [ b"y*w*",     [   1, 0x5B      ], X,           AUTO_VEXL | VEX_OP;
] "cvtpd2dq"    = [ b"yowo",     [0x0F, 0xE6      ], X, PREF_F2;
] "vcvtpd2dq"   = [ b"y*w*",     [   1, 0xE6      ], X, PREF_F2 | AUTO_VEXL | VEX_OP;
] "cvtpd2dS"    = [ b"yowo",     [0x0F, 0x5A      ], X, PREF_66;
] "vcvtpd2dS"   = [ b"y*w*",     [   1, 0x5A      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "cvtps2dq"    = [ b"yowo",     [0x0F, 0x5B      ], X, PREF_66;
] "vcvtps2dq"   = [ b"y*w*",     [   1, 0x5B      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "cvtps2pd"    = [ b"yoyo",     [0x0F, 0x5A      ], X;
                    b"yomq",     [0x0F, 0x5A      ], X;
] "vcvtps2pd"   = [ b"y*y*",     [   1, 0x5A      ], X,           AUTO_VEXL | VEX_OP;
                    b"yomq",     [   1, 0x5A      ], X,                       VEX_OP;
                    b"yhmo",     [   1, 0x5A      ], X,           WITH_VEXL | VEX_OP; // intel/amd disagree over this memory ops size
] "cvtsd2si"    = [ b"r*yo",     [0x0F, 0x2D      ], X, PREF_F2 | AUTO_REXW;
                    b"r*mq",     [0x0F, 0x2D      ], X, PREF_F2 | AUTO_REXW;
] "vcvtsd2si"   = [ b"r*yo",     [   1, 0x2D      ], X, PREF_F2 | AUTO_REXW | VEX_OP;
                    b"r*mq",     [   1, 0x2D      ], X, PREF_F2 | AUTO_REXW | VEX_OP;
] "cvtsd2ss"    = [ b"yoyo",     [0x0F, 0x5A      ], X, PREF_F2;
                    b"yomq",     [0x0F, 0x5A      ], X, PREF_F2;
] "vcvtsd2ss"   = [ b"yoyoyo",   [   1, 0x5A      ], X, PREF_F2             | VEX_OP;
                    b"yoyomq",   [   1, 0x5A      ], X, PREF_F2             | VEX_OP;
] "cvtsi2sd"    = [ b"yov*",     [0x0F, 0x2A      ], X, PREF_F2 | AUTO_REXW;
] "vcvtsi2sd"   = [ b"yoyov*",   [   1, 0x2A      ], X, PREF_F2 | AUTO_REXW | VEX_OP;
] "cvtsi2ss"    = [ b"yov*",     [0x0F, 0x2A      ], X, PREF_F3 | AUTO_REXW;
] "vcvtsi2ss"   = [ b"yoyov*",   [   1, 0x2A      ], X, PREF_F3 | AUTO_REXW | VEX_OP;
] "cvtss2sd"    = [ b"yoyo",     [0x0F, 0x5A      ], X, PREF_F3;
                    b"yomd",     [0x0F, 0x5A      ], X, PREF_F3;
] "vcvtss2sd"   = [ b"yoyo",     [   1, 0x5A      ], X, PREF_F3             | VEX_OP;
                    b"yomq",     [   1, 0x5A      ], X, PREF_F3             | VEX_OP;
] "cvtss2si"    = [ b"r*yo",     [0x0F, 0x2D      ], X, PREF_F3 | AUTO_REXW;
                    b"r*m*",     [0x0F, 0x2D      ], X, PREF_F3 | AUTO_REXW;
] "vcvtss2si"   = [ b"r*yo",     [   1, 0x2D      ], X, PREF_F3 | AUTO_REXW | VEX_OP;
                    b"r*m*",     [   1, 0x2D      ], X, PREF_F3 | AUTO_REXW | VEX_OP;
] "cvttpd2dq"   = [ b"yowo",     [0x0F, 0xE6      ], X, PREF_66;
] "vcvttpd2dq"  = [ b"y*w*",     [   1, 0xE6      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "cvttps2dq"   = [ b"yowo",     [0x0F, 0x5B      ], X, PREF_F3;
] "vcvttps2dq"  = [ b"y*w*",     [   1, 0x5B      ], X, PREF_F3 | AUTO_VEXL | VEX_OP;
] "cvttsd2si"   = [ b"r*yo",     [0x0F, 0x2C      ], X, PREF_F2 | AUTO_REXW;
                    b"r*mq",     [0x0F, 0x2C      ], X, PREF_F2 | AUTO_REXW;
] "vcvttsd2si"  = [ b"r*yo",     [   1, 0x2C      ], X, PREF_F2 | AUTO_REXW | VEX_OP;
                    b"r*mq",     [   1, 0x2C      ], X, PREF_F2 | AUTO_REXW | VEX_OP;
] "cvttss2si"   = [ b"r*yo",     [0x0F, 0x2C      ], X, PREF_F3 | AUTO_REXW;
                    b"r*m*",     [0x0F, 0x2C      ], X, PREF_F3 | AUTO_REXW;
] "vcvttss2si"  = [ b"r*yo",     [   1, 0x2C      ], X, PREF_F3 | AUTO_REXW | VEX_OP;
                    b"r*m*",     [   1, 0x2C      ], X, PREF_F3 | AUTO_REXW | VEX_OP;
]

  "divpd"       = [ b"yowo",     [0x0F, 0x5E      ], X, PREF_66;
] "vdivpd"      = [ b"y*y*w*",   [   1, 0x5E      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "divps"       = [ b"yowo",     [0x0F, 0x5E      ], X;
] "vdivps"      = [ b"y*y*w*",   [   1, 0x5E      ], X,           AUTO_VEXL | VEX_OP;
] "divsd"       = [ b"yoyo",     [0x0F, 0x5E      ], X, PREF_F2;
                    b"yomq",     [0x0F, 0x5E      ], X, PREF_F2;
] "vdivsd"      = [ b"yoyoyo",   [   1, 0x5E      ], X, PREF_F2             | VEX_OP;
                    b"yoyomq",   [   1, 0x5E      ], X, PREF_F2             | VEX_OP;
] "divss"       = [ b"yoyo",     [0x0F, 0x5E      ], X, PREF_F3;
                    b"yomd",     [0x0F, 0x5E      ], X, PREF_F3;
] "vdivss"      = [ b"yoyoyo",   [   1, 0x5E      ], X, PREF_F3             | VEX_OP;
                    b"yoyomd",   [   1, 0x5E      ], X, PREF_F3             | VEX_OP;
] "dppd"        = [ b"yowoib",   [0x0F, 0x3A, 0x41], X, PREF_66;
] "vdppd"       = [ b"yoyowoib", [   3, 0x41      ], X, PREF_66             | VEX_OP;
] "dpps"        = [ b"yowoib",   [0x0F, 0x3A, 0x40], X, PREF_66;
] "vdpps"       = [ b"y*y*w*ib", [   3, 0x40      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "extractps"   = [ b"vwyoib",   [0x0F, 0x3A, 0x17], X, PREF_66;
] "vextractps"  = [ b"vwyoib",   [   3, 0x17      ], X, PREF_66             | VEX_OP;
] "haddpd"      = [ b"yowo",     [0x0F, 0x7C      ], X, PREF_66;
] "vhaddpd"     = [ b"y*y*w*",   [   1, 0x7C      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "haddps"      = [ b"yowo",     [0x0F, 0x7C      ], X, PREF_F2;
] "vhaddps"     = [ b"y*y*w*",   [   1, 0x7C      ], X, PREF_F2 | AUTO_VEXL | VEX_OP;
] "hsubpd"      = [ b"yowo",     [0x0F, 0x7D      ], X, PREF_66;
] "vhsubpd"     = [ b"y*y*w*",   [   1, 0x7D      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "hsubps"      = [ b"yowo",     [0x0F, 0x7D      ], X, PREF_F2;
] "vhsubps"     = [ b"y*y*w*",   [   1, 0x7D      ], X, PREF_F2 | AUTO_VEXL | VEX_OP;
] "insertps"    = [ b"yowoib",   [0x0F, 0x3A, 0x21], X, PREF_66;
] "vinsertps"   = [ b"yoyowoib", [   3, 0x21      ], X, PREF_66             | VEX_OP;
] "lddqu"       = [ b"yomo",     [0x0F, 0xF0      ], X, PREF_F2;
] "vlddqu"      = [ b"y*m*",     [   1, 0xF0      ], X, PREF_F2 | AUTO_VEXL | VEX_OP;
] "ldmxcsr"     = [ b"md",       [0x0F, 0xAE      ], 2;
] "vldmxcsr"    = [ b"md",       [   1, 0xAE      ], 2,                       VEX_OP;
] "maskmovdqu"  = [ b"yoyo",     [0x0F, 0xF7      ], X, PREF_66;
] "vmaskmovdqu" = [ b"yoyo",     [   1, 0xF7      ], X, PREF_66             | VEX_OP;
] "maxpd"       = [ b"yowo",     [0x0F, 0x5F      ], X, PREF_66;
] "vmaxpd"      = [ b"y*y*w*",   [   1, 0x5F      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "maxps"       = [ b"yowo",     [0x0F, 0x5F      ], X;
] "vmaxps"      = [ b"y*y*w*",   [   1, 0x5F      ], X,           AUTO_VEXL | VEX_OP;
] "maxsd"       = [ b"yoyo",     [0x0F, 0x5F      ], X, PREF_F2;
                    b"yomq",     [0x0F, 0x5F      ], X, PREF_F2;
] "vmaxsd"      = [ b"yoyoyo",   [   1, 0x5F      ], X, PREF_F2             | VEX_OP;
                    b"yoyomq",   [   1, 0x5F      ], X, PREF_F2             | VEX_OP;
] "maxss"       = [ b"yoyo",     [0x0F, 0x5F      ], X, PREF_F3;
                    b"yomd",     [0x0F, 0x5F      ], X, PREF_F3;
] "vmaxss"      = [ b"yoyoyo",   [   1, 0x5F      ], X, PREF_F3             | VEX_OP;
                    b"yoyomd",   [   1, 0x5F      ], X, PREF_F3             | VEX_OP;
] "minpd"       = [ b"yowo",     [0x0F, 0x5D      ], X, PREF_66;
] "vminpd"      = [ b"y*y*w*",   [   1, 0x5D      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "minps"       = [ b"yowo",     [0x0F, 0x5D      ], X;
] "vminps"      = [ b"y*y*w*",   [   1, 0x5D      ], X,           AUTO_VEXL | VEX_OP;
] "minsd"       = [ b"yoyo",     [0x0F, 0x5D      ], X, PREF_F2;
                    b"yomq",     [0x0F, 0x5D      ], X, PREF_F2;
] "vminsd"      = [ b"yoyoyo",   [   1, 0x5D      ], X, PREF_F2             | VEX_OP;
                    b"yoyomq",   [   1, 0x5D      ], X, PREF_F2             | VEX_OP;
] "minss"       = [ b"yoyo",     [0x0F, 0x5D      ], X, PREF_F3;
                    b"yomd",     [0x0F, 0x5D      ], X, PREF_F3;
] "vminss"      = [ b"yoyoyo",   [   1, 0x5D      ], X, PREF_F3             | VEX_OP;
                    b"yoyomd",   [   1, 0x5D      ], X, PREF_F3             | VEX_OP;
]

  "movapd"      = [ b"yowo",     [0x0F, 0x28      ], X, PREF_66;
                    b"woyo",     [0x0F, 0x29      ], X, PREF_66;
] "vmovapd"     = [ b"y*w*",     [   1, 0x28      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"w*y*",     [   1, 0x29      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "movaps"      = [ b"yowo",     [0x0F, 0x28      ], X;
                    b"woyo",     [0x0F, 0x29      ], X;
] "vmovaps"     = [ b"y*w*",     [   1, 0x28      ], X,           AUTO_VEXL | VEX_OP;
                    b"w*y*",     [   1, 0x29      ], X,           AUTO_VEXL | VEX_OP;
] // movd is found under the general purpose instructions
  "vmovd"       = [ b"yov*",     [   1, 0x6E      ], X, PREF_66 | AUTO_REXW | VEX_OP;
                    b"v*yo",     [   1, 0x7E      ], X, PREF_66 | AUTO_REXW | VEX_OP;
] "movddup"     = [ b"yoyo",     [0x0F, 0x12      ], X, PREF_F2;
                    b"yomq",     [0x0F, 0x12      ], X, PREF_F2;
] "vmovddup"    = [ b"y*y*",     [   1, 0x12      ], X, PREF_F2 | AUTO_VEXL | VEX_OP;
                    b"yomq",     [   1, 0x12      ], X, PREF_F2             | VEX_OP;
                    b"yhmh",     [   1, 0x12      ], X, PREF_F2 | WITH_VEXL | VEX_OP;
] "movdqa"      = [ b"yowo",     [0x0F, 0x6F      ], X, PREF_66;
                    b"woyo",     [0x0F, 0x7F      ], X, PREF_66;
] "vmovdqa"     = [ b"y*w*",     [   1, 0x6F      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"w*y*",     [   1, 0x7F      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "movdqu"      = [ b"yowo",     [0x0F, 0x6F      ], X, PREF_F3;
                    b"woyo",     [0x0F, 0x7F      ], X, PREF_F3;
] "vmovdqu"     = [ b"y*w*",     [   1, 0x6F      ], X, PREF_F3 | AUTO_VEXL | VEX_OP;
                    b"w*y*",     [   1, 0x7F      ], X, PREF_F3 | AUTO_VEXL | VEX_OP;
] "movhlps"     = [ b"yoyo",     [0x0F, 0x12      ], X;
] "vmovhlps"    = [ b"yoyoyo",   [   1, 0x12      ], X,                       VEX_OP;
] "movhpd"      = [ b"yomq",     [0x0F, 0x16      ], X, PREF_66;
                    b"mqyo",     [0x0F, 0x17      ], X, PREF_66;
] "vmovhpd"     = [ b"yoyomq",   [   1, 0x16      ], X, PREF_66             | VEX_OP;
                    b"mqyo",     [   1, 0x17      ], X, PREF_66             | VEX_OP;
] "movhps"      = [ b"yomq",     [0x0F, 0x16      ], X;
                    b"mqyo",     [0x0F, 0x17      ], X;
] "vmovhps"     = [ b"yoyomq",   [   1, 0x16      ], X,                       VEX_OP;
                    b"mqyo",     [   1, 0x17      ], X,                       VEX_OP;
] "movlhps"     = [ b"yoyo",     [0x0F, 0x16      ], X;
] "vmovlhps"    = [ b"yoyoyo",   [   1, 0x16      ], X,                       VEX_OP;
] "movlpd"      = [ b"yomq",     [0x0F, 0x12      ], X, PREF_66;
                    b"mqyo",     [0x0F, 0x13      ], X, PREF_66;
] "vmovlpd"     = [ b"yoyomq",   [   1, 0x12      ], X, PREF_66             | VEX_OP;
                    b"mqyo",     [   1, 0x13      ], X, PREF_66             | VEX_OP;
] "movlps"      = [ b"yomq",     [0x0F, 0x12      ], X;
                    b"mqyo",     [0x0F, 0x13      ], X;
] "vmovlps"     = [ b"yoyomq",   [   1, 0x12      ], X,                       VEX_OP;
                    b"mqyo",     [   1, 0x13      ], X,                       VEX_OP;
] // movmskpd is found under generic instrs
  "vmovmskpd"   = [ b"r?y*",     [   1, 0x50      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // same for movmskps
  "vmovmskps"   = [ b"r?y*",     [   1, 0x50      ], X,           AUTO_VEXL | VEX_OP;
] "movntdq"     = [ b"moyo",     [0x0F, 0xE7      ], X, PREF_66;
] "vmovntdq"    = [ b"m*y*",     [   1, 0xE7      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "movntdqa"    = [ b"moyo",     [0x0F, 0x38, 0x2A], X, PREF_66;
] "vmovntdqa"   = [ b"m*y*",     [   2, 0x2A      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "movntpd"     = [ b"moyo",     [0x0F, 0x2B      ], X, PREF_66;
] "vmovntpd"    = [ b"m*y*",     [   1, 0x2B      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "movntps"     = [ b"moyo",     [0x0F, 0x2B      ], X;
] "vmovntps"    = [ b"m*y*",     [   1, 0x2B      ], X,           AUTO_VEXL | VEX_OP;
] "movntsd"     = [ b"mqyo",     [0x0F, 0x2B      ], X, PREF_F2;
] "movntss"     = [ b"mdyo",     [0x0F, 0x2B      ], X, PREF_F3;
  // movq variants can be found in the MMX section
] "vmovq"       = [ b"yoyo",     [   1, 0x7E      ], X, PREF_F3             | VEX_OP;
                    b"yomq",     [   1, 0x7E      ], X, PREF_F3             | VEX_OP;
                    b"mqyo",     [   1, 0xD6      ], X, PREF_66             | VEX_OP;
  // movsd variants can be found in the general purpose section
] "vmovsd"      = [ b"yoyoyo",   [   1, 0x10      ], X, PREF_F2             | VEX_OP; // distinguished from the others by addressing bits
                    b"yomq",     [   1, 0x10      ], X, PREF_F2             | VEX_OP;
                    b"mqyo",     [   1, 0x11      ], X, PREF_F2             | VEX_OP;
] "movshdup"    = [ b"yowo",     [0x0F, 0x16      ], X, PREF_F3;
] "vmovshdup"   = [ b"y*w*",     [   1, 0x16      ], X, PREF_F3 | AUTO_VEXL | VEX_OP;
] "movsldup"    = [ b"yowo",     [0x0F, 0x12      ], X, PREF_F3;
] "vmovsldup"   = [ b"y*w*",     [   1, 0x12      ], X, PREF_F3 | AUTO_VEXL | VEX_OP;
] "movss"       = [ b"yoyo",     [0x0F, 0x10      ], X, PREF_F3;
                    b"yomd",     [0x0F, 0x10      ], X, PREF_F3;
                    b"mdyo",     [0x0F, 0x11      ], X, PREF_F3;
] "vmovss"      = [ b"yoyoyo",   [   1, 0x10      ], X, PREF_F3             | VEX_OP;
                    b"yomd",     [   1, 0x10      ], X, PREF_F3             | VEX_OP;
                    b"mdyo",     [   1, 0x11      ], X, PREF_F3             | VEX_OP;
] "movupd"      = [ b"yowo",     [0x0F, 0x10      ], X, PREF_66;
                    b"woyo",     [0x0F, 0x11      ], X, PREF_66;
] "vmovupd"     = [ b"y*w*",     [   1, 0x10      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"w*y*",     [   1, 0x11      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "movups"      = [ b"yowo",     [0x0F, 0x10      ], X;
                    b"woyo",     [0x0F, 0x11      ], X;
] "vmovups"     = [ b"y*w*",     [   1, 0x10      ], X,           AUTO_VEXL | VEX_OP;
                    b"w*y*",     [   1, 0x11      ], X,           AUTO_VEXL | VEX_OP;
]
// and we're done with mov ins.
  "mpsadbw"     = [ b"yowoib",   [0x0F, 0x3A, 0x42], X, PREF_66;
] "vmpsadbw"    = [ b"y*y*w*ib", [   3, 0x42      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "mulpd"       = [ b"yowo",     [0x0F, 0x59      ], X, PREF_66;
] "vmulpd"      = [ b"y*y*w*",   [   1, 0x59      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "mulps"       = [ b"yowo",     [0x0F, 0x59      ], X;
] "vmulps"      = [ b"y*y*w*",   [   1, 0x59      ], X,           AUTO_VEXL | VEX_OP;
] "mulsd"       = [ b"yoyo",     [0x0F, 0x59      ], X, PREF_F2;
                    b"yomq",     [0x0F, 0x59      ], X, PREF_F2;
] "vmulsd"      = [ b"yoyoyo",   [   1, 0x59      ], X, PREF_F2             | VEX_OP;
                    b"yoyomq",   [   1, 0x59      ], X, PREF_F2             | VEX_OP;
] "mulss"       = [ b"yoyo",     [0x0F, 0x59      ], X, PREF_F3;
                    b"yomd",     [0x0F, 0x59      ], X, PREF_F3;
] "vmulss"      = [ b"yoyoyo",   [   1, 0x59      ], X, PREF_F3             | VEX_OP;
                    b"yoyomd",   [   1, 0x59      ], X, PREF_F3             | VEX_OP;
] "orpd"        = [ b"yowo",     [0x0F, 0x56      ], X, PREF_66;
] "vorpd"       = [ b"y*y*w*",   [   1, 0x56      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "orps"        = [ b"yowo",     [0x0F, 0x56      ], X;
] "vorps"       = [ b"y*y*w*",   [   1, 0x56      ], X,           AUTO_VEXL | VEX_OP;
] "pabsb"       = [ b"yowo",     [0x0F, 0x38, 0x1C], X;
] "vpabsb"      = [ b"y*w*",     [   2, 0x1C      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pabsd"       = [ b"yowo",     [0x0F, 0x38, 0x1E], X;
] "vpabsd"      = [ b"y*w*",     [   2, 0x1E      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pabsw"       = [ b"yowo",     [0x0F, 0x38, 0x1D], X;
] "vpabsw"      = [ b"y*w*",     [   2, 0x1D      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // packssdw is found in the MMX section
  "vpackssdw"   = [ b"y*y*w*",   [   1, 0x6B      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // same for packsswb
  "vpacksswb"   = [ b"y*y*w*",   [   1, 0x63      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "packusdw"    = [ b"yowo",     [0x0F, 0x38, 0x2B], X, PREF_66;
] "vpackusdw"   = [ b"y*y*w*",   [   2, 0x2B      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
]  // and for packuswb
  "vpackuswb"   = [ b"y*y*w*",   [   1, 0x67      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // and all legacy padd forms
  "vpaddb"      = [ b"y*y*w*",   [   1, 0xFC      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpaddd"      = [ b"y*y*w*",   [   1, 0xFE      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpaddq"      = [ b"y*y*w*",   [   1, 0xD4      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpaddsb"     = [ b"y*y*w*",   [   1, 0xEC      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpaddsw"     = [ b"y*y*w*",   [   1, 0xED      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpaddusb"    = [ b"y*y*w*",   [   1, 0xDC      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpaddusw"    = [ b"y*y*w*",   [   1, 0xDD      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpaddw"      = [ b"y*y*w*",   [   1, 0xFD      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "palign"      = [ b"y*w*ib",   [0x0F, 0x3A, 0x0F], X, PREF_66;
] "vpalign"     = [ b"y*y*w*ib", [   3, 0x0F      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // pand/pandn/pavg are also in the MMX section
  "vpand"       = [ b"y*y*w*",   [   1, 0xDB      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpandn"      = [ b"y*y*w*",   [   1, 0xDF      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpavgb"      = [ b"y*y*w*",   [   1, 0xE0      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpavgw"      = [ b"y*y*w*",   [   1, 0xE3      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pblendvb"    = [ b"yowo",     [0x0F, 0x38, 0x10], X, PREF_66;
] "vpblendvb"   = [ b"y*y*w*y*", [   3, 0x4C      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pblenddw"    = [ b"yowoib",   [0x0F, 0x3A, 0x0E], X, PREF_66;
] "vpblenddw"   = [ b"y*y*w*ib", [   3, 0x0E      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pclmulqdq"   = [ b"yowoib",   [0x0F, 0x3A, 0x44], X, PREF_66;
] "vpclmulqdq"  = [ b"yoyowoib", [   3, 0x44      ], X, PREF_66             | VEX_OP;
] // pcmpeqb is in the MMX section
  "vpcmpeqb"    = [ b"y*y*w*",   [   1, 0x74      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // pcmpeqd is in the MMX section
  "vpcmpeqd"    = [ b"y*y*w*",   [   1, 0x76      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pcmpeqq"     = [ b"yowo",     [0x0F, 0x38, 0x29], X, PREF_66;
] "vpcmpeqq"    = [ b"y*y*w*",   [   2, 0x29      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // pcmpeqw is in the MMX section
  "vpcmpeqw"    = [ b"y*y*w*",   [   1, 0x75      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pcmpestri"   = [ b"yowoib",   [0x0F, 0x3A, 0x61], X, PREF_66;
] "vpcmpestri"  = [ b"yowoib",   [   3, 0x61      ], X, PREF_66             | VEX_OP;
] "pcmpestrm"   = [ b"yowoib",   [0x0F, 0x3A, 0x60], X, PREF_66;
] "vpcmpestrm"  = [ b"yowoib",   [   3, 0x60      ], X, PREF_66             | VEX_OP;
] // pcmpgtb is in the MMX section
  "vpcmpgtb"    = [ b"y*y*w*",   [   1, 0x64      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // pcmpgtd is in the MMX section
  "vpcmpgtd"    = [ b"y*y*w*",   [   1, 0x66      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pcmpgtq"     = [ b"yowo",     [0x0F, 0x38, 0x37], X, PREF_66;
] "vpcmpgtq"    = [ b"y*y*w*",   [   2, 0x37      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // pcmpgtw is in the MMX section
  "vpcmpgtw"    = [ b"y*y*w*",   [   1, 0x65      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pcmpistri"   = [ b"yowoib",   [0x0F, 0x3A, 0x63], X, PREF_66;
] "vpcmpistri"  = [ b"yowoib",   [   3, 0x63      ], X, PREF_66             | VEX_OP;
] "pcmpistrm"   = [ b"yowoib",   [0x0F, 0x3A, 0x62], X, PREF_66;
] "vpcmpistrm"  = [ b"yowoib",   [   3, 0x62      ], X, PREF_66             | VEX_OP;
] "pextrb"      = [ b"r?yoib",   [0x0F, 0x3A, 0x14], X, PREF_66                      | ENC_MR;
                    b"mbyoib",   [0x0F, 0x3A, 0x14], X, PREF_66;
] "vpextrb"     = [ b"r?yoib",   [   3, 0x14      ], X, PREF_66             | VEX_OP | ENC_MR;
                    b"mbyoib",   [   3, 0x14      ], X, PREF_66             | VEX_OP;
] "pextrd"      = [ b"vdyoib",   [0x0F, 0x3A, 0x16], X, PREF_66;
] "vpextrd"     = [ b"vdyoib",   [   3, 0x16      ], X, PREF_66             | VEX_OP;
] "pextrq"      = [ b"vqyoib",   [0x0F, 0x3A, 0x16], X, PREF_66 | WITH_REXW;
] "vpextrq"     = [ b"vqyoib",   [   3, 0x16      ], X, PREF_66 | WITH_REXW| VEX_OP;
] // pextrw is in the MMX section
  "vpextrw"     = [ b"r?yoib",   [   1, 0xC5      ], X, PREF_66             | VEX_OP | ENC_MR;
                    b"mwyoib",   [   3, 0x15      ], X, PREF_66             | VEX_OP;
] "phaddd"      = [ b"yowo",     [0x0F, 0x38, 0x02], X, PREF_66;
] "vphaddd"     = [ b"y*y*w*",   [   2, 0x02      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "phaddsw"     = [ b"yowo",     [0x0F, 0x38, 0x03], X, PREF_66;
] "vphaddsw"    = [ b"y*y*w*",   [   2, 0x03      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "phaddw"      = [ b"yowo",     [0x0F, 0x38, 0x01], X, PREF_66;
] "vphaddw"     = [ b"y*y*w*",   [   2, 0x01      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "phminposuw"  = [ b"yowo",     [0x0F, 0x38, 0x41], X, PREF_66;
] "vphminposuw" = [ b"yowo",     [   2, 0x41      ], X, PREF_66             | VEX_OP;
] "phsubd"      = [ b"yowo",     [0x0F, 0x38, 0x06], X, PREF_66;
] "vphsubd"     = [ b"y*y*w*",   [   2, 0x06      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "phsubsw"     = [ b"yowo",     [0x0F, 0x38, 0x07], X, PREF_66;
] "vphsubsw"    = [ b"y*y*w*",   [   2, 0x07      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "phsubw"      = [ b"yowo",     [0x0F, 0x38, 0x05], X, PREF_66;
] "vphsubw"     = [ b"y*y*w*",   [   2, 0x05      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pinsrb"      = [ b"yordib",   [0x0F, 0x3A, 0x20], X, PREF_66;
                    b"yombib",   [0x0F, 0x3A, 0x20], X, PREF_66;
] "vpinsrb"     = [ b"yordyoib", [   3, 0x20      ], X, PREF_66             | VEX_OP;
                    b"yombyoib", [   3, 0x20      ], X, PREF_66             | VEX_OP;
] "pinsrd"      = [ b"yovdib",   [0x0F, 0x3A, 0x22], X, PREF_66;
] "vpinsrd"     = [ b"yovdyoib", [   3, 0x22      ], X, PREF_66             | VEX_OP;
] "pinsrq"      = [ b"yovqib",   [0x0F, 0x3A, 0x22], X, PREF_66 | WITH_REXW;
] "vpinsrq"     = [ b"yovqyoib", [   3, 0x22      ], X, PREF_66 | WITH_REXW| VEX_OP;
] // pinsrw is in thbe MMX section
  "vpinsrw"     = [ b"yordyoib", [   1, 0xC4      ], X, PREF_66             | VEX_OP | ENC_MR;
                    b"yomwyoib", [   1, 0xC4      ], X, PREF_66             | VEX_OP;
] "pmaddubsw"   = [ b"yowo",     [0x0F, 0x38, 0x04], X, PREF_66;
] "vpmaddubsw"  = [ b"y*y*w*",   [   2, 0x04      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // pmaddwd is in the MMX section
  "vpmaddwd"    = [ b"y*y*w*",   [   1, 0xF5      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pmaxsb"      = [ b"yowo",     [0x0F, 0x38, 0x3C], X, PREF_66;
] "vpmaxsb"     = [ b"y*y*w*",   [   2, 0x3C      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pmaxsd"      = [ b"yowo",     [0x0F, 0x38, 0x3D], X, PREF_66;
] "vpmaxsd"     = [ b"y*y*w*",   [   2, 0x3D      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // pmaxsw is in the MMX section
  "vpmaxsw"     = [ b"y*y*w*",   [   1, 0xEE      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // pmaxub is in the MMX section
  "vpmaxub"     = [ b"y*y*w*",   [   1, 0xDE      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pmaxud"      = [ b"yowo",     [0x0F, 0x38, 0x3F], X, PREF_66;
] "vpmaxud"     = [ b"y*y*w*",   [   2, 0x3F      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pmaxuw"      = [ b"yowo",     [0x0F, 0x38, 0x3E], X, PREF_66;
] "vpmaxuw"     = [ b"y*y*w*",   [   2, 0x3E      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
]
  "pminsb"      = [ b"yowo",     [0x0F, 0x38, 0x38], X, PREF_66;
] "vpminsb"     = [ b"y*y*w*",   [   2, 0x38      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pminsd"      = [ b"yowo",     [0x0F, 0x38, 0x39], X, PREF_66;
] "vpminsd"     = [ b"y*y*w*",   [   2, 0x39      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // pminsw is in the MMX section
  "vpminsw"     = [ b"y*y*w*",   [   1, 0xEA      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // pminub is in the MMX section
  "vpminub"     = [ b"y*y*w*",   [   1, 0xDA      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pminud"      = [ b"yowo",     [0x0F, 0x38, 0x3B], X, PREF_66;
] "vpminud"     = [ b"y*y*w*",   [   2, 0x3B      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pminuw"      = [ b"yowo",     [0x0F, 0x38, 0x3A], X, PREF_66;
] "vpminuw"     = [ b"y*y*w*",   [   2, 0x3A      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
]
// back to move ops
  // pmovmskb is in the MMX section
  "vpmovmskb"   = [ b"rqy*",     [   1, 0xD7      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pmovsxbd"    = [ b"yoyo",     [0x0F, 0x38, 0x21], X, PREF_66;
                    b"yomd",     [0x0F, 0x38, 0x21], X, PREF_66;
] "vpmovsxbd"   = [ b"y*y*",     [   2, 0x21      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"yomd",     [   2, 0x21      ], X, PREF_66             | VEX_OP;
                    b"yhmq",     [   2, 0x21      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "pmovsxbq"    = [ b"yoyo",     [0x0F, 0x38, 0x22], X, PREF_66;
                    b"yomw",     [0x0F, 0x38, 0x22], X, PREF_66;
] "vpmovsxbq"   = [ b"y*y*",     [   2, 0x22      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"yomw",     [   2, 0x22      ], X, PREF_66             | VEX_OP;
                    b"yhmd",     [   2, 0x22      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "pmovsxbw"    = [ b"yoyo",     [0x0F, 0x38, 0x20], X, PREF_66;
                    b"yomq",     [0x0F, 0x38, 0x20], X, PREF_66;
] "vpmovsxbw"   = [ b"y*y*",     [   2, 0x20      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"yomq",     [   2, 0x20      ], X, PREF_66             | VEX_OP;
                    b"yhmo",     [   2, 0x20      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "pmovsxdq"    = [ b"yoyo",     [0x0F, 0x38, 0x25], X, PREF_66;
                    b"yomq",     [0x0F, 0x38, 0x25], X, PREF_66;
] "vpmovsxdq"   = [ b"y*y*",     [   2, 0x25      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"yomq",     [   2, 0x25      ], X, PREF_66             | VEX_OP;
                    b"yhmo",     [   2, 0x25      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "pmovsxwd"    = [ b"yoyo",     [0x0F, 0x38, 0x23], X, PREF_66;
                    b"yomq",     [0x0F, 0x38, 0x23], X, PREF_66;
] "vpmovsxwd"   = [ b"y*y*",     [   2, 0x23      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"yomq",     [   2, 0x23      ], X, PREF_66             | VEX_OP;
                    b"yhmo",     [   2, 0x23      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "pmovsxwq"    = [ b"yoyo",     [0x0F, 0x38, 0x24], X, PREF_66;
                    b"yomd",     [0x0F, 0x38, 0x24], X, PREF_66;
] "vpmovsxwq"   = [ b"y*y*",     [   2, 0x24      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"yomd",     [   2, 0x24      ], X, PREF_66             | VEX_OP;
                    b"yhmq",     [   2, 0x24      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "pmovzxbd"    = [ b"yoyo",     [0x0F, 0x38, 0x31], X, PREF_66;
                    b"yomd",     [0x0F, 0x38, 0x31], X, PREF_66;
] "vpmovzxbd"   = [ b"y*y*",     [   2, 0x31      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"yomd",     [   2, 0x31      ], X, PREF_66             | VEX_OP;
                    b"yhmq",     [   2, 0x31      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "pmovzxbq"    = [ b"yoyo",     [0x0F, 0x38, 0x32], X, PREF_66;
                    b"yomw",     [0x0F, 0x38, 0x32], X, PREF_66;
] "vpmovzxbq"   = [ b"y*y*",     [   2, 0x32      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"yomw",     [   2, 0x32      ], X, PREF_66             | VEX_OP;
                    b"yhmd",     [   2, 0x32      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "pmovzxbw"    = [ b"yoyo",     [0x0F, 0x38, 0x30], X, PREF_66;
                    b"yomq",     [0x0F, 0x38, 0x30], X, PREF_66;
] "vpmovzxbw"   = [ b"y*y*",     [   2, 0x30      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"yomq",     [   2, 0x30      ], X, PREF_66             | VEX_OP;
                    b"yhmo",     [   2, 0x30      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "pmovzxdq"    = [ b"yoyo",     [0x0F, 0x38, 0x35], X, PREF_66;
                    b"yomq",     [0x0F, 0x38, 0x35], X, PREF_66;
] "vpmovzxdq"   = [ b"y*y*",     [   2, 0x35      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"yomq",     [   2, 0x35      ], X, PREF_66             | VEX_OP;
                    b"yhmo",     [   2, 0x35      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "pmovzxwd"    = [ b"yoyo",     [0x0F, 0x38, 0x33], X, PREF_66;
                    b"yomq",     [0x0F, 0x38, 0x33], X, PREF_66;
] "vpmovzxwd"   = [ b"y*y*",     [   2, 0x33      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"yomq",     [   2, 0x33      ], X, PREF_66             | VEX_OP;
                    b"yhmo",     [   2, 0x33      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "pmovzxwq"    = [ b"yoyo",     [0x0F, 0x38, 0x34], X, PREF_66;
                    b"yomd",     [0x0F, 0x38, 0x34], X, PREF_66;
] "vpmovzxwq"   = [ b"y*y*",     [   2, 0x34      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"yomd",     [   2, 0x34      ], X, PREF_66             | VEX_OP;
                    b"yhmq",     [   2, 0x34      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] // and back to arithmetric
  "pmuldq"      = [ b"yowo",     [0x0F, 0x38, 0x28], X, PREF_66;
] "vpmuldq"     = [ b"y*y*w*",   [   2, 0x28      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pmulhrsw"    = [ b"yowo",     [0x0F, 0x38, 0x0B], X, PREF_66;
] "vpmulhrsw"   = [ b"y*y*w*",   [   2, 0x0B      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // legacy form is in the MMX section
  "vpmulhuw"    = [ b"y*y*w*",   [   1, 0xE4      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // legacy form is in the MMX section
  "vpmulhw"     = [ b"y*y*w*",   [   1, 0xE5      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pmulld"      = [ b"yowo",     [0x0F, 0x38, 0x40], X, PREF_66;
] "vpmulld"     = [ b"y*y*w*",   [   2, 0x40      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // legacy form is in the MMX section
  "vpmullw"     = [ b"y*y*w*",   [   1, 0xD5      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // legacy form is in the MMX section
  "vpmuludq"    = [ b"y*y*w*",   [   1, 0xF4      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // legacy form is in the MMX section
  "vpor"        = [ b"y*y*w*",   [   1, 0xEB      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // legacy form is in the MMX section
  "vpsadbw"     = [ b"y*y*w*",   [   1, 0xF6      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pshufb"      = [ b"yowo",     [0x0F, 0x38, 0x00], X, PREF_66;
] "vpshufb"     = [ b"y*y*w*",   [   2, 0x00      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pshufd"      = [ b"yowoib",   [0x0F, 0x70      ], X, PREF_66;
] "vpshufd"     = [ b"y*w*ib",   [   1, 0x70      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // legacy form is in the MMX section
  "vpshufw"     = [ b"y*w*ib",   [   1, 0x70      ], X, PREF_F3 | AUTO_VEXL | VEX_OP;
] "pshuflw"     = [ b"yowoib",   [0x0F, 0x70      ], X, PREF_F2;
] "vpshuflw"    = [ b"y*w*ib",   [   1, 0x70      ], X, PREF_F2 | AUTO_VEXL | VEX_OP;
] "psignb"      = [ b"yowo",     [0x0F, 0x38, 0x08], X, PREF_66;
] "vpsignb"     = [ b"y*y*w*",   [   2, 0x08      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "psignd"      = [ b"yowo",     [0x0F, 0x38, 0x0A], X, PREF_66;
] "vpsignd"     = [ b"y*y*w*",   [   2, 0x0A      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "psignw"      = [ b"yowo",     [0x0F, 0x38, 0x09], X, PREF_66;
] "vpsignw"     = [ b"y*y*w*",   [   2, 0x09      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // Legacy forms of the shift instructions are in the MMX section
  "vpslld"      = [ b"y*y*wo",   [   1, 0xF2      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"y*y*ib",   [   1, 0x72      ], 6, PREF_66 | AUTO_VEXL | VEX_OP | ENC_VM;
] "pslldq"      = [ b"yoib",     [0x0F, 0x73      ], 7, PREF_66;
] "vpslldq"     = [ b"y*y*ib",   [   1, 0x73      ], 7, PREF_66 | AUTO_VEXL | VEX_OP | ENC_VM;
] "vpsllq"      = [ b"y*y*wo",   [   1, 0xF3      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"y*y*ib",   [   1, 0x73      ], 6, PREF_66 | AUTO_VEXL | VEX_OP | ENC_VM;
] "vpsllw"      = [ b"y*y*wo",   [   1, 0xF1      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"y*y*ib",   [   1, 0x71      ], 6, PREF_66 | AUTO_VEXL | VEX_OP | ENC_VM;
] "vpsrad"      = [ b"y*y*wo",   [   1, 0xE2      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"y*y*ib",   [   1, 0x72      ], 4, PREF_66 | AUTO_VEXL | VEX_OP | ENC_VM;
] "vpsraw"      = [ b"y*y*wo",   [   1, 0xE1      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"y*y*ib",   [   1, 0x71      ], 4, PREF_66 | AUTO_VEXL | VEX_OP | ENC_VM;
] "vpsrld"      = [ b"y*y*wo",   [   1, 0xD2      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"y*y*ib",   [   1, 0x72      ], 2, PREF_66 | AUTO_VEXL | VEX_OP;
] "psrldq"      = [ b"yoib",     [0x0F, 0x73      ], 3, PREF_66;
] "vpsrldq"     = [ b"y*y*ib",   [   1, 0x73      ], 3, PREF_66 | AUTO_VEXL | VEX_OP | ENC_VM;
] "vpsrlq"      = [ b"y*y*wo",   [   1, 0xD3      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"y*y*ib",   [   1, 0x73      ], 2, PREF_66 | AUTO_VEXL | VEX_OP | ENC_VM;
] "vpsrlw"      = [ b"y*y*wo",   [   1, 0xD1      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"y*y*ib",   [   1, 0x71      ], 2, PREF_66 | AUTO_VEXL | VEX_OP | ENC_VM;
] // legacy padd forms are in the MMX section
  "vpsubb"      = [ b"y*y*w*",   [   1, 0xF8      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpsubd"      = [ b"y*y*w*",   [   1, 0xFA      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpsubq"      = [ b"y*y*w*",   [   1, 0xFB      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpsubsb"     = [ b"y*y*w*",   [   1, 0xE8      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpsubsw"     = [ b"y*y*w*",   [   1, 0xE9      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpsubusb"    = [ b"y*y*w*",   [   1, 0xD8      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpsubusw"    = [ b"y*y*w*",   [   1, 0xD9      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpsubw"      = [ b"y*y*w*",   [   1, 0xF9      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "ptest"       = [ b"yowo",     [0x0F, 0x38, 0x17], X, PREF_66;
] "vptest"      = [ b"y*w*",     [   2, 0x17      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // legacy punpck forms too
  "vpunpckhbw"  = [ b"y*y*w*",   [   1, 0x68      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpunpckhdq"  = [ b"y*y*w*",   [   1, 0x6A      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "punpckhqdq"  = [ b"yowo",     [0x0F, 0x6D      ], X, PREF_66;
] "vpunpckhqdq" = [ b"y*y*w*",   [   1, 0x6D      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpunpckhwd"  = [ b"y*y*w*",   [   1, 0x69      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpunpcklbw"  = [ b"y*y*w*",   [   1, 0x60      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpunpckldq"  = [ b"y*y*w*",   [   1, 0x62      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "punpcklqdq"  = [ b"yowo",     [0x0F, 0x6C      ], X, PREF_66;
] "vpunpcklqdq" = [ b"y*y*w*",   [   1, 0x6C      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpunpcklwd"  = [ b"y*y*w*",   [   1, 0x61      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // pxor is in the MMX section too
  "vpxor"       = [ b"y*y*w*",   [   1, 0xEF      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "rcpps"       = [ b"yowo",     [0x0F, 0x53      ], X;
] "vrcpps"      = [ b"y*w*",     [   1, 0x53      ], X,           AUTO_VEXL | VEX_OP;
] "rcpss"       = [ b"yowo",     [0x0F, 0x53      ], X, PREF_F3;
] "vrcpss"      = [ b"y*y*w*",   [   1, 0x53      ], X, PREF_F3 | AUTO_VEXL | VEX_OP;
] "roundpd"     = [ b"yowoib",   [0x0F, 0x3A, 0x09], X, PREF_66;
] "vroundpd"    = [ b"y*w*ib",   [   3, 0x09      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "roundps"     = [ b"yowoib",   [0x0F, 0x3A, 0x08], X, PREF_66;
] "vroundps"    = [ b"y*w*ib",   [   3, 0x08      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "roundsd"     = [ b"yoyoib",   [0x0F, 0x3A, 0x0B], X, PREF_66;
                    b"yomqib",   [0x0F, 0x3A, 0x0B], X, PREF_66;
] "vroundsd"    = [ b"yoyoyoib", [   3, 0x0B      ], X, PREF_66             | VEX_OP;
                    b"yoyomqib", [   3, 0x0B      ], X, PREF_66             | VEX_OP;
] "roundss"     = [ b"yoyoib",   [0x0F, 0x3A, 0x0A], X, PREF_66;
                    b"yomqib",   [0x0F, 0x3A, 0x0A], X, PREF_66;
] "vroundss"    = [ b"yoyoyoib", [   3, 0x0A      ], X, PREF_66             | VEX_OP;
                    b"yoyomqib", [   3, 0x0A      ], X, PREF_66             | VEX_OP;
] "rsqrtps"     = [ b"yowo",     [0x0F, 0x52      ], X;
] "vrsqrtps"    = [ b"y*w*",     [   1, 0x52      ], X,           AUTO_VEXL | VEX_OP;
] "rsqrtss"     = [ b"yoyo",     [0x0F, 0x52      ], X, PREF_F3;
                    b"yomd",     [0x0F, 0x52      ], X, PREF_F3;
] "vrsqrtss"    = [ b"yoyoyo",   [   1, 0x52      ], X, PREF_F3             | VEX_OP;
                    b"yoyomd",   [   1, 0x52      ], X, PREF_F3             | VEX_OP;
] "shufpd"      = [ b"yowoib",   [0x0F, 0xC6      ], X, PREF_66;
] "vshufpd"     = [ b"y*y*w*ib", [   1, 0xC6      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "shufps"      = [ b"yowoib",   [0x0F, 0xC6      ], X;
] "vshufps"     = [ b"y*y*w*ib", [   1, 0xC6      ], X,           AUTO_VEXL | VEX_OP;
] "sqrtpd"      = [ b"yowo",     [0x0F, 0x51      ], X, PREF_66;
] "vsqrtpd"     = [ b"y*w*",     [   1, 0x51      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "sqrtps"      = [ b"yowo",     [0x0F, 0x51      ], X;
] "vsqrtps"     = [ b"y*w*",     [   1, 0x51      ], X,           AUTO_VEXL | VEX_OP;
] "sqrtsd"      = [ b"yoyo",     [0x0F, 0x51      ], X, PREF_F2;
                    b"yomq",     [0x0F, 0x51      ], X, PREF_F2;
] "vsqrtsd"     = [ b"yoyoyo",   [   1, 0x51      ], X, PREF_F2             | VEX_OP;
                    b"yoyomq",   [   1, 0x51      ], X, PREF_F2             | VEX_OP;
] "sqrtss"      = [ b"yoyo",     [0x0F, 0x51      ], X, PREF_F3;
                    b"yomd",     [0x0F, 0x51      ], X, PREF_F3;
] "vsqrtss"     = [ b"yoyoyo",   [   1, 0x51      ], X, PREF_F3             | VEX_OP;
                    b"yoyomd",   [   1, 0x51      ], X, PREF_F3             | VEX_OP;
] "stmxcsr"     = [ b"md",       [0x0F, 0xAE      ], 3;
] "vstmxcsr"    = [ b"md",       [   1, 0xAE      ], 3;
] "subpd"       = [ b"yowo",     [0x0F, 0x5C      ], X, PREF_66;
] "vsubpd"      = [ b"y*y*w*",   [   1, 0x5C      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "subps"       = [ b"yowo",     [0x0F, 0x5C      ], X;
] "vsubps"      = [ b"y*y*w*",   [   1, 0x5C      ], X,           AUTO_VEXL | VEX_OP;
] "subsd"       = [ b"yoyo",     [0x0F, 0x5C      ], X, PREF_F2;
                    b"yomq",     [0x0F, 0x5C      ], X, PREF_F2;
] "vsubsd"      = [ b"yoyoyo",   [   1, 0x5C      ], X, PREF_F2             | VEX_OP;
                    b"yoyomq",   [   1, 0x5C      ], X, PREF_F2             | VEX_OP;
] "subss"       = [ b"yoyo",     [0x0F, 0x5C      ], X, PREF_F3;
                    b"yomd",     [0x0F, 0x5C      ], X, PREF_F3;
] "vsubss"      = [ b"yoyoyo",   [   1, 0x5C      ], X, PREF_F3             | VEX_OP;
                    b"yoyomd",   [   1, 0x5C      ], X, PREF_F3             | VEX_OP;
] "ucomisd"     = [ b"yoyo",     [0x0F, 0x2E      ], X, PREF_66;
                    b"yomq",     [0x0F, 0x2E      ], X, PREF_66;
] "vucomisd"    = [ b"yoyoyo",   [   1, 0x2E      ], X, PREF_66             | VEX_OP;
                    b"yoyomq",   [   1, 0x2E      ], X, PREF_66             | VEX_OP;
] "ucomiss"     = [ b"yoyo",     [0x0F, 0x2E      ], X;
                    b"yomd",     [0x0F, 0x2E      ], X;
] "vucomiss"    = [ b"yoyoyo",   [   1, 0x2E      ], X,                       VEX_OP;
                    b"yoyomd",   [   1, 0x2E      ], X,                       VEX_OP;
] "unpckhpd"    = [ b"yowo",     [0x0F, 0x15      ], X, PREF_66;
] "vunpckhpd"   = [ b"y*y*w*",   [   1, 0x15      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "unpckhps"    = [ b"yowo",     [0x0F, 0x15      ], X;
] "vunpckhps"   = [ b"y*y*w*",   [   1, 0x15      ], X,           AUTO_VEXL | VEX_OP;
] "unpcklpd"    = [ b"yowo",     [0x0F, 0x14      ], X, PREF_66;
] "vunpcklpd"   = [ b"y*y*w*",   [   1, 0x14      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "unpcklps"    = [ b"yowo",     [0x0F, 0x14      ], X;
] "vunpcklps"   = [ b"y*y*w*",   [   1, 0x14      ], X,           AUTO_VEXL | VEX_OP;
] // vex only operand forms
  "vbroadcastf128"
                = [ b"yhmo",     [   2, 0x1A      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "vbroadcasti128"
                = [ b"yhmo",     [   2, 0x5A      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "vbroadcastsd"= [ b"yhyo",     [   2, 0x19      ], X, PREF_66 | WITH_VEXL | VEX_OP;
                    b"yhmq",     [   2, 0x19      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "vbroadcastss"= [ b"y*yo",     [   2, 0x18      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"y*md",     [   2, 0x18      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vcvtph2ps"   = [ b"y*yo",     [   2, 0x13      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"yomq",     [   2, 0x13      ], X, PREF_66             | VEX_OP;
                    b"yhmo",     [   2, 0x13      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "vcvtps2ph"   = [ b"yoy*ib",   [   3, 0x1D      ], X, PREF_66 | AUTO_VEXL | VEX_OP | ENC_MR;
                    b"mqyoib",   [   3, 0x1D      ], X, PREF_66             | VEX_OP;
                    b"moyhib",   [   3, 0x1D      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "vextractf128"= [ b"woyhib",   [   3, 0x19      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "vextracti128"= [ b"woyhib",   [   3, 0x39      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "vfmaddpd"    = [ b"y*y*w*y*", [   3, 0x69      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"y*y*y*w*", [   3, 0x69      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmadd132pd" = [ b"y*y*w*",   [   2, 0x98      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmadd213pd" = [ b"y*y*w*",   [   2, 0xA8      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmadd231pd" = [ b"y*y*w*",   [   2, 0xB8      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmaddps"    = [ b"y*y*w*y*", [   3, 0x68      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"y*y*y*w*", [   3, 0x68      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmadd132ps" = [ b"y*y*w*",   [   2, 0x98      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfmadd213ps" = [ b"y*y*w*",   [   2, 0xA8      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfmadd231ps" = [ b"y*y*w*",   [   2, 0xB8      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfmaddsd"    = [ b"yoyoyoyo", [   3, 0x6B      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    b"yoyoyomq", [   3, 0x6B      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    b"yoyomqyo", [   3, 0x6B      ], X, PREF_66             | VEX_OP;
] "vfmadd132sd" = [ b"yoyoyo",   [   2, 0x99      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    b"yoyomq",   [   2, 0x99      ], X, PREF_66             | VEX_OP | WITH_REXW;
] "vfmadd213sd" = [ b"yoyoyo",   [   2, 0xA9      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    b"yoyomq",   [   2, 0xA9      ], X, PREF_66             | VEX_OP | WITH_REXW;
] "vfmadd231sd" = [ b"yoyoyo",   [   2, 0xB9      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    b"yoyomq",   [   2, 0xB9      ], X, PREF_66             | VEX_OP | WITH_REXW;
] "vfmaddss"    = [ b"yoyoyoyo", [   3, 0x6A      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    b"yoyoyomq", [   3, 0x6A      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    b"yoyomqyo", [   3, 0x6A      ], X, PREF_66             | VEX_OP;
] "vfmadd132ss" = [ b"yoyoyo",   [   2, 0x99      ], X, PREF_66             | VEX_OP;
                    b"yoyomq",   [   2, 0x99      ], X, PREF_66             | VEX_OP;
] "vfmadd213ss" = [ b"yoyoyo",   [   2, 0xA9      ], X, PREF_66             | VEX_OP;
                    b"yoyomq",   [   2, 0xA9      ], X, PREF_66             | VEX_OP;
] "vfmadd231ss" = [ b"yoyoyo",   [   2, 0xB9      ], X, PREF_66             | VEX_OP;
                    b"yoyomq",   [   2, 0xB9      ], X, PREF_66             | VEX_OP;
] "vfmaddsuppd"   =[b"y*y*w*y*", [   3, 0x5D      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"y*y*y*w*", [   3, 0x5D      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmaddsub132pd"=[b"y*y*w*",   [   2, 0x96      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmaddsub213pd"=[b"y*y*w*",   [   2, 0xA6      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmaddsub231pd"=[b"y*y*w*",   [   2, 0xB6      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmaddsubps"   =[b"y*y*w*y*", [   3, 0x5C      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"y*y*y*w*", [   3, 0x5C      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmaddsub132ps"=[b"y*y*w*",   [   2, 0x96      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfmaddsub213ps"=[b"y*y*w*",   [   2, 0xA6      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfmaddsub231ps"=[b"y*y*w*",   [   2, 0xB6      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfmsubaddpd"   =[b"y*y*w*y*", [   3, 0x5F      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"y*y*y*w*", [   3, 0x5F      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmsubadd132pd"=[b"y*y*w*",   [   2, 0x97      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmsubadd213pd"=[b"y*y*w*",   [   2, 0xA7      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmsubadd231pd"=[b"y*y*w*",   [   2, 0xB7      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmsubaddps"   =[b"y*y*w*y*", [   3, 0x5E      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"y*y*y*w*", [   3, 0x5E      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmsubadd132ps"=[b"y*y*w*",   [   2, 0x97      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfmsubadd213ps"=[b"y*y*w*",   [   2, 0xA7      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfmsubadd231ps"=[b"y*y*w*",   [   2, 0xB7      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfmsubpd"    = [ b"y*y*w*y*", [   3, 0x6D      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"y*y*y*w*", [   3, 0x6D      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmsub132pd" = [ b"y*y*w*",   [   2, 0x9A      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmsub213pd" = [ b"y*y*w*",   [   2, 0xAA      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmsub231pd" = [ b"y*y*w*",   [   2, 0xBA      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmsubps"    = [ b"y*y*w*y*", [   3, 0x6C      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"y*y*y*w*", [   3, 0x6C      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmsub132ps" = [ b"y*y*w*",   [   2, 0x9A      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfmsub213ps" = [ b"y*y*w*",   [   2, 0xAA      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfmsub231ps" = [ b"y*y*w*",   [   2, 0xBA      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfmsubsd"    = [ b"yoyoyoyo", [   3, 0x6F      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    b"yoyoyomq", [   3, 0x6F      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    b"yoyomqyo", [   3, 0x6F      ], X, PREF_66             | VEX_OP;
] "vfmsub132sd" = [ b"yoyoyo",   [   2, 0x9B      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    b"yoyomq",   [   2, 0x9B      ], X, PREF_66             | VEX_OP | WITH_REXW;
] "vfmsub213sd" = [ b"yoyoyo",   [   2, 0xAB      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    b"yoyomq",   [   2, 0xAB      ], X, PREF_66             | VEX_OP | WITH_REXW;
] "vfmsub231sd" = [ b"yoyoyo",   [   2, 0xBB      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    b"yoyomq",   [   2, 0xBB      ], X, PREF_66             | VEX_OP | WITH_REXW;
] "vfmsubss"    = [ b"yoyoyoyo", [   3, 0x6E      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    b"yoyoyomq", [   3, 0x6E      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    b"yoyomqyo", [   3, 0x6E      ], X, PREF_66             | VEX_OP;
] "vfmsub132ss" = [ b"yoyoyo",   [   2, 0x9B      ], X, PREF_66             | VEX_OP;
                    b"yoyomq",   [   2, 0x9B      ], X, PREF_66             | VEX_OP;
] "vfmsub213ss" = [ b"yoyoyo",   [   2, 0xAB      ], X, PREF_66             | VEX_OP;
                    b"yoyomq",   [   2, 0xAB      ], X, PREF_66             | VEX_OP;
] "vfmsub231ss" = [ b"yoyoyo",   [   2, 0xBB      ], X, PREF_66             | VEX_OP;
                    b"yoyomq",   [   2, 0xBB      ], X, PREF_66             | VEX_OP;
] "vfnmaddpd"   = [ b"y*y*w*y*", [   3, 0x79      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"y*y*y*w*", [   3, 0x79      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfnmadd132pd"= [ b"y*y*w*",   [   2, 0x9C      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfnmadd213pd"= [ b"y*y*w*",   [   2, 0xAC      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfnmadd231pd"= [ b"y*y*w*",   [   2, 0xBC      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfnmaddps"   = [ b"y*y*w*y*", [   3, 0x78      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"y*y*y*w*", [   3, 0x78      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfnmadd132ps"= [ b"y*y*w*",   [   2, 0x9C      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfnmadd213ps"= [ b"y*y*w*",   [   2, 0xAC      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfnmadd231ps"= [ b"y*y*w*",   [   2, 0xBC      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfnmaddsd"   = [ b"yoyoyoyo", [   3, 0x7B      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    b"yoyoyomq", [   3, 0x7B      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    b"yoyomqyo", [   3, 0x7B      ], X, PREF_66             | VEX_OP;
] "vfnmadd132sd"= [ b"yoyoyo",   [   2, 0x9D      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    b"yoyomq",   [   2, 0x9D      ], X, PREF_66             | VEX_OP | WITH_REXW;
] "vfnmadd213sd"= [ b"yoyoyo",   [   2, 0xAD      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    b"yoyomq",   [   2, 0xAD      ], X, PREF_66             | VEX_OP | WITH_REXW;
] "vfnmadd231sd"= [ b"yoyoyo",   [   2, 0xBD      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    b"yoyomq",   [   2, 0xBD      ], X, PREF_66             | VEX_OP | WITH_REXW;
] "vfnmaddss"   = [ b"yoyoyoyo", [   3, 0x7A      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    b"yoyoyomq", [   3, 0x7A      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    b"yoyomqyo", [   3, 0x7A      ], X, PREF_66             | VEX_OP;
] "vfnmadd132ss"= [ b"yoyoyo",   [   2, 0x9D      ], X, PREF_66             | VEX_OP;
                    b"yoyomq",   [   2, 0x9D      ], X, PREF_66             | VEX_OP;
] "vfnmadd213ss"= [ b"yoyoyo",   [   2, 0xAD      ], X, PREF_66             | VEX_OP;
                    b"yoyomq",   [   2, 0xAD      ], X, PREF_66             | VEX_OP;
] "vfnmadd231ss"= [ b"yoyoyo",   [   2, 0xBD      ], X, PREF_66             | VEX_OP;
                    b"yoyomq",   [   2, 0xBD      ], X, PREF_66             | VEX_OP;
] "vfnmsubpd"   = [ b"y*y*w*y*", [   3, 0x7D      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"y*y*y*w*", [   3, 0x7D      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfnmsub132pd"= [ b"y*y*w*",   [   2, 0x9E      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfnmsub213pd"= [ b"y*y*w*",   [   2, 0xAE      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfnmsub231pd"= [ b"y*y*w*",   [   2, 0xBE      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfnmsubps"   = [ b"y*y*w*y*", [   3, 0x7C      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"y*y*y*w*", [   3, 0x7C      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfnmsub132ps"= [ b"y*y*w*",   [   2, 0x9E      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfnmsub213ps"= [ b"y*y*w*",   [   2, 0xAE      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfnmsub231ps"= [ b"y*y*w*",   [   2, 0xBE      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfnmsubsd"   = [ b"yoyoyoyo", [   3, 0x7F      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    b"yoyoyomq", [   3, 0x7F      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    b"yoyomqyo", [   3, 0x7F      ], X, PREF_66             | VEX_OP;
] "vfnmsub132sd"= [ b"yoyoyo",   [   2, 0x9F      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    b"yoyomq",   [   2, 0x9F      ], X, PREF_66             | VEX_OP | WITH_REXW;
] "vfnmsub213sd"= [ b"yoyoyo",   [   2, 0xAF      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    b"yoyomq",   [   2, 0xAF      ], X, PREF_66             | VEX_OP | WITH_REXW;
] "vfnmsub231sd"= [ b"yoyoyo",   [   2, 0xBF      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    b"yoyomq",   [   2, 0xBF      ], X, PREF_66             | VEX_OP | WITH_REXW;
] "vfnmsubss"   = [ b"yoyoyoyo", [   3, 0x7E      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    b"yoyoyomq", [   3, 0x7E      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    b"yoyomqyo", [   3, 0x7E      ], X, PREF_66             | VEX_OP;
] "vfnmsub132ss"= [ b"yoyoyo",   [   2, 0x9F      ], X, PREF_66             | VEX_OP;
                    b"yoyomq",   [   2, 0x9F      ], X, PREF_66             | VEX_OP;
] "vfnmsub213ss"= [ b"yoyoyo",   [   2, 0xAF      ], X, PREF_66             | VEX_OP;
                    b"yoyomq",   [   2, 0xAF      ], X, PREF_66             | VEX_OP;
] "vfnmsub231ss"= [ b"yoyoyo",   [   2, 0xBF      ], X, PREF_66             | VEX_OP;
                    b"yoyomq",   [   2, 0xBF      ], X, PREF_66             | VEX_OP;
] "vfrczpd"     = [ b"y*w*",     [   9, 0x81      ], X,           AUTO_VEXL | XOP_OP;
] "vfrczps"     = [ b"y*w*",     [   9, 0x80      ], X,           AUTO_VEXL | XOP_OP;
] "vfrczsd"     = [ b"yoyo",     [   9, 0x83      ], X,                       XOP_OP;
                    b"yomq",     [   9, 0x83      ], X,                       XOP_OP;
] "vfrczss"     = [ b"yoyo",     [   9, 0x82      ], X,                       XOP_OP;
                    b"yomd",     [   9, 0x82      ], X,                       XOP_OP;
] "vgatherdpd"  = [ b"y*koy*",   [   2, 0x92      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vgatherdps"  = [ b"y*k*y*",   [   2, 0x92      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vgatherqpd"  = [ b"y*l*y*",   [   2, 0x93      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vgatherqps"  = [ b"yol*yo",   [   2, 0x93      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vinsertf128" = [ b"yhyhwoib", [   3, 0x18      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "vinserti128" = [ b"yhyhwoib", [   3, 0x38      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "vmaskmovpd"  = [ b"y*y*m*",   [   2, 0x2D      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"m*y*y*",   [   2, 0x2F      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vmaskmovps"  = [ b"y*y*m*",   [   2, 0x2C      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"m*y*y*",   [   2, 0x2E      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpblendd"    = [ b"y*y*w*ib", [   3, 0x02      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpbroadcastb"= [ b"y*yo",     [   2, 0x78      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"y*mb",     [   2, 0x78      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpbroadcastd"= [ b"y*yo",     [   2, 0x58      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"y*md",     [   2, 0x58      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpbroadcastq"= [ b"y*yo",     [   2, 0x59      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"y*mq",     [   2, 0x59      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpbroadcastw"= [ b"y*yo",     [   2, 0x79      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"y*mw",     [   2, 0x79      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpcmov"      = [ b"y*y*w*y*", [   8, 0xA2      ], X,           AUTO_VEXL | XOP_OP;
                    b"y*y*y*w*", [   8, 0xA2      ], X,           AUTO_VEXL | XOP_OP | WITH_REXW;
] "vpcomb"      = [ b"yoyowoib", [   8, 0xCC      ], X,                       XOP_OP;
] "vpcomd"      = [ b"yoyowoib", [   8, 0xCE      ], X,                       XOP_OP;
] "vpcomq"      = [ b"yoyowoib", [   8, 0xCF      ], X,                       XOP_OP;
] "vpcomub"     = [ b"yoyowoib", [   8, 0xEC      ], X,                       XOP_OP;
] "vpcomud"     = [ b"yoyowoib", [   8, 0xEE      ], X,                       XOP_OP;
] "vpcomuq"     = [ b"yoyowoib", [   8, 0xEF      ], X,                       XOP_OP;
] "vpcomuw"     = [ b"yoyowoib", [   8, 0xED      ], X,                       XOP_OP;
] "vpcomw"      = [ b"yoyowoib", [   8, 0xCD      ], X,                       XOP_OP;
] "vperm2f128"  = [ b"yhyhwhib", [   3, 0x06      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "vperm2i128"  = [ b"yhyhwhib", [   3, 0x46      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "vpermd"      = [ b"yhyhwh",   [   3, 0x36      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "vpermil2pd"  = [ b"y*y*w*y*ib",[  3, 0x49      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"y*y*y*w*ib",[  3, 0x49      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vpermil2pS"  = [ b"y*y*w*y*ib",[  3, 0x48      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"y*y*y*w*ib",[  3, 0x48      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vpermilpd"   = [ b"y*y*w*",   [   2, 0x0D      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"y*w*ib",   [   3, 0x05      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpermilps"   = [ b"y*y*w*",   [   2, 0x0C      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    b"y*w*ib",   [   3, 0x04      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpermpd"     = [ b"yhwhib",   [   3, 0x01      ], X, PREF_66 | WITH_VEXL | VEX_OP | WITH_REXW;
] "vpermps"     = [ b"yhyhwh",   [   2, 0x01      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "vpermq"      = [ b"yhwhib",   [   3, 0x00      ], X, PREF_66 | WITH_VEXL | VEX_OP | WITH_REXW;
] "vpgatherdd"  = [ b"y*k*y*",   [   2, 0x90      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpgatherdq"  = [ b"y*koy*",   [   2, 0x90      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vpgatherqd"  = [ b"yok*yo",   [   2, 0x91      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpgatherqq"  = [ b"y*k*y*",   [   2, 0x91      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vphaddbd"    = [ b"yowo",     [   9, 0xC2      ], X,                       XOP_OP;
] "vphaddbq"    = [ b"yowo",     [   9, 0xC3      ], X,                       XOP_OP;
] "vphaddbw"    = [ b"yowo",     [   9, 0xC1      ], X,                       XOP_OP;
] "vphadddq"    = [ b"yowo",     [   9, 0xCB      ], X,                       XOP_OP;
] "vphaddubd"   = [ b"yowo",     [   9, 0xD2      ], X,                       XOP_OP;
] "vphaddubq"   = [ b"yowo",     [   9, 0xD3      ], X,                       XOP_OP;
] "vphaddubw"   = [ b"yowo",     [   9, 0xD1      ], X,                       XOP_OP;
] "vphaddudq"   = [ b"yowo",     [   9, 0xDB      ], X,                       XOP_OP;
] "vphadduwd"   = [ b"yowo",     [   9, 0xD6      ], X,                       XOP_OP;
] "vphadduwq"   = [ b"yowo",     [   9, 0xD7      ], X,                       XOP_OP;
] "vphaddwd"    = [ b"yowo",     [   9, 0xC6      ], X,                       XOP_OP;
] "vphaddwq"    = [ b"yowo",     [   9, 0xC7      ], X,                       XOP_OP;
] "vphsubbw"    = [ b"yowo",     [   9, 0xE1      ], X,                       XOP_OP;
] "vphsubdq"    = [ b"yowo",     [   9, 0xE3      ], X,                       XOP_OP;
] "vphsubwd"    = [ b"yowo",     [   9, 0xE2      ], X,                       XOP_OP;
] "vpmacsdd"    = [ b"yoyowoyo", [   8, 0x9E      ], X,                       XOP_OP;
] "vpmacsdqh"   = [ b"yoyowoyo", [   8, 0x9F      ], X,                       XOP_OP;
] "vpmacsdql"   = [ b"yoyowoyo", [   8, 0x97      ], X,                       XOP_OP;
] "vpmacssdd"   = [ b"yoyowoyo", [   8, 0x8E      ], X,                       XOP_OP;
] "vpmacssdqh"  = [ b"yoyowoyo", [   8, 0x8F      ], X,                       XOP_OP;
] "vpmacssdql"  = [ b"yoyowoyo", [   8, 0x87      ], X,                       XOP_OP;
] "vpmacsswd"   = [ b"yoyowoyo", [   8, 0x86      ], X,                       XOP_OP;
] "vpmacssww"   = [ b"yoyowoyo", [   8, 0x85      ], X,                       XOP_OP;
] "vpmacswd"    = [ b"yoyowoyo", [   8, 0x96      ], X,                       XOP_OP;
] "vpmacsww"    = [ b"yoyowoyo", [   8, 0x95      ], X,                       XOP_OP;
] "vpmadcsswd"  = [ b"yoyowoyo", [   8, 0xA6      ], X,                       XOP_OP;
] "vpmadcswd"   = [ b"yoyowoyo", [   8, 0xB6      ], X,                       XOP_OP;
] "vpmaskmovd"  = [ b"y*y*m*",   [   2, 0x8C      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
                    b"m*y*y*",   [   2, 0x8E      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vpmaskmovq"  = [ b"y*y*m*",   [   2, 0x8C      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
                    b"m*y*y*",   [   2, 0x8E      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vpperm"      = [ b"yoyowoyo", [   8, 0xA3      ], X,                       XOP_OP;
                    b"yoyoyowo", [   8, 0xA3      ], X,                       XOP_OP | WITH_REXW;
] "vprotb"      = [ b"yowoyo",   [   9, 0x90      ], X,                       XOP_OP;
                    b"yoyowo",   [   9, 0x90      ], X,                       XOP_OP | WITH_REXW;
                    b"yowoib",   [   8, 0xC0      ], X,                       XOP_OP;
] "vprotd"      = [ b"yowoyo",   [   9, 0x92      ], X,                       XOP_OP;
                    b"yoyowo",   [   9, 0x92      ], X,                       XOP_OP | WITH_REXW;
                    b"yowoib",   [   8, 0xC2      ], X,                       XOP_OP;
] "vprotq"      = [ b"yowoyo",   [   9, 0x93      ], X,                       XOP_OP;
                    b"yoyowo",   [   9, 0x93      ], X,                       XOP_OP | WITH_REXW;
                    b"yowoib",   [   8, 0xC3      ], X,                       XOP_OP;
] "vprotw"      = [ b"yowoyo",   [   9, 0x91      ], X,                       XOP_OP;
                    b"yoyowo",   [   9, 0x91      ], X,                       XOP_OP | WITH_REXW;
                    b"yowoib",   [   8, 0xC1      ], X,                       XOP_OP;
] "vpshab"      = [ b"yowoyo",   [   9, 0x98      ], X,                       XOP_OP;
                    b"yoyowo",   [   9, 0x98      ], X,                       XOP_OP | WITH_REXW;
] "vpshad"      = [ b"yowoyo",   [   9, 0x9A      ], X,                       XOP_OP;
                    b"yoyowo",   [   9, 0x9A      ], X,                       XOP_OP | WITH_REXW;
] "vpshaq"      = [ b"yowoyo",   [   9, 0x9B      ], X,                       XOP_OP;
                    b"yoyowo",   [   9, 0x9B      ], X,                       XOP_OP | WITH_REXW;
] "vpshaw"      = [ b"yowoyo",   [   9, 0x99      ], X,                       XOP_OP;
                    b"yoyowo",   [   9, 0x99      ], X,                       XOP_OP | WITH_REXW;
] "vpshlb"      = [ b"yowoyo",   [   9, 0x94      ], X,                       XOP_OP;
                    b"yoyowo",   [   9, 0x94      ], X,                       XOP_OP | WITH_REXW;
] "vpshld"      = [ b"yowoyo",   [   9, 0x96      ], X,                       XOP_OP;
                    b"yoyowo",   [   9, 0x96      ], X,                       XOP_OP | WITH_REXW;
] "vpshlq"      = [ b"yowoyo",   [   9, 0x97      ], X,                       XOP_OP;
                    b"yoyowo",   [   9, 0x97      ], X,                       XOP_OP | WITH_REXW;
] "vpshlw"      = [ b"yowoyo",   [   9, 0x95      ], X,                       XOP_OP;
                    b"yoyowo",   [   9, 0x95      ], X,                       XOP_OP | WITH_REXW;
] "vpsllvd"     = [ b"y*y*w*",   [   2, 0x47      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpsllvq"     = [ b"y*y*w*",   [   2, 0x47      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vpsravd"     = [ b"y*y*w*",   [   2, 0x46      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpsrlvd"     = [ b"y*y*w*",   [   2, 0x45      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpsrlvq"     = [ b"y*y*w*",   [   2, 0x45      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vtestpd"     = [ b"y*w*",     [   2, 0x0F      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vtestps"     = [ b"y*w*",     [   2, 0x0E      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vzeroall"    = [ b"",         [   1, 0x77      ], X,           WITH_VEXL | VEX_OP;
] "vzeroupper"  = [ b"",         [   1, 0x77      ], X,                       VEX_OP;
] "xgetbv"      = [ b"",         [0x0F, 0x01, 0xD0], X;
] // Do not ask me why there are separate mnemnonics for single and double precision float xors. This
  // is a bitwise operation, it doesn't care about the bitwidth. Why does this even operate on floats
  // to begin with.
  "xorpd"       = [ b"yowo",     [0x0F, 0x57      ], X, PREF_66;
] "vxorpd"      = [ b"y*y*w*",   [   1, 0x57      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "xorps"       = [ b"yowo",     [0x0F, 0x57      ], X;
] "vxorps"      = [ b"y*y*w*",   [   1, 0x57      ], X,           AUTO_VEXL | VEX_OP;
] "xrstor"      = [ b"m!",       [0x0F, 0xAE      ], 5;
] "xsave"       = [ b"m!",       [0x0F, 0xAE      ], 4;
] "xsaveopt"    = [ b"m!",       [0x0F, 0xAE      ], 6;
] "xsetbv"      = [ b"",         [0x0F, 0x01, 0xD1], X;
] // and we're done. well, until intel's new extensions get more use
);
