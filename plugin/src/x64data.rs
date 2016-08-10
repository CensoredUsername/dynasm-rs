use std::collections::HashMap;

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
            const AUTO_SIZE = 0x0000_0008, // 16 bit -> OPSIZE , 32-bit -> None   , 64-bit -> REX.W/VEX.W/XOP.W
            const AUTO_NO32 = 0x0000_0010, // 16 bit -> OPSIZE , 32-bit -> illegal, 64-bit -> None
            const AUTO_REXW = 0x0000_0020, // 16 bit -> illegal, 32-bit -> None   , 64-bit -> REX.W/VEX.W/XOP.W
            const AUTO_VEXL = 0x0000_0040, // 128bit -> None   , 256bit -> VEX.L
            const WORD_SIZE = 0x0000_0080, // implies opsize prefix
            const WITH_REXW = 0x0000_0100, // implies REX.W/VEX.W/XOP.W
            const WITH_VEXL = 0x0000_0200, // implies VEX.L/XOP.L

            const PREF_66   = WORD_SIZE.bits,// mandatory prefix (same as WORD_SIZE)
            const PREF_67   = 0x0000_0400, // mandatory prefix (same as SMALL_ADDRESS)
            const PREF_F0   = 0x0000_0800, // mandatory prefix (same as LOCK)
            const PREF_F2   = 0x0000_1000, // mandatory prefix (REPNE)
            const PREF_F3   = 0x0000_2000, // mandatory prefix (REP)

            const LOCK      = 0x0000_4000, // user lock prefix is valid with this instruction
            const REP       = 0x0000_8000, // user rep prefix is valid with this instruction

            const SHORT_ARG = 0x0000_0004, // a register argument is encoded in the last byte of the opcode
            const ENC_MR    = 0x0001_0000, //  select alternate arg encoding
            const ENC_VM    = 0x0002_0000, //  select alternate arg encoding
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
const ENC_MR   : u32 = flags::flag_bits(flags::ENC_MR);
const ENC_VM   : u32 = flags::flag_bits(flags::ENC_VM);

