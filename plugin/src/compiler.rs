use syntax::ext::base::ExtCtxt;
use syntax::ext::build::AstBuilder;
use syntax::ast;
use syntax::ptr::P;
use syntax::codemap::Spanned;

use parser::{self, Item, Arg, Ident, MemoryRef, Register, RegKind, RegFamily, RegId, Size, LabelType, JumpType};
use x64data::get_mnemnonic_data;
use serialize::or_mask_shift_expr;

use std::mem::swap;
use itertools::Itertools;

/*
 * Compilation output
 */

pub type StmtBuffer = Vec<Stmt>;

#[derive(Clone, Debug)]
pub enum Stmt {
    Const(u8),
    ExprConst(P<ast::Expr>),

    Var(P<ast::Expr>, Size),

    Align(P<ast::Expr>),

    GlobalLabel(Ident),
    LocalLabel(Ident),
    DynamicLabel(P<ast::Expr>),

    GlobalJumpTarget(Ident, Size),
    ForwardJumpTarget(Ident, Size),
    BackwardJumpTarget(Ident, Size),
    DynamicJumpTarget(P<ast::Expr>, Size)
}

/*
 * Instruction encoding data formats
 */

pub mod flags {
    pub const VEX_OP: u16 =      0x0001; // this instruction requires a VEX prefix to be encoded
    pub const XOP_OP: u16 =      0x0002; // this instruction requires a XOP prefix to be encoded

    pub const SHORT_ARG: u16 =   0x0004; // a register argument is encoded in the last byte of the opcode
    pub const DEST_IN_REG: u16 = 0x0008; // destination operand should be encoded as the modrm.reg operand

    // note: the first 4 in this block are mutually exclusive
    pub const AUTO_SIZE: u16 =   0x0010; // 16 bit -> OPSIZE , 32-bit -> None   , 64-bit -> REX.W/VEX.W/XOP.W
    pub const AUTO_NO32: u16 =   0x0020; // 16 bit -> OPSIZE , 32-bit -> illegal, 64-bit -> None
    pub const AUTO_REXW: u16 =   0x0040; // 16 bit -> illegal, 32-bit -> None   , 64-bit -> REX.W/VEX.W/XOP.W
    pub const AUTO_VEXL: u16 =   0x0080; // 128bit -> None   , 256bit -> VEX.L
    pub const SMALL_SIZE: u16 =  0x0100; // implies opsize prefix
    pub const LARGE_SIZE: u16 =  0x0200; // implies REX.W/VEX.W/XOP.W

    pub const PREF_66: u16 =     SMALL_SIZE; // mandatory prefix (same as SMALL_SIZE)
    pub const PREF_67: u16 =     0x0400; // mandatory prefix (same as SMALL_ADDRESS)
    pub const PREF_F0: u16 =     0x0800; // mandatory prefix (same as LOCK)
    pub const PREF_F2: u16 =     0x1000; // mandatory prefix (REPNE)
    pub const PREF_F3: u16 =     0x2000; // mandatory prefix (REP)

    pub const LOCK: u16 =        0x4000; // user lock prefix is valid with this instruction
    pub const REP: u16 =         0x8000; // user rep prefix is valid with this instruction

    pub const LARGE_VEC: u16 = 0xFFFF;
    pub const FOURTH_ARG: u16 = 0xFFFF;

}

pub struct Opdata {
    pub args:  &'static str,  // format string of arg format
    pub ops:   &'static [u8],
    pub reg:   u8,
    pub flags: u16
}

/*
 * Instruction encoding constants
 */

const MOD_DIRECT: u8 = 0b11;
const MOD_NODISP: u8 = 0b00;
const MOD_DISP8:  u8 = 0b01;
const MOD_DISP32: u8 = 0b10;

/* 
 * Implmementation
 */

pub fn compile(ecx: &ExtCtxt, nodes: Vec<parser::Item>) -> Result<StmtBuffer, ()>  {
    let mut stmts = StmtBuffer::new();

    let mut successful = true;

    for node in nodes {
        match node {
            Item::Instruction(mut ops, args, span) => {
                let op = ops.pop().unwrap();
                match compile_op(ecx, &mut stmts, op, ops, args) {
                    Ok(_) => (),
                    Err(e) => {
                        successful = false;
                        if let Some(e) = e {
                            ecx.span_err(span, &e)
                        }
                    }
                }
            },
            Item::Label(label) => compile_label(&mut stmts, label),
            Item::Directive(op, args, span) => {
                match compile_directive(ecx, &mut stmts, op, args) {
                    Ok(_) => (),
                    Err(e) => {
                        successful = false;
                        if let Some(e) = e {
                            ecx.span_err(span, &e)
                        }
                    }
                }
            }
        }
    }

    if successful {
        Ok(stmts)
    } else {
        Err(())
    }
}

