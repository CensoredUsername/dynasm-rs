use compiler::{Opdata, flags};

macro_rules! constify {
    ($t:ty, $e:expr) => { {const C: &'static $t = &$e; C} }
}

macro_rules! Op {
    ($( $( $e:expr ),+ ; )+)                   => { {const C: &'static [Opdata] = &[$( Op!($( $e ),*) ,)+]; C} };
    (           $ops:expr)                     => { Opdata {args: "",   ops: constify!([u8], $ops), reg: 0   , flags:  0}  };
    ($fmt:expr, $ops:expr)                     => { Opdata {args: $fmt, ops: constify!([u8], $ops), reg: 0   , flags:  0}  };
    ($fmt:expr, $ops:expr, $reg:expr)          => { Opdata {args: $fmt, ops: constify!([u8], $ops), reg: $reg, flags:  0}  };
    ($fmt:expr, $ops:expr, $reg:expr, $f:expr) => { Opdata {args: $fmt, ops: constify!([u8], $ops), reg: $reg, flags: $f}  };
}

pub fn get_mnemnonic_data(name: &str) -> Option<&'static [Opdata]> {
    // note: currently only listing instructions that are usable in long mode, with 64-bit addrs, no VEX/XOP prefixes or segment overrides, without requiring privileges, that are not an extension
    // this helps preserve my sanity for now

    // I blame intel for the following match
    Some(match name {
        "adc" => Op!(
            "v*i*", [0x81], 2, flags::CAN_LOCK;
            "v*ib", [0x83], 2, flags::CAN_LOCK;
            "vbib", [0x80], 2, flags::CAN_LOCK;
            "v*r*", [0x11], 0, flags::CAN_LOCK;
            "vbrb", [0x10], 0, flags::CAN_LOCK;
            "r*v*", [0x13], 0, flags::CAN_LOCK;
            "rbvb", [0x12], 0, flags::CAN_LOCK;
        ), "add" => Op!(
            "v*i*", [0x81], 0, flags::CAN_LOCK;
            "v*ib", [0x83], 0, flags::CAN_LOCK;
            "vbib", [0x80], 0, flags::CAN_LOCK;
            "v*r*", [0x01], 0, flags::CAN_LOCK;
            "vbrb", [0x00], 0, flags::CAN_LOCK;
            "r*v*", [0x03], 0, flags::CAN_LOCK;
            "rbvb", [0x02], 0, flags::CAN_LOCK;
        ), "and" => Op!(
            "v*i*", [0x81], 4, flags::CAN_LOCK;
            "v*ib", [0x83], 4, flags::CAN_LOCK;
            "vbib", [0x80], 4, flags::CAN_LOCK;
            "v*r*", [0x21], 0, flags::CAN_LOCK;
            "vbrb", [0x20], 0, flags::CAN_LOCK;
            "r*v*", [0x23], 0, flags::CAN_LOCK;
            "rbvb", [0x22], 0, flags::CAN_LOCK;
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
            "v*r*", [0x0F, 0xBB], 0, flags::CAN_LOCK;
            "v*ib", [0x0F, 0xBA], 7, flags::CAN_LOCK;
        ), "btr" => Op!(
            "v*r*", [0x0F, 0xB3], 0, flags::CAN_LOCK;
            "v*ib", [0x0F, 0xBA], 6, flags::CAN_LOCK;
        ), "bts" => Op!(
            "v*r*", [0x0F, 0xAB], 0, flags::CAN_LOCK;
            "v*ib", [0x0F, 0xBA], 5, flags::CAN_LOCK;
        ), "call" => Op!(
            "c*", [0xE8];
            "r*", [0xFF], 2;
        ), "cbw"  | "cwde" | "cdqe" => Op!(
            [0x98];
        ), "cwd" | "cdq" | "cqo" => Op!(
            [0x99];
        ), "clc" => Op!(
            [0xF8];
        ), "cld" => Op!(
            [0xFC];
        ), "clflush" => Op!(
            "mb", [0x0F, 0xAE], 7;
        ), "cmc" => Op!(
            [0xF5];
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
            "v*i*", [0x81], 7;
            "v*ib", [0x83], 7;
            "vbib", [0x80], 7;
            "v*r*", [0x39];
            "vbrb", [0x38];
            "r*v*", [0x3B];
            "rbvb", [0x3A];
        ), "cmpsb" => Op!(
            "", [0xA6], 0, flags::CAN_REP;
        ), "cmpsw" => Op!(
            "", [0xA7], 0, flags::CAN_REP | flags::REQUIRES_OPSIZE;
        ), "cmpsd" => Op!(
            "", [0xA7], 0, flags::CAN_REP;
        ), "cmpsq" => Op!(
            "", [0xA7], 0, flags::CAN_REP | flags::REQUIRES_REXSIZE;
        ), "cmpxchg" => Op!(
            "v*r*", [0x0F, 0xB1], 0, flags::CAN_LOCK;
            "vbrb", [0x0F, 0xB0], 0, flags::CAN_LOCK;
        ), "cmpxchg8b"  => Op!( // actually a QWORD
            "m!", [0x0F, 0xC7], 1, flags::SIZE_OVERRIDE | flags::CAN_LOCK;
        ), "cmpxchg16b" => Op!( // actually an OWORD
            "m!", [0x0F, 0xC7], 1, flags::SIZE_OVERRIDE | flags::CAN_LOCK | flags::REQUIRES_REXSIZE;
        ), "cpuid" => Op!(
            [0x0F, 0xA2];
        ), "dec" => Op!(
            "v*", [0xFF], 1, flags::CAN_LOCK;
            "vb", [0xFE], 1, flags::CAN_LOCK;
        ), "div" => Op!(
            "v*", [0xF7], 6;
            "vb", [0xF6], 6;
        ), "enter" => Op!(
            "iwib", [0xC8], 0, flags::SIZE_OVERRIDE;
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
            "r*i*", [0xE5], 0, flags::EAX_ONLY;
            "rbi*", [0xE4], 0, flags::EAX_ONLY;
            "r*",   [0xED], 0, flags::EAX_ONLY;
            "rb",   [0xEC], 0, flags::EAX_ONLY;
        ), "inc" => Op!(
            "v*", [0xFF], 0, flags::CAN_LOCK;
            "vb", [0xFE], 0, flags::CAN_LOCK;
        ), "insb" => Op!(
            [0x6C];
        ), "insw" => Op!(
            "", [0x6D], 0, flags::REQUIRES_OPSIZE;
        ), "insd" => Op!(
            [0x6D];
        ), "int" => Op!(
            "ib", [0xCD];
        ), "jo"   => Op!(
            "c*", [0x0F, 0x80];
            "cb", [0x70];
        ), "jno"  => Op!(
            "c*", [0x0F, 0x81];
            "cb", [0x71];
        ), "jb"   | "jc"  | "jnae" => Op!(
            "c*", [0x0F, 0x82];
            "cb", [0x72];
        ), "jnb"  | "jnc" | "jae"  => Op!(
            "c*", [0x0F, 0x83];
            "cb", [0x73];
        ), "jz"   | "je"  => Op!(
            "c*", [0x0F, 0x84];
            "cb", [0x74];
        ), "jnz"  | "jne" => Op!(
            "c*", [0x0F, 0x85];
            "cb", [0x75];
        ), "jbe"  | "jna" => Op!(
            "c*", [0x0F, 0x86];
            "cb", [0x76];
        ), "jnbe" | "ja"  => Op!(
            "c*", [0x0F, 0x87];
            "cb", [0x77];
        ), "js"   => Op!(
            "c*", [0x0F, 0x88];
            "cb", [0x78];
        ), "jns"  => Op!(
            "c*", [0x0F, 0x89];
            "cb", [0x79];
        ), "jp"   | "jpe" => Op!(
            "c*", [0x0F, 0x8A];
            "cb", [0x7A];
        ), "jnp"  | "jpo" => Op!(
            "c*", [0x0F, 0x8B];
            "cb", [0x7B];
        ), "jl"   | "jnge"=> Op!(
            "c*", [0x0F, 0x8C];
            "cb", [0x7C];
        ), "jnl"  | "jge" => Op!(
            "c*", [0x0F, 0x8D];
            "cb", [0x7D];
        ), "jle"  | "jng" => Op!(
            "c*", [0x0F, 0x8E];
            "cb", [0x7E];
        ), "jnle" | "jg"  => Op!(
            "c*", [0x0F, 0x8F];
            "cb", [0x7F];
        ), "jecxz" => Op!(
            "cb", [0xE3], 0, flags::REQUIRES_ADDRSIZE;
        ), "jrcxz" => Op!(
            "cb", [0xE3];
        ), "jmp" => Op!(
            "c*", [0xE9];
            "cb", [0xEB];
            "v*", [0xFF], 4;
        ), "lahf" => Op!(
            [0x9F];
        ), "lea" => Op!(
            "r*m*", [0xED];
        ), "leave" => Op!(
            [0xC9];
        ), "lfence" => Op!(
            [0x0F, 0xAE, 0xE8];
        ), "lodsb" => Op!(
            [0xAC];
        ), "lodsw" => Op!(
            "", [0xAD], 0, flags::REQUIRES_OPSIZE;
        ), "lodsd" => Op!(
            [0xAD];
        ), "lodsq" => Op!(
            "", [0xAD], 0, flags::REQUIRES_REXSIZE;
        ), "loop" => Op!(
            "cb", [0xE2];
        ), "loope" | "loopz" => Op!(
            "cb", [0xE1];
        ), "loopne" | "loopnz" => Op!(
            "cb", [0xE0];
        ), "lzcnt" => Op!(
            "r*m*", [0x0F, 0xBD], 0, flags::REQUIRES_REP;
        ), "mfence" => Op!(
            [0x0F, 0xAE, 0xF0];
        ), "mov" => Op!( // note: several other forms exist but aren't recorded here due to their weirdness. (64 bit displacements, *ax only).
            "v*r*", [0x89];
            "vbrb", [0x88];
            "r*v*", [0x8B];
            "rbvb", [0x8A];
            "rbib", [0xB0], 0, flags::REGISTER_IN_OPCODE;
            "rwiw", [0xB8], 0, flags::REGISTER_IN_OPCODE;
            "rdid", [0xB8], 0, flags::REGISTER_IN_OPCODE;
            "v*i*", [0xC7], 0;
            "rqiq", [0xB8], 0, flags::REGISTER_IN_OPCODE;
            "vbib", [0xC6], 0;
        ), "movbe" => Op!(
            "r*m*", [0x0F, 0x38, 0xF0];
            "m*r*", [0x0F, 0x38, 0xF1];
        ), "movsb" => Op!(
            [0xA4]; 
        ), "movsw" => Op!(
            "", [0xA5], 0, flags::REQUIRES_OPSIZE;
        ), "movsd" => Op!(
            [0xA5];
        ), "movsq" => Op!(
            "", [0xA5], 0, flags::REQUIRES_REXSIZE; 
        ), "movsx" => Op!( // currently this defaults to 32-bit memory if not specified. do we want this?
            "rdvw", [0x0F, 0xBF], 0, flags::SIZE_OVERRIDE;
            "rqvw", [0x0F, 0xBF], 0, flags::SIZE_OVERRIDE | flags::REQUIRES_REXSIZE;
            "rwvb", [0x0F, 0xBE], 0, flags::SIZE_OVERRIDE | flags::REQUIRES_OPSIZE;
            "rdvb", [0x0F, 0xBE], 0, flags::SIZE_OVERRIDE;
            "rqvb", [0x0F, 0xBE], 0, flags::SIZE_OVERRIDE | flags::REQUIRES_REXSIZE;
        ), "movsxd" => Op!(
            "rqvd", [0x63], 0, flags::SIZE_OVERRIDE | flags::REQUIRES_REXSIZE;
        ), "movzx" => Op!( // currently this defaults to 32-bit memory if not specified. do we want this?
            "rdvw", [0x0F, 0xBF], 0, flags::SIZE_OVERRIDE;
            "rqvw", [0x0F, 0xBF], 0, flags::SIZE_OVERRIDE | flags::REQUIRES_REXSIZE;
            "rwvb", [0x0F, 0xBE], 0, flags::SIZE_OVERRIDE | flags::REQUIRES_OPSIZE;
            "rdvb", [0x0F, 0xBE], 0, flags::SIZE_OVERRIDE;
            "rqvb", [0x0F, 0xBE], 0, flags::SIZE_OVERRIDE | flags::REQUIRES_REXSIZE;
        ), "mul" => Op!(
            "v*", [0xF7], 4;
            "vb", [0xF6], 4;
        ), "neg" => Op!(
            "v*", [0xF7], 3, flags::CAN_LOCK;
            "vb", [0xF6], 3, flags::CAN_LOCK;
        ), "nop" => Op!(
            [0x90];
            "v*", [0x0F, 0x1F], 0;
        ),"not" => Op!(
            "v*", [0xF7], 2, flags::CAN_LOCK;
            "vb", [0xF6], 2, flags::CAN_LOCK;
        ), "or" => Op!(
            "v*i*", [0x81], 1, flags::CAN_LOCK;
            "v*ib", [0x83], 1, flags::CAN_LOCK;
            "vbib", [0x80], 1, flags::CAN_LOCK;
            "v*r*", [0x09], 0, flags::CAN_LOCK;
            "vbrb", [0x08], 0, flags::CAN_LOCK;
            "r*v*", [0x0B], 0, flags::CAN_LOCK;
            "rbvb", [0x0A], 0, flags::CAN_LOCK;
        ), "out" => Op!(
            "ibr*", [0xE7], 0, flags::EAX_ONLY;
            "ibrb", [0xE6], 0, flags::EAX_ONLY;
            "r*",  [0xEF], 0, flags::EAX_ONLY;
            "rb",  [0xEE], 0, flags::EAX_ONLY;
        ), "outsb" => Op!(
            "", [0xEE], 0, flags::CAN_REP;
        ), "outsw" => Op!(
            "", [0x6F], 0, flags::CAN_REP | flags::REQUIRES_OPSIZE ;
        ), "outsd" => Op!(
            "", [0x6F], 0, flags::CAN_REP;
        ), "pause" => Op!(
            [0xF3, 0x90];
        ), "pop" => Op!(
            "r*", [0x8F], 0, flags::DEFAULT_REXSIZE;
            "v*", [0x58], 0, flags::DEFAULT_REXSIZE | flags::REGISTER_IN_OPCODE;
        ), "popcnt" => Op!(
            "r*v*", [0x0F, 0xB8], 0, flags::REQUIRES_REP;
        ), "popf" => Op!(
            "", [0x9D], 0, flags::REQUIRES_OPSIZE;
        ), "popfq" => Op!(
            "", [0x9D], 0, flags::DEFAULT_REXSIZE;
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
            "r*", [0x50], 0, flags::DEFAULT_REXSIZE | flags::REGISTER_IN_OPCODE;
            "v*", [0xFF], 6, flags::DEFAULT_REXSIZE;
            "ib", [0x6A], 0, flags::DEFAULT_REXSIZE;
            "id", [0x68], 0, flags::DEFAULT_REXSIZE;
            "iw", [0x68], 0, flags::DEFAULT_REXSIZE;
            "iq", [0x68], 0, flags::DEFAULT_REXSIZE;
        ), "pushf" => Op!(
            "", [0x9C], 0, flags::REQUIRES_OPSIZE;
        ), "pushfq" => Op!(
            "", [0x9C], 0, flags::DEFAULT_REXSIZE;
        ), "rcl" => Op!(
            "v*",  [0xD3], 2;
            "vb",  [0xD2], 2;
            "v*ib", [0xC1], 2;
            "vbib", [0xC0], 2;
        ), "rcr" => Op!(
            "v*",  [0xD3], 3;
            "vb",  [0xD2], 3;
            "v*ib", [0xC1], 3;
            "vbib", [0xC0], 3;
        ), "rdfsbase" => Op!(
            "rd", [0x0F, 0xAE], 0, flags::REQUIRES_REP;
            "rq", [0x0F, 0xAE], 0, flags::REQUIRES_REP;
        ), "rdgsbase" => Op!(
            "rd", [0x0F, 0xAE], 1, flags::REQUIRES_REP;
            "rq", [0x0F, 0xAE], 0, flags::REQUIRES_REP;
        ), "rdrand" => Op!(
            "r*", [0x0F, 0xC7], 6;
        ), "ret" => Op!(
            [0xC3];
            "iw", [0xC2], 0, flags::SIZE_OVERRIDE;
        ), "rol" => Op!(
            "v*",  [0xD3], 0;
            "vb",  [0xD2], 0;
            "v*ib", [0xC1], 0;
            "vbib", [0xC0], 0;
        ), "ror" => Op!(
            "v*",  [0xD3], 1;
            "vb",  [0xD2], 1;
            "v*ib", [0xC1], 1;
            "vbib", [0xC0], 1;
        ), "sahf" => Op!(
           [0x9E];
        ), "sal" => Op!(
            "v*",  [0xD3], 4;
            "vb",  [0xD2], 4;
            "v*ib", [0xC1], 4;
            "vbib", [0xC0], 4;
        ), "shl" => Op!(
            "v*",  [0xD3], 4;
            "vb",  [0xD2], 4;
            "v*ib", [0xC1], 4;
            "vbib", [0xC0], 4;
        ), "sar" => Op!(
            "v*",  [0xD3], 7;
            "vb",  [0xD2], 7;
            "v*ib", [0xC1], 7;
            "vbib", [0xC0], 7;
        ), "sbb" => Op!(
            "v*i*", [0x81], 3, flags::CAN_LOCK;
            "v*ib", [0x83], 3, flags::CAN_LOCK;
            "vbib", [0x80], 3, flags::CAN_LOCK;
            "v*r*", [0x19], 0, flags::CAN_LOCK;
            "vbrb", [0x18], 0, flags::CAN_LOCK;
            "r*v*", [0x1B], 0, flags::CAN_LOCK;
            "rbvb", [0x1A], 0, flags::CAN_LOCK;
        ), "scasb" => Op!(
            "", [0xAE], 0, flags::CAN_REP;
        ), "scasw" => Op!(
            "", [0xAF], 0, flags::CAN_REP | flags::REQUIRES_OPSIZE;
        ), "scasd" => Op!(
            "", [0xAF], 0, flags::CAN_REP;
        ), "scasq" => Op!(
            "", [0xAF], 0, flags::CAN_REP | flags::REQUIRES_REXSIZE;
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
            [0x0F, 0xAE, 0xF8];
        ), "shld" => Op!(
            "v*r*",  [0x0F, 0xA5];
            "v*r*ib", [0x0F, 0xA4];
        ), "shr" => Op!(
            "v*",  [0xD3], 5;
            "vb",  [0xD2], 5;
            "v*ib", [0xC1], 5;
            "vbib", [0xC0], 5;
        ), "shrd" => Op!(
            "v*r*",  [0x0F, 0xAD];
            "v*r*ib", [0x0F, 0xAC];
        ), "stc" => Op!(
            [0xF9];
        ), "std" => Op!(
            [0xFD];
        ), "stosb" => Op!(
            "", [0xA4], 0, flags::CAN_REP;
        ), "stosw" => Op!(
            "", [0xAB], 0, flags::CAN_REP | flags::REQUIRES_OPSIZE;
        ), "stosd" => Op!(
            "", [0xAB], 0, flags::CAN_REP;
        ), "stosq" => Op!(
            "", [0xAB], 0, flags::CAN_REP | flags::REQUIRES_REXSIZE;
        ), "sub" => Op!(
            "v*i*", [0x81], 5, flags::CAN_LOCK;
            "v*ib", [0x83], 5, flags::CAN_LOCK;
            "vbib", [0x80], 5, flags::CAN_LOCK;
            "v*r*", [0x29], 0, flags::CAN_LOCK;
            "vbrb", [0x28], 0, flags::CAN_LOCK;
            "r*v*", [0x2B], 0, flags::CAN_LOCK;
            "rbvb", [0x2A], 0, flags::CAN_LOCK;
        ), "test" => Op!(
            "v*i*", [0xF7], 0;
            "vbib", [0xF6], 0;
            "v*r*", [0x85];
            "vbrb", [0x84];
        ), "tzcnt" => Op!(
            "r*v*", [0x0F, 0xBC], 0, flags::REQUIRES_REP;
        ), "wrfsbase" => Op!(
            "r*", [0x0F, 0xAE], 2, flags::REQUIRES_REP;
        ), "wrgsbase" => Op!(
            "r*", [0x0F, 0xAE], 3, flags::REQUIRES_REP;
        ), "xadd" => Op!(
            "vbib", [0x0F, 0xC0], 0, flags::CAN_LOCK;
            "v*r*", [0x0F, 0xC1], 0, flags::CAN_LOCK;
        ), "xchg" => Op!(
            "v*r*", [0x87], 0, flags::CAN_LOCK;
            "r*v*", [0x87], 0, flags::CAN_LOCK;
            "vbrb", [0x86], 0, flags::CAN_LOCK;
            "rbvb", [0x86], 0, flags::CAN_LOCK;
        ), "xlatb" => Op!(
            [0xD7];
        ), "xor" => Op!(
            "v*i*", [0x81], 6, flags::CAN_LOCK;
            "v*ib", [0x83], 6, flags::CAN_LOCK;
            "vbib", [0x80], 6, flags::CAN_LOCK;
            "v*r*", [0x31], 0, flags::CAN_LOCK;
            "vbrb", [0x30], 0, flags::CAN_LOCK;
            "r*v*", [0x33], 0, flags::CAN_LOCK;
            "rbvb", [0x32], 0, flags::CAN_LOCK;
        ),
        _ => return None
    })
}
