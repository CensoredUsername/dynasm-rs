use std::collections::{HashMap, hash_map};

use super::compiler::Opdata;

macro_rules! constify {
    ($t:ty, $e:expr) => { {const C: &'static $t = &$e; C} }
}

macro_rules! OpInner {
    ($fmt:expr, $ops:expr, $reg:expr)          => { Opdata {args: $fmt, ops: constify!([u8], $ops), reg: $reg, flags: Flags::DEFAULT,  features: Features::X64_IMPLICIT}  };
    ($fmt:expr, $ops:expr, $reg:expr, $f:expr) => { Opdata {args: $fmt, ops: constify!([u8], $ops), reg: $reg, flags: Flags::make($f), features: Features::X64_IMPLICIT}  };
    ($fmt:expr, $ops:expr, $reg:expr, $f:expr, $ft:expr) => { Opdata {args: $fmt, ops: constify!([u8], $ops), reg: $reg, flags: Flags::make($f), features: Features::make($ft)}  };

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
    OPMAP.get(&name).map(|x| *x)
}

bitflags! {
    pub struct Flags: u32 {
        const DEFAULT   = 0x0000_0000; // this instruction has default encoding
        const VEX_OP    = 0x0000_0001; // this instruction requires a VEX prefix to be encoded
        const XOP_OP    = 0x0000_0002; // this instruction requires a XOP prefix to be encoded
        const IMM_OP    = 0x0000_0004; // this instruction encodes the final opcode byte in the immediate position, like 3DNow! ops.

        // note: the first 4 in this block are mutually exclusive
        const AUTO_SIZE = 0x0000_0008; // 16 bit -> OPSIZE , 32-bit -> None     , 64-bit -> REX.W/VEX.W/XOP.W
        const AUTO_NO32 = 0x0000_0010; // 16 bit -> OPSIZE , 32-bit -> None(x86), 64-bit -> None(x64)
        const AUTO_REXW = 0x0000_0020; // 16 bit -> illegal, 32-bit -> None     , 64-bit -> REX.W/VEX.W/XOP.W
        const AUTO_VEXL = 0x0000_0040; // 128bit -> None   , 256bit -> VEX.L
        const WORD_SIZE = 0x0000_0080; // implies opsize prefix
        const WITH_REXW = 0x0000_0100; // implies REX.W/VEX.W/XOP.W
        const WITH_VEXL = 0x0000_0200; // implies VEX.L/XOP.L
        const EXACT_SIZE= 0x0000_0400; // operands with unknown sizes cannot be assumed to match

        const PREF_66   = WORD_SIZE;   // mandatory prefix (same as WORD_SIZE)
        const PREF_67   = 0x0000_0800; // mandatory prefix (same as SMALL_ADDRESS)
        const PREF_F0   = 0x0000_1000; // mandatory prefix (same as LOCK)
        const PREF_F2   = 0x0000_2000; // mandatory prefix (REPNE)
        const PREF_F3   = 0x0000_4000; // mandatory prefix (REP)

        const LOCK      = 0x0000_8000; // user lock prefix is valid with this instruction
        const REP       = 0x0001_0000; // user rep prefix is valid with this instruction
        const REPE      = 0x0002_0000;

        const SHORT_ARG = 0x0004_0000; // a register argument is encoded in the last byte of the opcode
        const ENC_MR    = 0x0008_0000; // select alternate arg encoding
        const ENC_VM    = 0x0010_0000; // select alternate arg encoding
        const ENC_MIB   = 0x0020_0000; // A special encoding using the SIB to specify an immediate and two registers
        const X86_ONLY  = 0x0040_0000; // instructions available in protected mode, but not long mode
    }
}

impl Flags {
    const fn make(bits: u32) -> Flags {
        Flags {
            bits: bits
        }
    }
}

bitflags! {
    pub struct Features: u32 {
        const X64_IMPLICIT = 0x0000_0000;
        const FPU          = 0x0000_0001;
          const MMX          = 0x0000_0002;
        const TDNOW        = 0x0000_0004;
        const SSE          = 0x0000_0008;
        const SSE2         = 0x0000_0010;
        const SSE3         = 0x0000_0020;
        const VMX          = 0x0000_0040;
        const SSSE3        = 0x0000_0080;
        const SSE4A        = 0x0000_0100;
        const SSE41        = 0x0000_0200;
        const SSE42        = 0x0000_0400;
        const SSE5         = 0x0000_0800;
        const AVX          = 0x0000_1000;
        const AVX2         = 0x0000_2000;
        const FMA          = 0x0000_4000;
        const BMI1         = 0x0000_8000;
        const BMI2         = 0x0001_0000;
        const TBM          = 0x0002_0000;
        const RTM          = 0x0004_0000;
        const INVPCID      = 0x0008_0000;
        const MPX          = 0x0010_0000;
        const SHA          = 0x0020_0000;
        const PREFETCHWT1  = 0x0040_0000;
        const CYRIX        = 0x0080_0000;
        const AMD          = 0x0100_0000;
    }
}

impl Features {
    const fn make(bits: u32) -> Features {
        Features {
            bits: bits
        }
    }
}



pub fn mnemnonics() -> hash_map::Keys<'static, &'static str, &'static [Opdata]> {
    OPMAP.keys()
}

// workaround until bitflags can be used in const
const DEFAULT    : u32 = Flags::DEFAULT.bits;
const VEX_OP     : u32 = Flags::VEX_OP.bits;
const XOP_OP     : u32 = Flags::XOP_OP.bits;
const IMM_OP     : u32 = Flags::IMM_OP.bits;
const SHORT_ARG  : u32 = Flags::SHORT_ARG.bits;
const AUTO_SIZE  : u32 = Flags::AUTO_SIZE.bits;
const AUTO_NO32  : u32 = Flags::AUTO_NO32.bits;
const AUTO_REXW  : u32 = Flags::AUTO_REXW.bits;
const AUTO_VEXL  : u32 = Flags::AUTO_VEXL.bits;
const WORD_SIZE  : u32 = Flags::WORD_SIZE.bits;
const WITH_REXW  : u32 = Flags::WITH_REXW.bits;
const WITH_VEXL  : u32 = Flags::WITH_VEXL.bits;
const EXACT_SIZE : u32 = Flags::EXACT_SIZE.bits;
const PREF_66    : u32 = Flags::PREF_66.bits;
const PREF_67    : u32 = Flags::PREF_67.bits;
const PREF_F0    : u32 = Flags::PREF_F0.bits;
const PREF_F2    : u32 = Flags::PREF_F2.bits;
const PREF_F3    : u32 = Flags::PREF_F3.bits;
const LOCK       : u32 = Flags::LOCK.bits;
const REP        : u32 = Flags::REP.bits;
const REPE       : u32 = Flags::REPE.bits;
const ENC_MR     : u32 = Flags::ENC_MR.bits;
const ENC_VM     : u32 = Flags::ENC_VM.bits;
const ENC_MIB    : u32 = Flags::ENC_MIB.bits;
const X86_ONLY   : u32 = Flags::X86_ONLY.bits;

#[allow(dead_code)]
const X64_IMPLICIT : u32 = Features::X64_IMPLICIT.bits;
const FPU          : u32 = Features::FPU.bits;
const MMX          : u32 = Features::MMX.bits;
const TDNOW        : u32 = Features::TDNOW.bits;
const SSE          : u32 = Features::SSE.bits;
const SSE2         : u32 = Features::SSE2.bits;
const SSE3         : u32 = Features::SSE3.bits;
const VMX          : u32 = Features::VMX.bits;
const SSSE3        : u32 = Features::SSSE3.bits;
const SSE4A        : u32 = Features::SSE4A.bits;
const SSE41        : u32 = Features::SSE41.bits;
const SSE42        : u32 = Features::SSE42.bits;
const SSE5         : u32 = Features::SSE5.bits;
const AVX          : u32 = Features::AVX.bits;
const AVX2         : u32 = Features::AVX2.bits;
const FMA          : u32 = Features::FMA.bits;
const BMI1         : u32 = Features::BMI1.bits;
const BMI2         : u32 = Features::BMI2.bits;
const TBM          : u32 = Features::TBM.bits;
const RTM          : u32 = Features::RTM.bits;
const INVPCID      : u32 = Features::INVPCID.bits;
const MPX          : u32 = Features::MPX.bits;
const SHA          : u32 = Features::SHA.bits;
const PREFETCHWT1  : u32 = Features::PREFETCHWT1.bits;
const CYRIX        : u32 = Features::CYRIX.bits;
const AMD          : u32 = Features::AMD.bits;

include!("gen_opmap.rs");