fn compile_directive(ecx: &ExtCtxt, buffer: &mut StmtBuffer, dir: Ident, mut args: Vec<Arg>) -> Result<(), Option<String>> {
    match &*dir.node.name.as_str() {
        // TODO: oword, qword, float, double, long double
        // TODO: iterators <- gives us strings and bytestrings for free
        "byte"  => directive_const(ecx, buffer, args, Size::BYTE),
        "word"  => directive_const(ecx, buffer, args, Size::WORD),
        "dword" => directive_const(ecx, buffer, args, Size::DWORD),
        "qword" => directive_const(ecx, buffer, args, Size::QWORD),
        "align" => {
            if args.len() != 1 {
                return Err(Some(format!("Invalid amount of arguments")));
            }

            match args.pop().unwrap() {
                Arg::Immediate(expr, _) => {
                    buffer.push(Stmt::Align(expr));
                },
                _ => return Err(Some(format!("this directive only uses immediate arguments")))
            }
            Ok(())
        },
        d => {
            ecx.span_err(dir.span, &format!("unknown directive '{}'", d));
            Err(None)
        }
    }
}

fn directive_const(ecx: &ExtCtxt, buffer: &mut StmtBuffer, args: Vec<Arg>, size: Size) -> Result<(), Option<String>> {
    if args.len() == 0 {
        return Err(Some(format!("this directive requires at least one argument")));
    }

    for arg in args {
        match arg {
            Arg::Immediate(expr, s) => {
                if s.is_some() && s != Some(size) {
                    ecx.span_err(expr.span, "wrong argument size");
                    return Err(None)
                }
                buffer.push(Stmt::Var(expr, size));
            },
            _ => return Err(Some(format!("this directive only uses immediate arguments")))
        }
    }

    Ok(())
}

fn compile_label(stmts: &mut StmtBuffer, label: LabelType) {
    stmts.push(match label {
        LabelType::Global(ident) => Stmt::GlobalLabel(ident),
        LabelType::Local(ident)  => Stmt::LocalLabel(ident),
        LabelType::Dynamic(expr) => Stmt::DynamicLabel(expr),
    });
}

