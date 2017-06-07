use std::collections::{HashMap, hash_map};

use super::compiler::Opdata;

macro_rules! constify {
    ($t:ty, $e:expr) => { {const C: &'static $t = &$e; C} }
}

macro_rules! OpInner {
    ($fmt:expr, $ops:expr, $reg:expr)          => { Opdata {args: $fmt, ops: constify!([u8], $ops), reg: $reg, flags: flags::make_flag(0),  features: features::make_flag(0) }  };
    ($fmt:expr, $ops:expr, $reg:expr, $f:expr) => { Opdata {args: $fmt, ops: constify!([u8], $ops), reg: $reg, flags: flags::make_flag($f), features: features::make_flag(0)}  };
    ($fmt:expr, $ops:expr, $reg:expr, $f:expr, $ft:expr) => { Opdata {args: $fmt, ops: constify!([u8], $ops), reg: $reg, flags: flags::make_flag($f), features: features::make_flag($ft)}  };

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

pub mod flags {
    bitflags! {
        pub struct Flags: u32 {
            const DEFAULT   = 0x0000_0000; // this instruction has default encoding
            const VEX_OP    = 0x0000_0001; // this instruction requires a VEX prefix to be encoded
            const XOP_OP    = 0x0000_0002; // this instruction requires a XOP prefix to be encoded
            const IMM_OP    = 0x0000_0004; // this instruction encodes the final opcode byte in the immediate position, like 3DNow! ops.

            // note: the first 4 in this block are mutually exclusive
            const AUTO_SIZE = 0x0000_0008; // 16 bit -> OPSIZE , 32-bit -> None   , 64-bit -> REX.W/VEX.W/XOP.W
            const AUTO_NO32 = 0x0000_0010; // 16 bit -> OPSIZE , 32-bit -> illegal, 64-bit -> None
            const AUTO_REXW = 0x0000_0020; // 16 bit -> illegal, 32-bit -> None   , 64-bit -> REX.W/VEX.W/XOP.W
            const AUTO_VEXL = 0x0000_0040; // 128bit -> None   , 256bit -> VEX.L
            const WORD_SIZE = 0x0000_0080; // implies opsize prefix
            const WITH_REXW = 0x0000_0100; // implies REX.W/VEX.W/XOP.W
            const WITH_VEXL = 0x0000_0200; // implies VEX.L/XOP.L
            const EXACT_SIZE= 0x0000_0400; // operands with unknown sizes cannot be assumed to match

            const PREF_66   = WORD_SIZE.bits;// mandatory prefix (same as WORD_SIZE)
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

pub mod features {
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

    pub const fn flag_bits(flag: Features) -> u32 {
        flag.bits
    }

    pub const fn make_flag(bits: u32) -> Features {
        Features {bits: bits}
    }

}

pub fn mnemnonics() -> hash_map::Keys<'static, &'static str, &'static [Opdata]> {
    OPMAP.keys()
}

// workaround until bitflags can be used in const
const DEFAULT    : u32 = flags::flag_bits(flags::DEFAULT);
const VEX_OP    : u32 = flags::flag_bits(flags::VEX_OP);
const XOP_OP    : u32 = flags::flag_bits(flags::XOP_OP);
const IMM_OP    : u32 = flags::flag_bits(flags::IMM_OP);
const SHORT_ARG : u32 = flags::flag_bits(flags::SHORT_ARG);
const AUTO_SIZE : u32 = flags::flag_bits(flags::AUTO_SIZE);
const AUTO_NO32 : u32 = flags::flag_bits(flags::AUTO_NO32);
const AUTO_REXW : u32 = flags::flag_bits(flags::AUTO_REXW);
const AUTO_VEXL : u32 = flags::flag_bits(flags::AUTO_VEXL);
const WORD_SIZE : u32 = flags::flag_bits(flags::WORD_SIZE);
const WITH_REXW : u32 = flags::flag_bits(flags::WITH_REXW);
const WITH_VEXL : u32 = flags::flag_bits(flags::WITH_VEXL);
const EXACT_SIZE: u32 = flags::flag_bits(flags::EXACT_SIZE);
const PREF_66   : u32 = flags::flag_bits(flags::PREF_66);
const PREF_67   : u32 = flags::flag_bits(flags::PREF_67);
const PREF_F0   : u32 = flags::flag_bits(flags::PREF_F0);
const PREF_F2   : u32 = flags::flag_bits(flags::PREF_F2);
const PREF_F3   : u32 = flags::flag_bits(flags::PREF_F3);
const LOCK      : u32 = flags::flag_bits(flags::LOCK);
const REP       : u32 = flags::flag_bits(flags::REP);
const REPE      : u32 = flags::flag_bits(flags::REPE);
const ENC_MR    : u32 = flags::flag_bits(flags::ENC_MR);
const ENC_VM    : u32 = flags::flag_bits(flags::ENC_VM);
const ENC_MIB   : u32 = flags::flag_bits(flags::ENC_MIB);

#[allow(dead_code)]
const X64_IMPLICIT : u32 = features::flag_bits(features::X64_IMPLICIT);
const FPU          : u32 = features::flag_bits(features::FPU);
const MMX          : u32 = features::flag_bits(features::MMX);
const TDNOW        : u32 = features::flag_bits(features::TDNOW);
const SSE          : u32 = features::flag_bits(features::SSE);
const SSE2         : u32 = features::flag_bits(features::SSE2);
const SSE3         : u32 = features::flag_bits(features::SSE3);
const VMX          : u32 = features::flag_bits(features::VMX);
const SSSE3        : u32 = features::flag_bits(features::SSSE3);
const SSE4A        : u32 = features::flag_bits(features::SSE4A);
const SSE41        : u32 = features::flag_bits(features::SSE41);
const SSE42        : u32 = features::flag_bits(features::SSE42);
const SSE5         : u32 = features::flag_bits(features::SSE5);
const AVX          : u32 = features::flag_bits(features::AVX);
const AVX2         : u32 = features::flag_bits(features::AVX2);
const FMA          : u32 = features::flag_bits(features::FMA);
const BMI1         : u32 = features::flag_bits(features::BMI1);
const BMI2         : u32 = features::flag_bits(features::BMI2);
const TBM          : u32 = features::flag_bits(features::TBM);
const RTM          : u32 = features::flag_bits(features::RTM);
const INVPCID      : u32 = features::flag_bits(features::INVPCID);
const MPX          : u32 = features::flag_bits(features::MPX);
const SHA          : u32 = features::flag_bits(features::SHA);
const PREFETCHWT1  : u32 = features::flag_bits(features::PREFETCHWT1);
const CYRIX        : u32 = features::flag_bits(features::CYRIX);
const AMD          : u32 = features::flag_bits(features::AMD);

include!("gen_opmap.rs");
