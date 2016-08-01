use compiler::Opdata;
use compiler::flags::*;

macro_rules! constify {
    ($t:ty, $e:expr) => { {const C: &'static $t = &$e; C} }
}

macro_rules! OpInner {
    ($fmt:expr, $ops:expr, $reg:expr)          => { Opdata {args: $fmt, ops: constify!([u8], $ops), reg: $reg, flags:  0}  };
    ($fmt:expr, $ops:expr, $reg:expr, $f:expr) => { Opdata {args: $fmt, ops: constify!([u8], $ops), reg: $reg, flags: $f}  };
}

macro_rules! Ops {
    ($search:expr, $notfound:expr, {
        $( $name:pat $(| $more:pat)* = [ $( $( $e:expr ),+ ; )+ ] )*
    } ) => { match $search {
        $( $name $(| $more)* => {const C: &'static [Opdata] = &[$( OpInner!($( $e ),*) ,)+]; C}, )+
        _ => $notfound
    } };
}

pub fn get_mnemnonic_data(name: &str) -> Option<&'static [Opdata]> {
    // note: currently only listing instructions that are usable in long mode, with 64-bit addrs, no VEX/XOP prefixes or segment overrides, without requiring privileges, that are not an extension
    // this helps preserve my sanity for now

    // marking for when the reg replacement isn't used
    const X: u8 = 0xFF;

    // I blame intel for the following match
    Some(Ops!(name, return None, {
// general purpose instructions according to AMD's AMD64 Arch Programmer's Manual Vol. 3
  "adc"         = [ "A*i*",   [0x15            ], X, AUTO_SIZE;
                    "Abib",   [0x14            ], X;
                    "v*i*",   [0x81            ], 2, AUTO_SIZE | LOCK;
                    "v*ib",   [0x83            ], 2, AUTO_SIZE | LOCK;
                    "vbib",   [0x80            ], 2,             LOCK;
                    "v*r*",   [0x11            ], X, AUTO_SIZE | LOCK;
                    "vbrb",   [0x10            ], X,             LOCK;
                    "r*v*",   [0x13            ], X, AUTO_SIZE;
                    "rbvb",   [0x12            ], X;
] "add"         = [ "A*i*",   [0x05            ], X, AUTO_SIZE;
                    "Abib",   [0x04            ], X;
                    "v*i*",   [0x81            ], 0, AUTO_SIZE | LOCK;
                    "v*ib",   [0x83            ], 0, AUTO_SIZE | LOCK;
                    "vbib",   [0x80            ], 0,             LOCK;
                    "v*r*",   [0x01            ], X, AUTO_SIZE | LOCK;
                    "vbrb",   [0x00            ], X,             LOCK;
                    "r*v*",   [0x03            ], X, AUTO_SIZE;
                    "rbvb",   [0x02            ], X;
] "and"         = [ "A*i*",   [0x25            ], X, AUTO_SIZE;
                    "Abib",   [0x24            ], X;
                    "v*i*",   [0x81            ], 4, AUTO_SIZE | LOCK;
                    "v*ib",   [0x83            ], 4, AUTO_SIZE | LOCK;
                    "vbib",   [0x80            ], 4,             LOCK;
                    "v*r*",   [0x21            ], X, AUTO_SIZE | LOCK;
                    "vbrb",   [0x20            ], X,             LOCK;
                    "r*v*",   [0x23            ], X, AUTO_SIZE;
                    "rbvb",   [0x22            ], X;
] "bsf"         = [ "r*v*",   [0x0F, 0xBC      ], X, AUTO_SIZE;
] "bsr"         = [ "r*v*",   [0x0F, 0xBD      ], X, AUTO_SIZE;
] "bswap"       = [ "rd",     [0x0F, 0xC8      ], 0;
                    "rq",     [0x0F, 0xC8      ], 0, LARGE_SIZE;
] "bt"          = [ "v*r*",   [0x0F, 0xA3      ], X, AUTO_SIZE;
                    "v*ib",   [0x0F, 0xBA      ], 4, AUTO_SIZE;
] "btc"         = [ "v*r*",   [0x0F, 0xBB      ], X, AUTO_SIZE | LOCK;
                    "v*ib",   [0x0F, 0xBA      ], 7, AUTO_SIZE | LOCK;
] "btr"         = [ "v*r*",   [0x0F, 0xB3      ], X, AUTO_SIZE | LOCK;
                    "v*ib",   [0x0F, 0xBA      ], 6, AUTO_SIZE | LOCK;
] "bts"         = [ "v*r*",   [0x0F, 0xAB      ], X, AUTO_SIZE | LOCK;
                    "v*ib",   [0x0F, 0xBA      ], 5, AUTO_SIZE | LOCK;
] "call"        = [ "o*",     [0xE8            ], X, AUTO_SIZE;
                    "r*",     [0xFF            ], 2, AUTO_SIZE;
] "cbw"         = [ "",       [0x98            ], X, SMALL_SIZE;
] "cwde"        = [ "",       [0x98            ], X;
] "cdqe"        = [ "",       [0x98            ], X, LARGE_SIZE;
] "cwd"         = [ "",       [0x99            ], X, SMALL_SIZE;
] "cdq"         = [ "",       [0x99            ], X;
] "cqo"         = [ "",       [0x99            ], X, LARGE_SIZE;
] "clc"         = [ "",       [0xF8            ], X;
] "cld"         = [ "",       [0xFC            ], X;
] "clflush"     = [ "mb",     [0x0F, 0xAE      ], 7;
] "cmc"         = [ "",       [0xF5            ], X;
] "cmovo"       = [ "r*v*",   [0x0F, 0x40      ], X, AUTO_SIZE;
] "cmovno"      = [ "r*v*",   [0x0F, 0x41      ], X, AUTO_SIZE;
] "cmovb"       |
  "cmovc"       |
  "cmovnae"     = [ "r*v*",   [0x0F, 0x42      ], X, AUTO_SIZE;
] "cmovnb"      |
  "cmovnc"      |
  "cmovae"      = [ "r*v*",   [0x0F, 0x43      ], X, AUTO_SIZE;
] "cmovz"       |
  "cmove"       = [ "r*v*",   [0x0F, 0x44      ], X, AUTO_SIZE;
] "cmovnz"      |
  "cmovne"      = [ "r*v*",   [0x0F, 0x45      ], X, AUTO_SIZE;
] "cmovbe"      |
  "cmovna"      = [ "r*v*",   [0x0F, 0x46      ], X, AUTO_SIZE;
] "cmovnbe"     |
  "cmova"       = [ "r*v*",   [0x0F, 0x47      ], X, AUTO_SIZE;
] "cmovs"       = [ "r*v*",   [0x0F, 0x48      ], X, AUTO_SIZE;
] "cmovns"      = [ "r*v*",   [0x0F, 0x49      ], X, AUTO_SIZE;
] "cmovp"       |
  "cmovpe"      = [ "r*v*",   [0x0F, 0x4A      ], X, AUTO_SIZE;
] "cmovnp"      |
  "cmovpo"      = [ "r*v*",   [0x0F, 0x4B      ], X, AUTO_SIZE;
] "cmovl"       |
  "cmovnge"     = [ "r*v*",   [0x0F, 0x4C      ], X, AUTO_SIZE;
] "cmovnl"      |
  "cmovge"      = [ "r*v*",   [0x0F, 0x4D      ], X, AUTO_SIZE;
] "cmovle"      |
  "cmovng"      = [ "r*v*",   [0x0F, 0x4E      ], X, AUTO_SIZE;
] "cmovnle"     |
  "cmovg"       = [ "r*v*",   [0x0F, 0x4F      ], X, AUTO_SIZE;
] "cmp"         = [ "A*i*",   [0x3C            ], X, AUTO_SIZE;
                    "Abib",   [0x3D            ], X;
                    "v*i*",   [0x81            ], 7, AUTO_SIZE;
                    "v*ib",   [0x83            ], 7, AUTO_SIZE;
                    "vbib",   [0x80            ], 7;
                    "v*r*",   [0x39            ], X, AUTO_SIZE;
                    "vbrb",   [0x38            ], X;
                    "r*v*",   [0x3B            ], X, AUTO_SIZE;
                    "rbvb",   [0x3A            ], X;
] "cmpsb"       = [ "",       [0xA6            ], X,              REP;
] "cmpsw"       = [ "",       [0xA7            ], X, SMALL_SIZE | REP;
] "cmpsd"       = [ "",       [0xA7            ], X,              REP;
] "cmpsq"       = [ "",       [0xA7            ], X, LARGE_SIZE | REP;
] "cmpxchg"     = [ "v*r*",   [0x0F, 0xB1      ], X, AUTO_SIZE  | LOCK;
                    "vbrb",   [0x0F, 0xB0      ], X,              LOCK;
] "cmpxchg8b"   = [ "mq",     [0x0F, 0xC7      ], 1,              LOCK;
] "cmpxchg16b"  = [ "mo",     [0x0F, 0xC7      ], 1, LARGE_SIZE | LOCK;
] "cpuid"       = [ "",       [0x0F, 0xA2      ], X;
] "dec"         = [ "v*",     [0xFF            ], 1, AUTO_SIZE  | LOCK;
                    "vb",     [0xFE            ], 1, AUTO_SIZE  | LOCK;
] "div"         = [ "v*",     [0xF7            ], 6, AUTO_SIZE;
                    "vb",     [0xF6            ], 6;
] "enter"       = [ "iwib",   [0xC8            ], X;
] "idiv"        = [ "v*",     [0xF7            ], 7, AUTO_SIZE;
                    "vb",     [0xF6            ], 7;
] "imul"        = [ "v*",     [0xF7            ], 5, AUTO_SIZE;
                    "vb",     [0xF6            ], 5;
                    "r*v*",   [0x0F, 0xAF      ], X, AUTO_SIZE;
                    "r*v*i*", [0x69            ], X, AUTO_SIZE;
                    "r*v*ib", [0x68            ], X, AUTO_SIZE;
] "in"          = [ "Abib",   [0xE4            ], X;
                    "Awib",   [0xE5            ], X, SMALL_SIZE;
                    "Adib",   [0xE5            ], X;
                    "AbCw",   [0xEC            ], X;
                    "AwCw",   [0xED            ], X, SMALL_SIZE;
                    "AdCw",   [0xED            ], X;
] "inc"         = [ "v*",     [0xFF            ], 0, AUTO_SIZE | LOCK;
                    "vb",     [0xFE            ], 0,             LOCK;
] "insb"        = [ "",       [0x6C            ], X;
] "insw"        = [ "",       [0x6D            ], X, SMALL_SIZE;
] "insd"        = [ "",       [0x6D            ], X;
] "int"         = [ "ib",     [0xCD            ], X;
] "jo"          = [ "o*",     [0x0F, 0x80      ], X, AUTO_SIZE;
                    "ob",     [0x70            ], X;
] "jno"         = [ "o*",     [0x0F, 0x81      ], X, AUTO_SIZE;
                    "ob",     [0x71            ], X;
] "jb"          |
  "jc"          |
  "jnae"        = [ "o*",     [0x0F, 0x82      ], X, AUTO_SIZE;
                    "ob",     [0x72            ], X;
] "jnb"         |
  "jnc"         |
  "jae"         = [ "o*",     [0x0F, 0x83      ], X, AUTO_SIZE;
                    "ob",     [0x73            ], X;
] "jz"          |
  "je"          = [ "o*",     [0x0F, 0x84      ], X, AUTO_SIZE;
                    "ob",     [0x74            ], X;
] "jnz"         |
  "jne"         = [ "o*",     [0x0F, 0x85      ], X, AUTO_SIZE;
                    "ob",     [0x75            ], X;
] "jbe"         |
  "jna"         = [ "o*",     [0x0F, 0x86      ], X, AUTO_SIZE;
                    "ob",     [0x76            ], X;
] "jnbe"        |
  "ja"          = [ "o*",     [0x0F, 0x87      ], X, AUTO_SIZE;
                    "ob",     [0x77            ], X;
] "js"          = [ "o*",     [0x0F, 0x88      ], X, AUTO_SIZE;
                    "ob",     [0x78            ], X;
] "jns"         = [ "o*",     [0x0F, 0x89      ], X, AUTO_SIZE;
                    "ob",     [0x79            ], X;
] "jp"          |
  "jpe"         = [ "o*",     [0x0F, 0x8A      ], X, AUTO_SIZE;
                    "ob",     [0x7A            ], X;
] "jnp"         |
  "jpo"         = [ "o*",     [0x0F, 0x8B      ], X, AUTO_SIZE;
                    "ob",     [0x7B            ], X;
] "jl"          |
  "jnge"        = [ "o*",     [0x0F, 0x8C      ], X, AUTO_SIZE;
                    "ob",     [0x7C            ], X;
] "jnl"         |
  "jge"         = [ "o*",     [0x0F, 0x8D      ], X, AUTO_SIZE;
                    "ob",     [0x7D            ], X;
] "jle"         |
  "jng"         = [ "o*",     [0x0F, 0x8E      ], X, AUTO_SIZE;
                    "ob",     [0x7E            ], X;
] "jnle"        |
  "jg"          = [ "o*",     [0x0F, 0x8F      ], X, AUTO_SIZE;
                    "ob",     [0x7F            ], X;
] "jecxz"       = [ "ob",     [0xE3            ], X, PREF_67;
] "jrcxz"       = [ "ob",     [0xE3            ], X;
] "jmp"         = [ "o*",     [0xE9            ], X, AUTO_SIZE;
                    "ob",     [0xEB            ], X;
                    "v*",     [0xFF            ], 4, AUTO_LARGE;
] "lahf"        = [ "",       [0x9F            ], X;
] "lfs"         = [ "r*m!",   [0x0F, 0xB4      ], X, AUTO_SIZE;
] "lgs"         = [ "r*m!",   [0x0F, 0xB5      ], X, AUTO_SIZE;
] "lss"         = [ "r*m!",   [0x0F, 0xB2      ], X, AUTO_SIZE;
] "lea"         = [ "r*m!",   [0x8D            ], X, AUTO_SIZE;
] "leave"       = [ "",       [0xC9            ], X;
] "lfence"      = [ "",       [0x0F, 0xAE, 0xE8], X;
] "lodsb"       = [ "",       [0xAC            ], X;
] "lodsw"       = [ "",       [0xAD            ], X, SMALL_SIZE;
] "lodsd"       = [ "",       [0xAD            ], X;
] "lodsq"       = [ "",       [0xAD            ], X, LARGE_SIZE;
] "loop"        = [ "ob",     [0xE2            ], X;
] "loope"       |
  "loopz"       = [ "ob",     [0xE1            ], X;
] "loopne"      |
  "loopnz"      = [ "ob",     [0xE0            ], X;
] "lzcnt"       = [ "r*v*",   [0x0F, 0xBD      ], X, AUTO_SIZE | PREF_F3;
] "mfence"      = [ "",       [0x0F, 0xAE, 0xF0], X;
] "mov"         = [ "v*r*",   [0x89            ], X, AUTO_SIZE;
                    "vbrb",   [0x88            ], X;
                    "r*v*",   [0x8B            ], X, AUTO_SIZE;
                    "rbvb",   [0x8A            ], X;
                    "rwsw",   [0x8C            ], X, SMALL_SIZE;
                    "rdsw",   [0x8C            ], X;
                    "rqsw",   [0x8C            ], X, LARGE_SIZE;
                    "mwsw",   [0x8C            ], X;
                    "swmw",   [0x8C            ], X;
                    "swrw",   [0x8C            ], X, SMALL_SIZE;
                    "rbib",   [0xB0            ], X,              SHORT_ARG;
                    "rwiw",   [0xB8            ], X, SMALL_SIZE | SHORT_ARG;
                    "rdid",   [0xB8            ], X,              SHORT_ARG;
                    "v*i*",   [0xC7            ], 0, AUTO_SIZE;
                    "rqiq",   [0xB8            ], X, LARGE_SIZE | SHORT_ARG;
                    "vbib",   [0xC6            ], 0;
                    "cdrd",   [0x0F, 0x22      ], X; // can only match in 32 bit mode due to "cd"
                    "cqrq",   [0x0F, 0x22      ], X; // doesn't need a prefix to be encoded, as it's 64 bit natural in 64 bit mode
                    "rdcd",   [0x0F, 0x20      ], X;
                    "rqcq",   [0x0F, 0x20      ], X;
                    "Wdrd",   [0x0F, 0x22      ], 0, PREF_F0; // note: technically CR8 should actually be encoded, but the encoding is 0.
                    "Wqrq",   [0x0F, 0x22      ], 0, PREF_F0;
                    "rdWd",   [0x0F, 0x22      ], 0, PREF_F0;
                    "rqWq",   [0x0F, 0x22      ], 0, PREF_F0;
                    "ddrd",   [0x0F, 0x23      ], X; // 32 bit mode only
                    "dqrq",   [0x0F, 0x23      ], X;
                    "rddd",   [0x0F, 0x21      ], X;
                    "rqdq",   [0x0F, 0x21      ], X;
] "movabs"      = [ "Abib",   [0xA0            ], X; // special syntax for 64-bit disp only mov
                    "Awiw",   [0xA1            ], X, SMALL_SIZE;
                    "Adid",   [0xA1            ], X;
                    "Aqiq",   [0xA1            ], X, LARGE_SIZE;
                    "ibAb",   [0xA2            ], X;
                    "iwAw",   [0xA3            ], X, SMALL_SIZE;
                    "idAd",   [0xA3            ], X;
                    "iqAq",   [0xA3            ], X, LARGE_SIZE;
] "movbe"       = [ "r*m*",   [0x0F, 0x38, 0xF0], X, AUTO_SIZE;
                    "m*r*",   [0x0F, 0x38, 0xF1], X, AUTO_SIZE;
] "movd"        = [ "yovd",   [0x0F, 0x6E      ], X,              PREF_66;
                    "yovq",   [0x0F, 0x6E      ], X, LARGE_SIZE | PREF_66;
                    "vdyo",   [0x0F, 0x7E      ], X,              PREF_66;
                    "vqyo",   [0x0F, 0x7E      ], X, LARGE_SIZE | PREF_66;
                    "xqvd",   [0x0F, 0x6E      ], X;
                    "xqvq",   [0x0F, 0x6E      ], X, LARGE_SIZE;
                    "vdxq",   [0x0F, 0x7E      ], X;
                    "vqxq",   [0x0F, 0x7E      ], X, LARGE_SIZE;
] "movmskpd"    = [ "rdyo",   [0x0F, 0x50      ], X, DEST_IN_REG | PREF_66;
] "movmskps"    = [ "rdyo",   [0x0F, 0x50      ], X, DEST_IN_REG;
] "movnti"      = [ "mdrd",   [0x0F, 0xC3      ], X;
                    "mqrq",   [0x0F, 0xC3      ], X, LARGE_SIZE;
] "movsb"       = [ "",       [0xA4            ], X;
] "movsw"       = [ "",       [0xA5            ], X, SMALL_SIZE;
] "movsd"       = [ "",       [0xA5            ], X;
] "movsq"       = [ "",       [0xA5            ], X, LARGE_SIZE;
] "movsx"       = [ "rdvw",   [0x0F, 0xBF      ], X; // currently this defaults to a certain memory size
                    "rqvw",   [0x0F, 0xBF      ], X, LARGE_SIZE;
                    "rwvb",   [0x0F, 0xBE      ], X, SMALL_SIZE;
                    "rdvb",   [0x0F, 0xBE      ], X;
                    "rqvb",   [0x0F, 0xBE      ], X, LARGE_SIZE;
] "movsxd"      = [ "rqvd",   [0x63            ], X, LARGE_SIZE;
] "movzx"       = [ "rdvw",   [0x0F, 0xB7      ], X; // currently this defaults to a certain memory size
                    "rqvw",   [0x0F, 0xB7      ], X, LARGE_SIZE;
                    "rwvb",   [0x0F, 0xB6      ], X, SMALL_SIZE;
                    "rdvb",   [0x0F, 0xB6      ], X;
                    "rqvb",   [0x0F, 0xB6      ], X, LARGE_SIZE;
] "mul"         = [ "v*",     [0xF7            ], 4, AUTO_SIZE;
                    "vb",     [0xF6            ], 4;
] "neg"         = [ "v*",     [0xF7            ], 3, AUTO_SIZE | LOCK;
                    "vb",     [0xF6            ], 3,             LOCK;
] "nop"         = [ "",       [0x90            ], X;
                    "v*",     [0x0F, 0x1F      ], 0, AUTO_SIZE;
] "not"         = [ "v*",     [0xF7            ], 2, AUTO_SIZE | LOCK;
                    "vb",     [0xF6            ], 2,             LOCK;
] "or"          = [ "A*i*",   [0x0D            ], X, AUTO_SIZE;
                    "Abib",   [0x0C            ], X;
                    "v*i*",   [0x81            ], 1, AUTO_SIZE | LOCK;
                    "v*ib",   [0x83            ], 1, AUTO_SIZE | LOCK;
                    "vbib",   [0x80            ], 1,             LOCK;
                    "v*r*",   [0x09            ], X, AUTO_SIZE | LOCK;
                    "vbrb",   [0x08            ], X,             LOCK;
                    "r*v*",   [0x0B            ], X, AUTO_SIZE;
                    "rbvb",   [0x0A            ], X;
] "out"         = [ "ibAb",   [0xE6            ], X;
                    "ibAw",   [0xE7            ], X;
                    "ibAd",   [0xE7            ], X;
                    "CwAb",   [0xEE            ], X;
                    "CwAw",   [0xEF            ], X, SMALL_SIZE;
                    "CwAd",   [0xEF            ], X;
] "outsb"       = [ "",       [0x6E            ], X,              REP;
] "outsw"       = [ "",       [0x6F            ], X, SMALL_SIZE | REP;
] "outsd"       = [ "",       [0x6F            ], X,              REP;
] "pause"       = [ "",       [0xF3, 0x90      ], X;
] "pop"         = [ "r*",     [0x58            ], X, AUTO_LARGE | SHORT_ARG;
                    "v*",     [0x8F            ], 0, AUTO_LARGE;
                    "Uw",     [0x0F, 0xA1      ], X;
                    "Vw",     [0x0F, 0xA9      ], X;
] "popcnt"      = [ "r*v*",   [0x0F, 0xB8      ], X, AUTO_SIZE | PREF_F3;
] "popf"        = [ "",       [0x9D            ], X, PREF_66;
] "popfq"       = [ "",       [0x9D            ], X;
] "prefetch"    = [ "mb",     [0x0F, 0x0D      ], 0;
] "prefetchw"   = [ "mb",     [0x0F, 0x0D      ], 1;
] "prefetchnta" = [ "mb",     [0x0F, 0x18      ], 0;
] "prefetcht0"  = [ "mb",     [0x0F, 0x18      ], 1;
] "prefetcht1"  = [ "mb",     [0x0F, 0x18      ], 2;
] "prefetcht2"  = [ "mb",     [0x0F, 0x18      ], 3;
] "push"        = [ "r*",     [0x50            ], X, AUTO_LARGE | SHORT_ARG;
                    "v*",     [0xFF            ], 6, AUTO_LARGE;
                    "ib",     [0x6A            ], X;
                    "iw",     [0x68            ], X, SMALL_SIZE;
                    "iq",     [0x68            ], X;
                    "Uw",     [0x0F, 0xA0      ], X;
                    "Vw",     [0x0F, 0xA8      ], X;
] "pushf"       = [ "",       [0x9C            ], X, PREF_66;
] "pushfq"      = [ "",       [0x9C            ], X;
] "rcl"         = [ "v*Bb",   [0xD3            ], 2, AUTO_SIZE; // shift by one forms not supported as immediates are only resolved at runtime
                    "vbBb",   [0xD2            ], 2;
                    "v*ib",   [0xC1            ], 2, AUTO_SIZE;
                    "vbib",   [0xC0            ], 2;
] "rcr"         = [ "v*Bb",   [0xD3            ], 3, AUTO_SIZE;
                    "vbBb",   [0xD2            ], 3;
                    "v*ib",   [0xC1            ], 3, AUTO_SIZE;
                    "vbib",   [0xC0            ], 3;
] "rdfsbase"    = [ "rd",     [0x0F, 0xAE      ], 0,              PREF_F3;
                    "rq",     [0x0F, 0xAE      ], 0, LARGE_SIZE | PREF_F3;
] "rdgsbase"    = [ "rd",     [0x0F, 0xAE      ], 1,              PREF_F3;
                    "rq",     [0x0F, 0xAE      ], 1, LARGE_SIZE | PREF_F3;
] "rdrand"      = [ "r*",     [0x0F, 0xC7      ], 6, AUTO_SIZE;
] "ret"         = [ "",       [0xC3            ], X;
                    "iw",     [0xC2            ], X;
] "rol"         = [ "v*Bb",   [0xD3            ], 0, AUTO_SIZE;
                    "vbBb",   [0xD2            ], 0;
                    "v*ib",   [0xC1            ], 0, AUTO_SIZE;
                    "vbib",   [0xC0            ], 0;
] "ror"         = [ "v*Bb",   [0xD3            ], 1, AUTO_SIZE;
                    "vbBb",   [0xD2            ], 1;
                    "v*ib",   [0xC1            ], 1, AUTO_SIZE;
                    "vbib",   [0xC0            ], 1;
] "sahf"        = [ "",       [0x9E            ], X;
] "sal"         |
  "shl"         = [ "v*Bb",   [0xD3            ], 4, AUTO_SIZE;
                    "vbBb",   [0xD2            ], 4;
                    "v*ib",   [0xC1            ], 4, AUTO_SIZE;
                    "vbib",   [0xC0            ], 4;
] "sar"         = [ "v*Bb",   [0xD3            ], 7, AUTO_SIZE;
                    "vbBb",   [0xD2            ], 7;
                    "v*ib",   [0xC1            ], 7, AUTO_SIZE;
                    "vbib",   [0xC0            ], 7;
] "sbb"         = [ "A*i*",   [0x1D            ], X, AUTO_SIZE;
                    "Abib",   [0x1C            ], X;
                    "v*i*",   [0x81            ], 3, AUTO_SIZE  | LOCK;
                    "v*ib",   [0x83            ], 3, AUTO_SIZE  | LOCK;
                    "vbib",   [0x80            ], 3,              LOCK;
                    "v*r*",   [0x19            ], X, AUTO_SIZE  | LOCK;
                    "vbrb",   [0x18            ], X,              LOCK;
                    "r*v*",   [0x1B            ], X, AUTO_SIZE;
                    "rbvb",   [0x1A            ], X;
] "scasb"       = [ "",       [0xAE            ], X,              REP;
] "scasw"       = [ "",       [0xAF            ], X, SMALL_SIZE | REP;
] "scasd"       = [ "",       [0xAF            ], X,              REP;
] "scasq"       = [ "",       [0xAF            ], X, LARGE_SIZE | REP;
] "seto"        = [ "vb",     [0x0F, 0x90      ], 0;
] "setno"       = [ "vb",     [0x0F, 0x91      ], 0;
] "setb"        |
  "setc"        |
  "setnae"      = [ "vb",     [0x0F, 0x92      ], 0;
] "setnb"       |
  "setnc"       |
  "setae"       = [ "vb",     [0x0F, 0x93      ], 0;
] "setz"        |
  "sete"        = [ "vb",     [0x0F, 0x94      ], 0;
] "setnz"       |
  "setne"       = [ "vb",     [0x0F, 0x95      ], 0;
] "setbe"       |
  "setna"       = [ "vb",     [0x0F, 0x96      ], 0;
] "setnbe"      |
  "seta"        = [ "vb",     [0x0F, 0x97      ], 0;
] "sets"        = [ "vb",     [0x0F, 0x98      ], 0;
] "setns"       = [ "vb",     [0x0F, 0x99      ], 0;
] "setp"        |
  "setpe"       = [ "vb",     [0x0F, 0x9A      ], 0;
] "setnp"       |
  "setpo"       = [ "vb",     [0x0F, 0x9B      ], 0;
] "setl"        |
  "setnge"      = [ "vb",     [0x0F, 0x9C      ], 0;
] "setnl"       |
  "setge"       = [ "vb",     [0x0F, 0x9D      ], 0;
] "setle"       |
  "setng"       = [ "vb",     [0x0F, 0x9E      ], 0;
] "setnle"      |
  "setg"        = [ "vb",     [0x0F, 0x9F      ], 0;
] "sfence"      = [ "",       [0x0F, 0xAE, 0xF8], X;
] "shld"        = [ "v*r*Bb", [0x0F, 0xA5      ], X, AUTO_SIZE;
                    "v*r*ib", [0x0F, 0xA4      ], X, AUTO_SIZE;
] "shr"         = [ "v*Bb",   [0xD3            ], 5, AUTO_SIZE;
                    "vbBb",   [0xD2            ], 5;
                    "v*ib",   [0xC1            ], 5, AUTO_SIZE;
                    "vbib",   [0xC0            ], 5;
] "shrd"        = [ "v*r*Bb", [0x0F, 0xAD      ], X, AUTO_SIZE;
                    "v*r*ib", [0x0F, 0xAC      ], X, AUTO_SIZE;
] "stc"         = [ "",       [0xF9            ], X;
] "std"         = [ "",       [0xFD            ], X;
] "stosb"       = [ "",       [0xAA            ], X,              REP;
] "stosw"       = [ "",       [0xAB            ], X, SMALL_SIZE | REP;
] "stosd"       = [ "",       [0xAB            ], X,              REP;
] "stosq"       = [ "",       [0xAB            ], X, LARGE_SIZE | REP;
] "sub"         = [ "A*i*",   [0x2D            ], X, AUTO_SIZE;
                    "Abib",   [0x2C            ], X;
                    "v*i*",   [0x81            ], 5, AUTO_SIZE  | LOCK;
                    "v*ib",   [0x83            ], 5, AUTO_SIZE  | LOCK;
                    "vbib",   [0x80            ], 5,              LOCK;
                    "v*r*",   [0x29            ], X, AUTO_SIZE  | LOCK;
                    "vbrb",   [0x28            ], X,              LOCK;
                    "r*v*",   [0x2B            ], X, AUTO_SIZE;
                    "rbvb",   [0x2A            ], X;
] "test"        = [ "A*i*",   [0xA9            ], X, AUTO_SIZE;
                    "Abib",   [0xA8            ], X;
                    "v*i*",   [0xF7            ], 0, AUTO_SIZE;
                    "vbib",   [0xF6            ], 0;
                    "v*r*",   [0x85            ], X, AUTO_SIZE;
                    "vbrb",   [0x84            ], X;
] "tzcnt"       = [ "r*v*",   [0x0F, 0xBC      ], X, AUTO_SIZE  | PREF_F3;
] "wrfsbase"    = [ "rd",     [0x0F, 0xAE      ], 2,              PREF_F3;
                    "rq",     [0x0F, 0xAE      ], 2, LARGE_SIZE | PREF_F3;
] "wrgsbase"    = [ "rd",     [0x0F, 0xAE      ], 3,              PREF_F3;
                    "rq",     [0x0F, 0xAE      ], 3, LARGE_SIZE | PREF_F3;
] "xadd"        = [ "v*r*",   [0x0F, 0xC1      ], X, AUTO_SIZE  | LOCK;
                    "vbrb",   [0x0F, 0xC0      ], X,              LOCK;
] "xchg"        = [ "A*r*",   [0x90            ], X, AUTO_SIZE  | SHORT_ARG;
                    "r*A*",   [0x90            ], X, AUTO_SIZE  | SHORT_ARG;
                    "v*r*",   [0x87            ], X, AUTO_SIZE  | LOCK;
                    "r*v*",   [0x87            ], X, AUTO_SIZE  | LOCK;
                    "vbrb",   [0x86            ], X,              LOCK;
                    "rbvb",   [0x86            ], X,              LOCK;
] "xlatb"       = [ "",       [0xD7            ], X;
] "xor"         = [ "A*i*",   [0x35            ], X, AUTO_SIZE;
                    "Abib",   [0x34            ], X;
                    "v*i*",   [0x81            ], 6, AUTO_SIZE  | LOCK;
                    "v*ib",   [0x83            ], 6, AUTO_SIZE  | LOCK;
                    "vbib",   [0x80            ], 6,              LOCK;
                    "v*r*",   [0x31            ], X, AUTO_SIZE  | LOCK;
                    "vbrb",   [0x30            ], X,              LOCK;
                    "r*v*",   [0x33            ], X, AUTO_SIZE;
                    "rbvb",   [0x32            ], X;
]
// System instructions
  "clgi"        = [ "",       [0x0F, 0x01, 0xDD], X;
] "cli"         = [ "",       [0xFA            ], X;
] "clts"        = [ "",       [0x0F, 0x06      ], X;
] "hlt"         = [ "",       [0xF4            ], X;
] "int3"        = [ "",       [0xCC            ], X;
] "invd"        = [ "",       [0x0F, 0x08      ], X;
] "invlpg"      = [ "mb",     [0x0F, 0x01      ], 7;
] "invlpga"     = [ "AqBd",   [0x0F, 0x01, 0xDF], X;
] "iret"        = [ "",       [0xCF            ], X, SMALL_SIZE;
] "iretd"       = [ "",       [0xCF            ], X;
] "iretq"       = [ "",       [0xCF            ], X, LARGE_SIZE;
] "lar"         = [ "rwvw",   [0x0F, 0x02      ], X, SMALL_SIZE;
                    "rdvw",   [0x0F, 0x02      ], X;
                    "rqvw",   [0x0F, 0x02      ], X, LARGE_SIZE;
] "lgdt"        = [ "m!",     [0x0F, 0x01      ], 2;
] "lidt"        = [ "m!",     [0x0F, 0x01      ], 3;
] "lldt"        = [ "vw",     [0x0F, 0x00      ], 2;
] "lmsw"        = [ "vw",     [0x0F, 0x01      ], 6;
] "lsl"         = [ "rwvw",   [0x0F, 0x03      ], X, SMALL_SIZE;
                    "rdvw",   [0x0F, 0x03      ], X;
                    "rqvw",   [0x0F, 0x03      ], X, LARGE_SIZE;
] "ltr"         = [ "vw",     [0x0F, 0x00      ], 3;
] "monitor"     = [ "",       [0x0F, 0x01, 0xC8], X;
] "monitorx"    = [ "",       [0x0F, 0x01, 0xFA], X;
] "mwait"       = [ "",       [0x0F, 0x01, 0xC9], X;
] "mwaitx"      = [ "",       [0x0F, 0x01, 0xFB], X;
] "rdmsr"       = [ "",       [0x0F, 0x32      ], X;
] "rdpmc"       = [ "",       [0x0F, 0x33      ], X;
] "rdtsc"       = [ "",       [0x0F, 0x31      ], X;
] "rdtscp"      = [ "",       [0x0F, 0x01, 0xF9], X;
] "rsm"         = [ "",       [0x0F, 0xAA      ], X;
] "sgdt"        = [ "m!",     [0x0F, 0x01      ], 0;
] "sidt"        = [ "m!",     [0x0F, 0x01      ], 1;
] "skinit"      = [ "Ad",     [0x0F, 0x01, 0xDE], X;
] "sldt"        = [ "r*",     [0x0F, 0x00      ], 0, AUTO_SIZE;
                    "mw",     [0x0F, 0x00      ], 0;
] "smsw"        = [ "r*",     [0x0F, 0x01      ], 4, AUTO_SIZE;
                    "mw",     [0x0F, 0x01      ], 4;
] "sti"         = [ "",       [0xFB            ], X;
] "stgi"        = [ "",       [0x0F, 0x01, 0xDC], X;
] "str"         = [ "r*",     [0x0F, 0x00      ], 1, AUTO_SIZE;
                    "mw",     [0x0F, 0x00      ], 1;
] "swapgs"      = [ "",       [0x0F, 0x01, 0xF8], X;
] "syscall"     = [ "",       [0x0F, 0x05      ], X;
] "sysenter"    = [ "",       [0x0F, 0x34      ], X;
] "sysexit"     = [ "",       [0x0F, 0x35      ], X;
] "sysret"      = [ "",       [0x0F, 0x07      ], X;
] "ud2"         = [ "",       [0x0F, 0x0B      ], X;
] "verr"        = [ "vw",     [0x0F, 0x00      ], 4;
] "verw"        = [ "vw",     [0x0F, 0x00      ], 5;
] "vmload"      = [ "Aq",     [0x0F, 0x01, 0xDA], X;
] "vmmcall"     = [ "",       [0x0F, 0x01, 0xD9], X;
] "vmrun"       = [ "Aq",     [0x0F, 0x01, 0xD8], X;
] "vmsave"      = [ "Aq",     [0x0F, 0x01, 0xDB], X;
] "wbinvd"      = [ "",       [0x0F, 0x09      ], X;
] "wrmsr"       = [ "",       [0x0F, 0x30      ], X;
]
// x87 FPU instruction set, data taken from amd's programmer manual vol. 5
  "f2xm1"       = [ "",       [0xD9, 0xF0      ], X;
] "fabs"        = [ "",       [0xD9, 0xE1      ], X;
] "fadd"        = [ "Xpfp",   [0xD8, 0xC0      ], X, SHORT_ARG;
                    "fpXp",   [0xDC, 0xC0      ], X, SHORT_ARG;
                    "md",     [0xD8            ], 0;
                    "mq",     [0xDC            ], 0;
] "faddp"       = [ "",       [0xDE, 0xC1      ], X;
                    "fpXp",   [0xDE, 0xC0      ], X, SHORT_ARG;
] "fiadd"       = [ "mw",     [0xDE            ], 0;
                    "md",     [0xDA            ], 0;
] "fbld"        = [ "mp",     [0xDF            ], 4;
] "fbstp"       = [ "mp",     [0xDF            ], 6;
] "fchs"        = [ "",       [0xD9, 0xE0      ], X;
] "fclex"       = [ "",       [0x9B, 0xDB, 0xE2], X; // this is actually ;wait ;fnclex
] "fnclex"      = [ "",       [0xDB, 0xE2      ], X;
] "fcmovb"      = [ "Xpfp",   [0xDA, 0xC0      ], X, SHORT_ARG;
] "fcmovbe"     = [ "Xpfp",   [0xDA, 0xD0      ], X, SHORT_ARG;
] "fcmove"      = [ "Xpfp",   [0xDA, 0xC8      ], X, SHORT_ARG;
] "fcmovnb"     = [ "Xpfp",   [0xDB, 0xC0      ], X, SHORT_ARG;
] "fcmovnbe"    = [ "Xpfp",   [0xDB, 0xD0      ], X, SHORT_ARG;
] "fcmovne"     = [ "Xpfp",   [0xDB, 0xC8      ], X, SHORT_ARG;
] "fcmovnu"     = [ "Xpfp",   [0xDB, 0xD8      ], X, SHORT_ARG;
] "fcmovu"      = [ "Xpfp",   [0xDA, 0xD8      ], X, SHORT_ARG;
] "fcom"        = [ "",       [0xD8, 0xD1      ], X;
                    "fp",     [0xD8, 0xD0      ], X, SHORT_ARG;
                    "md",     [0xD8            ], 2;
                    "mq",     [0xDC            ], 2;
] "fcomp"       = [ "",       [0xD8, 0xD9      ], X;
                    "fp",     [0xD8, 0xD8      ], X, SHORT_ARG;
                    "md",     [0xD8            ], 3;
                    "mq",     [0xDC            ], 3;
] "fcompp"      = [ "",       [0xDE, 0xD9      ], X;
] "fcomi"       = [ "Xpfp",   [0xDB, 0xF0      ], X, SHORT_ARG;
] "fcomip"      = [ "fpXp",   [0xDF, 0xF0      ], X, SHORT_ARG;
] "fcos"        = [ "",       [0xD9, 0xFF      ], X;
] "fdecstp"     = [ "",       [0xD9, 0xF6      ], X;
] "fdiv"        = [ "Xpfp",   [0xD8, 0xF0      ], X, SHORT_ARG;
                    "fpXp",   [0xDC, 0xF8      ], X, SHORT_ARG;
                    "md",     [0xD8            ], 6;
                    "mq",     [0xDC            ], 6;
] "fdivp"       = [ "",       [0xDE, 0xF9      ], X;
                    "fpXp",   [0xDE, 0xF8      ], X, SHORT_ARG;
] "fidiv"       = [ "mw",     [0xDE            ], 6;
                    "md",     [0xDA            ], 6;
] "fdivr"       = [ "Xpfp",   [0xD8, 0xF8      ], X, SHORT_ARG;
                    "fpXp",   [0xDC, 0xF0      ], X, SHORT_ARG;
                    "md",     [0xD8            ], 7;
                    "mq",     [0xDC            ], 7;
] "fdivrp"      = [ "",       [0xDE, 0xF1      ], X;
                    "fpXp",   [0xDE, 0xF0      ], X, SHORT_ARG;
] "fidivr"      = [ "mw",     [0xDE            ], 7;
                    "md",     [0xDA            ], 7;
] "ffree"       = [ "fp",     [0xDD, 0xC0      ], X, SHORT_ARG;
] "ficom"       = [ "mw",     [0xDE            ], 2;
                    "md",     [0xDA            ], 2;
] "ficomp"      = [ "mw",     [0xDE            ], 3;
                    "md",     [0xDA            ], 3;
] "fild"        = [ "mw",     [0xDF            ], 0;
                    "md",     [0xDB            ], 0;
                    "mq",     [0xDF            ], 5;
] "fincstp"     = [ "",       [0xD9, 0xF7      ], X;
] "finit"       = [ "",       [0x9B, 0xDB, 0xE3], X; // this is actually ;wait ;fninit
] "fninit"      = [ "",       [0xDB, 0xE3      ], X;
] "fist"        = [ "mw",     [0xDF            ], 2;
                    "md",     [0xDB            ], 2;
                    "mw",     [0xDF            ], 3;
                    "md",     [0xDB            ], 3;
                    "mq",     [0xDF            ], 7;
] "fisttp"      = [ "mw",     [0xDF            ], 1;
                    "md",     [0xDB            ], 1;
                    "mq",     [0xDD            ], 1;
] "fld"         = [ "fp",     [0xD9, 0xC0      ], X, SHORT_ARG;
                    "md",     [0xD9            ], 0;
                    "mq",     [0xDD            ], 0;
                    "mp",     [0xDB            ], 5;
] "fld1"        = [ "",       [0xD9, 0xE8      ], X;
] "fldcw"       = [ "mw",     [0xD9            ], 5;
] "fldenv"      = [ "m!",     [0xD9            ], 4;
] "fldenvw"     = [ "m!",     [0xD9            ], 4, SMALL_SIZE;
] "fldl2e"      = [ "",       [0xD9, 0xEA      ], X;
] "fldl2t"      = [ "",       [0xD9, 0xE9      ], X;
] "fldlg2"      = [ "",       [0xD9, 0xEC      ], X;
] "fldln2"      = [ "",       [0xD9, 0xED      ], X;
] "fldpi"       = [ "",       [0xD9, 0xEB      ], X;
] "fldz"        = [ "",       [0xD9, 0xEE      ], X;
] "fmul"        = [ "Xpfp",   [0xD8, 0xC8      ], X, SHORT_ARG;
                    "fpXp",   [0xDC, 0xC8      ], X, SHORT_ARG;
                    "md",     [0xD8            ], 1;
                    "mq",     [0xDC            ], 1;
] "fmulp"       = [ "",       [0xDE, 0xC9      ], X;
                    "fpXp",   [0xDE, 0xC8      ], X, SHORT_ARG;
] "fimul"       = [ "mw",     [0xDE            ], 1;
                    "md",     [0xDA            ], 1;
] "fnop"        = [ "",       [0xD9, 0xD0      ], X;
] "fpatan"      = [ "",       [0xD9, 0xF3      ], X;
] "fprem"       = [ "",       [0xD9, 0xF8      ], X;
] "fprem1"      = [ "",       [0xD9, 0xF5      ], X;
] "fptan"       = [ "",       [0xD9, 0xF2      ], X;
] "frndint"     = [ "",       [0xD9, 0xFC      ], X;
] "frstor"      = [ "m!",     [0xDD            ], 4;
] "frstorw"     = [ "m!",     [0xDD            ], 4, SMALL_SIZE;
] "fsave"       = [ "m!",     [0x9B, 0xDD      ], 6; // note: this is actually ; wait; fnsavew
] "fsavew"      = [ "m!",     [0x9B, 0x66, 0xDD], 6; // note: this is actually ; wait; OPSIZE fnsave
] "fnsave"      = [ "m!",     [0xDD            ], 6;
] "fnsavew"     = [ "m!",     [0xDD            ], 6, SMALL_SIZE;
] "fscale"      = [ "",       [0xD9, 0xFD      ], X;
] "fsin"        = [ "",       [0xD9, 0xFE      ], X;
] "fsincos"     = [ "",       [0xD9, 0xFB      ], X;
] "fsqrt"       = [ "",       [0xD9, 0xFA      ], X;
] "fst"         = [ "fp",     [0xDD, 0xD0      ], X, SHORT_ARG;
                    "md",     [0xD9            ], 2;
                    "mq",     [0xDD            ], 2;
] "fstp"        = [ "fp",     [0xDD, 0xD8      ], X, SHORT_ARG;
                    "md",     [0xD9            ], 3;
                    "mq",     [0xDD            ], 3;
                    "mp",     [0xDB            ], 7;
] "fstcw"       = [ "mw",     [0x9B, 0xD9      ], 7; // note: this is actually ; wait; fnstcw
] "fnstcw"      = [ "mw",     [0xD9            ], 7;
] "fstenv"      = [ "m!",     [0x9B, 0xD9      ], 6; // note: this is actually ; wait; fnstenv
] "fstenvw"     = [ "m!",     [0x9B, 0x66, 0xD9], 6; // note: this is actually ; wait; OPSIZE fnsten
] "fnstenv"     = [ "m!",     [0xD9            ], 6;
] "fnstenvw"    = [ "m!",     [0xD9            ], 6, SMALL_SIZE;
] "fstsw"       = [ "Aw",     [0x9B, 0xDF, 0xE0], X; // note: this is actually ; wait; fnstsw
                    "mw",     [0x9B, 0xDD      ], 7;
] "fnstsw"      = [ "Aw",     [0xDF, 0xE0      ], X;
                    "mw",     [0xDD            ], 7;
] "fsub"        = [ "Xpfp",   [0xD8, 0xE0      ], X, SHORT_ARG;
                    "fpXp",   [0xDC, 0xE8      ], X, SHORT_ARG;
                    "md",     [0xD8            ], 4;
                    "mq",     [0xDC            ], 4;
] "fsubp"       = [ "",       [0xDE, 0xE9      ], X;
                    "fpXp",   [0xDE, 0xE8      ], X, SHORT_ARG;
] "fisub"       = [ "mw",     [0xDE            ], 4;
                    "md",     [0xDA            ], 4;
] "fsubr"       = [ "Xpfp",   [0xD8, 0xE8      ], X, SHORT_ARG;
                    "fpXp",   [0xDC, 0xE0      ], X, SHORT_ARG;
                    "md",     [0xD8            ], 5;
                    "mq",     [0xDC            ], 5;
] "fsubrp"      = [ "",       [0xDE, 0xE1      ], X;
                    "fpXp",   [0xDE, 0xE0      ], X, SHORT_ARG;
] "fisubr"      = [ "mw",     [0xDE            ], 5;
                    "md",     [0xDA            ], 5;
] "ftst"        = [ "",       [0xD9, 0xE4      ], X;
] "fucom"       = [ "",       [0xDD, 0xE1      ], X;
                    "fp",     [0xDD, 0xE0      ], X, SHORT_ARG;
] "fucomp"      = [ "",       [0xDD, 0xE9      ], X;
                    "fp",     [0xDD, 0xE8      ], X, SHORT_ARG;
] "fucompp"     = [ "",       [0xDA, 0xE9      ], X;
] "fucomi"      = [ "Xpfp",   [0xDB, 0xE8      ], X, SHORT_ARG;
                    "fpXp",   [0xDF, 0xE8      ], X, SHORT_ARG;
] "fwait"       |
  "wait"        = [ "",       [0x9B            ], X;
] "fxam"        = [ "",       [0xD9, 0xE5      ], X;
] "fxch"        = [ "",       [0xD9, 0xC9      ], X;
                    "fp",     [0xD9, 0xC8      ], X, SHORT_ARG;
] "fxrstor"     = [ "m!",     [0x0F, 0xAE      ], 1;
] "fxsave"      = [ "m!",     [0x0F, 0xAE      ], 0;
] "fxtract"     = [ "",       [0xD9, 0xF4      ], X;
] "fyl2x"       = [ "",       [0xD9, 0xF1      ], X;
] "fyl2xp1"     = [ "",       [0xD9, 0xF9      ], X;
]
// MMX instruction (also vol. 5)
        /*
  "cvtpd2pi"    = [ "xqwo",   [0x0F, 0x2D      ], 0, PREF_66;
] "cvtpi2pd"    = [ "youq",   [0x0F, 0x2A      ], 0, PREF_66;
] "cvtpi2ps"    = [ "youq",   [0x0F, 0x2A      ], 0;
] "cvtps2pi"    = [ "xqwo",   [0x0F, 0x2D      ], 0;
] "cvttpd2pi"   = [ "xqwo",   [0x0F, 0x2C      ], 0, PREF_66;
] "cvttps2pi"   = [ "xqyo",   [0x0F, 0x2C      ], 0, DEST_IN_REG;
                    "xqmq",   [0x0F, 0x2C      ], 0;
] "emms"        = [ "",       [0x0F, 0x77      ], 0;
] "maskmovq"

        "movdq2q" => Op!(
                    "xqyo",   [0x0F, 0xD6      ], 0, PREF_F3NE | ) // operand order?
        */

// AVX instructions (vol. 4)
    }))
}