fn compile_op(ecx: &ExtCtxt, buffer: &mut StmtBuffer, op: Ident, prefixes: Vec<Ident>, mut args: Vec<Arg>) -> Result<(), Option<String>> {
    // this call also inserts more size information in the AST if applicable.
    let data = try!(match_op_format(ecx, op, &mut args));

    if (data.flags & (flags::VEX_OP | flags::XOP_OP)) != 0 {
        panic!("vex/xop operations are currently not supported yet");
    }

    // determine legacy prefixes
    let (mut pref_mod, pref_seg) = try!(get_legacy_prefixes(ecx, data, prefixes));

    // determine address size
    let pref_addr = try!(get_address_size(ecx, &args)) != Size::QWORD;

    let mut op_size = Size::BYTE; // unused value, just here to please the compiler
    let mut pref_size = false;
    let mut rex_w = false;

    // determine if size prefixes are necessary
    if (data.flags & (flags::AUTO_SIZE | flags::AUTO_NO32)) != 0 {
        // determine operand size
        op_size = try!(get_operand_size(data, &args));
        // correct for ops which are by default 64 bit in long mode
        if (data.flags & flags::AUTO_NO32) != 0 {
            if op_size == Size::DWORD {
                return Err(Some(format!("'{}': Does not support 32 bit operands in 64-bit mode", &*op.node.name.as_str())));
            } else if op_size == Size::QWORD {
                op_size = Size::DWORD;
            }
        }
        // WORD => opsize, QWORD | HWORD => rex.W/VEX.~W/XOP.~W
        pref_size = op_size == Size::WORD;
        rex_w     = op_size == Size::QWORD || op_size == Size::HWORD;
    }

    // mandatory prefixes
    let pref_size = pref_size || (data.flags & flags::SMALL_SIZE) != 0;
    let rex_w     = rex_w     || (data.flags & flags::LARGE_SIZE) != 0;
    let pref_addr = pref_addr || (data.flags & flags::PREF_67) != 0;
    if        (data.flags & flags::PREF_F0) != 0 {
        pref_mod = Some(0xF0);
    } else if (data.flags & flags::PREF_F2) != 0 {
        pref_mod = Some(0xF2);
    } else if (data.flags & flags::PREF_F3) != 0 {
        pref_mod = Some(0xF3);
    }

    // push prefixes
    if let Some(pref) = pref_mod {
        buffer.push(Stmt::Const(pref));
    }
    if let Some(pref) = pref_seg {
        buffer.push(Stmt::Const(pref));
    }
    if pref_size {
        buffer.push(Stmt::Const(0x66));
    }
    if pref_addr {
        buffer.push(Stmt::Const(0x67));
    }

    // check if this combination of args can actually be encoded and whether a rex prefix is necessary
    let need_rex = try!(validate_args(data, &args, rex_w));

    // split args
    let (reg, rm, args) = extract_args(data, args);

    // here's where VEX/XOP/EVEX would be encoded

    // no register args
    if rm.is_none() {

        if need_rex {
            compile_rex(ecx, buffer, rex_w, RegKind::from_number(0), RegKind::from_number(0), RegKind::from_number(0));
        }
        buffer.extend(data.ops.iter().map(|x| Stmt::Const(*x)));

    // register encoded in opcode byte
    } else if (data.flags & flags::SHORT_ARG) != 0 {

        let reg = if let Some(Arg::Direct(reg)) = rm {
            reg.node.kind
        } else {
            panic!("bad encoding data, expected register");
        };

        if need_rex {
            compile_rex(ecx, buffer, rex_w, RegKind::from_number(0), RegKind::from_number(0), reg.clone());
        }

        let (last, rest) = data.ops.split_last().expect("bad encoding data, no opcode specified");
        buffer.extend(rest.into_iter().map(|x| Stmt::Const(*x)));

        if let RegKind::Dynamic(_, expr) = reg {
            let last = ecx.expr_lit(ecx.call_site(), ast::LitKind::Byte(*last));
            buffer.push(Stmt::ExprConst(or_mask_shift_expr(ecx, last, expr, 7, 0)));
        } else {
            buffer.push(Stmt::Const(last + (reg.encode() & 7)));
        }

    // ModRM byte
    } else if let Some(Arg::Direct(rm)) = rm {
        let rm = rm.node.kind;

        let reg = if let Some(Arg::Direct(reg)) = reg {
            reg.node.kind
        } else {
            // reg is given by the instruction encoding
            RegKind::from_number(data.reg)
        };

        if need_rex {
            compile_rex(ecx, buffer, rex_w, reg.clone(), RegKind::from_number(0), rm.clone());
        }
        buffer.extend(data.ops.iter().map(|x| Stmt::Const(*x)));
        compile_modrm_sib(ecx, buffer, MOD_DIRECT, reg, rm);

    // ModRM and SIB byte
    } else if let Some(Arg::Indirect(mut mem)) = rm {

        let reg = if let Some(Arg::Direct(reg)) = reg {
            reg.node.kind
        } else { // reg is given by the instruction encoding
            RegKind::from_number(data.reg)
        };

        // detect impossible to encode memoryrefs. also stops RIP in odd places.
        try!(sanitize_memoryref(ecx, &mut mem));

        // TODO: if the arg is constant we should be able to optimize to MOD_DISP8.
        // encoding special cases
        let rip_relative = mem.base == RegId::RIP;
        let rbp_relative = mem.base == RegId::RBP || mem.base == RegId::R13;
        let no_base      = mem.base.is_none();

        // RBP can only be encoded as base if a displacement is present.
        let mode = if rbp_relative && mem.disp.is_none() {
            MOD_DISP8
        // mode_nodisp has to be selected if RIP is encoded, or if no base is to be encoded. note that in these scenarions the disp should actually be encoded
        } else if mem.disp.is_none() || rip_relative || no_base {
            MOD_NODISP
        } else {
            MOD_DISP32
        };

        // if there's an index we need to escape into the SIB byte
        if let Some(index) = mem.index {

            let index = index.kind;
            // to encode the lack of a base we encode RBP
            let base = if let Some(base) = mem.base {
                base.kind
            } else {
                RegKind::Static(RegId::RBP)
            };

            if need_rex {
                compile_rex(ecx, buffer, rex_w, reg.clone(), index.clone(), base.clone());
            }
            buffer.extend(data.ops.iter().map(|x| Stmt::Const(*x)));
            compile_modrm_sib(ecx, buffer, mode, reg, RegKind::Static(RegId::RSP));
            compile_modrm_sib(ecx, buffer, mem.scale as u8, index, base);

        // no index, only a base.
        } else if let Some(base) = mem.base {

            // RBP at MOD_NODISP is used to encode RIP, but this is already handled
            if need_rex {
                compile_rex(ecx, buffer, rex_w, reg.clone(), RegKind::from_number(0), base.kind.clone());
            }
            buffer.extend(data.ops.iter().map(|x| Stmt::Const(*x)));
            compile_modrm_sib(ecx, buffer, mode, reg, base.kind);

        // no base, no index. only disp
        } else {
            // escape using RBP as base and RSP as index
            if need_rex {
                compile_rex(ecx, buffer, rex_w, reg.clone(), RegKind::from_number(0), RegKind::from_number(0));
            }
            buffer.extend(data.ops.iter().map(|x| Stmt::Const(*x)));
            compile_modrm_sib(ecx, buffer, mode, reg, RegKind::Static(RegId::RSP));
            compile_modrm_sib(ecx, buffer, 0, RegKind::Static(RegId::RSP), RegKind::Static(RegId::RBP));

        }

        // Disp
        if let Some(disp) = mem.disp {
            buffer.push(Stmt::Var(disp, Size::DWORD));
        } else if no_base || rip_relative {
            buffer.push(Stmt::Const(0));
            buffer.push(Stmt::Const(0));
            buffer.push(Stmt::Const(0));
            buffer.push(Stmt::Const(0));
        } else if rbp_relative {
            buffer.push(Stmt::Const(0));
        }
    // RIP-relative label. encoded as memoryref with only a base
    } else if let Some(Arg::IndirectJumpTarget(target, _)) = rm {
        let reg = if let Some(Arg::Direct(reg)) = reg {
            reg.node.kind
        } else { // reg is given by the instruction encoding
            RegKind::from_number(data.reg)
        };

        if need_rex {
            compile_rex(ecx, buffer, rex_w, reg.clone(), RegKind::from_number(0), RegKind::from_number(0));
        }
        buffer.extend(data.ops.iter().cloned().map(Stmt::Const));
        compile_modrm_sib(ecx, buffer, MOD_NODISP, reg, RegKind::Static(RegId::RBP));

        // note: get_oprand_size ensures that no immediates are encoded afterwards.
        // they potentially could be, but currently the runtime doens't support it
        for _ in 0..Size::DWORD.in_bytes() {
            buffer.push(Stmt::Const(0));
        }
        buffer.push(match target {
            JumpType::Global(ident)   => Stmt::GlobalJumpTarget(ident, Size::DWORD),
            JumpType::Forward(ident)  => Stmt::ForwardJumpTarget(ident, Size::DWORD),
            JumpType::Backward(ident) => Stmt::BackwardJumpTarget(ident, Size::DWORD),
            JumpType::Dynamic(expr)   => Stmt::DynamicJumpTarget(expr, Size::DWORD)
        });
    } else {
        unreachable!();
    }

    // immediates
    for arg in args {
        let stmt = match arg {
            Arg::Immediate(expr, Some(size)) => Stmt::Var(expr, size),
            Arg::Immediate(expr, None)       => Stmt::Var(expr, if op_size != Size::QWORD {op_size} else {Size::DWORD}),
            Arg::JumpTarget(target, size)    => {
                let size = size.unwrap_or(Size::DWORD);

                // placeholder
                for _ in 0..size.in_bytes() {
                    buffer.push(Stmt::Const(0));
                }

                match target {
                    JumpType::Global(ident)   => Stmt::GlobalJumpTarget(ident, size),
                    JumpType::Forward(ident)  => Stmt::ForwardJumpTarget(ident, size),
                    JumpType::Backward(ident) => Stmt::BackwardJumpTarget(ident, size),
                    JumpType::Dynamic(expr)   => Stmt::DynamicJumpTarget(expr, size)
                }
            }
            _ => continue 
        };
        buffer.push(stmt);
    }

    Ok(())
}

