use compiler::Opdata;
use compiler::flags::*;

macro_rules! constify {
    ($t:ty, $e:expr) => { {const C: &'static $t = &$e; C} }
}

macro_rules! Op {
    ($( $( $e:expr ),+ ; )+)                   => { {const C: &'static [Opdata] = &[$( Op!($( $e ),*) ,)+]; C} };
    ($fmt:expr, $ops:expr)                     => { Opdata {args: $fmt, ops: constify!([u8], $ops), reg: 0   , flags:  0}  };
    ($fmt:expr, $ops:expr, $reg:expr)          => { Opdata {args: $fmt, ops: constify!([u8], $ops), reg: $reg, flags:  0}  };
    ($fmt:expr, $ops:expr, $reg:expr, $f:expr) => { Opdata {args: $fmt, ops: constify!([u8], $ops), reg: $reg, flags: $f}  };
}

pub fn get_mnemnonic_data(name: &str) -> Option<&'static [Opdata]> {
    // note: currently only listing instructions that are usable in long mode, with 64-bit addrs, no VEX/XOP prefixes or segment overrides, without requiring privileges, that are not an extension
    // this helps preserve my sanity for now

    // I blame intel for the following match
    Some(match name {
        // general purpose instructions according to AMD's AMD64 Arch Programmer's Manual Vol. 3
        "adc" => Op!(
            "A*i*", [0x15], 0, CAN_LOCK;
            "Abib", [0x14], 0, CAN_LOCK;
            "v*i*", [0x81], 2, CAN_LOCK;
            "v*ib", [0x83], 2, CAN_LOCK;
            "vbib", [0x80], 2, CAN_LOCK;
            "v*r*", [0x11], 0, CAN_LOCK;
            "vbrb", [0x10], 0, CAN_LOCK;
            "r*v*", [0x13], 0, CAN_LOCK;
            "rbvb", [0x12], 0, CAN_LOCK;
        ), "add" => Op!(
            "A*i*", [0x05], 0, CAN_LOCK;
            "Abib", [0x04], 0, CAN_LOCK;
            "v*i*", [0x81], 0, CAN_LOCK;
            "v*ib", [0x83], 0, CAN_LOCK;
            "vbib", [0x80], 0, CAN_LOCK;
            "v*r*", [0x01], 0, CAN_LOCK;
            "vbrb", [0x00], 0, CAN_LOCK;
            "r*v*", [0x03], 0, CAN_LOCK;
            "rbvb", [0x02], 0, CAN_LOCK;
        ), "and" => Op!(
            "A*i*", [0x25], 0, CAN_LOCK;
            "Abib", [0x24], 0, CAN_LOCK;
            "v*i*", [0x81], 4, CAN_LOCK;
            "v*ib", [0x83], 4, CAN_LOCK;
            "vbib", [0x80], 4, CAN_LOCK;
            "v*r*", [0x21], 0, CAN_LOCK;
            "vbrb", [0x20], 0, CAN_LOCK;
            "r*v*", [0x23], 0, CAN_LOCK;
            "rbvb", [0x22], 0, CAN_LOCK;
        ), "bsf" => Op!(
            "r*v*", [0x0F, 0xBC];
        ), "bsr" => Op!(
            "r*v*", [0x0F, 0xBD];
        ), "bswap" => Op!(
            "rd", [0x0F, 0xC8];
            "rq", [0x0F, 0xC8];
        ), "bt"  => Op!(
            "v*r*", [0x0F, 0xA3];
            "v*ib", [0x0F, 0xBA], 4;
        ), "btc" => Op!(
            "v*r*", [0x0F, 0xBB], 0, CAN_LOCK;
            "v*ib", [0x0F, 0xBA], 7, CAN_LOCK;
        ), "btr" => Op!(
            "v*r*", [0x0F, 0xB3], 0, CAN_LOCK;
            "v*ib", [0x0F, 0xBA], 6, CAN_LOCK;
        ), "bts" => Op!(
            "v*r*", [0x0F, 0xAB], 0, CAN_LOCK;
            "v*ib", [0x0F, 0xBA], 5, CAN_LOCK;
        ), "call" => Op!(
            "o*", [0xE8];
            "r*", [0xFF], 2;
        ), "cbw" => Op!(
            "", [0x98], 0, REQUIRES_OPSIZE;
        ), "cwde" => Op!(
            "", [0x98];
        ), "cdqe" => Op!(
            "", [0x98], 0, REQUIRES_REXSIZE;
        ), "cwd" => Op!(
            "", [0x99], 0, REQUIRES_OPSIZE;
        ), "cdq" => Op!(
            "", [0x99];
        ), "cqo" => Op!(
            "", [0x99], 0, REQUIRES_REXSIZE;
        ), "clc" => Op!(
            "", [0xF8];
        ), "cld" => Op!(
            "", [0xFC];
        ), "clflush" => Op!(
            "mb", [0x0F, 0xAE], 7;
        ), "cmc" => Op!(
            "", [0xF5];
        ), "cmovo" => Op!(
            "r*v*", [0x0F, 0x40];
        ), "cmovno" => Op!(
            "r*v*", [0x0F, 0x41];
        ), "cmovb"   | "cmovc"  | "cmovnae" => Op!(
            "r*v*", [0x0F, 0x42];
        ), "cmovnb"  | "cmovnc" | "cmovae"  => Op!(
            "r*v*", [0x0F, 0x43];
        ), "cmovz"   | "cmove"  => Op!(
            "r*v*", [0x0F, 0x44];
        ), "cmovnz"  | "cmovne" => Op!(
            "r*v*", [0x0F, 0x45];
        ), "cmovbe"  | "cmovna" => Op!(
            "r*v*", [0x0F, 0x46];
        ), "cmovnbe" | "cmova"  => Op!(
            "r*v*", [0x0F, 0x47];
        ), "cmovs"   => Op!(
            "r*v*", [0x0F, 0x48];
        ), "cmovns"  => Op!(
            "r*v*", [0x0F, 0x49];
        ), "cmovp"   | "cmovpe" => Op!(
            "r*v*", [0x0F, 0x4A];
        ), "cmovnp"  | "cmovpo" => Op!(
            "r*v*", [0x0F, 0x4B];
        ), "cmovl"   | "cmovnge"=> Op!(
            "r*v*", [0x0F, 0x4C];
        ), "cmovnl"  | "cmovge" => Op!(
            "r*v*", [0x0F, 0x4D];
        ), "cmovle"  | "cmovng" => Op!(
            "r*v*", [0x0F, 0x4E];
        ), "cmovnle" | "cmovg"  => Op!(
            "r*v*", [0x0F, 0x4F];
        ), "cmp" => Op!(
            "A*i*", [0x3C];
            "Abib", [0x3D];
            "v*i*", [0x81], 7;
            "v*ib", [0x83], 7;
            "vbib", [0x80], 7;
            "v*r*", [0x39];
            "vbrb", [0x38];
            "r*v*", [0x3B];
            "rbvb", [0x3A];
        ), "cmpsb" => Op!(
            "", [0xA6], 0, CAN_REP;
        ), "cmpsw" => Op!(
            "", [0xA7], 0, CAN_REP | REQUIRES_OPSIZE;
        ), "cmpsd" => Op!(
            "", [0xA7], 0, CAN_REP;
        ), "cmpsq" => Op!(
            "", [0xA7], 0, CAN_REP | REQUIRES_REXSIZE;
        ), "cmpxchg" => Op!(
            "v*r*", [0x0F, 0xB1], 0, CAN_LOCK;
            "vbrb", [0x0F, 0xB0], 0, CAN_LOCK;
        ), "cmpxchg8b"  => Op!(
            "mq", [0x0F, 0xC7], 1, SIZE_OVERRIDE | CAN_LOCK;
        ), "cmpxchg16b" => Op!(
            "mo", [0x0F, 0xC7], 1, SIZE_OVERRIDE | CAN_LOCK | REQUIRES_REXSIZE;
        ), "cpuid" => Op!(
            "", [0x0F, 0xA2];
        ), "dec" => Op!(
            "v*", [0xFF], 1, CAN_LOCK;
            "vb", [0xFE], 1, CAN_LOCK;
        ), "div" => Op!(
            "v*", [0xF7], 6;
            "vb", [0xF6], 6;
        ), "enter" => Op!(
            "iwib", [0xC8], 0, SIZE_OVERRIDE;
        ), "idiv" => Op!(
            "v*", [0xF7], 7;
            "vb", [0xF6], 7;
        ), "imul" => Op!(
            "v*",     [0xF7], 5;
            "vb",     [0xF6], 5;
            "r*v*",   [0x0F, 0xAF];
            "r*v*i*", [0x69];
            "r*v*ib", [0x68];
        ), "in" => Op!(
            "Abib", [0xE4];
            "Awib", [0xE5];
            "Adib", [0xE5];
            "AbCw", [0xEC], 0, SIZE_OVERRIDE;
            "AwCw", [0xED], 0, SIZE_OVERRIDE | REQUIRES_OPSIZE;
            "AdCw", [0xED], 0, SIZE_OVERRIDE;
        ), "inc" => Op!(
            "v*", [0xFF], 0, CAN_LOCK;
            "vb", [0xFE], 0, CAN_LOCK;
        ), "insb" => Op!(
            "", [0x6C];
        ), "insw" => Op!(
            "", [0x6D], 0, REQUIRES_OPSIZE;
        ), "insd" => Op!(
            "", [0x6D];
        ), "int" => Op!(
            "ib", [0xCD];
        ), "jo"   => Op!(
            "o*", [0x0F, 0x80];
            "ob", [0x70];
        ), "jno"  => Op!(
            "o*", [0x0F, 0x81];
            "ob", [0x71];
        ), "jb"   | "jc"  | "jnae" => Op!(
            "o*", [0x0F, 0x82];
            "ob", [0x72];
        ), "jnb"  | "jnc" | "jae"  => Op!(
            "o*", [0x0F, 0x83];
            "ob", [0x73];
        ), "jz"   | "je"  => Op!(
            "o*", [0x0F, 0x84];
            "ob", [0x74];
        ), "jnz"  | "jne" => Op!(
            "o*", [0x0F, 0x85];
            "ob", [0x75];
        ), "jbe"  | "jna" => Op!(
            "o*", [0x0F, 0x86];
            "ob", [0x76];
        ), "jnbe" | "ja"  => Op!(
            "o*", [0x0F, 0x87];
            "ob", [0x77];
        ), "js"   => Op!(
            "o*", [0x0F, 0x88];
            "ob", [0x78];
        ), "jns"  => Op!(
            "o*", [0x0F, 0x89];
            "ob", [0x79];
        ), "jp"   | "jpe" => Op!(
            "o*", [0x0F, 0x8A];
            "ob", [0x7A];
        ), "jnp"  | "jpo" => Op!(
            "o*", [0x0F, 0x8B];
            "ob", [0x7B];
        ), "jl"   | "jnge"=> Op!(
            "o*", [0x0F, 0x8C];
            "ob", [0x7C];
        ), "jnl"  | "jge" => Op!(
            "o*", [0x0F, 0x8D];
            "ob", [0x7D];
        ), "jle"  | "jng" => Op!(
            "o*", [0x0F, 0x8E];
            "ob", [0x7E];
        ), "jnle" | "jg"  => Op!(
            "o*", [0x0F, 0x8F];
            "ob", [0x7F];
        ), "jecxz" => Op!(
            "ob", [0xE3], 0, REQUIRES_ADDRSIZE;
        ), "jrcxz" => Op!(
            "ob", [0xE3];
        ), "jmp" => Op!(
            "o*", [0xE9];
            "ob", [0xEB];
            "v*", [0xFF], 4, DEFAULT_REXSIZE;
        ), "lahf" => Op!(
            "", [0x9F];
        ), "lfs" => Op!(
            "r*m!", [0x0F, 0xB4];
        ), "lgs" => Op!(
            "r*m!", [0x0F, 0xB5];
        ), "lss" => Op!(
            "r*m!", [0x0F, 0xB2];
        ), "lea" => Op!(
            "r*m!", [0x8D];
        ), "leave" => Op!(
            "", [0xC9];
        ), "lfence" => Op!(
            "", [0x0F, 0xAE, 0xE8];
        ), "lodsb" => Op!(
            "", [0xAC];
        ), "lodsw" => Op!(
            "", [0xAD], 0, REQUIRES_OPSIZE;
        ), "lodsd" => Op!(
            "", [0xAD];
        ), "lodsq" => Op!(
            "", [0xAD], 0, REQUIRES_REXSIZE;
        ), "loop" => Op!(
            "ob", [0xE2];
        ), "loope" | "loopz" => Op!(
            "ob", [0xE1];
        ), "loopne" | "loopnz" => Op!(
            "ob", [0xE0];
        ), "lzcnt" => Op!(
            "r*m*", [0x0F, 0xBD], 0, REQUIRES_REP;
        ), "mfence" => Op!(
            "", [0x0F, 0xAE, 0xF0];
        ), "mov" => Op!(
            "v*r*", [0x89];
            "vbrb", [0x88];
            "r*v*", [0x8B];
            "rbvb", [0x8A];
            "rwsw", [0x8C], 0, SIZE_OVERRIDE | REQUIRES_OPSIZE;
            "rdsw", [0x8C], 0, SIZE_OVERRIDE;
            "rqsw", [0x8C], 0, SIZE_OVERRIDE | REQUIRES_REXSIZE;
            "mwsw", [0x8C], 0, SIZE_OVERRIDE;
            "swmw", [0x8C], 0, SIZE_OVERRIDE;
            "swr*", [0x8C], 0, SIZE_OVERRIDE | DEST_IN_REG;
            "rbib", [0xB0], 0, REGISTER_IN_OPCODE;
            "rwiw", [0xB8], 0, REGISTER_IN_OPCODE;
            "rdid", [0xB8], 0, REGISTER_IN_OPCODE;
            "v*i*", [0xC7], 0;
            "rqiq", [0xB8], 0, REGISTER_IN_OPCODE;
            "vbib", [0xC6], 0;
            "cdrd", [0x0F, 0x22], 0, DEST_IN_REG; // 32 bit mode only
            "cqrq", [0x0F, 0x22], 0, DEST_IN_REG | DEFAULT_REXSIZE;
            "rdcd", [0x0F, 0x20];
            "rqcq", [0x0F, 0x20], 0, DEFAULT_REXSIZE;
            "Wdrd", [0x0F, 0x22], 0, REQUIRES_LOCK;
            "Wqrq", [0x0F, 0x22], 0, REQUIRES_LOCK | DEFAULT_REXSIZE;
            "rdWd", [0x0F, 0x22], 0, REQUIRES_LOCK;
            "rqWq", [0x0F, 0x22], 0, REQUIRES_LOCK | DEFAULT_REXSIZE;
            "ddrd", [0x0F, 0x23], 0, DEST_IN_REG; // 32 bit mode only
            "dqrq", [0x0F, 0x23], 0, DEST_IN_REG | DEFAULT_REXSIZE;
            "rddd", [0x0F, 0x21];
            "rqdq", [0x0F, 0x21], 0, DEFAULT_REXSIZE;
        ), "movabs" => Op!( // special syntax for 64-bit disp only mov
            "Abib", [0xA0];
            "Awiw", [0xA1];
            "Adid", [0xA1];
            "Aqiq", [0xA1];
            "ibAb", [0xA2];
            "iwAw", [0xA3];
            "idAd", [0xA3];
            "iqAq", [0xA3];
        ), "movbe" => Op!(
            "r*m*", [0x0F, 0x38, 0xF0];
            "m*r*", [0x0F, 0x38, 0xF1];
        ), "movd" => Op!(
            "yovd", [0x0F, 0x6E], 0, SIZE_OVERRIDE | REQUIRES_OPSIZE;
            "yovq", [0x0F, 0x6E], 0, SIZE_OVERRIDE | REQUIRES_OPSIZE | REQUIRES_REXSIZE;
            "vdyo", [0x0F, 0x7E], 0, SIZE_OVERRIDE | REQUIRES_OPSIZE;
            "vqyo", [0x0F, 0x7E], 0, SIZE_OVERRIDE | REQUIRES_OPSIZE | REQUIRES_REXSIZE;
            "xqvd", [0x0F, 0x6E], 0, SIZE_OVERRIDE;
            "xqvq", [0x0F, 0x6E], 0, SIZE_OVERRIDE | REQUIRES_REXSIZE;
            "vdxq", [0x0F, 0x7E], 0, SIZE_OVERRIDE;
            "vqxq", [0x0F, 0x7E], 0, SIZE_OVERRIDE | REQUIRES_REXSIZE;
        ), "movmskpd" => Op!(
            "rdyo", [0x0F, 0x50], 0, DEST_IN_REG | SIZE_OVERRIDE | REQUIRES_OPSIZE;
        ), "movmskps" => Op!(
            "rdyo", [0x0F, 0x50], 0, DEST_IN_REG | SIZE_OVERRIDE;
        ), "movnti" => Op!(
            "mdrd", [0x0F, 0xC3];
            "mqrq", [0x0F, 0xC3];
        ), "movsb" => Op!(
            "", [0xA4]; 
        ), "movsw" => Op!(
            "", [0xA5], 0, REQUIRES_OPSIZE;
        ), "movsd" => Op!(
            "", [0xA5];
        ), "movsq" => Op!(
            "", [0xA5], 0, REQUIRES_REXSIZE; 
        ), "movsx" => Op!( // currently this defaults to a certain memory size
            "rdvw", [0x0F, 0xBF], 0, SIZE_OVERRIDE;
            "rqvw", [0x0F, 0xBF], 0, SIZE_OVERRIDE | REQUIRES_REXSIZE;
            "rwvb", [0x0F, 0xBE], 0, SIZE_OVERRIDE | REQUIRES_OPSIZE;
            "rdvb", [0x0F, 0xBE], 0, SIZE_OVERRIDE;
            "rqvb", [0x0F, 0xBE], 0, SIZE_OVERRIDE | REQUIRES_REXSIZE;
        ), "movsxd" => Op!(
            "rqvd", [0x63], 0, SIZE_OVERRIDE | REQUIRES_REXSIZE;
        ), "movzx" => Op!( // currently this defaults to a certain memory size
            "rdvw", [0x0F, 0xB7], 0, SIZE_OVERRIDE;
            "rqvw", [0x0F, 0xB7], 0, SIZE_OVERRIDE | REQUIRES_REXSIZE;
            "rwvb", [0x0F, 0xB6], 0, SIZE_OVERRIDE | REQUIRES_OPSIZE;
            "rdvb", [0x0F, 0xB6], 0, SIZE_OVERRIDE;
            "rqvb", [0x0F, 0xB6], 0, SIZE_OVERRIDE | REQUIRES_REXSIZE;
        ), "mul" => Op!(
            "v*", [0xF7], 4;
            "vb", [0xF6], 4;
        ), "neg" => Op!(
            "v*", [0xF7], 3, CAN_LOCK;
            "vb", [0xF6], 3, CAN_LOCK;
        ), "nop" => Op!(
            "", [0x90];
            "v*", [0x0F, 0x1F], 0;
        ),"not" => Op!(
            "v*", [0xF7], 2, CAN_LOCK;
            "vb", [0xF6], 2, CAN_LOCK;
        ), "or" => Op!(
            "A*i*", [0x0D], 0, CAN_LOCK;
            "Abib", [0x0C], 0, CAN_LOCK;
            "v*i*", [0x81], 1, CAN_LOCK;
            "v*ib", [0x83], 1, CAN_LOCK;
            "vbib", [0x80], 1, CAN_LOCK;
            "v*r*", [0x09], 0, CAN_LOCK;
            "vbrb", [0x08], 0, CAN_LOCK;
            "r*v*", [0x0B], 0, CAN_LOCK;
            "rbvb", [0x0A], 0, CAN_LOCK;
        ), "out" => Op!(
            "ibAb", [0xE6];
            "ibAw", [0xE7];
            "ibAd", [0xE7];
            "CwAb", [0xEE], 0, SIZE_OVERRIDE;
            "CwAw", [0xEF], 0, SIZE_OVERRIDE | REQUIRES_OPSIZE;
            "CwAd", [0xEF], 0, SIZE_OVERRIDE;
        ), "outsb" => Op!(
            "", [0x6E], 0, CAN_REP;
        ), "outsw" => Op!(
            "", [0x6F], 0, CAN_REP | REQUIRES_OPSIZE ;
        ), "outsd" => Op!(
            "", [0x6F], 0, CAN_REP;
        ), "pause" => Op!(
            "", [0xF3, 0x90];
        ), "pop" => Op!(
            "r*", [0x8F], 0, DEFAULT_REXSIZE;
            "v*", [0x58], 0, DEFAULT_REXSIZE | REGISTER_IN_OPCODE;
            "Uw", [0x0F, 0xA1], 0, SIZE_OVERRIDE;
            "vw", [0x0F, 0xA9], 0, SIZE_OVERRIDE;
        ), "popcnt" => Op!(
            "r*v*", [0x0F, 0xB8], 0, REQUIRES_REP;
        ), "popf" => Op!(
            "", [0x9D], 0, REQUIRES_OPSIZE;
        ), "popfq" => Op!(
            "", [0x9D];
        ), "prefetch" => Op!(
            "mb", [0x0F, 0x0D], 0;
        ), "prefetchw" => Op!(
            "mb", [0x0F, 0x0D], 1;
        ), "prefetchnta" => Op!(
            "mb", [0x0F, 0x18], 0;
        ), "prefetcht0" => Op!(
            "mb", [0x0F, 0x18], 1;
        ), "prefetcht1" => Op!(
            "mb", [0x0F, 0x18], 2;
        ), "prefetcht2" => Op!(
            "mb", [0x0F, 0x18], 3;
        ), "push" => Op!(
            "r*", [0x50], 0, DEFAULT_REXSIZE | REGISTER_IN_OPCODE;
            "v*", [0xFF], 6, DEFAULT_REXSIZE;
            "ib", [0x6A], 0;
            "iw", [0x68], 0, REQUIRES_OPSIZE;
            "iq", [0x68], 0;
            "Uw", [0x0F, 0xA0], 0, SIZE_OVERRIDE;
            "vw", [0x0F, 0xA8], 0, SIZE_OVERRIDE;
        ), "pushf" => Op!(
            "", [0x9C], 0, REQUIRES_OPSIZE;
        ), "pushfq" => Op!(
            "", [0x9C];
        ), "rcl" => Op!( // shift by one forms not supported as they'd never be used
            "v*Bb", [0xD3], 2;
            "vbBb", [0xD2], 2;
            "v*ib", [0xC1], 2;
            "vbib", [0xC0], 2;
        ), "rcr" => Op!(
            "v*Bb", [0xD3], 3;
            "vbBb", [0xD2], 3;
            "v*ib", [0xC1], 3;
            "vbib", [0xC0], 3;
        ), "rdfsbase" => Op!(
            "rd", [0x0F, 0xAE], 0, REQUIRES_REP;
            "rq", [0x0F, 0xAE], 0, REQUIRES_REP;
        ), "rdgsbase" => Op!(
            "rd", [0x0F, 0xAE], 1, REQUIRES_REP;
            "rq", [0x0F, 0xAE], 1, REQUIRES_REP;
        ), "rdrand" => Op!(
            "r*", [0x0F, 0xC7], 6;
        ), "ret" => Op!(
            "", [0xC3];
            "iw", [0xC2], 0, SIZE_OVERRIDE;
        ), "rol" => Op!(
            "v*Bb", [0xD3], 0;
            "vbBb", [0xD2], 0;
            "v*ib", [0xC1], 0;
            "vbib", [0xC0], 0;
        ), "ror" => Op!(
            "v*Bb", [0xD3], 1;
            "vbBb", [0xD2], 1;
            "v*ib", [0xC1], 1;
            "vbib", [0xC0], 1;
        ), "sahf" => Op!(
           "", [0x9E];
        ), "sal" | "shl" => Op!(
            "v*Bb", [0xD3], 4;
            "vbBb", [0xD2], 4;
            "v*ib", [0xC1], 4;
            "vbib", [0xC0], 4;
        ), "sar" => Op!(
            "v*Bb", [0xD3], 7;
            "vbBb", [0xD2], 7;
            "v*ib", [0xC1], 7;
            "vbib", [0xC0], 7;
        ), "sbb" => Op!(
            "A*i*", [0x1D], 0, CAN_LOCK;
            "Abib", [0x1C], 0, CAN_LOCK;
            "v*i*", [0x81], 3, CAN_LOCK;
            "v*ib", [0x83], 3, CAN_LOCK;
            "vbib", [0x80], 3, CAN_LOCK;
            "v*r*", [0x19], 0, CAN_LOCK;
            "vbrb", [0x18], 0, CAN_LOCK;
            "r*v*", [0x1B], 0, CAN_LOCK;
            "rbvb", [0x1A], 0, CAN_LOCK;
        ), "scasb" => Op!(
            "", [0xAE], 0, CAN_REP;
        ), "scasw" => Op!(
            "", [0xAF], 0, CAN_REP | REQUIRES_OPSIZE;
        ), "scasd" => Op!(
            "", [0xAF], 0, CAN_REP;
        ), "scasq" => Op!(
            "", [0xAF], 0, CAN_REP | REQUIRES_REXSIZE;
        ), "seto" => Op!(
            "vb", [0x0F, 0x90], 0;
        ), "setno" => Op!(
            "vb", [0x0F, 0x91], 0;
        ), "setb"   | "setc"  | "setnae" => Op!(
            "vb", [0x0F, 0x92], 0;
        ), "setnb"  | "setnc" | "setae"  => Op!(
            "vb", [0x0F, 0x93], 0;
        ), "setz"   | "sete"  => Op!(
            "vb", [0x0F, 0x94], 0;
        ), "setnz"  | "setne" => Op!(
            "vb", [0x0F, 0x95], 0;
        ), "setbe"  | "setna" => Op!(
            "vb", [0x0F, 0x96], 0;
        ), "setnbe" | "seta"  => Op!(
            "vb", [0x0F, 0x97], 0;
        ), "sets"   => Op!(
            "vb", [0x0F, 0x98], 0;
        ), "setns"  => Op!(
            "vb", [0x0F, 0x99], 0;
        ), "setp"   | "setpe" => Op!(
            "vb", [0x0F, 0x9A], 0;
        ), "setnp"  | "setpo" => Op!(
            "vb", [0x0F, 0x9B], 0;
        ), "setl"   | "setnge"=> Op!(
            "vb", [0x0F, 0x9C], 0;
        ), "setnl"  | "setge" => Op!(
            "vb", [0x0F, 0x9D], 0;
        ), "setle"  | "setng" => Op!(
            "vb", [0x0F, 0x9E], 0;
        ), "setnle" | "setg"  => Op!(
            "vb", [0x0F, 0x9F], 0;
        ), "sfence" => Op!(
            "", [0x0F, 0xAE, 0xF8];
        ), "shld" => Op!(
            "v*r*Bb", [0x0F, 0xA5];
            "v*r*ib", [0x0F, 0xA4];
        ), "shr" => Op!(
            "v*Bb", [0xD3], 5;
            "vbBb", [0xD2], 5;
            "v*ib", [0xC1], 5;
            "vbib", [0xC0], 5;
        ), "shrd" => Op!(
            "v*r*Bb", [0x0F, 0xAD];
            "v*r*ib", [0x0F, 0xAC];
        ), "stc" => Op!(
            "", [0xF9];
        ), "std" => Op!(
            "", [0xFD];
        ), "stosb" => Op!(
            "", [0xAA], 0, CAN_REP;
        ), "stosw" => Op!(
            "", [0xAB], 0, CAN_REP | REQUIRES_OPSIZE;
        ), "stosd" => Op!(
            "", [0xAB], 0, CAN_REP;
        ), "stosq" => Op!(
            "", [0xAB], 0, CAN_REP | REQUIRES_REXSIZE;
        ), "sub" => Op!(
            "A*i*", [0x2D], 0, CAN_LOCK;
            "Abib", [0x2C], 0, CAN_LOCK;
            "v*i*", [0x81], 5, CAN_LOCK;
            "v*ib", [0x83], 5, CAN_LOCK;
            "vbib", [0x80], 5, CAN_LOCK;
            "v*r*", [0x29], 0, CAN_LOCK;
            "vbrb", [0x28], 0, CAN_LOCK;
            "r*v*", [0x2B], 0, CAN_LOCK;
            "rbvb", [0x2A], 0, CAN_LOCK;
        ), "test" => Op!(
            "A*i*", [0xA9];
            "Abib", [0xA8];
            "v*i*", [0xF7], 0;
            "vbib", [0xF6], 0;
            "v*r*", [0x85];
            "vbrb", [0x84];
        ), "tzcnt" => Op!(
            "r*v*", [0x0F, 0xBC], 0, REQUIRES_REP;
        ), "wrfsbase" => Op!(
            "rd", [0x0F, 0xAE], 2, REQUIRES_REP;
            "rq", [0x0F, 0xAE], 2, REQUIRES_REP;
        ), "wrgsbase" => Op!(
            "rd", [0x0F, 0xAE], 3, REQUIRES_REP;
            "rq", [0x0F, 0xAE], 3, REQUIRES_REP;
        ), "xadd" => Op!(
            "v*r*", [0x0F, 0xC1], 0, CAN_LOCK;
            "vbrb", [0x0F, 0xC0], 0, CAN_LOCK;
        ), "xchg" => Op!(
            "A*r*", [0x90], 0, CAN_LOCK | REGISTER_IN_OPCODE;
            "r*A*", [0x90], 0, CAN_LOCK | REGISTER_IN_OPCODE;
            "v*r*", [0x87], 0, CAN_LOCK;
            "r*v*", [0x87], 0, CAN_LOCK;
            "vbrb", [0x86], 0, CAN_LOCK;
            "rbvb", [0x86], 0, CAN_LOCK;
        ), "xlatb" => Op!(
            "", [0xD7];
        ), "xor" => Op!(
            "A*i*", [0x35], 0, CAN_LOCK;
            "Abib", [0x34], 0, CAN_LOCK;
            "v*i*", [0x81], 6, CAN_LOCK;
            "v*ib", [0x83], 6, CAN_LOCK;
            "vbib", [0x80], 6, CAN_LOCK;
            "v*r*", [0x31], 0, CAN_LOCK;
            "vbrb", [0x30], 0, CAN_LOCK;
            "r*v*", [0x33], 0, CAN_LOCK;
            "rbvb", [0x32], 0, CAN_LOCK;
        ),
        // System instructions
        "clgi" => Op!(
            "", [0x0F, 0x01, 0xDD];
        ), "cli" => Op!(
            "", [0xFA];
        ), "clts" => Op!(
            "", [0x0F, 0x06];
        ), "hlt" => Op!(
            "", [0xF4];
        ), "int3" => Op!(
            "", [0xCC];
        ), "invd" => Op!(
            "", [0x0F, 0x08];
        ), "invlpg" => Op!(
            "mb", [0x0F, 0x01], 7;
        ), "invlpga" => Op!(
            "AqBd", [0x0F, 0x01, 0xDF];
        ), "iret" => Op!(
            "", [0xCF], 0, REQUIRES_OPSIZE;
        ), "iretd" => Op!(
            "", [0xCF];
        ), "iretq" => Op!(
            "", [0xCF], 0, REQUIRES_REXSIZE;
        ), "lar" => Op!(
            "rwvw", [0x0F, 0x02];
            "rdvw", [0x0F, 0x02], 0, SIZE_OVERRIDE;
            "rqvw", [0x0F, 0x02], 0, SIZE_OVERRIDE | REQUIRES_REXSIZE;
        ), "lgdt" => Op!(
            "m!", [0x0F, 0x01], 2;
        ), "lidt" => Op!(
            "m!", [0x0F, 0x01], 3;
        ), "lldt" => Op!(
            "vw", [0x0F, 0x00], 2, SIZE_OVERRIDE;
        ), "lmsw" => Op!(
            "vw", [0x0F, 0x01], 6, SIZE_OVERRIDE;
        ), "lsl" => Op!(
            "rwvw", [0x0F, 0x03];
            "rdvw", [0x0F, 0x03], 0, SIZE_OVERRIDE;
            "rqvw", [0x0F, 0x03], 0, SIZE_OVERRIDE | REQUIRES_REXSIZE;
        ), "ltr" => Op!(
            "vw", [0x0F, 0x00], 3, SIZE_OVERRIDE;
        ), "monitor" => Op!(
            "", [0x0F, 0x01, 0xC8];
        ), "monitorx" => Op!(
            "", [0x0F, 0x01, 0xFA];
        ), "mwait" => Op!(
            "", [0x0F, 0x01, 0xC9];
        ), "mwaitx" => Op!(
            "", [0x0F, 0x01, 0xFB];
        ), "rdmsr" => Op!(
            "", [0x0F, 0x32];
        ), "rdpmc" => Op!(
            "", [0x0F, 0x33];
        ), "rdtsc" => Op!(
            "", [0x0F, 0x31];
        ), "rdtscp" => Op!(
            "", [0x0F, 0x01, 0xF9];
        ), "rsm" => Op!(
            "", [0x0F, 0xAA];
        ), "sgdt" => Op!(
            "m!", [0x0F, 0x01], 0;
        ), "sidt" => Op!(
            "m!", [0x0F, 0x01, 1];
        ), "skinit" => Op!(
            "Ad", [0x0F, 0x01, 0xDE];
        ), "sldt" => Op!(
            "r*", [0x0F, 0x00], 0;
            "mw", [0x0F, 0x00], 0, SIZE_OVERRIDE;
        ), "smsw" => Op!(
            "r*", [0x0F, 0x01], 4;
            "mw", [0x0F, 0x01], 4, SIZE_OVERRIDE;
        ), "sti" => Op!(
            "", [0xFB];
        ), "stgi" => Op!(
            "", [0x0F, 0x01, 0xDC];
        ), "str" => Op!(
            "r*", [0x0F, 0x00], 1;
            "mw", [0x0F, 0x00], 1, SIZE_OVERRIDE;
        ), "swapgs" => Op!(
            "", [0x0F, 0x01, 0xF8];
        ), "syscall" => Op!(
            "", [0x0F, 0x05];
        ), "sysenter" => Op!(
            "", [0x0F, 0x34];
        ), "sysexit" => Op!(
            "", [0x0F, 0x35];
        ), "sysret" => Op!(
            "", [0x0F, 0x07];
        ), "ud2" => Op!(
            "", [0x0F, 0x0B];
        ), "verr" => Op!(
            "vw", [0x0F, 0x00], 4, SIZE_OVERRIDE;
        ), "verw" => Op!(
            "vw", [0x0F, 0x00], 5, SIZE_OVERRIDE;
        ), "vmload" => Op!(
            "Aq", [0x0F, 0x01, 0xDA];
        ), "vmmcall" => Op!(
            "", [0x0F, 0x01, 0xD9];
        ), "vmrun" => Op!(
            "Aq", [0x0F, 0x01, 0xD8];
        ), "vmsave" => Op!(
            "Aq", [0x0F, 0x01, 0xDB];
        ), "wbinvd" => Op!(
            "", [0x0F, 0x09];
        ), "wrmsr" => Op!(
            "", [0x0F, 0x30];
        ),
        // x87 FPU instruction set, data taken from amd's programmer manual vol. 5
        "f2xm1" => Op!(
            "", [0xD9, 0xF0];
        ), "fabs" => Op!(
            "", [0xD9, 0xE1];
        ), "fadd" => Op!(
            "Xpfp", [0xD8, 0xC0], 0, REGISTER_IN_OPCODE;
            "fpXp", [0xDC, 0xC0], 0, REGISTER_IN_OPCODE;
            "md", [0xD8], 0;
            "mq", [0xDC], 0, SIZE_OVERRIDE;
        ), "faddp" => Op!(
            "",     [0xDE, 0xC1], 0, REGISTER_IN_OPCODE;
            "fpXp", [0xDE, 0xC0], 0, REGISTER_IN_OPCODE;
        ), "fiadd" => Op!(
            "mw", [0xDE], 0, SIZE_OVERRIDE;
            "md", [0xDA], 0;
        ), "fbld" => Op!(
            "mp", [0xDF], 4;
        ), "fbstp" => Op!(
            "mp", [0xDF], 6;
        ), "fchs" => Op!(
            "", [0xD9, 0xE0];
        ), "fclex" => Op!( // this is actually ;wait ;fnclex
            "", [0x9B, 0xDB, 0xE2];
        ), "fnclex" => Op!(
            "", [0xDB, 0xE2];
        ), "fcmovb" => Op!(
            "Xpfp", [0xDA, 0xC0], 0, REGISTER_IN_OPCODE;
        ), "fcmovbe" => Op!(
            "Xpfp", [0xDA, 0xD0], 0, REGISTER_IN_OPCODE;
        ), "fcmove" => Op!(
            "Xpfp", [0xDA, 0xC8], 0, REGISTER_IN_OPCODE;
        ), "fcmovnb" => Op!(
            "Xpfp", [0xDB, 0xC0], 0, REGISTER_IN_OPCODE;
        ), "fcmovnbe" => Op!(
            "Xpfp", [0xDB, 0xD0], 0, REGISTER_IN_OPCODE;
        ), "fcmovne" => Op!(
            "Xpfp", [0xDB, 0xC8], 0, REGISTER_IN_OPCODE;
        ), "fcmovnu" => Op!(
            "Xpfp", [0xDB, 0xD8], 0, REGISTER_IN_OPCODE;
        ), "fcmovu" => Op!(
            "Xpfp", [0xDA, 0xD8], 0, REGISTER_IN_OPCODE;
        ), "fcom" => Op!(
            "", [0xD8, 0xD1];
            "fp", [0xD8, 0xD0], 0, REGISTER_IN_OPCODE;
            "md", [0xD8], 2;
            "mq", [0xDC], 2, SIZE_OVERRIDE;
        ), "fcomp" => Op!(
            "", [0xD8, 0xD9];
            "fp", [0xD8, 0xD8], 0, REGISTER_IN_OPCODE;
            "md", [0xD8], 3;
            "mq", [0xDC], 3, SIZE_OVERRIDE;
        ), "fcompp" => Op!(
            "", [0xDE, 0xD9];
        ), "fcomi" => Op!(
            "Xpfp", [0xDB, 0xF0], 0, REGISTER_IN_OPCODE;
        ), "fcomip" => Op!(
            "fpXp", [0xDF, 0xF0], 0, REGISTER_IN_OPCODE;
        ), "fcos" => Op!(
            "", [0xD9, 0xFF];
        ), "fdecstp" => Op!(
            "", [0xD9, 0xF6];
        ), "fdiv" => Op!(
            "Xpfp", [0xD8, 0xF0], 0, REGISTER_IN_OPCODE;
            "fpXp", [0xDC, 0xF8], 0, REGISTER_IN_OPCODE;
            "md", [0xD8], 6;
            "mq", [0xDC], 6, SIZE_OVERRIDE;
        ), "fdivp" => Op!(
            "", [0xDE, 0xF9];
            "fpXp", [0xDE, 0xF8], 0, REGISTER_IN_OPCODE;
        ), "fidiv" => Op!(
            "mw", [0xDE], 6, SIZE_OVERRIDE;
            "md", [0xDA], 6;
        ), "fdivr" => Op!(
            "Xpfp", [0xD8, 0xF8], 0, REGISTER_IN_OPCODE;
            "fpXp", [0xDC, 0xF0], 0, REGISTER_IN_OPCODE;
            "md", [0xD8], 7;
            "mq", [0xDC], 7, SIZE_OVERRIDE;
        ), "fdivrp" => Op!(
            "", [0xDE, 0xF1];
            "fpXp", [0xDE, 0xF0], 0, REGISTER_IN_OPCODE;
        ), "fidivr" => Op!(
            "mw", [0xDE], 7, SIZE_OVERRIDE;
            "md", [0xDA], 7;
        ), "ffree" => Op!(
            "fp", [0xDD, 0xC0], 0, REGISTER_IN_OPCODE;
        ), "ficom" => Op!(
            "mw", [0xDE], 2, SIZE_OVERRIDE;
            "md", [0xDA], 2;
        ), "ficomp" => Op!(
            "mw", [0xDE], 3, SIZE_OVERRIDE;
            "md", [0xDA], 3;
        ), "fild" => Op!(
            "mw", [0xDF], 0, SIZE_OVERRIDE;
            "md", [0xDB], 0;
            "mq", [0xDF], 5, SIZE_OVERRIDE;
        ), "fincstp" => Op!(
            "", [0xD9, 0xF7];
        ), "finit" => Op!( // this is actually ;wait ;fninit
            "", [0x9B, 0xDB, 0xE3];
        ), "fninit" => Op!(
            "", [0xDB, 0xE3];
        ), "fist" => Op!(
            "mw", [0xDF], 2, SIZE_OVERRIDE;
            "md", [0xDB], 2;
            "mw", [0xDF], 3, SIZE_OVERRIDE;
            "md", [0xDB], 3;
            "mq", [0xDF], 7, SIZE_OVERRIDE;
        ), "fisttp" => Op!(
            "mw", [0xDF], 1, SIZE_OVERRIDE;
            "md", [0xDB], 1;
            "mq", [0xDD], 1, SIZE_OVERRIDE;
        ), "fld" => Op!(
            "fp", [0xD9, 0xC0], 0, REGISTER_IN_OPCODE;
            "md", [0xD9], 0;
            "mq", [0xDD], 0, SIZE_OVERRIDE;
            "mp", [0xDB], 5;
        ), "fld1" => Op!(
            "", [0xD9, 0xE8];
        ), "fldcw" => Op!(
            "mw", [0xD9], 5, SIZE_OVERRIDE;
        ), "fldenv" => Op!(
            "m!", [0xD9], 4, SIZE_OVERRIDE;
        ), "fldenvw" => Op!(
            "m!", [0xD9], 4, SIZE_OVERRIDE | REQUIRES_OPSIZE;
        ), "fldl2e" => Op!(
            "", [0xD9, 0xEA];
        ), "fldl2t" => Op!(
            "", [0xD9, 0xE9];
        ), "fldlg2" => Op!(
            "", [0xD9, 0xEC];
        ), "fldln2" => Op!(
            "", [0xD9, 0xED];
        ), "fldpi" => Op!(
            "", [0xD9, 0xEB];
        ), "fldz" => Op!(
            "", [0xD9, 0xEE];
        ), "fmul" => Op!(
            "Xpfp", [0xD8, 0xC8], 0, REGISTER_IN_OPCODE;
            "fpXp", [0xDC, 0xC8], 0, REGISTER_IN_OPCODE;
            "md", [0xD8], 1;
            "mq", [0xDC], 1, SIZE_OVERRIDE;
        ), "fmulp" => Op!(
            "", [0xDE, 0xC9];
            "fpXp", [0xDE, 0xC8], 0, REGISTER_IN_OPCODE;
        ), "fimul" => Op!(
            "mw", [0xDE], 1, SIZE_OVERRIDE;
            "md", [0xDA], 1;
        ), "fnop" => Op!(
            "", [0xD9, 0xD0];
        ), "fpatan" => Op!(
            "", [0xD9, 0xF3];
        ), "fprem" => Op!(
            "", [0xD9, 0xF8];
        ), "fprem1" => Op!(
            "", [0xD9, 0xF5];
        ), "fptan" => Op!(
            "", [0xD9, 0xF2];
        ), "frndint" => Op!(
            "", [0xD9, 0xFC];
        ), "frstor" => Op!(
            "m!", [0xDD], 4, SIZE_OVERRIDE;
        ), "frstorw" => Op!(
            "m!", [0xDD], 4, SIZE_OVERRIDE | REQUIRES_OPSIZE;
        ), "fsave" => Op!( // note: this is actually ; wait; fnsavew
            "m!", [0x9B, 0xDD], 6, SIZE_OVERRIDE;
        ), "fsavew" => Op!( // note: this is actually ; wait; OPSIZE fnsave
            "m!", [0x9B, 0x66, 0xDD], 6, SIZE_OVERRIDE;
        ), "fnsave" => Op!(
            "m!", [0xDD], 6, SIZE_OVERRIDE;
        ), "fnsavew" => Op!(
            "m!", [0xDD], 6, SIZE_OVERRIDE | REQUIRES_OPSIZE;
        ), "fscale" => Op!(
            "", [0xD9, 0xFD];
        ), "fsin" => Op!(
            "", [0xD9, 0xFE];
        ), "fsincos" => Op!(
            "", [0xD9, 0xFB];
        ), "fsqrt" => Op!(
            "", [0xD9, 0xFA];
        ), "fst" => Op!(
            "fp", [0xDD, 0xD0], 0, REGISTER_IN_OPCODE;
            "md", [0xD9], 2;
            "mq", [0xDD], 2, SIZE_OVERRIDE;
        ), "fstp" => Op!(
            "fp", [0xDD, 0xD8], 0, REGISTER_IN_OPCODE;
            "md", [0xD9], 3;
            "mq", [0xDD], 3, SIZE_OVERRIDE;
            "mp", [0xDB], 7;
        ), "fstcw" => Op!( // note: this is actually ; wait; fnstcw
            "mw", [0x9B, 0xD9], 7, SIZE_OVERRIDE;
        ), "fnstcw" => Op!(
            "mw", [0xD9], 7, SIZE_OVERRIDE;
        ), "fstenv" => Op!( // note: this is actually ; wait; fnstenv
            "m!", [0x9B, 0xD9], 6, SIZE_OVERRIDE;
        ), "fstenvw" => Op!( // note: this is actually ; wait; OPSIZE fnsten
            "m!", [0x9B, 0x66, 0xD9], 6, SIZE_OVERRIDE;
        ), "fnstenv" => Op!(
            "m!", [0xD9], 6, SIZE_OVERRIDE;
        ), "fnstenvw" => Op!(
            "m!", [0xD9], 6, SIZE_OVERRIDE | REQUIRES_OPSIZE;
        ), "fstsw" => Op!( // note: this is actually ; wait; fnstsw
            "Aw", [0x9B, 0xDF, 0xE0], 0, SIZE_OVERRIDE;
            "mw", [0x9B, 0xDD], 7, SIZE_OVERRIDE;
        ), "fnstsw" => Op!(
            "Aw", [0xDF, 0xE0], 0, SIZE_OVERRIDE;
            "mw", [0xDD], 7, SIZE_OVERRIDE;
        ), "fsub" => Op!(
            "Xpfp", [0xD8, 0xE0], 0, REGISTER_IN_OPCODE;
            "fpXp", [0xDC, 0xE8], 0, REGISTER_IN_OPCODE;
            "md", [0xD8], 4;
            "mq", [0xDC], 4, SIZE_OVERRIDE;
        ), "fsubp" => Op!(
            "", [0xDE, 0xE9];
            "fpXp", [0xDE, 0xE8], 0, REGISTER_IN_OPCODE;
        ), "fisub" => Op!(
            "mw", [0xDE], 4, SIZE_OVERRIDE;
            "md", [0xDA], 4;
        ), "fsubr" => Op!(
            "Xpfp", [0xD8, 0xE8], 0, REGISTER_IN_OPCODE;
            "fpXp", [0xDC, 0xE0], 0, REGISTER_IN_OPCODE;
            "md", [0xD8], 5;
            "mq", [0xDC], 5, SIZE_OVERRIDE;
        ), "fsubrp" => Op!(
            "", [0xDE, 0xE1];
            "fpXp", [0xDE, 0xE0], 0, REGISTER_IN_OPCODE;
        ), "fisubr" => Op!(
            "mw", [0xDE], 5, SIZE_OVERRIDE;
            "md", [0xDA], 5;
        ), "ftst" => Op!(
            "", [0xD9, 0xE4];
        ), "fucom" => Op!(
            "", [0xDD, 0xE1];
            "fp", [0xDD, 0xE0], 0, REGISTER_IN_OPCODE;
        ), "fucomp" => Op!(
            "", [0xDD, 0xE9];
            "fp", [0xDD, 0xE8], 0, REGISTER_IN_OPCODE;
        ), "fucompp" => Op!(
            "", [0xDA, 0xE9];
        ), "fucomi" => Op!(
            "Xpfp", [0xDB, 0xE8], 0, REGISTER_IN_OPCODE;
            "fpXp", [0xDF, 0xE8], 0, REGISTER_IN_OPCODE;
        ), "fwait" | "wait" => Op!(
            "", [0x9B];
        ), "fxam" => Op!(
            "", [0xD9, 0xE5];
        ), "fxch" => Op!(
            "", [0xD9, 0xC9];
            "fp", [0xD9, 0xC8], 0, REGISTER_IN_OPCODE;
        ), "fxrstor" => Op!(
            "m!", [0x0F, 0xAE], 1, SIZE_OVERRIDE;
        ), "fxsave" => Op!(
            "m!", [0x0F, 0xAE], 0, SIZE_OVERRIDE;
        ), "fxtract" => Op!(
            "", [0xD9, 0xF4];
        ), "fyl2x" => Op!(
            "", [0xD9, 0xF1];
        ), "fyl2xp1" => Op!(
            "", [0xD9, 0xF9];
        ),

        _ => return None
    })
}
