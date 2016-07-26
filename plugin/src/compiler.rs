use syntax::ext::base::ExtCtxt;
use syntax::ext::build::AstBuilder;
use syntax::ast;
use syntax::ptr::P;
use syntax::codemap::Spanned;

use parser::{self, Item, Arg, Ident, MemoryRef, Register, RegKind, RegId, Size, LabelType, JumpType};
use x64data::get_mnemnonic_data;
use serialize::or_mask_shift_expr;

use std::mem::swap;

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
    pub const REGISTER_IN_OPCODE: u16 = 0x0001; // instead of encoding a ModRM byte, the register encoding is added to the opcode
    pub const DEFAULT_REXSIZE   : u16 = 0x0002; // this instruction defaults to a QWORD operand size. no DWORD size variant exists.
    // possible required prefixes
    pub const REQUIRES_REP      : u16 = 0x0004;
    pub const REQUIRES_ADDRSIZE : u16 = 0x0008;
    pub const REQUIRES_OPSIZE   : u16 = 0x0010;
    pub const REQUIRES_REXSIZE  : u16 = 0x0020;
    // parsing modifiers
    pub const SIZE_OVERRIDE     : u16 = 0x0040; // the operand sizes of the opcodes don't match up
    pub const RAX_ONLY          : u16 = 0x0080; // some instructions only operate on al/ax/eax
    // allowed prefixes
    pub const CAN_LOCK          : u16 = 0x0100;
    pub const CAN_REP           : u16 = 0x0200;
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

    // determine operand size if not overridden
    let mut op_size = if (data.flags & flags::SIZE_OVERRIDE) == 0 {
        try!(get_operand_size(ecx, &args))
    } else {
        Size::DWORD
    };

    // determine address size
    let addr_size = try!(get_address_size(ecx, &args));

    // correct for ops which by default operate on QWORDS instead of DWORDS
    if (data.flags & flags::DEFAULT_REXSIZE) != 0 {
        if op_size == Size::DWORD {
            ecx.span_err(op.span, &format!("'{}': argument size mismatch", &*op.node.name.as_str()));
            return Err(None);
        } else if op_size == Size::QWORD {
            op_size = Size::DWORD;
        }
    }

    // determine legacy prefixes
    let prefixes = try!(get_legacy_prefixes(ecx, data, prefixes, op_size, addr_size));
    buffer.extend(prefixes.into_iter().filter_map(|x| x.clone()));

    let (reg, rm, args) = extract_args(data, args);

    // do we need to encode a REX byte
    let need_rexsize = op_size == Size::QWORD || (data.flags & flags::REQUIRES_REXSIZE) != 0;

    // no register args
    if rm.is_none() {

        compile_rex(ecx, buffer, need_rexsize, RegKind::from_number(0), RegKind::from_number(0), RegKind::from_number(0));
        buffer.extend(data.ops.iter().map(|x| Stmt::Const(*x)));

    // register encoded in opcode byte
    } else if (data.flags & flags::REGISTER_IN_OPCODE) != 0 {

        let reg = if let Some(Arg::Direct(reg)) = rm {
            try!(guard_impossible_regs(ecx, data, reg))
        } else {
            panic!("bad encoding data, expected register");
        };

        compile_rex(ecx, buffer, need_rexsize, reg.clone(), RegKind::from_number(0), RegKind::from_number(0));

        let (last, rest) = data.ops.split_last().expect("bad encoding data, no opcode specified");
        buffer.extend(rest.into_iter().map(|x| Stmt::Const(*x)));

        if let RegKind::Dynamic(expr) = reg {
            let last = ecx.expr_lit(ecx.call_site(), ast::LitKind::Byte(*last));
            buffer.push(Stmt::ExprConst(or_mask_shift_expr(ecx, last, expr, 7, 0)));
        } else {
            buffer.push(Stmt::Const(last + (reg.encode() & 7)));
        }

    // ModRM byte
    } else if let Some(Arg::Direct(rm)) = rm {
        let rm = try!(guard_impossible_regs(ecx, data, rm));

        let reg = if let Some(Arg::Direct(reg)) = reg {
            try!(guard_impossible_regs(ecx, data, reg))
        } else {
            // reg is given by the instruction encoding
            RegKind::from_number(data.reg)
        };

        compile_rex(ecx, buffer, need_rexsize, reg.clone(), RegKind::from_number(0), rm.clone());
        buffer.extend(data.ops.iter().map(|x| Stmt::Const(*x)));
        // ModRM. if RAX_ONLY is used we don't encode this.
        if (data.flags & flags::RAX_ONLY) == 0 {
            compile_modrm_sib(ecx, buffer, MOD_DIRECT, reg, rm);
        }

    // ModRM and SIB byte
    } else if let Some(Arg::Indirect(mut mem)) = rm {

        let reg = if let Some(Arg::Direct(reg)) = reg {
            try!(guard_impossible_regs(ecx, data, reg))
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

            // rex prefix
            compile_rex(ecx, buffer, need_rexsize, reg.clone(), index.clone(), base.clone());
            buffer.extend(data.ops.iter().map(|x| Stmt::Const(*x)));
            compile_modrm_sib(ecx, buffer, mode, reg, RegKind::Static(RegId::RSP));
            compile_modrm_sib(ecx, buffer, mem.scale as u8, index, base);

        // no index, only a base.
        } else if let Some(base) = mem.base {

            // RBP at MOD_NODISP is used to encode RIP
            let base = if rip_relative {
                RegKind::Static(RegId::RBP)
            } else {
                base.kind
            };

            compile_rex(ecx, buffer, need_rexsize, reg.clone(), RegKind::from_number(0), base.clone());
            buffer.extend(data.ops.iter().map(|x| Stmt::Const(*x)));
            compile_modrm_sib(ecx, buffer, mode, reg, base);

        // no base, no index. only disp
        } else {
            // escape using RBP as base and RSP as index
            compile_rex(ecx, buffer, need_rexsize, reg.clone(), RegKind::from_number(0), RegKind::from_number(0));
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
            try!(guard_impossible_regs(ecx, data, reg))
        } else { // reg is given by the instruction encoding
            RegKind::from_number(data.reg)
        };

        compile_rex(ecx, buffer, need_rexsize, reg.clone(), RegKind::from_number(0), RegKind::from_number(0));
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

fn compile_rex(ecx: &ExtCtxt, buffer: &mut StmtBuffer, rexsize: bool, reg: RegKind, index: RegKind, base: RegKind) {
    if rexsize || reg.is_extended() || index.is_extended() || base.is_extended() {
        let rex = 0x40 | (rexsize as u8)      << 3 | 
                         (reg.encode()   & 8) >> 1 |
                         (index.encode() & 8) >> 2 |
                         (base.encode()  & 8) >> 3 ;
        let mut rex = ecx.expr_lit(ecx.call_site(), ast::LitKind::Byte(rex));

        if let RegKind::Dynamic(expr) = reg {
            rex = or_mask_shift_expr(ecx, rex, expr, 8, -1);
        }
        if let RegKind::Dynamic(expr) = index {
            rex = or_mask_shift_expr(ecx, rex, expr, 8, -2);
        }
        if let RegKind::Dynamic(expr) = base {
            rex = or_mask_shift_expr(ecx, rex, expr, 8, -3);
        }
        buffer.push(Stmt::ExprConst(rex));
    }
}

fn compile_modrm_sib(ecx: &ExtCtxt, buffer: &mut StmtBuffer, mode: u8, reg1: RegKind, reg2: RegKind) {
    let byte = mode               << 6 |
                (reg1.encode() & 7) << 3 |
                (reg2.encode()  & 7)      ;

    let mut byte = ecx.expr_lit(ecx.call_site(), ast::LitKind::Byte(byte));

    if let RegKind::Dynamic(expr) = reg1 {
        byte = or_mask_shift_expr(ecx, byte, expr, 7, 3);
    }
    if let RegKind::Dynamic(expr) = reg2 {
        byte = or_mask_shift_expr(ecx, byte, expr, 7, 0);
    }
    buffer.push(Stmt::ExprConst(byte));
}

fn extract_args(fmt: &'static Opdata, mut args: Vec<Arg>) -> (Option<Arg>, Option<Arg>, Vec<Arg>) {
    // determine the operand encoding
    let mut regidx = fmt.args.chars().position(|c| c == 'r').map(|i| i / 2);
    let mut rmidx  = fmt.args.chars().position(|c| c == 'm' || c == 'v').map(|i| i / 2);

    if rmidx.is_none() {
        rmidx = regidx;
        regidx = None;
    }

    if let Some(regidx) = regidx {
        let rmidx = rmidx.unwrap();
        if rmidx > regidx {
            let rm = args.remove(rmidx);
            let reg = args.remove(regidx);
            (Some(reg), Some(rm), args)
        } else {
            let reg = args.remove(regidx);
            let rm = args.remove(rmidx);
            (Some(reg), Some(rm), args)
        }
    } else if let Some(rmidx) = rmidx {
        let rm = args.remove(rmidx);
        (None, Some(rm), args)
    } else {
        (None, None, args)
    }
}

fn guard_impossible_regs(ecx: &ExtCtxt, fmt: &'static Opdata, reg: Spanned<Register>) -> Result<RegKind, Option<String>> {
    if reg.node == RegId::RIP {
        ecx.span_err(reg.span, "'rip' can only be used as a memory offset");
        Err(None)
    } else if (fmt.flags & flags::RAX_ONLY) != 0 && reg.node != RegId::RAX {
        ecx.span_err(reg.span, "this instruction only allows AL, AX, EAX or RAX to be used as argument");
        Err(None)
    } else {
        Ok(reg.node.kind)
    }
}

fn sanitize_memoryref(ecx: &ExtCtxt, mem: &mut MemoryRef) -> Result<(), Option<String>> {
    // sort out impossible scales
    mem.scale = match (mem.scale, &mut mem.base) {
        (0, _   ) => 0, // no index
        (1, _   ) => 0,
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

fn get_address_size(ecx: &ExtCtxt, args: &[Arg]) -> Result<Size, Option<String>> {
    let mut addr_size = None;
    for arg in args {
        if let Arg::Indirect(MemoryRef {span, ref index, ref base, ..}) = *arg {
            if let &Some(ref reg) = base {
                if addr_size.is_some() && addr_size != Some(reg.size()) {
                    ecx.span_err(span, "Conflicting address sizes");
                    return Err(None);
                }
                addr_size = Some(reg.size());
            }
            if let &Some(ref reg) = index {
                if addr_size.is_some() && addr_size != Some(reg.size()) {
                    ecx.span_err(span, "Conflicting address sizes");
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

fn get_operand_size(ecx: &ExtCtxt, args: &[Arg]) -> Result<Size, Option<String>> {
    // determine operand size.
    // ensures that all operands have the same size, and that the immediate size is smaller or equal.
    // if no operands are present, the immediate size is used. if no immediates are present, the default size is used

    let mut op_size = None;
    let mut im_size = None;
    let mut has_immediate = false;
    let mut has_args = false;
    let mut has_jumptarget = false;

    for arg in args {
        match *arg {
            Arg::Direct(Spanned {node: ref reg, span}) => {
                has_args = true;
                let size = reg.size();
                if op_size.is_some() && op_size != Some(size) {
                    ecx.span_err(span, "Conflicting operand sizes");
                    return Err(None);
                }
                op_size = Some(size)
            },
            Arg::Indirect(MemoryRef {size, span, ..}) => {
                has_args = true;
                if let Some(size) = size {
                    if op_size.is_some() && op_size != Some(size) {
                        ecx.span_err(span, "Conflicting operand sizes");
                        return Err(None);
                    }
                    op_size = Some(size);
                }
            },
            Arg::Immediate(_, size) => {
                has_immediate = true;
                if let Some(size) = size {
                    if im_size.is_none() || im_size.unwrap() < size {
                        im_size = Some(size);
                    }
                }
            },
            Arg::JumpTarget(_, size) => { // TODO: check if this codepath is still relevant
                if has_jumptarget {
                    panic!("bad encoding data: multiple jump targets in the same instruction");
                }
                has_jumptarget = true;
                if let Some(size) = size {
                    im_size = Some(size);
                }
            },
            Arg::IndirectJumpTarget(_, size) => {
                has_args = true;
                if has_jumptarget {
                    panic!("bad encoding data: multiple jump targets in the same instruction");
                }
                has_jumptarget = true;
                if let Some(size) = size {
                    if op_size.is_some() && op_size != Some(size) {
                        return Err(Some("conflicting operand sizes".to_string()));
                    }
                    op_size = Some(size)
                }
            },
            Arg::Invalid => unreachable!()
        }
    }

    if has_jumptarget && has_immediate {
        panic!("bad encoding data: jump target and immediate in the same instruction");
    }

    if let Some(op_size) = op_size {
        if let Some(im_size) = im_size {
            if im_size > op_size {
                return Err(Some(format!("Immediate size larger than operand size")));
            }
        }
        Ok(op_size)
    } else if let Some(im_size) = im_size {
        Ok(im_size)
    } else if has_args {
        return Err(Some(format!("Unknown argument size")));
    } else {
        Ok(Size::DWORD)
    }
}

fn get_legacy_prefixes(ecx: &ExtCtxt, fmt: &'static Opdata, idents: Vec<Ident>, op_size: Size, addr_size: Size) -> Result<[Option<Stmt>; 4], Option<String>> {
    let mut group1 = None;
    let mut group2 = None;
    let mut group3 = None;
    let mut group4 = None;

    for prefix in idents {
        let name = &*prefix.node.name.as_str();
        let (group, value) = match name {
            "rep"   |
            "repe"  |
            "repz"  => if fmt.flags & flags::CAN_REP != 0 {
                (&mut group1, 0xF3)
            } else {
                ecx.span_err(prefix.span, &format!("Cannot use prefix {} on this instruction", name));
                return Err(None);
            },
            "repnz" |
            "repne" => if fmt.flags & flags::CAN_REP != 0 {
                (&mut group1, 0xF2)
            } else {
                ecx.span_err(prefix.span, &format!("Cannot use prefix {} on this instruction", name));
                return Err(None);
            },
            "lock"  => if fmt.flags & flags::CAN_LOCK != 0 {
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
        *group = Some(Stmt::Const(value));
    }

    if (fmt.flags & flags::REQUIRES_REP) != 0 {
        group1 = Some(Stmt::Const(0xF3));
    }

    if (fmt.flags & flags::REQUIRES_OPSIZE) != 0 || op_size == Size::WORD {
        group3 = Some(Stmt::Const(0x66));
    }

    if (fmt.flags & flags::REQUIRES_ADDRSIZE) != 0 || addr_size == Size::DWORD {
        group4 = Some(Stmt::Const(0x67));
    }

    Ok([group1, group2, group3, group4])
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
    Err(Some(format!("'{}': argument type/size mismatch", name)))
}

fn match_format_string(fmt: &'static str, mut args: &mut [Arg]) -> Result<(), &'static str> {
    if fmt.len() != args.len() * 2 {
        return Err("argument length mismatch");
    }

    // i matches an immediate
    // c matches an instruction offset
    // r matches a reg
    // m matches memory
    // v matches r and m

    // b, w, d, q match a byte, word, doubleword and quadword.
    // * matches w, d, q if applied to reg/mem (default q), it matches w/d when applied to i or c. (default d)
    // ! matches a lack of size, only useful in combination of m and i
    {
        let mut fmt = fmt.chars();
        let mut args = args.iter();
        while let Some(code) = fmt.next() {
            let fsize = fmt.next().expect("invalid format string");
            let arg = args.next().unwrap();

            let size = match (code, arg) {
                ('i', &Arg::Immediate(_, size)) |
                ('c', &Arg::Immediate(_, size)) => size,
                ('c', &Arg::JumpTarget(_, size)) => size,
                ('r', &Arg::Direct(Spanned {node: ref reg, ..} )) |
                ('v', &Arg::Direct(Spanned {node: ref reg, ..} )) => Some(reg.size()),
                ('m', &Arg::Indirect(MemoryRef {size, ..} )) |
                ('m', &Arg::IndirectJumpTarget(_, size)) |
                ('v', &Arg::Indirect(MemoryRef {size, ..} )) => size,
                _ => return Err("argument type mismatch")
            };

            // if size is none it always matches (and will later be coerced to a more specific type if the match is successful)
            if let Some(size) = size {
                if !match (fsize, code) {
                    ('b', _)   => size == Size::BYTE,
                    ('w', _)   => size == Size::WORD,
                    ('d', _)   => size == Size::DWORD,
                    ('q', _)   => size == Size::QWORD,
                    ('*', 'i') => size == Size::WORD || size == Size::DWORD,
                    ('*', 'c') => size == Size::WORD || size == Size::DWORD,
                    ('*', _)   => size == Size::WORD || size == Size::DWORD || size == Size::QWORD,
                    ('!', _)   => false,
                    _ => panic!("invalid format string")
                } {
                    return Err("argument size mismatch");
                }
            }
        }
    }

    // we've found a match, update all specific constraints
    {
        let mut fmt = fmt.chars();
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