fn match_op_format(ecx: &ExtCtxt, ident: Ident, args: &mut [Arg]) -> Result<&'static Opdata, Option<String>> {
    let name = &*ident.node.name.as_str();

    let data = if let Some(data) = get_mnemnonic_data(name) {
        data
    } else {
        ecx.span_err(ident.span, &format!("'{}' is not a valid instruction", name));
        return Err(None);
    };

    for format in data {
        match match_format_string(format.args, args) {
            Ok(_) => return Ok(format),
            Err(_) => ()
        }
    }

    /*
    let allowed = String::new("Invaild arguments. Allowed formats:\n");
    for format in data {
        iter = format.args.iter();
        while Some(ty) = iter.next();
            let size = iter.next().expect("invalid format string");
            let size = match size {
                'b' => "8",
                'w' => "16",
                'd' => "32",
                'q' => "64",
                '*' => "*",
                '!' => "",
                _ => panic!("invalid format string")
            });
            allowed.push_str(match ty {
                'i' => format!("imm{}", size),
                'c' => format!("rel{}off", size),
                'r' => format!("reg", size),
                'v' => format!("reg/mem", size),
                'm' => format!("mem", size),
                'A' ... 'P' =>
                _   => panic!("invalid format string")
            });

    }*/

    Err(Some(format!("'{}': argument type/size mismatch", name)))
}