Ops!(OPMAP;
// general purpose instructions according to AMD's AMD64 Arch Programmer's Manual Vol. 3
  "adc"         = [ "A*i*",     [0x15            ], X, AUTO_SIZE;
                    "Abib",     [0x14            ], X;
                    "v*i*",     [0x81            ], 2, AUTO_SIZE | LOCK;
                    "v*ib",     [0x83            ], 2, AUTO_SIZE | LOCK;
                    "vbib",     [0x80            ], 2,             LOCK;
                    "v*r*",     [0x11            ], X, AUTO_SIZE | LOCK;
                    "vbrb",     [0x10            ], X,             LOCK;
                    "r*v*",     [0x13            ], X, AUTO_SIZE;
                    "rbvb",     [0x12            ], X;
] "add"         = [ "A*i*",     [0x05            ], X, AUTO_SIZE;
                    "Abib",     [0x04            ], X;
                    "v*i*",     [0x81            ], 0, AUTO_SIZE | LOCK;
                    "v*ib",     [0x83            ], 0, AUTO_SIZE | LOCK;
                    "vbib",     [0x80            ], 0,             LOCK;
                    "v*r*",     [0x01            ], X, AUTO_SIZE | LOCK;
                    "vbrb",     [0x00            ], X,             LOCK;
                    "r*v*",     [0x03            ], X, AUTO_SIZE;
                    "rbvb",     [0x02            ], X;
] "and"         = [ "A*i*",     [0x25            ], X, AUTO_SIZE;
                    "Abib",     [0x24            ], X;
                    "v*i*",     [0x81            ], 4, AUTO_SIZE | LOCK;
                    "v*ib",     [0x83            ], 4, AUTO_SIZE | LOCK;
                    "vbib",     [0x80            ], 4,             LOCK;
                    "v*r*",     [0x21            ], X, AUTO_SIZE | LOCK;
                    "vbrb",     [0x20            ], X,             LOCK;
                    "r*v*",     [0x23            ], X, AUTO_SIZE;
                    "rbvb",     [0x22            ], X;
] "andn"        = [ "r*r*v*",   [   2, 0xF2      ], X, AUTO_REXW | VEX_OP;
] "bextr"       = [ "r*v*r*",   [   2, 0xF7      ], X, AUTO_REXW | VEX_OP;
                    "r*v*id",   [  10, 0x10      ], X, AUTO_REXW | XOP_OP;
] "blcfill"     = [ "r*v*",     [   9, 0x01      ], 1, AUTO_REXW | XOP_OP | ENC_VM;
] "blci"        = [ "r*v*",     [   9, 0x02      ], 6, AUTO_REXW | XOP_OP | ENC_VM;
] "blcic"       = [ "r*v*",     [   9, 0x01      ], 5, AUTO_REXW | XOP_OP | ENC_VM;
] "blcmsk"      = [ "r*v*",     [   9, 0x02      ], 1, AUTO_REXW | XOP_OP | ENC_VM;
] "blcs"        = [ "r*v*",     [   9, 0x01      ], 3, AUTO_REXW | XOP_OP | ENC_VM;
] "blsfill"     = [ "r*v*",     [   9, 0x01      ], 2, AUTO_REXW | XOP_OP | ENC_VM;
] "blsi"        = [ "r*v*",     [   2, 0xF3      ], 3, AUTO_REXW | VEX_OP | ENC_VM;
] "blsic"       = [ "r*v*",     [   9, 0x01      ], 6, AUTO_REXW | XOP_OP | ENC_VM;
] "blsmsk"      = [ "r*v*",     [   2, 0xF3      ], 2, AUTO_REXW | VEX_OP | ENC_VM;
] "blsr"        = [ "r*v*",     [   2, 0xF3      ], 1, AUTO_REXW | VEX_OP | ENC_VM;
] "bsf"         = [ "r*v*",     [0x0F, 0xBC      ], X, AUTO_SIZE;
] "bsr"         = [ "r*v*",     [0x0F, 0xBD      ], X, AUTO_SIZE;
] "bswap"       = [ "r*",       [0x0F, 0xC8      ], 0, AUTO_REXW;
] "bt"          = [ "v*r*",     [0x0F, 0xA3      ], X, AUTO_SIZE;
                    "v*ib",     [0x0F, 0xBA      ], 4, AUTO_SIZE;
] "btc"         = [ "v*r*",     [0x0F, 0xBB      ], X, AUTO_SIZE | LOCK;
                    "v*ib",     [0x0F, 0xBA      ], 7, AUTO_SIZE | LOCK;
] "btr"         = [ "v*r*",     [0x0F, 0xB3      ], X, AUTO_SIZE | LOCK;
                    "v*ib",     [0x0F, 0xBA      ], 6, AUTO_SIZE | LOCK;
] "bts"         = [ "v*r*",     [0x0F, 0xAB      ], X, AUTO_SIZE | LOCK;
                    "v*ib",     [0x0F, 0xBA      ], 5, AUTO_SIZE | LOCK;
] "bzhi"        = [ "r*v*r*",   [   2, 0xF5      ], X, AUTO_REXW | VEX_OP;
] "call"        = [ "o*",       [0xE8            ], X, AUTO_SIZE;
                    "r*",       [0xFF            ], 2, AUTO_SIZE;
] "cbw"         = [ "",         [0x98            ], X, WORD_SIZE;
] "cwde"        = [ "",         [0x98            ], X;
] "cdqe"        = [ "",         [0x98            ], X, WITH_REXW;
] "cwd"         = [ "",         [0x99            ], X, WORD_SIZE;
] "cdq"         = [ "",         [0x99            ], X;
] "cqo"         = [ "",         [0x99            ], X, WITH_REXW;
] "clc"         = [ "",         [0xF8            ], X;
] "cld"         = [ "",         [0xFC            ], X;
] "clflush"     = [ "mb",       [0x0F, 0xAE      ], 7;
] "cmc"         = [ "",         [0xF5            ], X;
] "cmovo"       = [ "r*v*",     [0x0F, 0x40      ], X, AUTO_SIZE;
] "cmovno"      = [ "r*v*",     [0x0F, 0x41      ], X, AUTO_SIZE;
] "cmovb"       |
  "cmovc"       |
  "cmovnae"     = [ "r*v*",     [0x0F, 0x42      ], X, AUTO_SIZE;
] "cmovnb"      |
  "cmovnc"      |
  "cmovae"      = [ "r*v*",     [0x0F, 0x43      ], X, AUTO_SIZE;
] "cmovz"       |
  "cmove"       = [ "r*v*",     [0x0F, 0x44      ], X, AUTO_SIZE;
] "cmovnz"      |
  "cmovne"      = [ "r*v*",     [0x0F, 0x45      ], X, AUTO_SIZE;
] "cmovbe"      |
  "cmovna"      = [ "r*v*",     [0x0F, 0x46      ], X, AUTO_SIZE;
] "cmovnbe"     |
  "cmova"       = [ "r*v*",     [0x0F, 0x47      ], X, AUTO_SIZE;
] "cmovs"       = [ "r*v*",     [0x0F, 0x48      ], X, AUTO_SIZE;
] "cmovns"      = [ "r*v*",     [0x0F, 0x49      ], X, AUTO_SIZE;
] "cmovp"       |
  "cmovpe"      = [ "r*v*",     [0x0F, 0x4A      ], X, AUTO_SIZE;
] "cmovnp"      |
  "cmovpo"      = [ "r*v*",     [0x0F, 0x4B      ], X, AUTO_SIZE;
] "cmovl"       |
  "cmovnge"     = [ "r*v*",     [0x0F, 0x4C      ], X, AUTO_SIZE;
] "cmovnl"      |
  "cmovge"      = [ "r*v*",     [0x0F, 0x4D      ], X, AUTO_SIZE;
] "cmovle"      |
  "cmovng"      = [ "r*v*",     [0x0F, 0x4E      ], X, AUTO_SIZE;
] "cmovnle"     |
  "cmovg"       = [ "r*v*",     [0x0F, 0x4F      ], X, AUTO_SIZE;
] "cmp"         = [ "A*i*",     [0x3C            ], X, AUTO_SIZE;
                    "Abib",     [0x3D            ], X;
                    "v*i*",     [0x81            ], 7, AUTO_SIZE;
                    "v*ib",     [0x83            ], 7, AUTO_SIZE;
                    "vbib",     [0x80            ], 7;
                    "v*r*",     [0x39            ], X, AUTO_SIZE;
                    "vbrb",     [0x38            ], X;
                    "r*v*",     [0x3B            ], X, AUTO_SIZE;
                    "rbvb",     [0x3A            ], X;
] "cmpsb"       = [ "",         [0xA6            ], X,             REP;
] "cmpsw"       = [ "",         [0xA7            ], X, WORD_SIZE | REP;
] "cmpsd"       = [ "",         [0xA7            ], X,             REP;
                    "yowoib",   [0x0F, 0xC2      ], X, PREF_F2;
] "cmpsq"       = [ "",         [0xA7            ], X, WITH_REXW | REP;
] "cmpxchg"     = [ "v*r*",     [0x0F, 0xB1      ], X, AUTO_SIZE | LOCK;
                    "vbrb",     [0x0F, 0xB0      ], X,             LOCK;
] "cmpxchg8b"   = [ "mq",       [0x0F, 0xC7      ], 1,             LOCK;
] "cmpxchg16b"  = [ "mo",       [0x0F, 0xC7      ], 1, WITH_REXW | LOCK;
] "cpuid"       = [ "",         [0x0F, 0xA2      ], X;
] "crc32"       = [ "r*vb",     [0x0F, 0x38, 0xF0], X, AUTO_REXW | PREF_F2; // unique size encoding scheme
                    "rdvw",     [0x0F, 0x38, 0xF1], X, WORD_SIZE | PREF_F2; // also odd default
                    "r*v*",     [0x0F, 0x38, 0xF1], X, AUTO_REXW | PREF_F2;
] "dec"         = [ "v*",       [0xFF            ], 1, AUTO_SIZE | LOCK;
                    "vb",       [0xFE            ], 1,             LOCK;
] "div"         = [ "v*",       [0xF7            ], 6, AUTO_SIZE;
                    "vb",       [0xF6            ], 6;
] "enter"       = [ "iwib",     [0xC8            ], X;
] "idiv"        = [ "v*",       [0xF7            ], 7, AUTO_SIZE;
                    "vb",       [0xF6            ], 7;
] "imul"        = [ "v*",       [0xF7            ], 5, AUTO_SIZE;
                    "vb",       [0xF6            ], 5;
                    "r*v*",     [0x0F, 0xAF      ], X, AUTO_SIZE;
                    "r*v*i*",   [0x69            ], X, AUTO_SIZE;
                    "r*v*ib",   [0x68            ], X, AUTO_SIZE;
] "in"          = [ "Abib",     [0xE4            ], X;
                    "Awib",     [0xE5            ], X, WORD_SIZE;
                    "Adib",     [0xE5            ], X;
                    "AbCw",     [0xEC            ], X;
                    "AwCw",     [0xED            ], X, WORD_SIZE;
                    "AdCw",     [0xED            ], X;
] "inc"         = [ "v*",       [0xFF            ], 0, AUTO_SIZE | LOCK;
                    "vb",       [0xFE            ], 0,             LOCK;
] "insb"        = [ "",         [0x6C            ], X;
] "insw"        = [ "",         [0x6D            ], X, WORD_SIZE;
] "insd"        = [ "",         [0x6D            ], X;
] "int"         = [ "ib",       [0xCD            ], X;
] "jo"          = [ "o*",       [0x0F, 0x80      ], X, AUTO_SIZE;
                    "ob",       [0x70            ], X;
] "jno"         = [ "o*",       [0x0F, 0x81      ], X, AUTO_SIZE;
                    "ob",       [0x71            ], X;
] "jb"          |
  "jc"          |
  "jnae"        = [ "o*",       [0x0F, 0x82      ], X, AUTO_SIZE;
                    "ob",       [0x72            ], X;
] "jnb"         |
  "jnc"         |
  "jae"         = [ "o*",       [0x0F, 0x83      ], X, AUTO_SIZE;
                    "ob",       [0x73            ], X;
] "jz"          |
  "je"          = [ "o*",       [0x0F, 0x84      ], X, AUTO_SIZE;
                    "ob",       [0x74            ], X;
] "jnz"         |
  "jne"         = [ "o*",       [0x0F, 0x85      ], X, AUTO_SIZE;
                    "ob",       [0x75            ], X;
] "jbe"         |
  "jna"         = [ "o*",       [0x0F, 0x86      ], X, AUTO_SIZE;
                    "ob",       [0x76            ], X;
] "jnbe"        |
  "ja"          = [ "o*",       [0x0F, 0x87      ], X, AUTO_SIZE;
                    "ob",       [0x77            ], X;
] "js"          = [ "o*",       [0x0F, 0x88      ], X, AUTO_SIZE;
                    "ob",       [0x78            ], X;
] "jns"         = [ "o*",       [0x0F, 0x89      ], X, AUTO_SIZE;
                    "ob",       [0x79            ], X;
] "jp"          |
  "jpe"         = [ "o*",       [0x0F, 0x8A      ], X, AUTO_SIZE;
                    "ob",       [0x7A            ], X;
] "jnp"         |
  "jpo"         = [ "o*",       [0x0F, 0x8B      ], X, AUTO_SIZE;
                    "ob",       [0x7B            ], X;
] "jl"          |
  "jnge"        = [ "o*",       [0x0F, 0x8C      ], X, AUTO_SIZE;
                    "ob",       [0x7C            ], X;
] "jnl"         |
  "jge"         = [ "o*",       [0x0F, 0x8D      ], X, AUTO_SIZE;
                    "ob",       [0x7D            ], X;
] "jle"         |
  "jng"         = [ "o*",       [0x0F, 0x8E      ], X, AUTO_SIZE;
                    "ob",       [0x7E            ], X;
] "jnle"        |
  "jg"          = [ "o*",       [0x0F, 0x8F      ], X, AUTO_SIZE;
                    "ob",       [0x7F            ], X;
] "jecxz"       = [ "ob",       [0xE3            ], X, PREF_67;
] "jrcxz"       = [ "ob",       [0xE3            ], X;
] "jmp"         = [ "o*",       [0xE9            ], X, AUTO_SIZE;
                    "ob",       [0xEB            ], X;
                    "v*",       [0xFF            ], 4, AUTO_NO32 ;
] "lahf"        = [ "",         [0x9F            ], X;
] "lfs"         = [ "r*m!",     [0x0F, 0xB4      ], X, AUTO_SIZE;
] "lgs"         = [ "r*m!",     [0x0F, 0xB5      ], X, AUTO_SIZE;
] "lss"         = [ "r*m!",     [0x0F, 0xB2      ], X, AUTO_SIZE;
] "lea"         = [ "r*m!",     [0x8D            ], X, AUTO_SIZE;
] "leave"       = [ "",         [0xC9            ], X;
] "lfence"      = [ "",         [0x0F, 0xAE, 0xE8], X;
] "llwpcb"      = [ "r*",       [   9, 0x12      ], 0, AUTO_REXW | XOP_OP;
] "lodsb"       = [ "",         [0xAC            ], X;
] "lodsw"       = [ "",         [0xAD            ], X, WORD_SIZE;
] "lodsd"       = [ "",         [0xAD            ], X;
] "lodsq"       = [ "",         [0xAD            ], X, WITH_REXW;
] "loop"        = [ "ob",       [0xE2            ], X;
] "loope"       |
  "loopz"       = [ "ob",       [0xE1            ], X;
] "loopne"      |
  "loopnz"      = [ "ob",       [0xE0            ], X;
] "lwpins"      = [ "r*vdid",   [  10, 0x12      ], 0, AUTO_REXW | XOP_OP | ENC_VM;
] "lwpval"      = [ "r*vdid",   [  10, 0x12      ], 1, AUTO_REXW | XOP_OP | ENC_VM;
] "lzcnt"       = [ "r*v*",     [0x0F, 0xBD      ], X, AUTO_SIZE | PREF_F3;
] "mfence"      = [ "",         [0x0F, 0xAE, 0xF0], X;
] "mov"         = [ "v*r*",     [0x89            ], X, AUTO_SIZE;
                    "vbrb",     [0x88            ], X;
                    "r*v*",     [0x8B            ], X, AUTO_SIZE;
                    "rbvb",     [0x8A            ], X;
                    "r*sw",     [0x8C            ], X, AUTO_SIZE;
                    "mwsw",     [0x8C            ], X;
                    "swmw",     [0x8C            ], X;
                    "swrw",     [0x8C            ], X;
                    "rbib",     [0xB0            ], X,             SHORT_ARG;
                    "rwiw",     [0xB8            ], X, WORD_SIZE | SHORT_ARG;
                    "rdid",     [0xB8            ], X,             SHORT_ARG;
                    "v*i*",     [0xC7            ], 0, AUTO_SIZE;
                    "rqiq",     [0xB8            ], X, WITH_REXW | SHORT_ARG;
                    "vbib",     [0xC6            ], 0;
                    "cdrd",     [0x0F, 0x22      ], X; // can only match in 32 bit mode due to "cd"
                    "cqrq",     [0x0F, 0x22      ], X; // doesn't need a prefix to be encoded, as it's 64 bit natural in 64 bit mode
                    "rdcd",     [0x0F, 0x20      ], X;
                    "rqcq",     [0x0F, 0x20      ], X;
                    "Wdrd",     [0x0F, 0x22      ], 0, PREF_F0; // note: technically CR8 should actually be encoded, but the encoding is 0.
                    "Wqrq",     [0x0F, 0x22      ], 0, PREF_F0;
                    "rdWd",     [0x0F, 0x22      ], 0, PREF_F0;
                    "rqWq",     [0x0F, 0x22      ], 0, PREF_F0;
                    "ddrd",     [0x0F, 0x23      ], X; // 32 bit mode only
                    "dqrq",     [0x0F, 0x23      ], X;
                    "rddd",     [0x0F, 0x21      ], X;
                    "rqdq",     [0x0F, 0x21      ], X;
] "movabs"      = [ "Abib",     [0xA0            ], X; // special syntax for 64-bit disp only mov
                    "Awiw",     [0xA1            ], X, WORD_SIZE;
                    "Adid",     [0xA1            ], X;
                    "Aqiq",     [0xA1            ], X, WITH_REXW;
                    "ibAb",     [0xA2            ], X;
                    "iwAw",     [0xA3            ], X, WORD_SIZE;
                    "idAd",     [0xA3            ], X;
                    "iqAq",     [0xA3            ], X, WITH_REXW;
] "movbe"       = [ "r*m*",     [0x0F, 0x38, 0xF0], X, AUTO_SIZE;
                    "m*r*",     [0x0F, 0x38, 0xF1], X, AUTO_SIZE;
] "movd"        = [ "yov*",     [0x0F, 0x6E      ], X, AUTO_REXW | PREF_66;
                    "v*yo",     [0x0F, 0x7E      ], X, AUTO_REXW | PREF_66;
                    "xqv*",     [0x0F, 0x6E      ], X, AUTO_REXW;
                    "v*xq",     [0x0F, 0x7E      ], X, AUTO_REXW;
] "movmskpd"    = [ "r?yo",     [0x0F, 0x50      ], X, PREF_66;
] "movmskps"    = [ "r?yo",     [0x0F, 0x50      ], X;
] "movnti"      = [ "m*r*",     [0x0F, 0xC3      ], X, AUTO_REXW;
] "movsb"       = [ "",         [0xA4            ], X;
] "movsw"       = [ "",         [0xA5            ], X, WORD_SIZE;
] "movsd"       = [ "",         [0xA5            ], X;
                    "yoyo",     [0x0F, 0x10      ], X, PREF_F2;
                    "yomq",     [0x0F, 0x10      ], X, PREF_F2;
                    "mqyo",     [0x0F, 0x11      ], X, PREF_F2;
] "movsq"       = [ "",         [0xA5            ], X, WITH_REXW;
] "movsx"       = [ "r*vw",     [0x0F, 0xBF      ], X, AUTO_REXW; // currently this defaults to a certain memory size
                    "r*vb",     [0x0F, 0xBE      ], X, AUTO_SIZE;
] "movsxd"      = [ "rqvd",     [0x63            ], X, WITH_REXW;
] "movzx"       = [ "r*vw",     [0x0F, 0xB7      ], X, AUTO_REXW; // currently this defaults to a certain memory size
                    "r*vb",     [0x0F, 0xB6      ], X, AUTO_SIZE;
] "mul"         = [ "v*",       [0xF7            ], 4, AUTO_SIZE;
                    "vb",       [0xF6            ], 4;
] "mulx"        = [ "r*r*v*",   [   2, 0xF6      ], X, AUTO_REXW | VEX_OP | PREF_F2;
] "neg"         = [ "v*",       [0xF7            ], 3, AUTO_SIZE | LOCK;
                    "vb",       [0xF6            ], 3,             LOCK;
] "nop"         = [ "",         [0x90            ], X;
                    "v*",       [0x0F, 0x1F      ], 0, AUTO_SIZE;
] "not"         = [ "v*",       [0xF7            ], 2, AUTO_SIZE | LOCK;
                    "vb",       [0xF6            ], 2,             LOCK;
] "or"          = [ "A*i*",     [0x0D            ], X, AUTO_SIZE;
                    "Abib",     [0x0C            ], X;
                    "v*i*",     [0x81            ], 1, AUTO_SIZE | LOCK;
                    "v*ib",     [0x83            ], 1, AUTO_SIZE | LOCK;
                    "vbib",     [0x80            ], 1,             LOCK;
                    "v*r*",     [0x09            ], X, AUTO_SIZE | LOCK;
                    "vbrb",     [0x08            ], X,             LOCK;
                    "r*v*",     [0x0B            ], X, AUTO_SIZE;
                    "rbvb",     [0x0A            ], X;
] "out"         = [ "ibAb",     [0xE6            ], X;
                    "ibAw",     [0xE7            ], X;
                    "ibAd",     [0xE7            ], X;
                    "CwAb",     [0xEE            ], X;
                    "CwAw",     [0xEF            ], X, WORD_SIZE;
                    "CwAd",     [0xEF            ], X;
] "outsb"       = [ "",         [0x6E            ], X,             REP;
] "outsw"       = [ "",         [0x6F            ], X, WORD_SIZE | REP;
] "outsd"       = [ "",         [0x6F            ], X,             REP;
] "pause"       = [ "",         [0xF3, 0x90      ], X;
] "pdep"        = [ "r*r*v*",   [   2, 0xF5      ], X, AUTO_REXW | VEX_OP | PREF_F2;
] "pext"        = [ "r*r*v*",   [   2, 0xF5      ], X, AUTO_REXW | VEX_OP | PREF_F3;
] "pop"         = [ "r*",       [0x58            ], X, AUTO_NO32 | SHORT_ARG;
                    "v*",       [0x8F            ], 0, AUTO_NO32 ;
                    "Uw",       [0x0F, 0xA1      ], X;
                    "Vw",       [0x0F, 0xA9      ], X;
] "popcnt"      = [ "r*v*",     [0x0F, 0xB8      ], X, AUTO_SIZE | PREF_F3;
] "popf"        = [ "",         [0x9D            ], X, PREF_66;
] "popfq"       = [ "",         [0x9D            ], X;
] "prefetch"    = [ "mb",       [0x0F, 0x0D      ], 0;
] "prefetchw"   = [ "mb",       [0x0F, 0x0D      ], 1;
] "prefetchnta" = [ "mb",       [0x0F, 0x18      ], 0;
] "prefetcht0"  = [ "mb",       [0x0F, 0x18      ], 1;
] "prefetcht1"  = [ "mb",       [0x0F, 0x18      ], 2;
] "prefetcht2"  = [ "mb",       [0x0F, 0x18      ], 3;
] "push"        = [ "r*",       [0x50            ], X, AUTO_NO32 | SHORT_ARG;
                    "v*",       [0xFF            ], 6, AUTO_NO32 ;
                    "iq",       [0x68            ], X;
                    "iw",       [0x68            ], X, WORD_SIZE;
                    "ib",       [0x6A            ], X;
                    "Uw",       [0x0F, 0xA0      ], X;
                    "Vw",       [0x0F, 0xA8      ], X;
] "pushf"       = [ "",         [0x9C            ], X, PREF_66;
] "pushfq"      = [ "",         [0x9C            ], X;
] "rcl"         = [ "v*Bb",     [0xD3            ], 2, AUTO_SIZE; // shift by one forms not supported as immediates are only resolved at runtime
                    "vbBb",     [0xD2            ], 2;
                    "v*ib",     [0xC1            ], 2, AUTO_SIZE;
                    "vbib",     [0xC0            ], 2;
] "rcr"         = [ "v*Bb",     [0xD3            ], 3, AUTO_SIZE;
                    "vbBb",     [0xD2            ], 3;
                    "v*ib",     [0xC1            ], 3, AUTO_SIZE;
                    "vbib",     [0xC0            ], 3;
] "rdfsbase"    = [ "r*",       [0x0F, 0xAE      ], 0, AUTO_REXW | PREF_F3;
] "rdgsbase"    = [ "r*",       [0x0F, 0xAE      ], 1, AUTO_REXW | PREF_F3;
] "rdrand"      = [ "r*",       [0x0F, 0xC7      ], 6, AUTO_SIZE;
] "ret"         = [ "",         [0xC3            ], X;
                    "iw",       [0xC2            ], X;
] "rol"         = [ "v*Bb",     [0xD3            ], 0, AUTO_SIZE;
                    "vbBb",     [0xD2            ], 0;
                    "v*ib",     [0xC1            ], 0, AUTO_SIZE;
                    "vbib",     [0xC0            ], 0;
] "ror"         = [ "v*Bb",     [0xD3            ], 1, AUTO_SIZE;
                    "vbBb",     [0xD2            ], 1;
                    "v*ib",     [0xC1            ], 1, AUTO_SIZE;
                    "vbib",     [0xC0            ], 1;
] "rorx"        = [ "r*v*ib",   [   3, 0xF0      ], X, AUTO_REXW | VEX_OP | PREF_F2;
] "sahf"        = [ "",         [0x9E            ], X;
] "sal"         |
  "shl"         = [ "v*Bb",     [0xD3            ], 4, AUTO_SIZE;
                    "vbBb",     [0xD2            ], 4;
                    "v*ib",     [0xC1            ], 4, AUTO_SIZE;
                    "vbib",     [0xC0            ], 4;
] "sar"         = [ "v*Bb",     [0xD3            ], 7, AUTO_SIZE;
                    "vbBb",     [0xD2            ], 7;
                    "v*ib",     [0xC1            ], 7, AUTO_SIZE;
                    "vbib",     [0xC0            ], 7;
] "sarx"        = [ "r*v*r*",   [   2, 0xF7      ], X, AUTO_REXW | VEX_OP | PREF_F3;
] "sbb"         = [ "A*i*",     [0x1D            ], X, AUTO_SIZE;
                    "Abib",     [0x1C            ], X;
                    "v*i*",     [0x81            ], 3, AUTO_SIZE | LOCK;
                    "v*ib",     [0x83            ], 3, AUTO_SIZE | LOCK;
                    "vbib",     [0x80            ], 3,             LOCK;
                    "v*r*",     [0x19            ], X, AUTO_SIZE | LOCK;
                    "vbrb",     [0x18            ], X,             LOCK;
                    "r*v*",     [0x1B            ], X, AUTO_SIZE;
                    "rbvb",     [0x1A            ], X;
] "scasb"       = [ "",         [0xAE            ], X,             REP;
] "scasw"       = [ "",         [0xAF            ], X, WORD_SIZE | REP;
] "scasd"       = [ "",         [0xAF            ], X,             REP;
] "scasq"       = [ "",         [0xAF            ], X, WITH_REXW | REP;
] "seto"        = [ "vb",       [0x0F, 0x90      ], 0;
] "setno"       = [ "vb",       [0x0F, 0x91      ], 0;
] "setb"        |
  "setc"        |
  "setnae"      = [ "vb",       [0x0F, 0x92      ], 0;
] "setnb"       |
  "setnc"       |
  "setae"       = [ "vb",       [0x0F, 0x93      ], 0;
] "setz"        |
  "sete"        = [ "vb",       [0x0F, 0x94      ], 0;
] "setnz"       |
  "setne"       = [ "vb",       [0x0F, 0x95      ], 0;
] "setbe"       |
  "setna"       = [ "vb",       [0x0F, 0x96      ], 0;
] "setnbe"      |
  "seta"        = [ "vb",       [0x0F, 0x97      ], 0;
] "sets"        = [ "vb",       [0x0F, 0x98      ], 0;
] "setns"       = [ "vb",       [0x0F, 0x99      ], 0;
] "setp"        |
  "setpe"       = [ "vb",       [0x0F, 0x9A      ], 0;
] "setnp"       |
  "setpo"       = [ "vb",       [0x0F, 0x9B      ], 0;
] "setl"        |
  "setnge"      = [ "vb",       [0x0F, 0x9C      ], 0;
] "setnl"       |
  "setge"       = [ "vb",       [0x0F, 0x9D      ], 0;
] "setle"       |
  "setng"       = [ "vb",       [0x0F, 0x9E      ], 0;
] "setnle"      |
  "setg"        = [ "vb",       [0x0F, 0x9F      ], 0;
] "sfence"      = [ "",         [0x0F, 0xAE, 0xF8], X;
] "shld"        = [ "v*r*Bb",   [0x0F, 0xA5      ], X, AUTO_SIZE;
                    "v*r*ib",   [0x0F, 0xA4      ], X, AUTO_SIZE;
] "shlx"        = [ "r*v*r*",   [   2, 0xF7      ], X, AUTO_REXW | VEX_OP | PREF_66;
] "shr"         = [ "v*Bb",     [0xD3            ], 5, AUTO_SIZE;
                    "vbBb",     [0xD2            ], 5;
                    "v*ib",     [0xC1            ], 5, AUTO_SIZE;
                    "vbib",     [0xC0            ], 5;
] "shrd"        = [ "v*r*Bb",   [0x0F, 0xAD      ], X, AUTO_SIZE;
                    "v*r*ib",   [0x0F, 0xAC      ], X, AUTO_SIZE;
] "shrx"        = [ "r*v*r*",   [   2, 0xF7      ], X, AUTO_REXW | VEX_OP | PREF_F2;
] "slwpcb"      = [ "r*",       [   9, 0x12      ], 1, AUTO_REXW | XOP_OP;
] "stc"         = [ "",         [0xF9            ], X;
] "std"         = [ "",         [0xFD            ], X;
] "stosb"       = [ "",         [0xAA            ], X,             REP;
] "stosw"       = [ "",         [0xAB            ], X, WORD_SIZE | REP;
] "stosd"       = [ "",         [0xAB            ], X,             REP;
] "stosq"       = [ "",         [0xAB            ], X, WITH_REXW | REP;
] "sub"         = [ "A*i*",     [0x2D            ], X, AUTO_SIZE;
                    "Abib",     [0x2C            ], X;
                    "v*i*",     [0x81            ], 5, AUTO_SIZE | LOCK;
                    "v*ib",     [0x83            ], 5, AUTO_SIZE | LOCK;
                    "vbib",     [0x80            ], 5,             LOCK;
                    "v*r*",     [0x29            ], X, AUTO_SIZE | LOCK;
                    "vbrb",     [0x28            ], X,             LOCK;
                    "r*v*",     [0x2B            ], X, AUTO_SIZE;
                    "rbvb",     [0x2A            ], X;
] "t1mskc"      = [ "r*v*",     [   9, 0x01      ], 7, AUTO_REXW | XOP_OP | ENC_VM;
] "test"        = [ "A*i*",     [0xA9            ], X, AUTO_SIZE;
                    "Abib",     [0xA8            ], X;
                    "v*i*",     [0xF7            ], 0, AUTO_SIZE;
                    "vbib",     [0xF6            ], 0;
                    "v*r*",     [0x85            ], X, AUTO_SIZE;
                    "vbrb",     [0x84            ], X;
] "tzcnt"       = [ "r*v*",     [0x0F, 0xBC      ], X, AUTO_SIZE | PREF_F3;
] "tzmsk"       = [ "r*v*",     [   9, 0x01      ], 4, AUTO_REXW | XOP_OP  | ENC_VM;
] "wrfsbase"    = [ "r*",       [0x0F, 0xAE      ], 2, AUTO_REXW | PREF_F3;
] "wrgsbase"    = [ "r*",       [0x0F, 0xAE      ], 3, AUTO_REXW | PREF_F3;
] "xadd"        = [ "v*r*",     [0x0F, 0xC1      ], X, AUTO_SIZE | LOCK;
                    "vbrb",     [0x0F, 0xC0      ], X,             LOCK;
] "xchg"        = [ "A*r*",     [0x90            ], X, AUTO_SIZE | SHORT_ARG;
                    "r*A*",     [0x90            ], X, AUTO_SIZE | SHORT_ARG;
                    "v*r*",     [0x87            ], X, AUTO_SIZE | LOCK;
                    "r*v*",     [0x87            ], X, AUTO_SIZE | LOCK;
                    "vbrb",     [0x86            ], X,             LOCK;
                    "rbvb",     [0x86            ], X,             LOCK;
] "xlatb"       = [ "",         [0xD7            ], X;
] "xor"         = [ "A*i*",     [0x35            ], X, AUTO_SIZE;
                    "Abib",     [0x34            ], X;
                    "v*i*",     [0x81            ], 6, AUTO_SIZE | LOCK;
                    "v*ib",     [0x83            ], 6, AUTO_SIZE | LOCK;
                    "vbib",     [0x80            ], 6,             LOCK;
                    "v*r*",     [0x31            ], X, AUTO_SIZE | LOCK;
                    "vbrb",     [0x30            ], X,             LOCK;
                    "r*v*",     [0x33            ], X, AUTO_SIZE;
                    "rbvb",     [0x32            ], X;
]
// System instructions
  "clgi"        = [ "",         [0x0F, 0x01, 0xDD], X;
] "cli"         = [ "",         [0xFA            ], X;
] "clts"        = [ "",         [0x0F, 0x06      ], X;
] "hlt"         = [ "",         [0xF4            ], X;
] "int3"        = [ "",         [0xCC            ], X;
] "invd"        = [ "",         [0x0F, 0x08      ], X;
] "invlpg"      = [ "mb",       [0x0F, 0x01      ], 7;
] "invlpga"     = [ "AqBd",     [0x0F, 0x01, 0xDF], X;
] "iret"        = [ "",         [0xCF            ], X, WORD_SIZE;
] "iretd"       = [ "",         [0xCF            ], X;
] "iretq"       = [ "",         [0xCF            ], X, WITH_REXW;
] "lar"         = [ "r*vw",     [0x0F, 0x02      ], X, AUTO_SIZE;
] "lgdt"        = [ "m!",       [0x0F, 0x01      ], 2;
] "lidt"        = [ "m!",       [0x0F, 0x01      ], 3;
] "lldt"        = [ "vw",       [0x0F, 0x00      ], 2;
] "lmsw"        = [ "vw",       [0x0F, 0x01      ], 6;
] "lsl"         = [ "r*vw",     [0x0F, 0x03      ], X, AUTO_SIZE;
] "ltr"         = [ "vw",       [0x0F, 0x00      ], 3;
] "monitor"     = [ "",         [0x0F, 0x01, 0xC8], X;
] "monitorx"    = [ "",         [0x0F, 0x01, 0xFA], X;
] "mwait"       = [ "",         [0x0F, 0x01, 0xC9], X;
] "mwaitx"      = [ "",         [0x0F, 0x01, 0xFB], X;
] "rdmsr"       = [ "",         [0x0F, 0x32      ], X;
] "rdpmc"       = [ "",         [0x0F, 0x33      ], X;
] "rdtsc"       = [ "",         [0x0F, 0x31      ], X;
] "rdtscp"      = [ "",         [0x0F, 0x01, 0xF9], X;
] "rsm"         = [ "",         [0x0F, 0xAA      ], X;
] "sgdt"        = [ "m!",       [0x0F, 0x01      ], 0;
] "sidt"        = [ "m!",       [0x0F, 0x01      ], 1;
] "skinit"      = [ "Ad",       [0x0F, 0x01, 0xDE], X;
] "sldt"        = [ "r*",       [0x0F, 0x00      ], 0, AUTO_SIZE;
                    "mw",       [0x0F, 0x00      ], 0;
] "smsw"        = [ "r*",       [0x0F, 0x01      ], 4, AUTO_SIZE;
                    "mw",       [0x0F, 0x01      ], 4;
] "sti"         = [ "",         [0xFB            ], X;
] "stgi"        = [ "",         [0x0F, 0x01, 0xDC], X;
] "str"         = [ "r*",       [0x0F, 0x00      ], 1, AUTO_SIZE;
                    "mw",       [0x0F, 0x00      ], 1;
] "swapgs"      = [ "",         [0x0F, 0x01, 0xF8], X;
] "syscall"     = [ "",         [0x0F, 0x05      ], X;
] "sysenter"    = [ "",         [0x0F, 0x34      ], X;
] "sysexit"     = [ "",         [0x0F, 0x35      ], X;
] "sysret"      = [ "",         [0x0F, 0x07      ], X;
] "ud2"         = [ "",         [0x0F, 0x0B      ], X;
] "verr"        = [ "vw",       [0x0F, 0x00      ], 4;
] "verw"        = [ "vw",       [0x0F, 0x00      ], 5;
] "vmload"      = [ "Aq",       [0x0F, 0x01, 0xDA], X;
] "vmmcall"     = [ "",         [0x0F, 0x01, 0xD9], X;
] "vmrun"       = [ "Aq",       [0x0F, 0x01, 0xD8], X;
] "vmsave"      = [ "Aq",       [0x0F, 0x01, 0xDB], X;
] "wbinvd"      = [ "",         [0x0F, 0x09      ], X;
] "wrmsr"       = [ "",         [0x0F, 0x30      ], X;
]
// x87 FPU instruction set, d   ta taken from amd's programmer manual vol. 5
  "f2xm1"       = [ "",         [0xD9, 0xF0      ], X;
] "fabs"        = [ "",         [0xD9, 0xE1      ], X;
] "fadd"        = [ "Xpfp",     [0xD8, 0xC0      ], X, SHORT_ARG;
                    "fpXp",     [0xDC, 0xC0      ], X, SHORT_ARG;
                    "md",       [0xD8            ], 0;
                    "mq",       [0xDC            ], 0;
] "faddp"       = [ "",         [0xDE, 0xC1      ], X;
                    "fpXp",     [0xDE, 0xC0      ], X, SHORT_ARG;
] "fiadd"       = [ "mw",       [0xDE            ], 0;
                    "md",       [0xDA            ], 0;
] "fbld"        = [ "mp",       [0xDF            ], 4;
] "fbstp"       = [ "mp",       [0xDF            ], 6;
] "fchs"        = [ "",         [0xD9, 0xE0      ], X;
] "fclex"       = [ "",         [0x9B, 0xDB, 0xE2], X; // this is actually ;wait ;fnclex
] "fnclex"      = [ "",         [0xDB, 0xE2      ], X;
] "fcmovb"      = [ "Xpfp",     [0xDA, 0xC0      ], X, SHORT_ARG;
] "fcmovbe"     = [ "Xpfp",     [0xDA, 0xD0      ], X, SHORT_ARG;
] "fcmove"      = [ "Xpfp",     [0xDA, 0xC8      ], X, SHORT_ARG;
] "fcmovnb"     = [ "Xpfp",     [0xDB, 0xC0      ], X, SHORT_ARG;
] "fcmovnbe"    = [ "Xpfp",     [0xDB, 0xD0      ], X, SHORT_ARG;
] "fcmovne"     = [ "Xpfp",     [0xDB, 0xC8      ], X, SHORT_ARG;
] "fcmovnu"     = [ "Xpfp",     [0xDB, 0xD8      ], X, SHORT_ARG;
] "fcmovu"      = [ "Xpfp",     [0xDA, 0xD8      ], X, SHORT_ARG;
] "fcom"        = [ "",         [0xD8, 0xD1      ], X;
                    "fp",       [0xD8, 0xD0      ], X, SHORT_ARG;
                    "md",       [0xD8            ], 2;
                    "mq",       [0xDC            ], 2;
] "fcomp"       = [ "",         [0xD8, 0xD9      ], X;
                    "fp",       [0xD8, 0xD8      ], X, SHORT_ARG;
                    "md",       [0xD8            ], 3;
                    "mq",       [0xDC            ], 3;
] "fcompp"      = [ "",         [0xDE, 0xD9      ], X;
] "fcomi"       = [ "Xpfp",     [0xDB, 0xF0      ], X, SHORT_ARG;
] "fcomip"      = [ "fpXp",     [0xDF, 0xF0      ], X, SHORT_ARG;
] "fcos"        = [ "",         [0xD9, 0xFF      ], X;
] "fdecstp"     = [ "",         [0xD9, 0xF6      ], X;
] "fdiv"        = [ "Xpfp",     [0xD8, 0xF0      ], X, SHORT_ARG;
                    "fpXp",     [0xDC, 0xF8      ], X, SHORT_ARG;
                    "md",       [0xD8            ], 6;
                    "mq",       [0xDC            ], 6;
] "fdivp"       = [ "",         [0xDE, 0xF9      ], X;
                    "fpXp",     [0xDE, 0xF8      ], X, SHORT_ARG;
] "fidiv"       = [ "mw",       [0xDE            ], 6;
                    "md",       [0xDA            ], 6;
] "fdivr"       = [ "Xpfp",     [0xD8, 0xF8      ], X, SHORT_ARG;
                    "fpXp",     [0xDC, 0xF0      ], X, SHORT_ARG;
                    "md",       [0xD8            ], 7;
                    "mq",       [0xDC            ], 7;
] "fdivrp"      = [ "",         [0xDE, 0xF1      ], X;
                    "fpXp",     [0xDE, 0xF0      ], X, SHORT_ARG;
] "fidivr"      = [ "mw",       [0xDE            ], 7;
                    "md",       [0xDA            ], 7;
] "ffree"       = [ "fp",       [0xDD, 0xC0      ], X, SHORT_ARG;
] "ficom"       = [ "mw",       [0xDE            ], 2;
                    "md",       [0xDA            ], 2;
] "ficomp"      = [ "mw",       [0xDE            ], 3;
                    "md",       [0xDA            ], 3;
] "fild"        = [ "mw",       [0xDF            ], 0;
                    "md",       [0xDB            ], 0;
                    "mq",       [0xDF            ], 5;
] "fincstp"     = [ "",         [0xD9, 0xF7      ], X;
] "finit"       = [ "",         [0x9B, 0xDB, 0xE3], X; // this is actually ;wait ;fninit
] "fninit"      = [ "",         [0xDB, 0xE3      ], X;
] "fist"        = [ "mw",       [0xDF            ], 2;
                    "md",       [0xDB            ], 2;
                    "mw",       [0xDF            ], 3;
                    "md",       [0xDB            ], 3;
                    "mq",       [0xDF            ], 7;
] "fisttp"      = [ "mw",       [0xDF            ], 1;
                    "md",       [0xDB            ], 1;
                    "mq",       [0xDD            ], 1;
] "fld"         = [ "fp",       [0xD9, 0xC0      ], X, SHORT_ARG;
                    "md",       [0xD9            ], 0;
                    "mq",       [0xDD            ], 0;
                    "mp",       [0xDB            ], 5;
] "fld1"        = [ "",         [0xD9, 0xE8      ], X;
] "fldcw"       = [ "mw",       [0xD9            ], 5;
] "fldenv"      = [ "m!",       [0xD9            ], 4;
] "fldenvw"     = [ "m!",       [0xD9            ], 4, WORD_SIZE;
] "fldl2e"      = [ "",         [0xD9, 0xEA      ], X;
] "fldl2t"      = [ "",         [0xD9, 0xE9      ], X;
] "fldlg2"      = [ "",         [0xD9, 0xEC      ], X;
] "fldln2"      = [ "",         [0xD9, 0xED      ], X;
] "fldpi"       = [ "",         [0xD9, 0xEB      ], X;
] "fldz"        = [ "",         [0xD9, 0xEE      ], X;
] "fmul"        = [ "Xpfp",     [0xD8, 0xC8      ], X, SHORT_ARG;
                    "fpXp",     [0xDC, 0xC8      ], X, SHORT_ARG;
                    "md",       [0xD8            ], 1;
                    "mq",       [0xDC            ], 1;
] "fmulp"       = [ "",         [0xDE, 0xC9      ], X;
                    "fpXp",     [0xDE, 0xC8      ], X, SHORT_ARG;
] "fimul"       = [ "mw",       [0xDE            ], 1;
                    "md",       [0xDA            ], 1;
] "fnop"        = [ "",         [0xD9, 0xD0      ], X;
] "fpatan"      = [ "",         [0xD9, 0xF3      ], X;
] "fprem"       = [ "",         [0xD9, 0xF8      ], X;
] "fprem1"      = [ "",         [0xD9, 0xF5      ], X;
] "fptan"       = [ "",         [0xD9, 0xF2      ], X;
] "frndint"     = [ "",         [0xD9, 0xFC      ], X;
] "frstor"      = [ "m!",       [0xDD            ], 4;
] "frstorw"     = [ "m!",       [0xDD            ], 4, WORD_SIZE;
] "fsave"       = [ "m!",       [0x9B, 0xDD      ], 6; // note: this is actually ; wait; fnsavew
] "fsavew"      = [ "m!",       [0x9B, 0x66, 0xDD], 6; // note: this is actually ; wait; OPSIZE fnsave
] "fnsave"      = [ "m!",       [0xDD            ], 6;
] "fnsavew"     = [ "m!",       [0xDD            ], 6, WORD_SIZE;
] "fscale"      = [ "",         [0xD9, 0xFD      ], X;
] "fsin"        = [ "",         [0xD9, 0xFE      ], X;
] "fsincos"     = [ "",         [0xD9, 0xFB      ], X;
] "fsqrt"       = [ "",         [0xD9, 0xFA      ], X;
] "fst"         = [ "fp",       [0xDD, 0xD0      ], X, SHORT_ARG;
                    "md",       [0xD9            ], 2;
                    "mq",       [0xDD            ], 2;
] "fstp"        = [ "fp",       [0xDD, 0xD8      ], X, SHORT_ARG;
                    "md",       [0xD9            ], 3;
                    "mq",       [0xDD            ], 3;
                    "mp",       [0xDB            ], 7;
] "fstcw"       = [ "mw",       [0x9B, 0xD9      ], 7; // note: this is actually ; wait; fnstcw
] "fnstcw"      = [ "mw",       [0xD9            ], 7;
] "fstenv"      = [ "m!",       [0x9B, 0xD9      ], 6; // note: this is actually ; wait; fnstenv
] "fstenvw"     = [ "m!",       [0x9B, 0x66, 0xD9], 6; // note: this is actually ; wait; OPSIZE fnsten
] "fnstenv"     = [ "m!",       [0xD9            ], 6;
] "fnstenvw"    = [ "m!",       [0xD9            ], 6, WORD_SIZE;
] "fstsw"       = [ "Aw",       [0x9B, 0xDF, 0xE0], X; // note: this is actually ; wait; fnstsw
                    "mw",       [0x9B, 0xDD      ], 7;
] "fnstsw"      = [ "Aw",       [0xDF, 0xE0      ], X;
                    "mw",       [0xDD            ], 7;
] "fsub"        = [ "Xpfp",     [0xD8, 0xE0      ], X, SHORT_ARG;
                    "fpXp",     [0xDC, 0xE8      ], X, SHORT_ARG;
                    "md",       [0xD8            ], 4;
                    "mq",       [0xDC            ], 4;
] "fsubp"       = [ "",         [0xDE, 0xE9      ], X;
                    "fpXp",     [0xDE, 0xE8      ], X, SHORT_ARG;
] "fisub"       = [ "mw",       [0xDE            ], 4;
                    "md",       [0xDA            ], 4;
] "fsubr"       = [ "Xpfp",     [0xD8, 0xE8      ], X, SHORT_ARG;
                    "fpXp",     [0xDC, 0xE0      ], X, SHORT_ARG;
                    "md",       [0xD8            ], 5;
                    "mq",       [0xDC            ], 5;
] "fsubrp"      = [ "",         [0xDE, 0xE1      ], X;
                    "fpXp",     [0xDE, 0xE0      ], X, SHORT_ARG;
] "fisubr"      = [ "mw",       [0xDE            ], 5;
                    "md",       [0xDA            ], 5;
] "ftst"        = [ "",         [0xD9, 0xE4      ], X;
] "fucom"       = [ "",         [0xDD, 0xE1      ], X;
                    "fp",       [0xDD, 0xE0      ], X, SHORT_ARG;
] "fucomp"      = [ "",         [0xDD, 0xE9      ], X;
                    "fp",       [0xDD, 0xE8      ], X, SHORT_ARG;
] "fucompp"     = [ "",         [0xDA, 0xE9      ], X;
] "fucomi"      = [ "Xpfp",     [0xDB, 0xE8      ], X, SHORT_ARG;
                    "fpXp",     [0xDF, 0xE8      ], X, SHORT_ARG;
] "fwait"       |
  "wait"        = [ "",         [0x9B            ], X;
] "fxam"        = [ "",         [0xD9, 0xE5      ], X;
] "fxch"        = [ "",         [0xD9, 0xC9      ], X;
                    "fp",       [0xD9, 0xC8      ], X, SHORT_ARG;
] "fxrstor"     = [ "m!",       [0x0F, 0xAE      ], 1;
] "fxsave"      = [ "m!",       [0x0F, 0xAE      ], 0;
] "fxtract"     = [ "",         [0xD9, 0xF4      ], X;
] "fyl2x"       = [ "",         [0xD9, 0xF1      ], X;
] "fyl2xp1"     = [ "",         [0xD9, 0xF9      ], X;
]
// MMX instruction (also vol.   5) (note that 3DNow! instructions aren't supported)
        
  "cvtpd2pi"    = [ "xqwo",     [0x0F, 0x2D      ], X, PREF_66;
] "cvtpi2pd"    = [ "youq",     [0x0F, 0x2A      ], X, PREF_66;
] "cvtpi2ps"    = [ "youq",     [0x0F, 0x2A      ], X;
] "cvtps2pi"    = [ "xqwo",     [0x0F, 0x2D      ], X;
] "cvttpd2pi"   = [ "xqwo",     [0x0F, 0x2C      ], X, PREF_66;
] "cvttps2pi"   = [ "xqyo",     [0x0F, 0x2C      ], X;
                    "xqmq",     [0x0F, 0x2C      ], X;
] "emms"        = [ "",         [0x0F, 0x77      ], X;
] "maskmovq"    = [ "xqxq",     [0x0F, 0xF7      ], X;
] "movdq2q"     = [ "xqyo",     [0x0F, 0xD6      ], X, PREF_F2;
] "movntq"      = [ "mqxq",     [0x0F, 0xE7      ], X;
] "movq"        = [ "xquq",     [0x0F, 0x6F      ], X;
                    "uqxq",     [0x0F, 0x7F      ], X;
                    "yoyo",     [0x0F, 0x7E      ], X, PREF_F3;
                    "yomq",     [0x0F, 0x7E      ], X, PREF_F3;
                    "mqyo",     [0x0F, 0xD6      ], X, PREF_66;
] "movq2dq"     = [ "yoxq",     [0x0F, 0xD6      ], X, PREF_F3;
] "packssdw"    = [ "xquq",     [0x0F, 0x6B      ], X;
                    "yowo",     [0x0F, 0x6B      ], X, PREF_66;
] "packsswb"    = [ "xquq",     [0x0F, 0x63      ], X;
                    "yowo",     [0x0F, 0x63      ], X, PREF_66;
] "packuswb"    = [ "xquq",     [0x0F, 0x67      ], X;
                    "yowo",     [0x0F, 0x67      ], X, PREF_66;
] "paddb"       = [ "xquq",     [0x0F, 0xFC      ], X;
                    "yowo",     [0x0F, 0xFC      ], X, PREF_66;
] "paddd"       = [ "xquq",     [0x0F, 0xFE      ], X;
                    "yowo",     [0x0F, 0xFE      ], X, PREF_66;
] "paddq"       = [ "xquq",     [0x0F, 0xD4      ], X;
                    "yowo",     [0x0F, 0xD4      ], X, PREF_66;
] "paddsb"      = [ "xquq",     [0x0F, 0xEC      ], X;
                    "yowo",     [0x0F, 0xEC      ], X, PREF_66;
] "paddsw"      = [ "xquq",     [0x0F, 0xED      ], X;
                    "yowo",     [0x0F, 0xED      ], X, PREF_66;
] "paddusb"     = [ "xquq",     [0x0F, 0xDC      ], X;
                    "yowo",     [0x0F, 0xDC      ], X, PREF_66;
] "paddusw"     = [ "xquq",     [0x0F, 0xDD      ], X;
                    "yowo",     [0x0F, 0xDD      ], X, PREF_66;
] "paddw"       = [ "xquq",     [0x0F, 0xFD      ], X;
                    "yowo",     [0x0F, 0xFD      ], X, PREF_66;
] "pand"        = [ "xquq",     [0x0F, 0xDB      ], X;
                    "yowo",     [0x0F, 0xDB      ], X, PREF_66;
] "pandn"       = [ "xquq",     [0x0F, 0xDF      ], X;
                    "yowo",     [0x0F, 0xDF      ], X, PREF_66;
] "pavgb"       = [ "xquq",     [0x0F, 0xE0      ], X;
                    "yowo",     [0x0F, 0xE0      ], X, PREF_66;
] "pavgw"       = [ "xquq",     [0x0F, 0xE3      ], X;
                    "yowo",     [0x0F, 0xE3      ], X, PREF_66;
] "pcmpeqb"     = [ "xquq",     [0x0F, 0x74      ], X;
                    "yowo",     [0x0F, 0x74      ], X, PREF_66;
] "pcmpeqd"     = [ "xquq",     [0x0F, 0x76      ], X;
                    "yowo",     [0x0F, 0x76      ], X, PREF_66;
] "pcmpeqw"     = [ "xquq",     [0x0F, 0x75      ], X;
                    "yowo",     [0x0F, 0x75      ], X, PREF_66;
] "pcmpgtb"     = [ "xquq",     [0x0F, 0x64      ], X;
                    "yowo",     [0x0F, 0x64      ], X, PREF_66;
] "pcmpgtd"     = [ "xquq",     [0x0F, 0x66      ], X;
                    "yowo",     [0x0F, 0x66      ], X, PREF_66;
] "pcmpgtw"     = [ "xquq",     [0x0F, 0x65      ], X;
                    "yowo",     [0x0F, 0x65      ], X, PREF_66;
] "pextrw"      = [ "rdxqib",   [0x0F, 0xC5      ], X;
                    "r?yoib",   [0x0F, 0xC5      ], X, PREF_66;
                    "mwyoib",   [0x0F, 0x3A, 0x15], X, PREF_66;
] "pinsrw"      = [ "xqrdib",   [0x0F, 0xC4      ], X;
                    "xqmwib",   [0x0F, 0xC4      ], X;
                    "yordib",   [0x0F, 0xC4      ], X, PREF_66;
                    "yomwib",   [0x0F, 0xC4      ], X, PREF_66;
] "pmaddwd"     = [ "xquq",     [0x0F, 0xF5      ], X;
                    "yowo",     [0x0F, 0xF5      ], X, PREF_66;
] "pmaxsw"      = [ "xquq",     [0x0F, 0xEE      ], X;
                    "yowo",     [0x0F, 0xEE      ], X, PREF_66;
] "pmaxub"      = [ "xquq",     [0x0F, 0xDE      ], X;
                    "yowo",     [0x0F, 0xDE      ], X, PREF_66;
] "pminsw"      = [ "xquq",     [0x0F, 0xEA      ], X;
                    "yowo",     [0x0F, 0xEA      ], X, PREF_66;
] "pminub"      = [ "xquq",     [0x0F, 0xDA      ], X;
                    "yowo",     [0x0F, 0xDA      ], X, PREF_66;
] "pmovmskb"    = [ "rdxq",     [0x0F, 0xD7      ], X;
                    "rdyo",     [0x0F, 0xD7      ], X, PREF_66;
] "pmulhuw"     = [ "xquq",     [0x0F, 0xE4      ], X;
                    "yowo",     [0x0F, 0xE4      ], X, PREF_66;
] "pmulhw"      = [ "xquq",     [0x0F, 0xE5      ], X;
                    "yowo",     [0x0F, 0xE5      ], X, PREF_66;
] "pmullw"      = [ "xquq",     [0x0F, 0xD5      ], X;
                    "yowo",     [0x0F, 0xD5      ], X, PREF_66;
] "pmuludq"     = [ "xquq",     [0x0F, 0xF4      ], X;
                    "yowo",     [0x0F, 0xF4      ], X, PREF_66;
] "por"         = [ "xquq",     [0x0F, 0xEB      ], X;
                    "yowo",     [0x0F, 0xEB      ], X, PREF_66;
] "psadbw"      = [ "xquq",     [0x0F, 0xF6      ], X;
                    "yowo",     [0x0F, 0xF6      ], X, PREF_66;
] "pshufw"      = [ "xquqib",   [0x0F, 0x70      ], X;
                    "yowoib",   [0x0F, 0x70      ], X, PREF_F3;
] "pslld"       = [ "xquq",     [0x0F, 0xF2      ], X;
                    "xqib",     [0x0F, 0x72      ], 6;
                    "yowo",     [0x0F, 0xF2      ], X, PREF_66;
                    "yoib",     [0x0F, 0x72      ], 6, PREF_66;
] "psllq"       = [ "xquq",     [0x0F, 0xF3      ], X;
                    "xqib",     [0x0F, 0x73      ], 6;
                    "yowo",     [0x0F, 0xF3      ], X, PREF_66;
                    "yoib",     [0x0F, 0x73      ], 6, PREF_66;
] "psllw"       = [ "xquq",     [0x0F, 0xF1      ], X;
                    "xqib",     [0x0F, 0x71      ], 6;
                    "yowo",     [0x0F, 0xF1      ], X, PREF_66;
                    "yoib",     [0x0F, 0x71      ], 6, PREF_66;
] "psrad"       = [ "xquq",     [0x0F, 0xE2      ], X;
                    "xqib",     [0x0F, 0x72      ], 4;
                    "yowo",     [0x0F, 0xE2      ], X, PREF_66;
                    "yoib",     [0x0F, 0x72      ], 4, PREF_66;
] "psraw"       = [ "xquq",     [0x0F, 0xE1      ], X;
                    "xqib",     [0x0F, 0x71      ], 4;
                    "yowo",     [0x0F, 0xE1      ], X, PREF_66;
                    "yoib",     [0x0F, 0x71      ], 4, PREF_66;
] "psrld"       = [ "xquq",     [0x0F, 0xD2      ], X;
                    "xqib",     [0x0F, 0x72      ], 2;
                    "yowo",     [0x0F, 0xD2      ], X, PREF_66;
                    "yoib",     [0x0F, 0x72      ], 2, PREF_66;
] "psrlq"       = [ "xquq",     [0x0F, 0xD3      ], X;
                    "xqib",     [0x0F, 0x73      ], 2;
                    "yowo",     [0x0F, 0xD3      ], X, PREF_66;
                    "yoib",     [0x0F, 0x73      ], 2, PREF_66;
] "psrlw"       = [ "xquq",     [0x0F, 0xD1      ], X;
                    "xqib",     [0x0F, 0x71      ], 2;
                    "yowo",     [0x0F, 0xD1      ], X, PREF_66;
                    "yoib",     [0x0F, 0x71      ], 2, PREF_66;
] "psubb"       = [ "xquq",     [0x0F, 0xF8      ], X;
                    "yowo",     [0x0F, 0xF8      ], X, PREF_66;
] "psubd"       = [ "xquq",     [0x0F, 0xFA      ], X;
                    "yowo",     [0x0F, 0xFA      ], X, PREF_66;
] "psubq"       = [ "xquq",     [0x0F, 0xFB      ], X;
                    "yowo",     [0x0F, 0xFB      ], X, PREF_66;
] "psubsb"      = [ "xquq",     [0x0F, 0xE8      ], X;
                    "yowo",     [0x0F, 0xE8      ], X, PREF_66;
] "psubsw"      = [ "xquq",     [0x0F, 0xE9      ], X;
                    "yowo",     [0x0F, 0xE9      ], X, PREF_66;
] "psubusb"     = [ "xquq",     [0x0F, 0xD8      ], X;
                    "yowo",     [0x0F, 0xD8      ], X, PREF_66;
] "psubusw"     = [ "xquq",     [0x0F, 0xD9      ], X;
                    "yowo",     [0x0F, 0xD9      ], X, PREF_66;
] "psubw"       = [ "xquq",     [0x0F, 0xF9      ], X;
                    "yowo",     [0x0F, 0xF9      ], X, PREF_66;
] "punpckhbw"   = [ "xquq",     [0x0F, 0x68      ], X;
                    "yowo",     [0x0F, 0x68      ], X, PREF_66;
] "punpckhdq"   = [ "xquq",     [0x0F, 0x6A      ], X;
                    "yowo",     [0x0F, 0x6A      ], X, PREF_66;
] "punpckhwd"   = [ "xquq",     [0x0F, 0x69      ], X;
                    "yowo",     [0x0F, 0x69      ], X, PREF_66;
] "punpcklbw"   = [ "xquq",     [0x0F, 0x60      ], X;
                    "yowo",     [0x0F, 0x60      ], X, PREF_66;
] "punpckldq"   = [ "xquq",     [0x0F, 0x62      ], X;
                    "yowo",     [0x0F, 0x62      ], X, PREF_66;
] "punpcklwd"   = [ "xquq",     [0x0F, 0x61      ], X;
                    "yowo",     [0x0F, 0x61      ], X, PREF_66;
] "pxor"        = [ "xquq",     [0x0F, 0xEF      ], X;
                    "yowo",     [0x0F, 0xEF      ], X, PREF_66;
]
// SSE instructions (vol. 4)

  "addpd"       = [ "yowo",     [0x0F, 0x58      ], X, PREF_66;
] "vaddpd"      = [ "y*y*w*",   [   1, 0x58      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "addps"       = [ "yowo",     [0x0F, 0x58      ], X;
] "vaddps"      = [ "y*y*w*",   [   1, 0x58      ], X,           AUTO_VEXL | VEX_OP;
] "addsd"       = [ "yoyo",     [0x0F, 0x58      ], X, PREF_F2;
                    "yomq",     [0x0F, 0x58      ], X, PREF_F2;
] "vaddsd"      = [ "yoyoyo",   [   1, 0x58      ], X, PREF_F2             | VEX_OP;
                    "yoyomq",   [   1, 0x58      ], X, PREF_F2             | VEX_OP;
] "addss"       = [ "yoyo",     [0x0F, 0x58      ], X, PREF_F3;
                    "yomd",     [0x0F, 0x58      ], X, PREF_F3;
] "vaddss"      = [ "yoyoyo",   [   1, 0x58      ], X, PREF_F3             | VEX_OP;
                    "yoyomd",   [   1, 0x58      ], X, PREF_F3             | VEX_OP;
] "addsubpd"    = [ "yowo",     [0x0F, 0xD0      ], X, PREF_66;
] "vaddsubpd"   = [ "y*y*w*",   [   1, 0xD0      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "addsubps"    = [ "yowo",     [0x0F, 0xD0      ], X, PREF_F2;
] "vaddsubps"   = [ "y*y*w*",   [   1, 0xD0      ], X, PREF_F2 | AUTO_VEXL | VEX_OP;
] "aesdec"      = [ "yowo",     [0x0F, 0x38, 0xDE], X, PREF_66;
] "vaesdec"     = [ "yoyowo",   [   2, 0xDE      ], X, PREF_66             | VEX_OP;
] "aesdeclast"  = [ "yowo",     [0x0F, 0x38, 0xDF], X, PREF_66;
] "vaesdeclast" = [ "yoyowo",   [   2, 0xDF      ], X, PREF_66             | VEX_OP;
] "aesenc"      = [ "yowo",     [0x0F, 0x38, 0xDC], X, PREF_66;
] "vaesenc"     = [ "yoyowo",   [   2, 0xDC      ], X, PREF_66             | VEX_OP;
] "aesenclast"  = [ "yowo",     [0x0F, 0x38, 0xDD], X, PREF_66;
] "vaesenclast" = [ "yoyowo",   [   2, 0xDD      ], X, PREF_66             | VEX_OP;
] "aesimc"      = [ "yowo",     [0x0F, 0x38, 0xDB], X, PREF_66;
] "vaesimc"     = [ "yowo",     [   2, 0xDB      ], X, PREF_66             | VEX_OP;
] "aeskeygenassist"
                = [ "yowoib",   [0x0F, 0x3A, 0xDF], X, PREF_66;
] "vaeskeygenassist"
                = [ "yowoib",   [   3, 0xDF      ], X, PREF_66             | VEX_OP;
] "andnpd"      = [ "yowo",     [0x0F, 0x55      ], X, PREF_66;
] "vandnpd"     = [ "y*y*w*",   [   1, 0x55      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "andnps"      = [ "yowo",     [0x0F, 0x55      ], X;
] "vandnps"     = [ "y*y*w*",   [   1, 0x55      ], X,           AUTO_VEXL | VEX_OP;
] "andpd"       = [ "yowo",     [0x0F, 0x54      ], X, PREF_66;
] "vandpd"      = [ "y*y*w*",   [   1, 0x54      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "andps"       = [ "yowo",     [0x0F, 0x54      ], X;
] "vandps"      = [ "y*y*w*",   [   1, 0x54      ], X,           AUTO_VEXL | VEX_OP;
] "blendpd"     = [ "yowoib",   [0x0F, 0x3A, 0x0D], X, PREF_66;
] "vblendpd"    = [ "y*y*w*ib", [   3, 0x0D      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "blendps"     = [ "yowoib",   [0x0F, 0x3A, 0x0C], X, PREF_66;
] "vblendps"    = [ "y*y*w*ib", [   3, 0x0C      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "blendvpd"    = [ "yowo",     [0x0F, 0x38, 0x15], X, PREF_66;
] "vblendvpd"   = [ "y*y*w*y*", [   3, 0x4B      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "blendvps"    = [ "yowo",     [0x0F, 0x38, 0x14], X, PREF_66;
] "vblendvps"   = [ "y*y*w*y*", [   3, 0x4A      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "cmppd"       = [ "yowoib",   [0x0F, 0xC2      ], X, PREF_66;
] "vcmppd"      = [ "y*y*w*ib", [   1, 0xC2      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "cmpps"       = [ "yowoib",   [0x0F, 0xC2      ], X;
] "vcmpps"      = [ "y*y*w*ib", [   1, 0xC2      ], X,           AUTO_VEXL | VEX_OP;
] // cmpsd is found in generic instructions
  "vcmpsd"      = [ "y*y*w*ib", [   1, 0xC2      ], X, PREF_F2 | AUTO_VEXL | VEX_OP;
] "cmpss"       = [ "yowoib",   [0x0F, 0xC2      ], X, PREF_F3;
] "vcmpss"      = [ "y*y*w*ib", [   1, 0xC2      ], X, PREF_F3 | AUTO_VEXL | VEX_OP;
] "comisd"      = [ "yoyo",     [0x0F, 0x2F      ], X, PREF_66;
                    "yomq",     [0x0F, 0x2F      ], X, PREF_66;
] "vcomisd"     = [ "yoyo",     [   1, 0x2F      ], X, PREF_66             | VEX_OP;
                    "yomq",     [   1, 0x2F      ], X, PREF_66             | VEX_OP;
] "comiss"      = [ "yoyo",     [0x0F, 0x2F      ], X;
                    "yomd",     [0x0F, 0x2F      ], X;
] "vcomiss"     = [ "yoyo",     [   1, 0x2F      ], X,                       VEX_OP;
                    "yomd",     [   1, 0x2F      ], X,                       VEX_OP;
]

  "cvtdq2pd"    = [ "yoyo",     [0x0F, 0xE6      ], X, PREF_F3;
                    "yomq",     [0x0F, 0xE6      ], X, PREF_F3;
] "vcvtdq2pd"   = [ "y*y*",     [   1, 0xE6      ], X, PREF_F3 | AUTO_VEXL | VEX_OP;
                    "yomq",     [   1, 0xE6      ], X, PREF_F3             | VEX_OP;
                    "yhmo",     [   1, 0xE6      ], X, PREF_F3 | WITH_VEXL | VEX_OP; // intel/amd disagree over this memory ops size
] "cvtdq2ps"    = [ "yowo",     [0x0F, 0x5B      ], X;
] "vcvtdq2ps"   = [ "y*w*",     [   1, 0x5B      ], X,           AUTO_VEXL | VEX_OP;
] "cvtpd2dq"    = [ "yowo",     [0x0F, 0xE6      ], X, PREF_F2;
] "vcvtpd2dq"   = [ "y*w*",     [   1, 0xE6      ], X, PREF_F2 | AUTO_VEXL | VEX_OP;
] "cvtpd2dS"    = [ "yowo",     [0x0F, 0x5A      ], X, PREF_66;
] "vcvtpd2dS"   = [ "y*w*",     [   1, 0x5A      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "cvtps2dq"    = [ "yowo",     [0x0F, 0x5B      ], X, PREF_66;
] "vcvtps2dq"   = [ "y*w*",     [   1, 0x5B      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "cvtps2pd"    = [ "yoyo",     [0x0F, 0x5A      ], X;
                    "yomq",     [0x0F, 0x5A      ], X;
] "vcvtps2pd"   = [ "y*y*",     [   1, 0x5A      ], X,           AUTO_VEXL | VEX_OP;
                    "yomq",     [   1, 0x5A      ], X,                       VEX_OP;
                    "yhmo",     [   1, 0x5A      ], X,           WITH_VEXL | VEX_OP; // intel/amd disagree over this memory ops size
] "cvtsd2si"    = [ "r*yo",     [0x0F, 0x2D      ], X, PREF_F2 | AUTO_REXW;
                    "r*mq",     [0x0F, 0x2D      ], X, PREF_F2 | AUTO_REXW;
] "vcvtsd2si"   = [ "r*yo",     [   1, 0x2D      ], X, PREF_F2 | AUTO_REXW | VEX_OP;
                    "r*mq",     [   1, 0x2D      ], X, PREF_F2 | AUTO_REXW | VEX_OP;
] "cvtsd2ss"    = [ "yoyo",     [0x0F, 0x5A      ], X, PREF_F2;
                    "yomq",     [0x0F, 0x5A      ], X, PREF_F2;
] "vcvtsd2ss"   = [ "yoyoyo",   [   1, 0x5A      ], X, PREF_F2             | VEX_OP;
                    "yoyomq",   [   1, 0x5A      ], X, PREF_F2             | VEX_OP;
] "cvtsi2sd"    = [ "yov*",     [0x0F, 0x2A      ], X, PREF_F2 | AUTO_REXW;
] "vcvtsi2sd"   = [ "yoyov*",   [   1, 0x2A      ], X, PREF_F2 | AUTO_REXW | VEX_OP;
] "cvtsi2ss"    = [ "yov*",     [0x0F, 0x2A      ], X, PREF_F3 | AUTO_REXW;
] "vcvtsi2ss"   = [ "yoyov*",   [   1, 0x2A      ], X, PREF_F3 | AUTO_REXW | VEX_OP;
] "cvtss2sd"    = [ "yoyo",     [0x0F, 0x5A      ], X, PREF_F3;
                    "yomd",     [0x0F, 0x5A      ], X, PREF_F3;
] "vcvtss2sd"   = [ "yoyo",     [   1, 0x5A      ], X, PREF_F3             | VEX_OP;
                    "yomq",     [   1, 0x5A      ], X, PREF_F3             | VEX_OP;
] "cvtss2si"    = [ "r*yo",     [0x0F, 0x2D      ], X, PREF_F3 | AUTO_REXW;
                    "r*m*",     [0x0F, 0x2D      ], X, PREF_F3 | AUTO_REXW;
] "vcvtss2si"   = [ "r*yo",     [   1, 0x2D      ], X, PREF_F3 | AUTO_REXW | VEX_OP;
                    "r*m*",     [   1, 0x2D      ], X, PREF_F3 | AUTO_REXW | VEX_OP;
] "cvttpd2dq"   = [ "yowo",     [0x0F, 0xE6      ], X, PREF_66;
] "vcvttpd2dq"  = [ "y*w*",     [   1, 0xE6      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "cvttps2dq"   = [ "yowo",     [0x0F, 0x5B      ], X, PREF_F3;
] "vcvttps2dq"  = [ "y*w*",     [   1, 0x5B      ], X, PREF_F3 | AUTO_VEXL | VEX_OP;
] "cvttsd2si"   = [ "r*yo",     [0x0F, 0x2C      ], X, PREF_F2 | AUTO_REXW;
                    "r*mq",     [0x0F, 0x2C      ], X, PREF_F2 | AUTO_REXW;
] "vcvttsd2si"  = [ "r*yo",     [   1, 0x2C      ], X, PREF_F2 | AUTO_REXW | VEX_OP;
                    "r*mq",     [   1, 0x2C      ], X, PREF_F2 | AUTO_REXW | VEX_OP;
] "cvttss2si"   = [ "r*yo",     [0x0F, 0x2C      ], X, PREF_F3 | AUTO_REXW;
                    "r*m*",     [0x0F, 0x2C      ], X, PREF_F3 | AUTO_REXW;
] "vcvttss2si"  = [ "r*yo",     [   1, 0x2C      ], X, PREF_F3 | AUTO_REXW | VEX_OP;
                    "r*m*",     [   1, 0x2C      ], X, PREF_F3 | AUTO_REXW | VEX_OP;
]

  "divpd"       = [ "yowo",     [0x0F, 0x5E      ], X, PREF_66;
] "vdivpd"      = [ "y*y*w*",   [   1, 0x5E      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "divps"       = [ "yowo",     [0x0F, 0x5E      ], X;
] "vdivps"      = [ "y*y*w*",   [   1, 0x5E      ], X,           AUTO_VEXL | VEX_OP;
] "divsd"       = [ "yoyo",     [0x0F, 0x5E      ], X, PREF_F2;
                    "yomq",     [0x0F, 0x5E      ], X, PREF_F2;
] "vdivsd"      = [ "yoyoyo",   [   1, 0x5E      ], X, PREF_F2             | VEX_OP;
                    "yoyomq",   [   1, 0x5E      ], X, PREF_F2             | VEX_OP;
] "divss"       = [ "yoyo",     [0x0F, 0x5E      ], X, PREF_F3;
                    "yomd",     [0x0F, 0x5E      ], X, PREF_F3;
] "vdivss"      = [ "yoyoyo",   [   1, 0x5E      ], X, PREF_F3             | VEX_OP;
                    "yoyomd",   [   1, 0x5E      ], X, PREF_F3             | VEX_OP;
] "dppd"        = [ "yowoib",   [0x0F, 0x3A, 0x41], X, PREF_66;
] "vdppd"       = [ "yoyowoib", [   3, 0x41      ], X, PREF_66             | VEX_OP;
] "dpps"        = [ "yowoib",   [0x0F, 0x3A, 0x40], X, PREF_66;
] "vdpps"       = [ "y*y*w*ib", [   3, 0x40      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "extractps"   = [ "vwyoib",   [0x0F, 0x3A, 0x17], X, PREF_66;
] "vextractps"  = [ "vwyoib",   [   3, 0x17      ], X, PREF_66             | VEX_OP;
] "haddpd"      = [ "yowo",     [0x0F, 0x7C      ], X, PREF_66;
] "vhaddpd"     = [ "y*y*w*",   [   1, 0x7C      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "haddps"      = [ "yowo",     [0x0F, 0x7C      ], X, PREF_F2;
] "vhaddps"     = [ "y*y*w*",   [   1, 0x7C      ], X, PREF_F2 | AUTO_VEXL | VEX_OP;
] "hsubpd"      = [ "yowo",     [0x0F, 0x7D      ], X, PREF_66;
] "vhsubpd"     = [ "y*y*w*",   [   1, 0x7D      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "hsubps"      = [ "yowo",     [0x0F, 0x7D      ], X, PREF_F2;
] "vhsubps"     = [ "y*y*w*",   [   1, 0x7D      ], X, PREF_F2 | AUTO_VEXL | VEX_OP;
] "insertps"    = [ "yowoib",   [0x0F, 0x3A, 0x21], X, PREF_66;
] "vinsertps"   = [ "yoyowoib", [   3, 0x21      ], X, PREF_66             | VEX_OP;
] "lddqu"       = [ "yomo",     [0x0F, 0xF0      ], X, PREF_F2;
] "vlddqu"      = [ "y*m*",     [   1, 0xF0      ], X, PREF_F2 | AUTO_VEXL | VEX_OP;
] "ldmxcsr"     = [ "md",       [0x0F, 0xAE      ], 2;
] "vldmxcsr"    = [ "md",       [   1, 0xAE      ], 2,                       VEX_OP;
] "maskmovdqu"  = [ "yoyo",     [0x0F, 0xF7      ], X, PREF_66;
] "vmaskmovdqu" = [ "yoyo",     [   1, 0xF7      ], X, PREF_66             | VEX_OP;
] "maxpd"       = [ "yowo",     [0x0F, 0x5F      ], X, PREF_66;
] "vmaxpd"      = [ "y*y*w*",   [   1, 0x5F      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "maxps"       = [ "yowo",     [0x0F, 0x5F      ], X;
] "vmaxps"      = [ "y*y*w*",   [   1, 0x5F      ], X,           AUTO_VEXL | VEX_OP;
] "maxsd"       = [ "yoyo",     [0x0F, 0x5F      ], X, PREF_F2;
                    "yomq",     [0x0F, 0x5F      ], X, PREF_F2;
] "vmaxsd"      = [ "yoyoyo",   [   1, 0x5F      ], X, PREF_F2             | VEX_OP;
                    "yoyomq",   [   1, 0x5F      ], X, PREF_F2             | VEX_OP;
] "maxss"       = [ "yoyo",     [0x0F, 0x5F      ], X, PREF_F3;
                    "yomd",     [0x0F, 0x5F      ], X, PREF_F3;
] "vmaxss"      = [ "yoyoyo",   [   1, 0x5F      ], X, PREF_F3             | VEX_OP;
                    "yoyomd",   [   1, 0x5F      ], X, PREF_F3             | VEX_OP;
] "minpd"       = [ "yowo",     [0x0F, 0x5D      ], X, PREF_66;
] "vminpd"      = [ "y*y*w*",   [   1, 0x5D      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "minps"       = [ "yowo",     [0x0F, 0x5D      ], X;
] "vminps"      = [ "y*y*w*",   [   1, 0x5D      ], X,           AUTO_VEXL | VEX_OP;
] "minsd"       = [ "yoyo",     [0x0F, 0x5D      ], X, PREF_F2;
                    "yomq",     [0x0F, 0x5D      ], X, PREF_F2;
] "vminsd"      = [ "yoyoyo",   [   1, 0x5D      ], X, PREF_F2             | VEX_OP;
                    "yoyomq",   [   1, 0x5D      ], X, PREF_F2             | VEX_OP;
] "minss"       = [ "yoyo",     [0x0F, 0x5D      ], X, PREF_F3;
                    "yomd",     [0x0F, 0x5D      ], X, PREF_F3;
] "vminss"      = [ "yoyoyo",   [   1, 0x5D      ], X, PREF_F3             | VEX_OP;
                    "yoyomd",   [   1, 0x5D      ], X, PREF_F3             | VEX_OP;
]

  "movapd"      = [ "yowo",     [0x0F, 0x28      ], X, PREF_66;
                    "woyo",     [0x0F, 0x29      ], X, PREF_66;
] "vmovapd"     = [ "y*w*",     [   1, 0x28      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "w*y*",     [   1, 0x29      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "movaps"      = [ "yowo",     [0x0F, 0x28      ], X;
                    "woyo",     [0x0F, 0x29      ], X;
] "vmovaps"     = [ "y*w*",     [   1, 0x28      ], X,           AUTO_VEXL | VEX_OP;
                    "w*y*",     [   1, 0x29      ], X,           AUTO_VEXL | VEX_OP;
] // movd is found under the general purpose instructions
  "vmovd"       = [ "yov*",     [   1, 0x6E      ], X, PREF_66 | AUTO_REXW | VEX_OP;
                    "v*yo",     [   1, 0x7E      ], X, PREF_66 | AUTO_REXW | VEX_OP;
] "movddup"     = [ "yoyo",     [0x0F, 0x12      ], X, PREF_F2;
                    "yomq",     [0x0F, 0x12      ], X, PREF_F2;
] "vmovddup"    = [ "y*y*",     [   1, 0x12      ], X, PREF_F2 | AUTO_VEXL | VEX_OP;
                    "yomq",     [   1, 0x12      ], X, PREF_F2             | VEX_OP;
                    "yhmh",     [   1, 0x12      ], X, PREF_F2 | WITH_VEXL | VEX_OP;
] "movdqa"      = [ "yowo",     [0x0F, 0x6F      ], X, PREF_66;
                    "woyo",     [0x0F, 0x7F      ], X, PREF_66;
] "vmovdqa"     = [ "y*w*",     [   1, 0x6F      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "w*y*",     [   1, 0x7F      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "movdqu"      = [ "yowo",     [0x0F, 0x6F      ], X, PREF_F3;
                    "woyo",     [0x0F, 0x7F      ], X, PREF_F3;
] "vmovdqu"     = [ "y*w*",     [   1, 0x6F      ], X, PREF_F3 | AUTO_VEXL | VEX_OP;
                    "w*y*",     [   1, 0x7F      ], X, PREF_F3 | AUTO_VEXL | VEX_OP;
] "movhlps"     = [ "yoyo",     [0x0F, 0x12      ], X;
] "vmovhlps"    = [ "yoyoyo",   [   1, 0x12      ], X,                       VEX_OP;
] "movhpd"      = [ "yomq",     [0x0F, 0x16      ], X, PREF_66;
                    "mqyo",     [0x0F, 0x17      ], X, PREF_66;
] "vmovhpd"     = [ "yoyomq",   [   1, 0x16      ], X, PREF_66             | VEX_OP;
                    "mqyo",     [   1, 0x17      ], X, PREF_66             | VEX_OP;
] "movhps"      = [ "yomq",     [0x0F, 0x16      ], X;
                    "mqyo",     [0x0F, 0x17      ], X;
] "vmovhps"     = [ "yoyomq",   [   1, 0x16      ], X,                       VEX_OP;
                    "mqyo",     [   1, 0x17      ], X,                       VEX_OP;
] "movlhps"     = [ "yoyo",     [0x0F, 0x16      ], X;
] "vmovlhps"    = [ "yoyoyo",   [   1, 0x16      ], X,                       VEX_OP;
] "movlpd"      = [ "yomq",     [0x0F, 0x12      ], X, PREF_66;
                    "mqyo",     [0x0F, 0x13      ], X, PREF_66;
] "vmovlpd"     = [ "yoyomq",   [   1, 0x12      ], X, PREF_66             | VEX_OP;
                    "mqyo",     [   1, 0x13      ], X, PREF_66             | VEX_OP;
] "movlps"      = [ "yomq",     [0x0F, 0x12      ], X;
                    "mqyo",     [0x0F, 0x13      ], X;
] "vmovlps"     = [ "yoyomq",   [   1, 0x12      ], X,                       VEX_OP;
                    "mqyo",     [   1, 0x13      ], X,                       VEX_OP;
] // movmskpd is found under generic instrs
  "vmovmskpd"   = [ "r?y*",     [   1, 0x50      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // same for movmskps
  "vmovmskps"   = [ "r?y*",     [   1, 0x50      ], X,           AUTO_VEXL | VEX_OP;
] "movntdq"     = [ "moyo",     [0x0F, 0xE7      ], X, PREF_66;
] "vmovntdq"    = [ "m*y*",     [   1, 0xE7      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "movntdqa"    = [ "moyo",     [0x0F, 0x38, 0x2A], X, PREF_66;
] "vmovntdqa"   = [ "m*y*",     [   2, 0x2A      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "movntpd"     = [ "moyo",     [0x0F, 0x2B      ], X, PREF_66;
] "vmovntpd"    = [ "m*y*",     [   1, 0x2B      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "movntps"     = [ "moyo",     [0x0F, 0x2B      ], X;
] "vmovntps"    = [ "m*y*",     [   1, 0x2B      ], X,           AUTO_VEXL | VEX_OP;
] "movntsd"     = [ "mqyo",     [0x0F, 0x2B      ], X, PREF_F2;
] "movntss"     = [ "mdyo",     [0x0F, 0x2B      ], X, PREF_F3;
  // movq variants can be found in the MMX section
] "vmovq"       = [ "yoyo",     [   1, 0x7E      ], X, PREF_F3             | VEX_OP;
                    "yomq",     [   1, 0x7E      ], X, PREF_F3             | VEX_OP;
                    "mqyo",     [   1, 0xD6      ], X, PREF_66             | VEX_OP;
  // movsd variants can be found in the general purpose section
] "vmovsd"      = [ "yoyoyo",   [   1, 0x10      ], X, PREF_F2             | VEX_OP; // distinguished from the others by addressing bits
                    "yomq",     [   1, 0x10      ], X, PREF_F2             | VEX_OP;
                    "mqyo",     [   1, 0x11      ], X, PREF_F2             | VEX_OP;
] "movshdup"    = [ "yowo",     [0x0F, 0x16      ], X, PREF_F3;
] "vmovshdup"   = [ "y*w*",     [   1, 0x16      ], X, PREF_F3 | AUTO_VEXL | VEX_OP;
] "movsldup"    = [ "yowo",     [0x0F, 0x12      ], X, PREF_F3;
] "vmovsldup"   = [ "y*w*",     [   1, 0x12      ], X, PREF_F3 | AUTO_VEXL | VEX_OP;
] "movss"       = [ "yoyo",     [0x0F, 0x10      ], X, PREF_F3;
                    "yomd",     [0x0F, 0x10      ], X, PREF_F3;
                    "mdyo",     [0x0F, 0x11      ], X, PREF_F3;
] "vmovss"      = [ "yoyoyo",   [   1, 0x10      ], X, PREF_F3             | VEX_OP;
                    "yomd",     [   1, 0x10      ], X, PREF_F3             | VEX_OP;
                    "mdyo",     [   1, 0x11      ], X, PREF_F3             | VEX_OP;
] "movupd"      = [ "yowo",     [0x0F, 0x10      ], X, PREF_66;
                    "woyo",     [0x0F, 0x11      ], X, PREF_66;
] "vmovupd"     = [ "y*w*",     [   1, 0x10      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "w*y*",     [   1, 0x11      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "movups"      = [ "yowo",     [0x0F, 0x10      ], X;
                    "woyo",     [0x0F, 0x11      ], X;
] "vmovups"     = [ "y*w*",     [   1, 0x10      ], X,           AUTO_VEXL | VEX_OP;
                    "w*y*",     [   1, 0x11      ], X,           AUTO_VEXL | VEX_OP;
]
// and we're done with mov ins.
  "mpsadbw"     = ["yowoib",    [0x0F, 0x3A, 0x42], X, PREF_66;
] "vmpsadbw"    = ["y*y*w*ib",  [   3, 0x42      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "mulpd"       = [ "yowo",     [0x0F, 0x59      ], X, PREF_66;
] "vmulpd"      = [ "y*y*w*",   [   1, 0x59      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "mulps"       = [ "yowo",     [0x0F, 0x59      ], X;
] "vmulps"      = [ "y*y*w*",   [   1, 0x59      ], X,           AUTO_VEXL | VEX_OP;
] "mulsd"       = [ "yoyo",     [0x0F, 0x59      ], X, PREF_F2;
                    "yomq",     [0x0F, 0x59      ], X, PREF_F2;
] "vmulsd"      = [ "yoyoyo",   [   1, 0x59      ], X, PREF_F2             | VEX_OP;
                    "yoyomq",   [   1, 0x59      ], X, PREF_F2             | VEX_OP;
] "mulss"       = [ "yoyo",     [0x0F, 0x59      ], X, PREF_F3;
                    "yomd",     [0x0F, 0x59      ], X, PREF_F3;
] "vmulss"      = [ "yoyoyo",   [   1, 0x59      ], X, PREF_F3             | VEX_OP;
                    "yoyomd",   [   1, 0x59      ], X, PREF_F3             | VEX_OP;
] "orpd"        = [ "yowo",     [0x0F, 0x56      ], X, PREF_66;
] "vorpd"       = [ "y*y*w*",   [   1, 0x56      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "orps"        = [ "yowo",     [0x0F, 0x56      ], X;
] "vorps"       = [ "y*y*w*",   [   1, 0x56      ], X,           AUTO_VEXL | VEX_OP;
] "pabsb"       = [ "yowo",     [0x0F, 0x38, 0x1C], X;
] "vpabsb"      = [ "y*w*",     [   2, 0x1C      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pabsd"       = [ "yowo",     [0x0F, 0x38, 0x1E], X;
] "vpabsd"      = [ "y*w*",     [   2, 0x1E      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pabsw"       = [ "yowo",     [0x0F, 0x38, 0x1D], X;
] "vpabsw"      = [ "y*w*",     [   2, 0x1D      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // packssdw is found in the MMX section
  "vpackssdw"   = [ "y*y*w*",   [   1, 0x6B      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // same for packsswb
  "vpacksswb"   = [ "y*y*w*",   [   1, 0x63      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "packusdw"    = [ "yowo",     [0x0F, 0x38, 0x2B], X, PREF_66;
] "vpackusdw"   = [ "y*y*w*",   [   2, 0x2B      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
]  // and for packuswb
  "vpackuswb"   = [ "y*y*w*",   [   1, 0x67      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // and all legacy padd forms
  "vpaddb"      = [ "y*y*w*",   [   1, 0xFC      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpaddd"      = [ "y*y*w*",   [   1, 0xFE      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpaddq"      = [ "y*y*w*",   [   1, 0xD4      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpaddsb"     = [ "y*y*w*",   [   1, 0xEC      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpaddsw"     = [ "y*y*w*",   [   1, 0xED      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpaddusb"    = [ "y*y*w*",   [   1, 0xDC      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpaddusw"    = [ "y*y*w*",   [   1, 0xDD      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpaddw"      = [ "y*y*w*",   [   1, 0xFD      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "palign"      = [ "y*w*ib",   [0x0F, 0x3A, 0x0F], X, PREF_66;
] "vpalign"     = [ "y*y*w*ib", [   3, 0x0F      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // pand/pandn/pavg are also in the MMX section
  "vpand"       = [ "y*y*w*",   [   1, 0xDB      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpandn"      = [ "y*y*w*",   [   1, 0xDF      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpavgb"      = [ "y*y*w*",   [   1, 0xE0      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpavgw"      = [ "y*y*w*",   [   1, 0xE3      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pblendvb"    = [ "yowo",     [0x0F, 0x38, 0x10], X, PREF_66;
] "vpblendvb"   = [ "y*y*w*y*", [   3, 0x4C      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pblenddw"    = [ "yowoib",   [0x0F, 0x3A, 0x0E], X, PREF_66;
] "vpblenddw"   = [ "y*y*w*ib", [   3, 0x0E      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pclmulqdq"   = [ "yowoib",   [0x0F, 0x3A, 0x44], X, PREF_66;
] "vpclmulqdq"  = [ "yoyowoib", [   3, 0x44      ], X, PREF_66             | VEX_OP;
] // pcmpeqb is in the MMX section
  "vpcmpeqb"    = [ "y*y*w*",   [   1, 0x74      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // pcmpeqd is in the MMX section
  "vpcmpeqd"    = [ "y*y*w*",   [   1, 0x76      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pcmpeqq"     = [ "yowo",     [0x0F, 0x38, 0x29], X, PREF_66;
] "vpcmpeqq"    = [ "y*y*w*",   [   2, 0x29      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // pcmpeqw is in the MMX section
  "vpcmpeqw"    = [ "y*y*w*",   [   1, 0x75      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pcmpestri"   = [ "yowoib",   [0x0F, 0x3A, 0x61], X, PREF_66;
] "vpcmpestri"  = [ "yowoib",   [   3, 0x61      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pcmpestrm"   = [ "yowoib",   [0x0F, 0x3A, 0x60], X, PREF_66;
] "vpcmpestrm"  = [ "yowoib",   [   3, 0x60      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // pcmpgtb is in the MMX section
  "vpcmpgtb"    = [ "y*y*w*",   [   1, 0x64      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // pcmpgtd is in the MMX section
  "vpcmpgtd"    = [ "y*y*w*",   [   1, 0x66      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pcmpgtq"     = [ "yowo",     [0x0F, 0x38, 0x37], X, PREF_66;
] "vpcmpgtq"    = [ "y*y*w*",   [   2, 0x37      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // pcmpgtw is in the MMX section
  "vpcmpgtw"    = [ "y*y*w*",   [   1, 0x65      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pcmpistri"   = [ "yowoib",   [0x0F, 0x3A, 0x63], X, PREF_66;
] "vpcmpistri"  = [ "yowoib",   [   3, 0x63      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pcmpistrm"   = [ "yowoib",   [0x0F, 0x3A, 0x62], X, PREF_66;
] "vpcmpistrm"  = [ "yowoib",   [   3, 0x62      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pextrb"      = [ "r?yoib",   [0x0F, 0x3A, 0x14], X, PREF_66                      | ENC_MR;
                    "mbyoib",   [0x0F, 0x3A, 0x14], X, PREF_66;
] "vpextrb"     = [ "r?yoib",   [   3, 0x14      ], X, PREF_66             | VEX_OP | ENC_MR;
                    "mbyoib",   [   3, 0x14      ], X, PREF_66             | VEX_OP;
] "pextrd"      = [ "vdyoib",   [0x0F, 0x3A, 0x16], X, PREF_66;
] "vpextrd"     = [ "vdyoib",   [   3, 0x16      ], X, PREF_66             | VEX_OP;
] "pextrq"      = [ "vqyoib",   [0x0F, 0x3A, 0x16], X, PREF_66 | WITH_REXW;
] "vpextrq"     = [ "vqyoib",   [   3, 0x16      ], X, PREF_66 | WITH_REXW| VEX_OP;
] // pextrw is in the MMX section
  "vpextrw"     = [ "r?yoib",   [   1, 0xC5      ], X, PREF_66             | VEX_OP | ENC_MR;
                    "mwyoib",   [   3, 0x15      ], X, PREF_66             | VEX_OP;
] "phaddd"      = [ "yowo",     [0x0F, 0x38, 0x02], X, PREF_66;
] "vphaddd"     = [ "y*y*w*",   [   2, 0x02      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "phaddsw"     = [ "yowo",     [0x0F, 0x38, 0x03], X, PREF_66;
] "vphaddsw"    = [ "y*y*w*",   [   2, 0x03      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "phaddw"      = [ "yowo",     [0x0F, 0x38, 0x01], X, PREF_66;
] "vphaddw"     = [ "y*y*w*",   [   2, 0x01      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "phminposuw"  = [ "yowo",     [0x0F, 0x38, 0x41], X, PREF_66;
] "vphminposuw" = [ "yowo",     [   2, 0x41      ], X, PREF_66             | VEX_OP;
] "phsubd"      = [ "yowo",     [0x0F, 0x38, 0x06], X, PREF_66;
] "vphsubd"     = [ "y*y*w*",   [   2, 0x06      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "phsubsw"     = [ "yowo",     [0x0F, 0x38, 0x07], X, PREF_66;
] "vphsubsw"    = [ "y*y*w*",   [   2, 0x07      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "phsubw"      = [ "yowo",     [0x0F, 0x38, 0x05], X, PREF_66;
] "vphsubw"     = [ "y*y*w*",   [   2, 0x05      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pinsrb"      = [ "yordib",   [0x0F, 0x3A, 0x20], X, PREF_66;
                    "yombib",   [0x0F, 0x3A, 0x20], X, PREF_66;
] "vpinsrb"     = [ "yordyoib", [   3, 0x20      ], X, PREF_66             | VEX_OP;
                    "yombyoib", [   3, 0x20      ], X, PREF_66             | VEX_OP;
] "pinsrd"      = [ "yovdib",   [0x0F, 0x3A, 0x22], X, PREF_66;
] "vpinsrd"     = [ "yovdyoib", [   3, 0x22      ], X, PREF_66             | VEX_OP;
] "pinsrq"      = [ "yovqib",   [0x0F, 0x3A, 0x22], X, PREF_66 | WITH_REXW;
] "vpinsrq"     = [ "yovqyoib", [   3, 0x22      ], X, PREF_66 | WITH_REXW| VEX_OP;
] // pinsrw is in the MMX section
  "vpinsrw"     = [ "yordyoib", [   1, 0xC4      ], X, PREF_66             | VEX_OP | ENC_MR;
                    "yomwyoib", [   1, 0xC4      ], X, PREF_66             | VEX_OP;
] "pmaddubsw"   = [ "yowo",     [0x0F, 0x38, 0x04], X, PREF_66;
] "vpmaddubsw"  = [ "y*y*w*",   [   2, 0x04      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // pmaddwd is in the MMX section
  "vpmaddwd"    = [ "y*y*w*",   [   1, 0xF5      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pmaxsb"      = [ "yowo",     [0x0F, 0x38, 0x3C], X, PREF_66;
] "vpmaxsb"     = [ "y*y*w*",   [   2, 0x3C      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pmaxsd"      = [ "yowo",     [0x0F, 0x38, 0x3D], X, PREF_66;
] "vpmaxsd"     = [ "y*y*w*",   [   2, 0x3D      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // pmaxsw is in the MMX section
  "vpmaxsw"     = [ "y*y*w*",   [   1, 0xEE      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // pmaxub is in the MMX section
  "vpmaxub"     = [ "y*y*w*",   [   1, 0xDE      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pmaxud"      = [ "yowo",     [0x0F, 0x38, 0x3F], X, PREF_66;
] "vpmaxud"     = [ "y*y*w*",   [   2, 0x3F      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pmaxuw"      = [ "yowo",     [0x0F, 0x38, 0x3E], X, PREF_66;
] "vpmaxuw"     = [ "y*y*w*",   [   2, 0x3E      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
]
  "pminsb"      = [ "yowo",     [0x0F, 0x38, 0x38], X, PREF_66;
] "vpminsb"     = [ "y*y*w*",   [   2, 0x38      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pminsd"      = [ "yowo",     [0x0F, 0x38, 0x39], X, PREF_66;
] "vpminsd"     = [ "y*y*w*",   [   2, 0x39      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // pminsw is in the MMX section
  "vpminsw"     = [ "y*y*w*",   [   1, 0xEA      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // pminub is in the MMX section
  "vpminub"     = [ "y*y*w*",   [   1, 0xDA      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pminud"      = [ "yowo",     [0x0F, 0x38, 0x3B], X, PREF_66;
] "vpminud"     = [ "y*y*w*",   [   2, 0x3B      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pminuw"      = [ "yowo",     [0x0F, 0x38, 0x3A], X, PREF_66;
] "vpminuw"     = [ "y*y*w*",   [   2, 0x3A      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
]
// back to move ops
  // pmovmskb is in the MMX section
  "vpmovmskb"   = [ "rqy*",     [   1, 0xD7      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pmovsxbd"    = [ "yoyo",     [0x0F, 0x38, 0x21], X, PREF_66;
                    "yomd",     [0x0F, 0x38, 0x21], X, PREF_66;
] "vpmovsxbd"   = [ "y*y*",     [   2, 0x21      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "yomd",     [   2, 0x21      ], X, PREF_66             | VEX_OP;
                    "yhmq",     [   2, 0x21      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "pmovsxbq"    = [ "yoyo",     [0x0F, 0x38, 0x22], X, PREF_66;
                    "yomw",     [0x0F, 0x38, 0x22], X, PREF_66;
] "vpmovsxbq"   = [ "y*y*",     [   2, 0x22      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "yomw",     [   2, 0x22      ], X, PREF_66             | VEX_OP;
                    "yhmd",     [   2, 0x22      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "pmovsxbw"    = [ "yoyo",     [0x0F, 0x38, 0x20], X, PREF_66;
                    "yomq",     [0x0F, 0x38, 0x20], X, PREF_66;
] "vpmovsxbw"   = [ "y*y*",     [   2, 0x20      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "yomq",     [   2, 0x20      ], X, PREF_66             | VEX_OP;
                    "yhmo",     [   2, 0x20      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "pmovsxdq"    = [ "yoyo",     [0x0F, 0x38, 0x25], X, PREF_66;
                    "yomq",     [0x0F, 0x38, 0x25], X, PREF_66;
] "vpmovsxdq"   = [ "y*y*",     [   2, 0x25      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "yomq",     [   2, 0x25      ], X, PREF_66             | VEX_OP;
                    "yhmo",     [   2, 0x25      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "pmovsxwd"    = [ "yoyo",     [0x0F, 0x38, 0x23], X, PREF_66;
                    "yomq",     [0x0F, 0x38, 0x23], X, PREF_66;
] "vpmovsxwd"   = [ "y*y*",     [   2, 0x23      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "yomq",     [   2, 0x23      ], X, PREF_66             | VEX_OP;
                    "yhmo",     [   2, 0x23      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "pmovsxwq"    = [ "yoyo",     [0x0F, 0x38, 0x24], X, PREF_66;
                    "yomd",     [0x0F, 0x38, 0x24], X, PREF_66;
] "vpmovsxwq"   = [ "y*y*",     [   2, 0x24      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "yomd",     [   2, 0x24      ], X, PREF_66             | VEX_OP;
                    "yhmq",     [   2, 0x24      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "pmovzxbd"    = [ "yoyo",     [0x0F, 0x38, 0x31], X, PREF_66;
                    "yomd",     [0x0F, 0x38, 0x31], X, PREF_66;
] "vpmovzxbd"   = [ "y*y*",     [   2, 0x31      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "yomd",     [   2, 0x31      ], X, PREF_66             | VEX_OP;
                    "yhmq",     [   2, 0x31      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "pmovzxbq"    = [ "yoyo",     [0x0F, 0x38, 0x32], X, PREF_66;
                    "yomw",     [0x0F, 0x38, 0x32], X, PREF_66;
] "vpmovzxbq"   = [ "y*y*",     [   2, 0x32      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "yomw",     [   2, 0x32      ], X, PREF_66             | VEX_OP;
                    "yhmd",     [   2, 0x32      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "pmovzxbw"    = [ "yoyo",     [0x0F, 0x38, 0x30], X, PREF_66;
                    "yomq",     [0x0F, 0x38, 0x30], X, PREF_66;
] "vpmovzxbw"   = [ "y*y*",     [   2, 0x30      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "yomq",     [   2, 0x30      ], X, PREF_66             | VEX_OP;
                    "yhmo",     [   2, 0x30      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "pmovzxdq"    = [ "yoyo",     [0x0F, 0x38, 0x35], X, PREF_66;
                    "yomq",     [0x0F, 0x38, 0x35], X, PREF_66;
] "vpmovzxdq"   = [ "y*y*",     [   2, 0x35      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "yomq",     [   2, 0x35      ], X, PREF_66             | VEX_OP;
                    "yhmo",     [   2, 0x35      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "pmovzxwd"    = [ "yoyo",     [0x0F, 0x38, 0x33], X, PREF_66;
                    "yomq",     [0x0F, 0x38, 0x33], X, PREF_66;
] "vpmovzxwd"   = [ "y*y*",     [   2, 0x33      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "yomq",     [   2, 0x33      ], X, PREF_66             | VEX_OP;
                    "yhmo",     [   2, 0x33      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "pmovzxwq"    = [ "yoyo",     [0x0F, 0x38, 0x34], X, PREF_66;
                    "yomd",     [0x0F, 0x38, 0x34], X, PREF_66;
] "vpmovzxwq"   = [ "y*y*",     [   2, 0x34      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "yomd",     [   2, 0x34      ], X, PREF_66             | VEX_OP;
                    "yhmq",     [   2, 0x34      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] // and back to arithmetric
  "pmuldq"      = [ "yowo",     [0x0F, 0x38, 0x28], X, PREF_66;
] "vpmuldq"     = [ "y*y*w*",   [   2, 0x28      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pmulhrsw"    = [ "yowo",     [0x0F, 0x38, 0x0B], X, PREF_66;
] "vpmulhrsw"   = [ "y*y*w*",   [   2, 0x0B      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // legacy form is in the MMX section
  "vpmulhuw"    = [ "y*y*w*",   [   1, 0xE4      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // legacy form is in the MMX section
  "vpmulhw"     = [ "y*y*w*",   [   1, 0xE5      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pmulld"      = [ "yowo",     [0x0F, 0x38, 0x40], X, PREF_66;
] "vpmulld"     = [ "y*y*w*",   [   2, 0x40      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // legacy form is in the MMX section
  "vpmullw"     = [ "y*y*w*",   [   1, 0xD5      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // legacy form is in the MMX section
  "vpmuludq"    = [ "y*y*w*",   [   1, 0xF4      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // legacy form is in the MMX section
  "vpor"        = [ "y*y*w*",   [   1, 0xEB      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // legacy form is in the MMX section
  "vpsadbw"     = [ "y*y*w*",   [   1, 0xF6      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pshufb"      = [ "yowo",     [0x0F, 0x38, 0x00], X, PREF_66;
] "vpshufb"     = [ "y*y*w*",   [   2, 0x00      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "pshufd"      = [ "yowoib",   [0x0F, 0x70      ], X, PREF_66;
] "vpshufd"     = [ "y*w*ib",   [   1, 0x70      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // legacy form is in the MMX section
  "vpshufw"     = [ "y*w*ib",   [   1, 0x70      ], X, PREF_F3 | AUTO_VEXL | VEX_OP;
] "pshuflw"     = [ "yowoib",   [0x0F, 0x70      ], X, PREF_F2;
] "vpshuflw"    = [ "y*w*ib",   [   1, 0x70      ], X, PREF_F2 | AUTO_VEXL | VEX_OP;
] "psignb"      = [ "yowo",     [0x0F, 0x38, 0x08], X, PREF_66;
] "vpsignb"     = [ "y*y*w*",   [   2, 0x08      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "psignd"      = [ "yowo",     [0x0F, 0x38, 0x0A], X, PREF_66;
] "vpsignd"     = [ "y*y*w*",   [   2, 0x0A      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "psignw"      = [ "yowo",     [0x0F, 0x38, 0x09], X, PREF_66;
] "vpsignw"     = [ "y*y*w*",   [   2, 0x09      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // Legacy forms of the shift instructions are in the MMX section
  "vpslld"      = [ "y*y*wo",   [   1, 0xF2      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "y*y*ib",   [   1, 0x72      ], 6, PREF_66 | AUTO_VEXL | VEX_OP | ENC_VM;
] "pslldq"      = [ "yoib",     [0x0F, 0x73      ], 7, PREF_66;
] "vpslldq"     = [ "y*y*ib",   [   1, 0x73      ], 7, PREF_66 | AUTO_VEXL | VEX_OP | ENC_VM;
] "vpsllq"      = [ "y*y*wo",   [   1, 0xF3      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "y*y*ib",   [   1, 0x73      ], 6, PREF_66 | AUTO_VEXL | VEX_OP | ENC_VM;
] "vpsllw"      = [ "y*y*wo",   [   1, 0xF1      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "y*y*ib",   [   1, 0x71      ], 6, PREF_66 | AUTO_VEXL | VEX_OP | ENC_VM;
] "vpsrad"      = [ "y*y*wo",   [   1, 0xE2      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "y*y*ib",   [   1, 0x72      ], 4, PREF_66 | AUTO_VEXL | VEX_OP | ENC_VM;
] "vpsraw"      = [ "y*y*wo",   [   1, 0xE1      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "y*y*ib",   [   1, 0x71      ], 4, PREF_66 | AUTO_VEXL | VEX_OP | ENC_VM;
] "vpsrld"      = [ "y*y*wo",   [   1, 0xD2      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "y*y*ib",   [   1, 0x72      ], 2, PREF_66 | AUTO_VEXL | VEX_OP;
] "psrldq"      = [ "yoib",     [0x0F, 0x73      ], 3, PREF_66;
] "vpsrldq"     = [ "y*y*ib",   [   1, 0x73      ], 3, PREF_66 | AUTO_VEXL | VEX_OP | ENC_VM;
] "vpsrlq"      = [ "y*y*wo",   [   1, 0xD3      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "y*y*ib",   [   1, 0x73      ], 2, PREF_66 | AUTO_VEXL | VEX_OP | ENC_VM;
] "vpsrlw"      = [ "y*y*wo",   [   1, 0xD1      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "y*y*ib",   [   1, 0x71      ], 2, PREF_66 | AUTO_VEXL | VEX_OP | ENC_VM;
] // legacy padd forms are in the MMX section
  "vpsubb"      = [ "y*y*w*",   [   1, 0xF8      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpsubd"      = [ "y*y*w*",   [   1, 0xFA      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpsubq"      = [ "y*y*w*",   [   1, 0xFB      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpsubsb"     = [ "y*y*w*",   [   1, 0xE8      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpsubsw"     = [ "y*y*w*",   [   1, 0xE9      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpsubusb"    = [ "y*y*w*",   [   1, 0xD8      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpsubusw"    = [ "y*y*w*",   [   1, 0xD9      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpsubw"      = [ "y*y*w*",   [   1, 0xF9      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "ptest"       = [ "yowo",     [0x0F, 0x38, 0x17], X, PREF_66;
] "vptest"      = [ "y*w*",     [   2, 0x17      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // legacy punpck forms too
  "vpunpckhbw"  = [ "y*y*w*",   [   1, 0x68      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpunpckhdq"  = [ "y*y*w*",   [   1, 0x6A      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "punpckhqdq"  = [ "yowo",     [0x0F, 0x6D      ], X, PREF_66;
] "vpunpckhqdq" = [ "y*y*w*",   [   1, 0x6D      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpunpckhwd"  = [ "y*y*w*",   [   1, 0x69      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpunpcklbw"  = [ "y*y*w*",   [   1, 0x60      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpunpckldq"  = [ "y*y*w*",   [   1, 0x62      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "punpcklqdq"  = [ "yowo",     [0x0F, 0x6C      ], X, PREF_66;
] "vpunpcklqdq" = [ "y*y*w*",   [   1, 0x6C      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpunpcklwd"  = [ "y*y*w*",   [   1, 0x61      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] // pxor is in the MMX section too
  "vpxor"       = [ "y*y*w*",   [   1, 0xEF      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "rcpps"       = [ "yowo",     [0x0F, 0x53      ], X;
] "vrcpps"      = [ "y*w*",     [   1, 0x53      ], X,           AUTO_VEXL | VEX_OP;
] "rcpss"       = [ "yowo",     [0x0F, 0x53      ], X, PREF_F3;
] "vrcpss"      = [ "y*y*w*",   [   1, 0x53      ], X, PREF_F3 | AUTO_VEXL | VEX_OP;
] "roundpd"     = [ "yowoib",   [0x0F, 0x3A, 0x09], X, PREF_66;
] "vroundpd"    = [ "y*w*ib",   [   3, 0x09      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "roundps"     = [ "yowoib",   [0x0F, 0x3A, 0x08], X, PREF_66;
] "vroundps"    = [ "y*w*ib",   [   3, 0x08      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "roundsd"     = [ "yoyoib",   [0x0F, 0x3A, 0x0B], X, PREF_66;
                    "yomqib",   [0x0F, 0x3A, 0x0B], X, PREF_66;
] "vroundsd"    = [ "yoyoyoib", [   3, 0x0B      ], X, PREF_66             | VEX_OP;
                    "yoyomqib", [   3, 0x0B      ], X, PREF_66             | VEX_OP;
] "roundss"     = [ "yoyoib",   [0x0F, 0x3A, 0x0A], X, PREF_66;
                    "yomqib",   [0x0F, 0x3A, 0x0A], X, PREF_66;
] "vroundss"    = [ "yoyoyoib", [   3, 0x0A      ], X, PREF_66             | VEX_OP;
                    "yoyomqib", [   3, 0x0A      ], X, PREF_66             | VEX_OP;
] "rsqrtps"     = [ "yowo",     [0x0F, 0x52      ], X;
] "vrsqrtps"    = [ "y*w*",     [   1, 0x52      ], X,           AUTO_VEXL | VEX_OP;
] "rsqrtss"     = [ "yoyo",     [0x0F, 0x52      ], X, PREF_F3;
                    "yomd",     [0x0F, 0x52      ], X, PREF_F3;
] "vrsqrtss"    = [ "yoyoyo",   [   1, 0x52      ], X, PREF_F3 | AUTO_VEXL | VEX_OP;
                    "yoyomd",   [   1, 0x52      ], X, PREF_F3 | AUTO_VEXL | VEX_OP;
] "shufpd"      = [ "yowoib",   [0x0F, 0xC6      ], X, PREF_66;
] "vshufpd"     = [ "y*y*w*ib", [   1, 0xC6      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "shufps"      = [ "yowoib",   [0x0F, 0xC6      ], X;
] "vshufps"     = [ "y*y*w*ib", [   1, 0xC6      ], X,           AUTO_VEXL | VEX_OP;
] "sqrtpd"      = [ "yowo",     [0x0F, 0x51      ], X, PREF_66;
] "vsqrtpd"     = [ "y*w*",     [   1, 0x51      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "sqrtps"      = [ "yowo",     [0x0F, 0x51      ], X;
] "vsqrtps"     = [ "y*w*",     [   1, 0x51      ], X,           AUTO_VEXL | VEX_OP;
] "sqrtsd"      = [ "yoyo",     [0x0F, 0x51      ], X, PREF_F2;
                    "yomq",     [0x0F, 0x51      ], X, PREF_F2;
] "vsqrtsd"     = [ "yoyoyo",   [   1, 0x51      ], X, PREF_F2             | VEX_OP;
                    "yoyomq",   [   1, 0x51      ], X, PREF_F2             | VEX_OP;
] "sqrtss"      = [ "yoyo",     [0x0F, 0x51      ], X, PREF_F3;
                    "yomd",     [0x0F, 0x51      ], X, PREF_F3;
] "vsqrtss"     = [ "yoyoyo",   [   1, 0x51      ], X, PREF_F3             | VEX_OP;
                    "yoyomd",   [   1, 0x51      ], X, PREF_F3             | VEX_OP;
] "stmxcsr"     = [ "md",       [0x0F, 0xAE      ], 3;
] "vstmxcsr"    = [ "md",       [   1, 0xAE      ], 3;
] "subpd"       = [ "yowo",     [0x0F, 0x5C      ], X, PREF_66;
] "vsubpd"      = [ "y*y*w*",   [   1, 0x5C      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "subps"       = [ "yowo",     [0x0F, 0x5C      ], X;
] "vsubps"      = [ "y*y*w*",   [   1, 0x5C      ], X,           AUTO_VEXL | VEX_OP;
] "subsd"       = [ "yoyo",     [0x0F, 0x5C      ], X, PREF_F2;
                    "yomq",     [0x0F, 0x5C      ], X, PREF_F2;
] "vsubsd"      = [ "yoyoyo",   [   1, 0x5C      ], X, PREF_F2             | VEX_OP;
                    "yoyomq",   [   1, 0x5C      ], X, PREF_F2             | VEX_OP;
] "subss"       = [ "yoyo",     [0x0F, 0x5C      ], X, PREF_F3;
                    "yomd",     [0x0F, 0x5C      ], X, PREF_F3;
] "vsubss"      = [ "yoyoyo",   [   1, 0x5C      ], X, PREF_F3             | VEX_OP;
                    "yoyomd",   [   1, 0x5C      ], X, PREF_F3             | VEX_OP;
] "ucomisd"     = [ "yoyo",     [0x0F, 0x2E      ], X, PREF_66;
                    "yomq",     [0x0F, 0x2E      ], X, PREF_66;
] "vucomisd"    = [ "yoyoyo",   [   1, 0x2E      ], X, PREF_66             | VEX_OP;
                    "yoyomq",   [   1, 0x2E      ], X, PREF_66             | VEX_OP;
] "ucomiss"     = [ "yoyo",     [0x0F, 0x2E      ], X;
                    "yomd",     [0x0F, 0x2E      ], X;
] "vucomiss"    = [ "yoyoyo",   [   1, 0x2E      ], X,                       VEX_OP;
                    "yoyomd",   [   1, 0x2E      ], X,                       VEX_OP;
] "unpckhpd"    = [ "yowo",     [0x0F, 0x15      ], X, PREF_66;
] "vunpckhpd"   = [ "y*y*w*",   [   1, 0x15      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "unpckhps"    = [ "yowo",     [0x0F, 0x15      ], X;
] "vunpckhps"   = [ "y*y*w*",   [   1, 0x15      ], X,           AUTO_VEXL | VEX_OP;
] "unpcklpd"    = [ "yowo",     [0x0F, 0x14      ], X, PREF_66;
] "vunpcklpd"   = [ "y*y*w*",   [   1, 0x14      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "unpcklps"    = [ "yowo",     [0x0F, 0x14      ], X;
] "vunpcklps"   = [ "y*y*w*",   [   1, 0x14      ], X,           AUTO_VEXL | VEX_OP;
] // vex only operand forms
  "vbroadcastf128"
                = [ "yhmo",     [   2, 0x1A      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "vbroadcasti128"
                = [ "yhmo",     [   2, 0x5A      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "vbroadcastsd"= [ "yhyo",     [   2, 0x19      ], X, PREF_66 | WITH_VEXL | VEX_OP;
                    "yhmq",     [   2, 0x19      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "vbroadcastss"= [ "y*yo",     [   2, 0x18      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "y*md",     [   2, 0x18      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vcvtph2ps"   = [ "y*yo",     [   2, 0x13      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "yomq",     [   2, 0x13      ], X, PREF_66             | VEX_OP;
                    "yhmo",     [   2, 0x13      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "vcvtps2ph"   = [ "yoy*ib",   [   3, 0x1D      ], X, PREF_66 | AUTO_VEXL | VEX_OP | ENC_MR;
                    "mqyoib",   [   3, 0x1D      ], X, PREF_66             | VEX_OP;
                    "moyhib",   [   3, 0x1D      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "vextractf128"= [ "woyhib",   [   3, 0x19      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "vextracti128"= [ "woyhib",   [   3, 0x39      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "vfmaddpd"    = [ "y*y*w*y*", [   3, 0x69      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "y*y*y*w*", [   3, 0x69      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmadd132pd" = [ "y*y*w*",   [   2, 0x98      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmadd213pd" = [ "y*y*w*",   [   2, 0xA8      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmadd231pd" = [ "y*y*w*",   [   2, 0xB8      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmaddps"    = [ "y*y*w*y*", [   3, 0x68      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "y*y*y*w*", [   3, 0x68      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmadd132ps" = [ "y*y*w*",   [   2, 0x98      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfmadd213ps" = [ "y*y*w*",   [   2, 0xA8      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfmadd231ps" = [ "y*y*w*",   [   2, 0xB8      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfmaddsd"    = [ "yoyoyoyo", [   3, 0x6B      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    "yoyoyomq", [   3, 0x6B      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    "yoyomqyo", [   3, 0x6B      ], X, PREF_66             | VEX_OP;
] "vfmadd132sd" = [ "yoyoyo",   [   2, 0x99      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    "yoyomq",   [   2, 0x99      ], X, PREF_66             | VEX_OP | WITH_REXW;
] "vfmadd213sd" = [ "yoyoyo",   [   2, 0xA9      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    "yoyomq",   [   2, 0xA9      ], X, PREF_66             | VEX_OP | WITH_REXW;
] "vfmadd231sd" = [ "yoyoyo",   [   2, 0xB9      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    "yoyomq",   [   2, 0xB9      ], X, PREF_66             | VEX_OP | WITH_REXW;
] "vfmaddss"    = [ "yoyoyoyo", [   3, 0x6A      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    "yoyoyomq", [   3, 0x6A      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    "yoyomqyo", [   3, 0x6A      ], X, PREF_66             | VEX_OP;
] "vfmadd132ss" = [ "yoyoyo",   [   2, 0x99      ], X, PREF_66             | VEX_OP;
                    "yoyomq",   [   2, 0x99      ], X, PREF_66             | VEX_OP;
] "vfmadd213ss" = [ "yoyoyo",   [   2, 0xA9      ], X, PREF_66             | VEX_OP;
                    "yoyomq",   [   2, 0xA9      ], X, PREF_66             | VEX_OP;
] "vfmadd231ss" = [ "yoyoyo",   [   2, 0xB9      ], X, PREF_66             | VEX_OP;
                    "yoyomq",   [   2, 0xB9      ], X, PREF_66             | VEX_OP;
] "vfmaddsuppd"   =["y*y*w*y*", [   3, 0x5D      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "y*y*y*w*", [   3, 0x5D      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmaddsub132pd"=["y*y*w*",   [   2, 0x96      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmaddsub213pd"=["y*y*w*",   [   2, 0xA6      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmaddsub231pd"=["y*y*w*",   [   2, 0xB6      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmaddsubps"   =["y*y*w*y*", [   3, 0x5C      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "y*y*y*w*", [   3, 0x5C      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmaddsub132ps"=["y*y*w*",   [   2, 0x96      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfmaddsub213ps"=["y*y*w*",   [   2, 0xA6      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfmaddsub231ps"=["y*y*w*",   [   2, 0xB6      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfmsubaddpd"   =["y*y*w*y*", [   3, 0x5F      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "y*y*y*w*", [   3, 0x5F      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmsubadd132pd"=["y*y*w*",   [   2, 0x97      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmsubadd213pd"=["y*y*w*",   [   2, 0xA7      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmsubadd231pd"=["y*y*w*",   [   2, 0xB7      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmsubaddps"   =["y*y*w*y*", [   3, 0x5E      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "y*y*y*w*", [   3, 0x5E      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmsubadd132ps"=["y*y*w*",   [   2, 0x97      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfmsubadd213ps"=["y*y*w*",   [   2, 0xA7      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfmsubadd231ps"=["y*y*w*",   [   2, 0xB7      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfmsubpd"    = [ "y*y*w*y*", [   3, 0x6D      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "y*y*y*w*", [   3, 0x6D      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmsub132pd" = [ "y*y*w*",   [   2, 0x9A      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmsub213pd" = [ "y*y*w*",   [   2, 0xAA      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmsub231pd" = [ "y*y*w*",   [   2, 0xBA      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmsubps"    = [ "y*y*w*y*", [   3, 0x6C      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "y*y*y*w*", [   3, 0x6C      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfmsub132ps" = [ "y*y*w*",   [   2, 0x9A      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfmsub213ps" = [ "y*y*w*",   [   2, 0xAA      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfmsub231ps" = [ "y*y*w*",   [   2, 0xBA      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfmsubsd"    = [ "yoyoyoyo", [   3, 0x6F      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    "yoyoyomq", [   3, 0x6F      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    "yoyomqyo", [   3, 0x6F      ], X, PREF_66             | VEX_OP;
] "vfmsub132sd" = [ "yoyoyo",   [   2, 0x9B      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    "yoyomq",   [   2, 0x9B      ], X, PREF_66             | VEX_OP | WITH_REXW;
] "vfmsub213sd" = [ "yoyoyo",   [   2, 0xAB      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    "yoyomq",   [   2, 0xAB      ], X, PREF_66             | VEX_OP | WITH_REXW;
] "vfmsub231sd" = [ "yoyoyo",   [   2, 0xBB      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    "yoyomq",   [   2, 0xBB      ], X, PREF_66             | VEX_OP | WITH_REXW;
] "vfmsubss"    = [ "yoyoyoyo", [   3, 0x6E      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    "yoyoyomq", [   3, 0x6E      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    "yoyomqyo", [   3, 0x6E      ], X, PREF_66             | VEX_OP;
] "vfmsub132ss" = [ "yoyoyo",   [   2, 0x9B      ], X, PREF_66             | VEX_OP;
                    "yoyomq",   [   2, 0x9B      ], X, PREF_66             | VEX_OP;
] "vfmsub213ss" = [ "yoyoyo",   [   2, 0xAB      ], X, PREF_66             | VEX_OP;
                    "yoyomq",   [   2, 0xAB      ], X, PREF_66             | VEX_OP;
] "vfmsub231ss" = [ "yoyoyo",   [   2, 0xBB      ], X, PREF_66             | VEX_OP;
                    "yoyomq",   [   2, 0xBB      ], X, PREF_66             | VEX_OP;
] "vfnmaddpd"   = [ "y*y*w*y*", [   3, 0x79      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "y*y*y*w*", [   3, 0x79      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfnmadd132pd"= [ "y*y*w*",   [   2, 0x9C      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfnmadd213pd"= [ "y*y*w*",   [   2, 0xAC      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfnmadd231pd"= [ "y*y*w*",   [   2, 0xBC      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfnmaddps"   = [ "y*y*w*y*", [   3, 0x78      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "y*y*y*w*", [   3, 0x78      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfnmadd132ps"= [ "y*y*w*",   [   2, 0x9C      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfnmadd213ps"= [ "y*y*w*",   [   2, 0xAC      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfnmadd231ps"= [ "y*y*w*",   [   2, 0xBC      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfnmaddsd"   = [ "yoyoyoyo", [   3, 0x7B      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    "yoyoyomq", [   3, 0x7B      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    "yoyomqyo", [   3, 0x7B      ], X, PREF_66             | VEX_OP;
] "vfnmadd132sd"= [ "yoyoyo",   [   2, 0x9D      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    "yoyomq",   [   2, 0x9D      ], X, PREF_66             | VEX_OP | WITH_REXW;
] "vfnmadd213sd"= [ "yoyoyo",   [   2, 0xAD      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    "yoyomq",   [   2, 0xAD      ], X, PREF_66             | VEX_OP | WITH_REXW;
] "vfnmadd231sd"= [ "yoyoyo",   [   2, 0xBD      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    "yoyomq",   [   2, 0xBD      ], X, PREF_66             | VEX_OP | WITH_REXW;
] "vfnmaddss"   = [ "yoyoyoyo", [   3, 0x7A      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    "yoyoyomq", [   3, 0x7A      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    "yoyomqyo", [   3, 0x7A      ], X, PREF_66             | VEX_OP;
] "vfnmadd132ss"= [ "yoyoyo",   [   2, 0x9D      ], X, PREF_66             | VEX_OP;
                    "yoyomq",   [   2, 0x9D      ], X, PREF_66             | VEX_OP;
] "vfnmadd213ss"= [ "yoyoyo",   [   2, 0xAD      ], X, PREF_66             | VEX_OP;
                    "yoyomq",   [   2, 0xAD      ], X, PREF_66             | VEX_OP;
] "vfnmadd231ss"= [ "yoyoyo",   [   2, 0xBD      ], X, PREF_66             | VEX_OP;
                    "yoyomq",   [   2, 0xBD      ], X, PREF_66             | VEX_OP;
] "vfnmsubpd"   = [ "y*y*w*y*", [   3, 0x7D      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "y*y*y*w*", [   3, 0x7D      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfnmsub132pd"= [ "y*y*w*",   [   2, 0x9E      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfnmsub213pd"= [ "y*y*w*",   [   2, 0xAE      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfnmsub231pd"= [ "y*y*w*",   [   2, 0xBE      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfnmsubps"   = [ "y*y*w*y*", [   3, 0x7C      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "y*y*y*w*", [   3, 0x7C      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vfnmsub132ps"= [ "y*y*w*",   [   2, 0x9E      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfnmsub213ps"= [ "y*y*w*",   [   2, 0xAE      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfnmsub231ps"= [ "y*y*w*",   [   2, 0xBE      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vfnmsubsd"   = [ "yoyoyoyo", [   3, 0x7F      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    "yoyoyomq", [   3, 0x7F      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    "yoyomqyo", [   3, 0x7F      ], X, PREF_66             | VEX_OP;
] "vfnmsub132sd"= [ "yoyoyo",   [   2, 0x9F      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    "yoyomq",   [   2, 0x9F      ], X, PREF_66             | VEX_OP | WITH_REXW;
] "vfnmsub213sd"= [ "yoyoyo",   [   2, 0xAF      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    "yoyomq",   [   2, 0xAF      ], X, PREF_66             | VEX_OP | WITH_REXW;
] "vfnmsub231sd"= [ "yoyoyo",   [   2, 0xBF      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    "yoyomq",   [   2, 0xBF      ], X, PREF_66             | VEX_OP | WITH_REXW;
] "vfnmsubss"   = [ "yoyoyoyo", [   3, 0x7E      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    "yoyoyomq", [   3, 0x7E      ], X, PREF_66             | VEX_OP | WITH_REXW;
                    "yoyomqyo", [   3, 0x7E      ], X, PREF_66             | VEX_OP;
] "vfnmsub132ss"= [ "yoyoyo",   [   2, 0x9F      ], X, PREF_66             | VEX_OP;
                    "yoyomq",   [   2, 0x9F      ], X, PREF_66             | VEX_OP;
] "vfnmsub213ss"= [ "yoyoyo",   [   2, 0xAF      ], X, PREF_66             | VEX_OP;
                    "yoyomq",   [   2, 0xAF      ], X, PREF_66             | VEX_OP;
] "vfnmsub231ss"= [ "yoyoyo",   [   2, 0xBF      ], X, PREF_66             | VEX_OP;
                    "yoyomq",   [   2, 0xBF      ], X, PREF_66             | VEX_OP;
] "vfrczpd"     = [ "y*w*",     [   9, 0x81      ], X,           AUTO_VEXL | XOP_OP;
] "vfrczps"     = [ "y*w*",     [   9, 0x80      ], X,           AUTO_VEXL | XOP_OP;
] "vfrczsd"     = [ "yoyo",     [   9, 0x83      ], X,                       XOP_OP;
                    "yomq",     [   9, 0x83      ], X,                       XOP_OP;
] "vfrczss"     = [ "yoyo",     [   9, 0x82      ], X,                       XOP_OP;
                    "yomd",     [   9, 0x82      ], X,                       XOP_OP;
] "vgatherdpd"  = [ "y*koy*",   [   2, 0x92      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vgatherdps"  = [ "y*k*y*",   [   2, 0x92      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vgatherqpd"  = [ "y*l*y*",   [   2, 0x93      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vgatherqps"  = [ "yol*yo",   [   2, 0x93      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vinsertf128" = [ "yhyhwoib", [   3, 0x18      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "vinserti128" = [ "yhyhwoib", [   3, 0x38      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "vmaskmovpd"  = [ "y*y*m*",   [   2, 0x2D      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "m*y*y*",   [   2, 0x2F      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vmaskmovps"  = [ "y*y*m*",   [   2, 0x2C      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "m*y*y*",   [   2, 0x2E      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpblendd"    = [ "y*y*w*ib", [   3, 0x02      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpbroadcastb"= [ "y*yo",     [   2, 0x78      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "y*mb",     [   2, 0x78      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpbroadcastd"= [ "y*yo",     [   2, 0x58      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "y*md",     [   2, 0x58      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpbroadcastq"= [ "y*yo",     [   2, 0x59      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "y*mq",     [   2, 0x59      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpbroadcastw"= [ "y*yo",     [   2, 0x79      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "y*mw",     [   2, 0x79      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpcmov"      = [ "y*y*w*y*", [   8, 0xA2      ], X,           AUTO_VEXL | XOP_OP;
                    "y*y*y*w*", [   8, 0xA2      ], X,           AUTO_VEXL | XOP_OP | WITH_REXW;
] "vpcomb"      = [ "yoyowoib", [   8, 0xCC      ], X,                       XOP_OP;
] "vpcomd"      = [ "yoyowoib", [   8, 0xCE      ], X,                       XOP_OP;
] "vpcomq"      = [ "yoyowoib", [   8, 0xCF      ], X,                       XOP_OP;
] "vpcomub"     = [ "yoyowoib", [   8, 0xEC      ], X,                       XOP_OP;
] "vpcomud"     = [ "yoyowoib", [   8, 0xEE      ], X,                       XOP_OP;
] "vpcomuq"     = [ "yoyowoib", [   8, 0xEF      ], X,                       XOP_OP;
] "vpcomuw"     = [ "yoyowoib", [   8, 0xED      ], X,                       XOP_OP;
] "vpcomw"      = [ "yoyowoib", [   8, 0xCD      ], X,                       XOP_OP;
] "vperm2f128"  = [ "yhyhwhib", [   3, 0x06      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "vperm2i128"  = [ "yhyhwhib", [   3, 0x46      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "vpermd"      = [ "yhyhwh",   [   3, 0x36      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "vpermil2pd"  = [ "y*y*w*y*ib",[  3, 0x49      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "y*y*y*w*ib",[  3, 0x49      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vpermil2pS"  = [ "y*y*w*y*ib",[  3, 0x48      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "y*y*y*w*ib",[  3, 0x48      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vpermilpd"   = [ "y*y*w*",   [   2, 0x0D      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "y*w*ib",   [   3, 0x05      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpermilps"   = [ "y*y*w*",   [   2, 0x0C      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
                    "y*w*ib",   [   3, 0x04      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpermpd"     = [ "yhwhib",   [   3, 0x01      ], X, PREF_66 | WITH_VEXL | VEX_OP | WITH_REXW;
] "vpermps"     = [ "yhyhwh",   [   2, 0x01      ], X, PREF_66 | WITH_VEXL | VEX_OP;
] "vpermq"      = [ "yhwhib",   [   3, 0x00      ], X, PREF_66 | WITH_VEXL | VEX_OP | WITH_REXW;
] "vpgatherdd"  = [ "y*k*y*",   [   2, 0x90      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpgatherdq"  = [ "y*koy*",   [   2, 0x90      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vpgatherqd"  = [ "yok*yo",   [   2, 0x91      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpgatherqq"  = [ "y*k*y*",   [   2, 0x91      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vphaddbd"    = [ "yowo",     [   9, 0xC2      ], X,                       XOP_OP;
] "vphaddbq"    = [ "yowo",     [   9, 0xC3      ], X,                       XOP_OP;
] "vphaddbw"    = [ "yowo",     [   9, 0xC1      ], X,                       XOP_OP;
] "vphadddq"    = [ "yowo",     [   9, 0xCB      ], X,                       XOP_OP;
] "vphaddubd"   = [ "yowo",     [   9, 0xD2      ], X,                       XOP_OP;
] "vphaddubq"   = [ "yowo",     [   9, 0xD3      ], X,                       XOP_OP;
] "vphaddubw"   = [ "yowo",     [   9, 0xD1      ], X,                       XOP_OP;
] "vphaddudq"   = [ "yowo",     [   9, 0xDB      ], X,                       XOP_OP;
] "vphadduwd"   = [ "yowo",     [   9, 0xD6      ], X,                       XOP_OP;
] "vphadduwq"   = [ "yowo",     [   9, 0xD7      ], X,                       XOP_OP;
] "vphaddwd"    = [ "yowo",     [   9, 0xC6      ], X,                       XOP_OP;
] "vphaddwq"    = [ "yowo",     [   9, 0xC7      ], X,                       XOP_OP;
] "vphsubbw"    = [ "yowo",     [   9, 0xE1      ], X,                       XOP_OP;
] "vphsubdq"    = [ "yowo",     [   9, 0xE3      ], X,                       XOP_OP;
] "vphsubwd"    = [ "yowo",     [   9, 0xE2      ], X,                       XOP_OP;
] "vpmacsdd"    = [ "yoyowoyo", [   8, 0x9E      ], X,                       XOP_OP;
] "vpmacsdqh"   = [ "yoyowoyo", [   8, 0x9F      ], X,                       XOP_OP;
] "vpmacsdql"   = [ "yoyowoyo", [   8, 0x97      ], X,                       XOP_OP;
] "vpmacssdd"   = [ "yoyowoyo", [   8, 0x8E      ], X,                       XOP_OP;
] "vpmacssdqh"  = [ "yoyowoyo", [   8, 0x8F      ], X,                       XOP_OP;
] "vpmacssdql"  = [ "yoyowoyo", [   8, 0x87      ], X,                       XOP_OP;
] "vpmacsswd"   = [ "yoyowoyo", [   8, 0x86      ], X,                       XOP_OP;
] "vpmacssww"   = [ "yoyowoyo", [   8, 0x85      ], X,                       XOP_OP;
] "vpmacswd"    = [ "yoyowoyo", [   8, 0x96      ], X,                       XOP_OP;
] "vpmacsww"    = [ "yoyowoyo", [   8, 0x95      ], X,                       XOP_OP;
] "vpmadcsswd"  = [ "yoyowoyo", [   8, 0xA6      ], X,                       XOP_OP;
] "vpmadcswd"   = [ "yoyowoyo", [   8, 0xB6      ], X,                       XOP_OP;
] "vpmaskmovd"  = [ "y*y*m*",   [   2, 0x8C      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
                    "m*y*y*",   [   2, 0x8E      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vpmaskmovq"  = [ "y*y*m*",   [   2, 0x8C      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
                    "m*y*y*",   [   2, 0x8E      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vpperm"      = [ "yoyowoyo", [   8, 0xA3      ], X,                       XOP_OP;
                    "yoyoyowo", [   8, 0xA3      ], X,                       XOP_OP | WITH_REXW;
] "vprotb"      = [ "yowoyo",   [   9, 0x90      ], X,                       XOP_OP;
                    "yoyowo",   [   9, 0x90      ], X,                       XOP_OP | WITH_REXW;
                    "yowoib",   [   8, 0xC0      ], X,                       XOP_OP;
] "vprotd"      = [ "yowoyo",   [   9, 0x92      ], X,                       XOP_OP;
                    "yoyowo",   [   9, 0x92      ], X,                       XOP_OP | WITH_REXW;
                    "yowoib",   [   8, 0xC2      ], X,                       XOP_OP;
] "vprotq"      = [ "yowoyo",   [   9, 0x93      ], X,                       XOP_OP;
                    "yoyowo",   [   9, 0x93      ], X,                       XOP_OP | WITH_REXW;
                    "yowoib",   [   8, 0xC3      ], X,                       XOP_OP;
] "vprotw"      = [ "yowoyo",   [   9, 0x91      ], X,                       XOP_OP;
                    "yoyowo",   [   9, 0x91      ], X,                       XOP_OP | WITH_REXW;
                    "yowoib",   [   8, 0xC1      ], X,                       XOP_OP;
] "vpshab"      = [ "yowoyo",   [   9, 0x98      ], X,                       XOP_OP;
                    "yoyowo",   [   9, 0x98      ], X,                       XOP_OP | WITH_REXW;
] "vpshad"      = [ "yowoyo",   [   9, 0x9A      ], X,                       XOP_OP;
                    "yoyowo",   [   9, 0x9A      ], X,                       XOP_OP | WITH_REXW;
] "vpshaq"      = [ "yowoyo",   [   9, 0x9B      ], X,                       XOP_OP;
                    "yoyowo",   [   9, 0x9B      ], X,                       XOP_OP | WITH_REXW;
] "vpshaw"      = [ "yowoyo",   [   9, 0x99      ], X,                       XOP_OP;
                    "yoyowo",   [   9, 0x99      ], X,                       XOP_OP | WITH_REXW;
] "vpshlb"      = [ "yowoyo",   [   9, 0x94      ], X,                       XOP_OP;
                    "yoyowo",   [   9, 0x94      ], X,                       XOP_OP | WITH_REXW;
] "vpshld"      = [ "yowoyo",   [   9, 0x96      ], X,                       XOP_OP;
                    "yoyowo",   [   9, 0x96      ], X,                       XOP_OP | WITH_REXW;
] "vpshlq"      = [ "yowoyo",   [   9, 0x97      ], X,                       XOP_OP;
                    "yoyowo",   [   9, 0x97      ], X,                       XOP_OP | WITH_REXW;
] "vpshlw"      = [ "yowoyo",   [   9, 0x95      ], X,                       XOP_OP;
                    "yoyowo",   [   9, 0x95      ], X,                       XOP_OP | WITH_REXW;
] "vpsllvd"     = [ "y*y*w*",   [   2, 0x47      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpsllvq"     = [ "y*y*w*",   [   2, 0x47      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vpsravd"     = [ "y*y*w*",   [   2, 0x46      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpsrlvd"     = [ "y*y*w*",   [   2, 0x45      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vpsrlvq"     = [ "y*y*w*",   [   2, 0x45      ], X, PREF_66 | AUTO_VEXL | VEX_OP | WITH_REXW;
] "vtestpd"     = [ "y*w*",     [   2, 0x0F      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vtestps"     = [ "y*w*",     [   2, 0x0E      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "vzeroall"    = [ "",         [   1, 0x77      ], X,           WITH_VEXL | VEX_OP;
] "vzeroupper"  = [ "",         [   1, 0x77      ], X,                       VEX_OP;
] "xgetbv"      = [ "",         [0x0F, 0x01, 0xD0], X;
] // Do not ask me why there are separate mnemnonics for single and double precision float xors. This
  // is a bitwise operation, it doesn't care about the bitwidth. Why does this even operate on floats
  // to begin with.
  "xorpd"       = [ "yowo",     [0x0F, 0x57      ], X, PREF_66;
] "vxorpd"      = [ "y*y*w*",   [   1, 0x57      ], X, PREF_66 | AUTO_VEXL | VEX_OP;
] "xorps"       = [ "yowo",     [0x0F, 0x57      ], X;
] "vxorps"      = [ "y*y*w*",   [   1, 0x57      ], X,           AUTO_VEXL | VEX_OP;
] "xrstor"      = [ "m!",       [0x0F, 0xAE      ], 5;
] "xsave"       = [ "m!",       [0x0F, 0xAE      ], 4;
] "xsaveopt"    = [ "m!",       [0x0F, 0xAE      ], 6;
] "xsetbv"      = [ "",         [0x0F, 0x01, 0xD1], X;
] // and we're done. well, until intel's new extensions get more use
);