fn match_format_string(fmtstr: &'static str, mut args: &mut [Arg]) -> Result<(), &'static str> {
    if fmtstr.len() != args.len() * 2 {
        return Err("argument length mismatch");
    }

    // i : immediate
    // o : instruction offset

    // m : memory
    // k : vsib addressing, 32 bit result, size determines xmm or ymm
    // l : vsib addressing, 64 bit result, size determines xmm or ymm

    // r : legacy reg
    // f : fp reg
    // x : mmx reg
    // y : xmm/ymm reg
    // s : segment reg
    // c : control reg
    // d : debug reg

    // v : r and m
    // u : x and m
    // w : y and m

    // A ... P: match rax - r15
    // Q ... V: match es, cs, ss, ds, fs, gs
    // W: matches CR8

    // b, w, d, q match a byte, word, doubleword and quadword.
    // * matches all possible sizes for this operand (w/d for i/o, w/d/q for r/v, o/h for y/w and everything for m)
    // ! matches a lack of size, only useful in combination of m and i
    // ? matches any size and doesn't participate in the operand size calculation
    {
        let mut fmt = fmtstr.chars();
        let mut args = args.iter();
        while let Some(code) = fmt.next() {
            let fsize = fmt.next().expect("invalid format string");
            let arg = args.next().unwrap();

            let size = match (code, arg) {
                // immediates
                ('i', &Arg::Immediate(_, size))  => size,
                ('o', &Arg::Immediate(_, size))  => size,
                ('o', &Arg::JumpTarget(_, size)) => size,

                // specific legacy regs
                (x @ 'A' ... 'P', &Arg::Direct(Spanned {node: ref reg, ..} )) if
                    reg.kind.family() == RegFamily::LEGACY &&
                    reg.kind.code() == Some(x as u8 - 'A' as u8) => Some(reg.size()),

                // specific segment regs
                (x @ 'Q' ... 'V', &Arg::Direct(Spanned {node: ref reg, ..} )) if
                    reg.kind.family() == RegFamily::SEGMENT &&
                    reg.kind.code() == Some(x as u8 - 'Q' as u8) => Some(reg.size()),

                // CR8 can be specially referenced
                ('W', &Arg::Direct(Spanned {node: ref reg, ..} )) if
                    reg.kind == RegId::CR8 => Some(reg.size()),

                // top of the fp stack is also often used
                ('X', &Arg::Direct(Spanned {node: ref reg, ..} )) if
                    reg.kind == RegId::ST0 => Some(reg.size()),

                // generic legacy regs
                ('r', &Arg::Direct(Spanned {node: ref reg, ..} )) |
                ('v', &Arg::Direct(Spanned {node: ref reg, ..} )) if
                    reg.kind.family() == RegFamily::LEGACY ||
                    reg.kind.family() == RegFamily::HIGHBYTE => Some(reg.size()),

                // other reg types often mixed with memory refs
                ('x', &Arg::Direct(Spanned {node: ref reg, ..} )) |
                ('u', &Arg::Direct(Spanned {node: ref reg, ..} )) if
                    reg.kind.family() == RegFamily::MMX => Some(reg.size()),
                ('y', &Arg::Direct(Spanned {node: ref reg, ..} )) |
                ('w', &Arg::Direct(Spanned {node: ref reg, ..} )) if
                    reg.kind.family() == RegFamily::XMM => Some(reg.size()),

                // other reg types
                ('f', &Arg::Direct(Spanned {node: ref reg, ..} )) if
                    reg.kind.family() == RegFamily::FP => Some(reg.size()),
                ('s', &Arg::Direct(Spanned {node: ref reg, ..} )) if
                    reg.kind.family() == RegFamily::SEGMENT => Some(reg.size()),
                ('c', &Arg::Direct(Spanned {node: ref reg, ..} )) if
                    reg.kind.family() == RegFamily::CONTROL => Some(reg.size()),
                ('d', &Arg::Direct(Spanned {node: ref reg, ..} )) if
                    reg.kind.family() == RegFamily::DEBUG => Some(reg.size()),

                // memory offsets
                ('m', &Arg::Indirect(MemoryRef {size, ..} )) |
                ('m', &Arg::IndirectJumpTarget(_, size))     |
                ('u' ... 'w', &Arg::Indirect(MemoryRef {size, ..} )) |
                ('u' ... 'w', &Arg::IndirectJumpTarget(_, size))     => size,
                _ => return Err("argument type mismatch")
            };

            // if size is none it always matches (and will later be coerced to a more specific type if the match is successful)
            if let Some(size) = size {
                if !match (fsize, code) {
                    ('b', _)   => size == Size::BYTE,
                    ('w', _)   => size == Size::WORD,
                    ('d', _)   => size == Size::DWORD,
                    ('q', _)   => size == Size::QWORD,
                    ('p', _)   => size == Size::PWORD,
                    ('o', _)   => size == Size::OWORD,
                    ('h', _)   => size == Size::HWORD,
                    ('*', 'i') |
                    ('*', 'o') => size == Size::WORD || size == Size::DWORD,
                    ('*', 'y') |
                    ('*', 'w') => size == Size::OWORD || size == Size::HWORD,
                    ('*', 'r') |
                    ('*', 'A' ... 'P') |
                    ('*', 'v') => size == Size::WORD || size == Size::DWORD || size == Size::QWORD,
                    ('*', 'm') => true,
                    ('*', _)   => panic!("Invalid size wildcard"),
                    ('?', _)   => true,
                    ('!', _)   => false,
                    _ => panic!("invalid format string '{}'", fmtstr)
                } {
                    return Err("argument size mismatch");
                }
            }
        }
    }

    // we've found a match, update all specific constraints
    {
        let mut fmt = fmtstr.chars();
        let mut args = args.iter_mut();
        while let Some(_) = fmt.next() {
            let fsize = fmt.next().unwrap();
            let arg: &mut Arg = args.next().unwrap();

            match *arg {
                Arg::Immediate(_, ref mut size @ None) |
                Arg::JumpTarget(_, ref mut size @ None) |
                Arg::Indirect(MemoryRef {size: ref mut size @ None, ..} ) => *size = match fsize {
                    'b' => Some(Size::BYTE),
                    'w' => Some(Size::WORD),
                    'd' => Some(Size::DWORD),
                    'q' => Some(Size::QWORD),
                    'p' => Some(Size::PWORD),
                    'o' => Some(Size::OWORD),
                    'h' => Some(Size::HWORD),
                    '*' => None,
                    '!' => None,
                    _ => unreachable!()
                },
                _ => ()
            };
        }
    }

    Ok(())
}

fn get_legacy_prefixes(ecx: &ExtCtxt, fmt: &'static Opdata, idents: Vec<Ident>) -> Result<(Option<u8>, Option<u8>), Option<String>> {
    let mut group1 = None;
    let mut group2 = None;

    for prefix in idents {
        let name = &*prefix.node.name.as_str();
        let (group, value) = match name {
            "rep"   |
            "repe"  |
            "repz"  => if fmt.flags & flags::REP != 0 {
                (&mut group1, 0xF3)
            } else {
                ecx.span_err(prefix.span, &format!("Cannot use prefix {} on this instruction", name));
                return Err(None);
            },
            "repnz" |
            "repne" => if fmt.flags & flags::REP != 0 {
                (&mut group1, 0xF2)
            } else {
                ecx.span_err(prefix.span, &format!("Cannot use prefix {} on this instruction", name));
                return Err(None);
            },
            "lock"  => if fmt.flags & flags::LOCK != 0 {
                (&mut group1, 0xF0)
            } else {
                ecx.span_err(prefix.span, &format!("Cannot use prefix {} on this instruction", name));
                return Err(None);
            },
            "ss"    => (&mut group2, 0x36),
            "cs"    => (&mut group2, 0x2E),
            "ds"    => (&mut group2, 0x3E),
            "es"    => (&mut group2, 0x26),
            "fs"    => (&mut group2, 0x64),
            "gs"    => (&mut group2, 0x65),
            _       => panic!("unimplemented prefix")
        };
        if group.is_some() {
            ecx.span_err(prefix.span, "Duplicate prefix group");
            return Err(None);
        }
        *group = Some(value);
    }

    Ok((group1, group2))
}

fn get_address_size(ecx: &ExtCtxt, args: &[Arg]) -> Result<Size, Option<String>> {
    let mut addr_size = None;
    for arg in args {
        if let Arg::Indirect(MemoryRef {span, ref index, ref base, ..}) = *arg {
            if let &Some(ref reg) = base {
                if addr_size.is_some() && addr_size != Some(reg.size()) {
                    ecx.span_err(span, "Conflicting address sizes");
                    return Err(None);
                }
                if reg.kind.family() != RegFamily::LEGACY && reg.kind.family() != RegFamily::RIP {
                    ecx.span_err(span, "Can only use normal registers as addresses");
                    return Err(None);
                }
                addr_size = Some(reg.size());
            }
            if let &Some(ref reg) = index {
                if addr_size.is_some() && addr_size != Some(reg.size()) {
                    ecx.span_err(span, "Conflicting address sizes");
                    return Err(None);
                }
                if reg.kind.family() != RegFamily::LEGACY && reg.kind.family() != RegFamily::RIP {
                    ecx.span_err(span, "Can only use normal registers as addresses");
                    return Err(None);
                }
                addr_size = Some(reg.size());
            }
        }
    }

    let addr_size = addr_size.unwrap_or(Size::QWORD);
    if addr_size != Size::DWORD && addr_size != Size::QWORD {
        return Err(Some(format!("Impossible address size")));
    }
    Ok(addr_size)
}

fn get_operand_size(fmt: &'static Opdata, args: &[Arg]) -> Result<Size, Option<String>> {
    // determine operand size to automatically determine appropriate prefixes
    // ensures that all operands have the same size, and that the immediate size is smaller or equal.

    let mut has_args = false;
    let mut op_size = None;
    let mut im_size = None;

    let mut sizes = fmt.args.chars();
    sizes.next();

    // only scan args which have wildcarded size
    for (arg, c) in args.iter().zip(sizes.step(2)) {
        if c != '*' {
            continue
        }
        match *arg {
            Arg::Direct(Spanned {node: ref reg, ..}) => {
                has_args = true;
                if op_size.is_some() && op_size.unwrap() != reg.size() {
                    return Err(Some("Conflicting operand sizes".to_string()));
                }
                op_size = Some(reg.size());
            },
            Arg::IndirectJumpTarget(_, size)    |
            Arg::Indirect(MemoryRef {size, ..}) => {
                has_args = true;
                if let Some(size) = size {
                    if op_size.is_some() && op_size.unwrap() != size {
                        return Err(Some("Conflicting operand sizes".to_string()));
                    }
                    op_size = Some(size);
                }
            },
            Arg::Immediate(_, size)  |
            Arg::JumpTarget(_, size) => {
                if let Some(size) = size { if im_size.is_none() || im_size.unwrap() < size {
                    im_size = Some(size);
                } }
            },
            Arg::Invalid => unreachable!()
        }
    }

    if has_args {
        if let Some(op_size) = op_size {
            if let Some(im_size) = im_size {
                if im_size > op_size {
                    return Err(Some("Immediate size larger than operand size".to_string()));
                }
            }
            Ok(op_size)
        } else {
            Err(Some("Unknown operand size".to_string()))
        }
    } else {
        // largest usual immediate size is assumed
        Ok(im_size.unwrap_or(Size::DWORD))
    }
}

fn validate_args(fmt: &'static Opdata, args: &[Arg], rex_w: bool) -> Result<bool, Option<String>> {
    // performs checks for (currently) not encodable arg combinations
    // output arg indicates if a rex prefix can be encoded
    let mut has_immediate   = false;
    let mut has_jumptarget  = false;
    let mut requires_rex    = rex_w;
    let mut requires_no_rex = false;

    for (arg, c) in args.iter().zip(fmt.args.chars().step(2)) {
        // only scan args that are actually encoded
        if let 'a' ... 'z' = c {
            match *arg {
                Arg::Direct(Spanned {node: ref reg, ..}) => {
                    if reg.kind.family() == RegFamily::HIGHBYTE {
                        requires_no_rex = true;

                    } else if reg.kind.is_extended() || (reg.size() == Size::BYTE &&
                        (reg.kind == RegId::RSP || reg.kind == RegId::RBP || reg.kind == RegId::RSI || reg.kind == RegId::RDI)) {
                        requires_rex = true;
                    }
                },
                Arg::Indirect(MemoryRef {ref base, ref index, ..}) => {
                    if let &Some(ref reg) = base {
                        requires_rex = requires_rex || reg.kind.is_extended();
                    }
                    if let &Some(ref reg) = index {
                        requires_rex = requires_rex || reg.kind.is_extended();
                    }
                },
                Arg::Immediate(_, _) => {
                    has_immediate = true;
                },
                Arg::JumpTarget(_, _)         |
                Arg::IndirectJumpTarget(_, _) => {
                    if has_jumptarget {
                        panic!("bad encoding data: multiple jump targets in the same instruction");
                    }
                    has_jumptarget = true;
                },
                Arg::Invalid => unreachable!()
            }
        }
    }

    if has_jumptarget && has_immediate {
        // note: this is a limitation in the encoding runtime, not in x64 itself
        Err(Some("Cannot encode jump target and immediate in the same instruction".to_string()))
    } else if requires_rex && requires_no_rex {
        Err(Some("High byte register combined with extended registers or 64-bit operand size".to_string()))
    } else {
        Ok(requires_rex)
    }
}

fn extract_args(fmt: &'static Opdata, args: Vec<Arg>) -> (Option<Arg>, Option<Arg>, Vec<Arg>) {
    // way operand order works:

    // if there's a memory/reg operand, this operand goes into modrm.r/m
    // if there's a segment/control/debug register, it goes into reg.

    // otherwise, the default is to put the first arg in modrm.r/m, which can be
    // changed using the DEST_IN_REG flag

    let mut memarg = None;
    let mut regarg = None;
    let mut regs = Vec::new();
    let mut immediates = Vec::new();
    let mut argcount = 0;

    for (arg, c) in args.into_iter().zip(fmt.args.chars().step(2)) {
        match c {
            'm' | 'u' | 'v' | 'w' => if memarg.is_some() {
                panic!("multiple memory arguments in format string");
            } else {
                argcount += 1;
                memarg = Some(arg);
            },
            'f' | 'x' | 'r' | 'y' => regs.push(arg),
            'c' | 'd' | 's'       => if regarg.is_some() {
                panic!("multiple segment, debug or control registers in format string");
            } else {
                argcount += 1;
                regarg = Some(arg);
            },
            'i' | 'o'             => immediates.push(arg),
            _ => () // hardcoded regs don't have to be encoded
        }
    }

    argcount += regs.len();
    if argcount > 2 {
        panic!("too many arguments");
    }

    if regarg.is_some() && memarg.is_some() {
        // empty on purpose
    } else if regarg.is_some() {
        // can't have a regarg without an memarg. This suggests invalid encoding data
        memarg = Some(regs.pop().unwrap());
    } else if memarg.is_some() {
        regarg = regs.pop();
    } else {
        let mut regs = regs.drain(..).fuse();
        memarg = regs.next();
        regarg = regs.next();
        if regarg.is_some() && (fmt.flags & flags::DEST_IN_REG) != 0 {
            swap(&mut regarg, &mut memarg);
        }
    }
    (regarg, memarg, immediates)
}

fn sanitize_memoryref(ecx: &ExtCtxt, mem: &mut MemoryRef) -> Result<(), Option<String>> {
    // sort out impossible scales
    mem.scale = match (mem.scale, &mut mem.base) {
        (0, _   ) => 0, // no index
        (1, _   ) => 0,
        (2, base @ &mut None) => { // size optimization. splits up [index * 2] into [index + index] 
            *base = mem.index.clone();
            0
        },
        (2, _   ) => 1,
        (4, _   ) => 2,
        (8, _   ) => 3,
        (3, base @ &mut None) => {
            *base = mem.index.clone();
            1
        },
        (5, base @ &mut None) => {
            *base = mem.index.clone();
            2
        },
        (9, base @ &mut None) => {
            *base = mem.index.clone();
            3
        },
        (scale, _) => {
            ecx.span_err(mem.span, &format!("Scale '{}' cannot be encoded", scale));
            return Err(None);
        }
    };

    // RSP as index field can not be represented.
    if mem.index == RegId::RSP {
        if mem.scale == 0 && mem.base != RegId::RSP {
            // swap index and base to make it encodable
            swap(&mut mem.base, &mut mem.index);
        } else {
            // as we always fill the base field first this is impossible to satisfy
            ecx.span_err(mem.span, "'rsp' cannot be used as index field");
            return Err(None);
        }
    }

    // RSP or R12 as base without index (add an index so we escape into SIB)
    if (mem.base == RegId::RSP || mem.base == RegId::R12) && mem.index.is_none() {
        mem.index = Some(Register::new_static(Size::QWORD, RegId::RSP));
        mem.scale = 0;
    }

    // RIP as index
    if mem.index == RegId::RIP {
        ecx.span_err(mem.span, "'rip' cannot be used as index");
        return Err(None);
    }

    // RIP as base with index
    if mem.base == RegId::RIP && mem.index.is_some() {
        ecx.span_err(mem.span, "'rip' cannot be used as base when an index is to be encoded");
        return Err(None);
    }

    // RBP as base field just requires a mandatory MOD_DISP8. we don't sort that out here.
    // same for no base, as this requires a MOD_DISP32
    Ok(())
}

fn compile_rex(ecx: &ExtCtxt, buffer: &mut StmtBuffer, rex_w: bool, reg: RegKind, index: RegKind, base: RegKind) {
    let rex = 0x40 | (rex_w as u8)        << 3 |
                     (reg.encode()   & 8) >> 1 |
                     (index.encode() & 8) >> 2 |
                     (base.encode()  & 8) >> 3 ;
    let mut rex = ecx.expr_lit(ecx.call_site(), ast::LitKind::Byte(rex));

    if let RegKind::Dynamic(_, expr) = reg {
        rex = or_mask_shift_expr(ecx, rex, expr, 8, -1);
    }
    if let RegKind::Dynamic(_, expr) = index {
        rex = or_mask_shift_expr(ecx, rex, expr, 8, -2);
    }
    if let RegKind::Dynamic(_, expr) = base {
        rex = or_mask_shift_expr(ecx, rex, expr, 8, -3);
    }
    buffer.push(Stmt::ExprConst(rex));
}

// fn compile_vex(ecx: &ExtCtxt, fmt: &'static Opdata, rex_w, )

fn compile_modrm_sib(ecx: &ExtCtxt, buffer: &mut StmtBuffer, mode: u8, reg1: RegKind, reg2: RegKind) {
    let byte = mode               << 6 |
                (reg1.encode() & 7) << 3 |
                (reg2.encode()  & 7)      ;

    let mut byte = ecx.expr_lit(ecx.call_site(), ast::LitKind::Byte(byte));

    if let RegKind::Dynamic(_, expr) = reg1 {
        byte = or_mask_shift_expr(ecx, byte, expr, 7, 3);
    }
    if let RegKind::Dynamic(_, expr) = reg2 {
        byte = or_mask_shift_expr(ecx, byte, expr, 7, 0);
    }
    buffer.push(Stmt::ExprConst(byte));
}
