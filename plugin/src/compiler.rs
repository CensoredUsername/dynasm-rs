use syntax::ext::base::ExtCtxt;
use syntax::ext::build::AstBuilder;
use syntax::ast;
use syntax::ptr::P;
use syntax::codemap::Spanned;

use parser::{self, Item, Arg, Ident, MemoryRef, Register, RegKind, RegFamily, RegId, Size, LabelType, JumpType};
use x64data::get_mnemnonic_data;
use x64data::flags::*;
use serialize::or_mask_shift_expr;
use debug::format_opdata_list;

use std::mem::swap;
use std::slice;
use std::iter;
use std::collections::hash_map::Entry;

/*
 * Compilation output
 */

pub type StmtBuffer = Vec<Stmt>;

#[derive(Clone, Debug)]
pub enum Stmt {
    Const(u8),
    ExprConst(P<ast::Expr>),

    Var(P<ast::Expr>, Size),
    Extend(P<ast::Expr>),

    DynScale(P<ast::Expr>, P<ast::Expr>),

    Align(P<ast::Expr>),

    GlobalLabel(Ident),
    LocalLabel(Ident),
    DynamicLabel(P<ast::Expr>),

    GlobalJumpTarget(Ident, Size),
    ForwardJumpTarget(Ident, Size),
    BackwardJumpTarget(Ident, Size),
    DynamicJumpTarget(P<ast::Expr>, Size),

    Stmt(ast::Stmt),
}

/*
 * Instruction encoding data formats
 */

pub struct Opdata {
    pub args:  &'static [u8],  // format string of arg format
    pub ops:   &'static [u8],
    pub reg:   u8,
    pub flags: Flags
}

pub struct FormatStringIterator<'a> {
    inner: iter::Cloned<slice::Iter<'a, u8>>
}

impl<'a> FormatStringIterator<'a> {
    pub fn new(buf: &'a [u8]) -> FormatStringIterator<'a> {
        FormatStringIterator {inner: buf.into_iter().cloned()}
    }
}

impl<'a> Iterator for FormatStringIterator<'a> {
    type Item = (u8, u8);

    fn next(&mut self) -> Option<(u8, u8)> {
        if let Some(ty) = self.inner.next() {
            let size = self.inner.next().expect("Invalid format string data");
            Some((ty, size))
        } else {
            None
        }
    }
}

/*
 * Instruction encoding constants
 */

const MOD_DIRECT: u8 = 0b11;
const MOD_NODISP: u8 = 0b00; // normal addressing
const MOD_NOBASE: u8 = 0b00; // VSIB addressing
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
            },
            Item::Stmt(stmt) => {
                stmts.push(Stmt::Stmt(stmt));
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
        "bytes" => directive_iter(ecx, buffer, args),
        "align" => {
            if args.len() != 1 {
                return Err(Some("Invalid amount of arguments".into()));
            }

            match args.pop().unwrap() {
                Arg::Immediate(expr, _) => {
                    buffer.push(Stmt::Align(expr));
                },
                _ => return Err(Some("this directive only uses immediate arguments".into()))
            }
            Ok(())
        },
        "alias" => {
            if args.len() != 2 {
                return Err(Some("Invalid amount of arguments".into()));
            }

            let reg = match args.pop().unwrap() {
                Arg::Direct(Spanned {node: Register {kind: RegKind::Static(id), size}, ..}) => (id, size),
                _ => {
                    return Err(Some("The second argument to alias should be a static register".into()));
                }
            };

            let alias = match args.pop().unwrap() {
                Arg::Immediate(expr, _) => parser::as_simple_name(&*expr),
                _ => None
            };

            let alias = if let Some(alias) = alias {
                alias.node.name
            } else {
                return Err(Some("The first argument to alias should be a non-keyword immediate".into()));
            };

            let global_data = super::crate_local_data(ecx);
            let mut lock = global_data.write();
            match lock.aliases.entry(alias) {
                Entry::Occupied(_) => return Err(Some(format!("Duplicate alias definition, alias '{}' was earlier defined", alias.as_str()))),
                Entry::Vacant(v) => v.insert(reg)
            };
            Ok(())
        },
        d => {
            ecx.span_err(dir.span, &format!("unknown directive '{}'", d));
            Err(None)
        }
    }
}

fn directive_const(ecx: &ExtCtxt, buffer: &mut StmtBuffer, args: Vec<Arg>, size: Size) -> Result<(), Option<String>> {
    if args.is_empty() {
        return Err(Some("this directive requires at least one argument".into()));
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
            _ => return Err(Some("this directive only uses immediate arguments".into()))
        }
    }

    Ok(())
}

fn directive_iter(_ecx: &ExtCtxt, buffer: &mut StmtBuffer, mut args: Vec<Arg>) -> Result<(), Option<String>> {
    if args.len() != 1 {
        return Err(Some("Wrong amount of arguments for this directive".into()))
    }

    if let Arg::Immediate(expr, None) = args.pop().unwrap() {
        buffer.push(Stmt::Extend(expr));
    } else {
        return Err(Some("wrong argument size".into()));
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
    // sanitize memory references and determine address size
    let pref_addr = try!(sanitize_addresses(ecx, &mut args));

    // this call also inserts more size information in the AST if applicable.
    let data = try!(match_op_format(ecx, op, &mut args));

    // determine legacy prefixes
    let (mut pref_mod, pref_seg) = try!(get_legacy_prefixes(ecx, data, prefixes));

    let mut op_size = Size::BYTE; // unused value, just here to please the compiler
    let mut pref_size = false;
    let mut rex_w = false;
    let mut vex_l = false;

    // determine if size prefixes are necessary
    if data.flags.intersects(AUTO_SIZE | AUTO_NO32 | AUTO_REXW | AUTO_VEXL) {
        // determine operand size
        op_size = try!(get_operand_size(data, &args));

        if data.flags.contains(AUTO_NO32) {
            if op_size == Size::WORD {
                pref_size = true;
            } else if op_size != Size::QWORD {
                return Err(Some(format!("'{}': Does not support 32 bit operands in 64-bit mode", &*op.node.name.as_str())));
            }
        } else if data.flags.contains(AUTO_REXW) {
            if op_size == Size::QWORD {
                rex_w = true;
            } else if op_size != Size::DWORD {
                return Err(Some(format!("'{}': Does not support 16-bit operands", &*op.node.name.as_str())));
            }
        } else if data.flags.contains(AUTO_VEXL) {
            if op_size == Size::HWORD {
                vex_l = true;
            } else if op_size != Size::OWORD {
                panic!("bad formatting data");
            }
        } else {
            if op_size == Size::WORD {
                pref_size = true;
            } else if op_size == Size::QWORD {
                rex_w = true;
            } else if op_size != Size::DWORD {
                panic!("bad formatting data");
            }
        }
    }

    // mandatory prefixes
    let pref_size = pref_size || data.flags.contains(WORD_SIZE);
    let rex_w     = rex_w     || data.flags.contains(WITH_REXW);
    let vex_l     = vex_l     || data.flags.contains(WITH_VEXL);
    let pref_addr = pref_addr || data.flags.contains(PREF_67);

    if        data.flags.contains(PREF_F0) { pref_mod = Some(0xF0);
    } else if data.flags.contains(PREF_F2) { pref_mod = Some(0xF2);
    } else if data.flags.contains(PREF_F3) { pref_mod = Some(0xF3);
    }

    // check if this combination of args can actually be encoded and whether a rex prefix is necessary
    let need_rex = try!(validate_args(data, &args, rex_w));

    // split args
    let (mut rm, reg, vvvv, ireg, mut args) = extract_args(data, args);

    let mut ops = data.ops;

    // legacy-only prefixes
    if let Some(pref) = pref_seg {
        buffer.push(Stmt::Const(pref));
    }
    if pref_addr {
        buffer.push(Stmt::Const(0x67));
    }

    // VEX/XOP prefixes embed the operand size prefix / modification prefixes in them.
    if data.flags.intersects(VEX_OP | XOP_OP) {
        let prefix = if pref_size        { 0b01
        } else if pref_mod == Some(0xF3) { 0b10
        } else if pref_mod == Some(0xF2) { 0b11
        } else                           { 0
        };
        // map_sel is stored in the first byte of the opcode
        let (map_sel, tail) = ops.split_first().expect("bad formatting data");
        ops = tail;
        compile_vex_xop(ecx, buffer, data, &reg, &rm, *map_sel, rex_w, &vvvv, vex_l, prefix);
    // otherwise, the size/mod prefixes have to be pushed and check if a rex prefix has to be generated.
    } else {
        if let Some(pref) = pref_mod {
            buffer.push(Stmt::Const(pref));
        }
        if pref_size {
            buffer.push(Stmt::Const(0x66));
        }
        if need_rex {
            compile_rex(ecx, buffer, rex_w, &reg, &rm);
        }
    }

    // if rm is embedded in the last opcode byte, push it here
    if data.flags.contains(SHORT_ARG) {
        let (last, head) = ops.split_last().expect("bad formatting data");
        ops = head;
        buffer.extend(ops.iter().cloned().map(Stmt::Const));

        let rm_k = if let Some(Arg::Direct(rm)) = rm.take() {
            rm.node.kind
        } else {
            panic!("bad formatting data")
        };

        if let RegKind::Dynamic(_, expr) = rm_k {
            let last = ecx.expr_lit(ecx.call_site(), ast::LitKind::Byte(*last));
            buffer.push(Stmt::ExprConst(or_mask_shift_expr(ecx, last, expr, 7, 0)));
        } else {
            buffer.push(Stmt::Const(last + (rm_k.encode() & 7)));
        }
    // just push the opcode
    } else {
        buffer.extend(ops.iter().cloned().map(Stmt::Const))
    }

    // Direct ModRM addressing
    if let Some(Arg::Direct(rm)) = rm {
        let reg_k = if let Some(Arg::Direct(reg)) = reg {
            reg.node.kind
        } else {
            RegKind::from_number(data.reg)
        };

        compile_modrm_sib(ecx, buffer, MOD_DIRECT, reg_k, rm.node.kind);
    // Indirect ModRM (+SIB) addressing
    } else if let Some(Arg::Indirect(mem)) = rm {
        let reg_k = if let Some(Arg::Direct(reg)) = reg {
            reg.node.kind
        } else {
            RegKind::from_number(data.reg)
        };
        // VSIB has different mode rules
        if mem.index.as_ref().map_or(false, |x| x.kind.family() == RegFamily::XMM) {
            let index = mem.index.unwrap().kind;
            let (base, mode) = if let Some(base) = mem.base {
                (base.kind, if mem.disp.is_some() {MOD_DISP32} else {MOD_DISP8})
            } else {
                (RegKind::Static(RegId::RBP), MOD_NOBASE)
            };
            compile_modrm_sib(ecx, buffer, mode, reg_k, RegKind::Static(RegId::RSP));
            if let Some(expr) = mem.scale_expr {
                compile_sib_dynscale(ecx, buffer, expr, index, base);
            } else {
                compile_modrm_sib(ecx, buffer, mem.scale as u8, index, base);
            }

            if mode == MOD_DISP8 {
                buffer.push(Stmt::Const(0));
            } else  if let Some(disp) = mem.disp {
                buffer.push(Stmt::Var(disp, Size::DWORD));
            } else {
                for _ in 0..4 {
                    buffer.push(Stmt::Const(0));
                }
            }
        // normal indirect addressing
        } else {

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
                // to encode the lack of a base we encode RBP
                let base = if let Some(base) = mem.base {
                    base.kind
                } else {
                    RegKind::Static(RegId::RBP)
                };

                compile_modrm_sib(ecx, buffer, mode, reg_k, RegKind::Static(RegId::RSP));
                if let Some(expr) = mem.scale_expr {
                    compile_sib_dynscale(ecx, buffer, expr, index.kind, base);
                } else {
                    compile_modrm_sib(ecx, buffer, mem.scale as u8, index.kind, base);
                }

            // no index, only a base. RBP at MOD_NODISP is used to encode RIP, but this is already handled
            } else if let Some(base) = mem.base {
                compile_modrm_sib(ecx, buffer, mode, reg_k, base.kind);

            // no base, no index. only disp. escape, use RBP as base and RSP as index
            } else {
                compile_modrm_sib(ecx, buffer, mode, reg_k, RegKind::Static(RegId::RSP));
                compile_modrm_sib(ecx, buffer, 0, RegKind::Static(RegId::RSP), RegKind::Static(RegId::RBP));
            }

            // Disp
            if let Some(disp) = mem.disp {
                buffer.push(Stmt::Var(disp, Size::DWORD));
            } else if no_base || rip_relative {
                for _ in 0..4 {
                    buffer.push(Stmt::Const(0));
                }
            } else if rbp_relative {
                buffer.push(Stmt::Const(0));
            }
        }
    // jump-target relative addressing
    } else if let Some(Arg::IndirectJumpTarget(target, _)) = rm {
        let reg_k = if let Some(Arg::Direct(reg)) = reg {
            reg.node.kind
        } else {
            RegKind::from_number(data.reg)
        };
        compile_modrm_sib(ecx, buffer, MOD_NODISP, reg_k, RegKind::Static(RegId::RBP));

        // note: validate_args ensures that no immediates are encoded afterwards.
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
    }

    // register in immediate argument
    if let Some(Arg::Direct(ireg)) = ireg {
        let ireg = ireg.node.kind;
        let byte = ireg.encode() << 4;

        let mut byte = ecx.expr_lit(ecx.call_site(), ast::LitKind::Byte(byte));
        if let RegKind::Dynamic(_, expr) = ireg {
            byte = or_mask_shift_expr(ecx, byte, expr, 0xF, 4);
        }
        // if immediates are present, the register argument will be merged into the
        // first immediate byte.
        if !args.is_empty() {
            if let Arg::Immediate(expr, Some(Size::BYTE)) = args.remove(0) {
                byte = or_mask_shift_expr(ecx, byte, expr, 0xF, 0);
            } else {
                panic!("bad formatting data")
            }
        }
        buffer.push(Stmt::ExprConst(byte))
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
            _ => panic!("bad immediate data")
        };
        buffer.push(stmt);
    }

    Ok(())
}

fn sanitize_addresses(ecx: &ExtCtxt, args: &mut [Arg]) -> Result<bool, Option<String>> {
    // determine if an address size prefix is necessary, and sanitize the register choice for memoryrefs
    let mut addr_size = None;
    for arg in args {
        if let Arg::Indirect(ref mut mem) = *arg {
            try!(sanitize_memoryref(ecx, mem));

            if let Some(ref reg) = mem.base {
                if reg.kind.family() == RegFamily::LEGACY || reg.kind.family() == RegFamily::RIP {
                    if addr_size.is_some() && addr_size != Some(reg.size()) {
                        ecx.span_err(mem.span, "Conflicting address sizes");
                        return Err(None);
                    }
                    addr_size = Some(reg.size());
                }
            }
            if let Some(ref reg) = mem.index {
                if reg.kind.family() == RegFamily::LEGACY || reg.kind.family() == RegFamily::RIP {
                    if addr_size.is_some() && addr_size != Some(reg.size()) {
                        ecx.span_err(mem.span, "Conflicting address sizes");
                        return Err(None);
                    }
                    addr_size = Some(reg.size());
                }
            }
        }
    }

    let addr_size = addr_size.unwrap_or(Size::QWORD);
    if addr_size != Size::DWORD && addr_size != Size::QWORD {
        return Err(Some("Impossible address size".into()));
    }
    Ok(addr_size != Size::QWORD)
}

fn sanitize_memoryref(ecx: &ExtCtxt, mem: &mut MemoryRef) -> Result<(), Option<String>> {
    // sort out impossible scales
    if let Some(ref index) = mem.index {
        mem.scale = match (mem.scale, mem.base.is_none()) {
            (1, _   ) => 0,
            (2, true) if index.kind.family() == RegFamily::LEGACY => {
                 // size optimization. splits up [index * 2] into [index + index]
                mem.base = mem.index.clone();
                0
            },
            (2, _   ) => 1,
            (4, _   ) => 2,
            (8, _   ) => 3,
            (3, true) if index.kind.family() == RegFamily::LEGACY => {
                mem.base = mem.index.clone();
                1
            },
            (5, true) if index.kind.family() == RegFamily::LEGACY => {
                mem.base = mem.index.clone();
                2
            },
            (9, true) if index.kind.family() == RegFamily::LEGACY => {
                mem.base = mem.index.clone();
                3
            },
            (scale, _) => {
                ecx.span_err(mem.span, &format!("Scale '{}' cannot be encoded", scale));
                return Err(None);
            }
        };
    }

    // VSIB addressing has simpler encoding rules, so detect it and return if it's used here.
    match mem.base {
        ref mut base @ Some(_) if base.as_ref().unwrap().kind.family() == RegFamily::XMM => match mem.index {
            ref mut index @ None => {
                swap(base, index);
                mem.scale = 0;
                return Ok(());
            },
            ref mut index @ Some(_) if index.as_ref().unwrap().kind.family() == RegFamily::LEGACY && mem.scale == 0 => {
                swap(base, index);
                return Ok(());
            },
            _ => {
                ecx.span_err(mem.span, "vsib addressing requires a general purpose register as base");
                return Err(None);
            }
        },
        ref base => match mem.index {
            Some(ref index) if index.kind.family() == RegFamily::XMM => {
                if base.as_ref().map_or(true, |x| x.kind.family() == RegFamily::LEGACY) {
                    return Ok(());
                } else {
                    ecx.span_err(mem.span, "vsib addressing requires a general purpose register as base");
                    return Err(None);
                }
            },
            _ => ()
        }
    }

    // check that only legacy regs / rip are used:
    if mem.base.as_ref().map_or(false, |x| x.kind.family() != RegFamily::LEGACY && x.kind != RegId::RIP) {
        ecx.span_err(mem.span, "bad register type as base");
        return Err(None);
    } else if mem.index.as_ref().map_or(false, |x| x.kind.family() != RegFamily::LEGACY) {
        ecx.span_err(mem.span, "bad register type as index");
        return Err(None);
    }

    // RIP as base with index
    if mem.base == RegId::RIP && mem.index.is_some() {
        ecx.span_err(mem.span, "'rip' cannot be used as base when an index is present");
        return Err(None);
    }

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

    // RBP as base field just requires a mandatory MOD_DISP8. we don't sort that out here.
    // same for no base, as this requires a MOD_DISP32
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
        if let Ok(_) = match_format_string(format.args, args) {
            return Ok(format)
        }
    }

    Err(Some(
        format!("'{}': argument type/size mismatch, expected one of the following forms:\n{}", name, format_opdata_list(name, data))
    ))
}

fn match_format_string(fmtstr: &'static [u8], mut args: &mut [Arg]) -> Result<(), &'static str> {
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
    // X: matches st0

    // b, w, d, q match a byte, word, doubleword and quadword.
    // * matches all possible sizes for this operand (w/d for i/o, w/d/q for r/v, o/h for y/w and everything for m)
    // ! matches a lack of size, only useful in combination of m and i
    // ? matches any size and doesn't participate in the operand size calculation
    {
        let mut args = args.iter();
        for (code, fsize) in FormatStringIterator::new(fmtstr) {
            let arg = args.next().unwrap();

            let size = match (code, arg) {
                // immediates
                (b'i', &Arg::Immediate(_, size))  |
                (b'o', &Arg::Immediate(_, size))  |
                (b'o', &Arg::JumpTarget(_, size)) => size,

                // specific legacy regs
                (x @ b'A' ... b'P', &Arg::Direct(Spanned {node: ref reg, ..} )) if
                    reg.kind.family() == RegFamily::LEGACY &&
                    reg.kind.code() == Some(x - b'A') => Some(reg.size()),

                // specific segment regs
                (x @ b'Q' ... b'V', &Arg::Direct(Spanned {node: ref reg, ..} )) if
                    reg.kind.family() == RegFamily::SEGMENT &&
                    reg.kind.code() == Some(x - b'Q') => Some(reg.size()),

                // CR8 can be specially referenced
                (b'W', &Arg::Direct(Spanned {node: ref reg, ..} )) if
                    reg.kind == RegId::CR8 => Some(reg.size()),

                // top of the fp stack is also often used
                (b'X', &Arg::Direct(Spanned {node: ref reg, ..} )) if
                    reg.kind == RegId::ST0 => Some(reg.size()),

                // generic legacy regs
                (b'r', &Arg::Direct(Spanned {node: ref reg, ..} )) |
                (b'v', &Arg::Direct(Spanned {node: ref reg, ..} )) if
                    reg.kind.family() == RegFamily::LEGACY ||
                    reg.kind.family() == RegFamily::HIGHBYTE => Some(reg.size()),

                // other reg types often mixed with memory refs
                (b'x', &Arg::Direct(Spanned {node: ref reg, ..} )) |
                (b'u', &Arg::Direct(Spanned {node: ref reg, ..} )) if
                    reg.kind.family() == RegFamily::MMX => Some(reg.size()),
                (b'y', &Arg::Direct(Spanned {node: ref reg, ..} )) |
                (b'w', &Arg::Direct(Spanned {node: ref reg, ..} )) if
                    reg.kind.family() == RegFamily::XMM => Some(reg.size()),

                // other reg types
                (b'f', &Arg::Direct(Spanned {node: ref reg, ..} )) if
                    reg.kind.family() == RegFamily::FP => Some(reg.size()),
                (b's', &Arg::Direct(Spanned {node: ref reg, ..} )) if
                    reg.kind.family() == RegFamily::SEGMENT => Some(reg.size()),
                (b'c', &Arg::Direct(Spanned {node: ref reg, ..} )) if
                    reg.kind.family() == RegFamily::CONTROL => Some(reg.size()),
                (b'd', &Arg::Direct(Spanned {node: ref reg, ..} )) if
                    reg.kind.family() == RegFamily::DEBUG => Some(reg.size()),

                // memory offsets
                (b'm',          &Arg::Indirect(MemoryRef {size, ref index, ..} )) |
                (b'u' ... b'w', &Arg::Indirect(MemoryRef {size, ref index, ..} )) if
                    index.is_none() || index.as_ref().unwrap().kind.family() != RegFamily::XMM => size,

                (b'm',          &Arg::IndirectJumpTarget(_, size)) |
                (b'u' ... b'w', &Arg::IndirectJumpTarget(_, size)) => size,

                // vsib addressing. as they have two sizes that must be checked they check one of the sizes here
                (b'k', &Arg::Indirect(MemoryRef {size, index: Some(ref index), ..} )) if
                    (size.is_none() || size == Some(Size::DWORD)) &&
                    index.kind.family() == RegFamily::XMM => Some(index.size()),
                (b'l', &Arg::Indirect(MemoryRef {size, index: Some(ref index), ..} )) if
                    (size.is_none() ||  size == Some(Size::QWORD)) &&
                    index.kind.family() == RegFamily::XMM => Some(index.size()),
                _ => return Err("argument type mismatch")
            };

            // if size is none it always matches (and will later be coerced to a more specific type if the match is successful)
            if let Some(size) = size {
                if !match (fsize, code) {
                    (b'b', _)    => size == Size::BYTE,
                    (b'w', _)    => size == Size::WORD,
                    (b'd', _)    => size == Size::DWORD,
                    (b'q', _)    => size == Size::QWORD,
                    (b'p', _)    => size == Size::PWORD,
                    (b'o', _)    => size == Size::OWORD,
                    (b'h', _)    => size == Size::HWORD,
                    (b'*', b'i') |
                    (b'*', b'o') => size == Size::WORD || size == Size::DWORD,
                    (b'*', b'k') |
                    (b'*', b'l') |
                    (b'*', b'y') |
                    (b'*', b'w') => size == Size::OWORD || size == Size::HWORD,
                    (b'*', b'r') |
                    (b'*', b'A' ... b'P') |
                    (b'*', b'v') => size == Size::WORD || size == Size::DWORD || size == Size::QWORD,
                    (b'*', b'm') => true,
                    (b'*', _)    => panic!("Invalid size wildcard"),
                    (b'?', _)    => true,
                    (b'!', _)    => false,
                    _ => panic!("invalid format string")
                } {
                    return Err("argument size mismatch");
                }
            }
        }
    }

    // we've found a match, update all specific constraints
    {
        let mut args = args.iter_mut();
        for (code, fsize) in FormatStringIterator::new(fmtstr) {
            let arg: &mut Arg = args.next().unwrap();

            match *arg {
                Arg::Immediate(_, ref mut size @ None) |
                Arg::JumpTarget(_, ref mut size @ None) |
                Arg::Indirect(MemoryRef {size: ref mut size @ None, ..} ) => *size = match (fsize, code) {
                    (b'b', _) => Some(Size::BYTE),
                    (b'w', _) => Some(Size::WORD),
                    (_, b'k') |
                    (b'd', _) => Some(Size::DWORD),
                    (_, b'l') |
                    (b'q', _) => Some(Size::QWORD),
                    (b'p', _) => Some(Size::PWORD),
                    (b'o', _) => Some(Size::OWORD),
                    (b'h', _) => Some(Size::HWORD),
                    (b'*', _) |
                    (b'!', _) => None,
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
            "rep"   => if fmt.flags.contains(REP) {
                (&mut group1, 0xF3)
            } else {
                ecx.span_err(prefix.span, &format!("Cannot use prefix {} on this instruction", name));
                return Err(None);
            },
            "repe"  |
            "repz"  => if fmt.flags.contains(REPE) {
                (&mut group1, 0xF3)
            } else {
                ecx.span_err(prefix.span, &format!("Cannot use prefix {} on this instruction", name));
                return Err(None);
            },
            "repnz" |
            "repne" => if fmt.flags.contains(REP) {
                (&mut group1, 0xF2)
            } else {
                ecx.span_err(prefix.span, &format!("Cannot use prefix {} on this instruction", name));
                return Err(None);
            },
            "lock"  => if fmt.flags.contains(LOCK) {
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

fn get_operand_size(fmt: &'static Opdata, args: &[Arg]) -> Result<Size, Option<String>> {
    // determine operand size to automatically determine appropriate prefixes
    // ensures that all operands have the same size, and that the immediate size is smaller or equal.

    let mut has_args = false;
    let mut op_size = None;
    let mut im_size = None;

    // only scan args which have wildcarded size
    for (arg, (_, size)) in args.iter().zip(FormatStringIterator::new(fmt.args)) {
        if size != b'*' {
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
            Arg::IndirectJumpTarget(_, size) => {
                has_args = true;
                if let Some(size) = size {
                    if op_size.is_some() && op_size.unwrap() != size {
                        return Err(Some("Conflicting operand sizes".to_string()));
                    }
                    op_size = Some(size);
                }
            },
            Arg::Indirect(MemoryRef {mut size, ref index, ..}) => {
                has_args = true;
                // for vsib addressing we're interested in the size of the address vector
                if let Some(ref reg) = *index {
                    if reg.kind.family() == RegFamily::XMM {
                        size = Some(reg.size());
                    }
                }
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
                if im_size != op_size && !(im_size == Size::DWORD && op_size == Size::QWORD) {
                    return Err(Some("Immediate size mismatch".to_string()));
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

    for (arg, (c, _)) in args.iter().zip(FormatStringIterator::new(fmt.args)) {
        // only scan args that are actually encoded
        if let b'a' ... b'z' = c {
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
                    if let Some(ref reg) = *base {
                        requires_rex = requires_rex || reg.kind.is_extended();
                    }
                    if let Some(ref reg) = *index {
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

fn extract_args(fmt: &'static Opdata, args: Vec<Arg>) -> (Option<Arg>, Option<Arg>, Option<Arg>, Option<Arg>, Vec<Arg>) {
    // way operand order works:

    // if there's a memory/reg operand, this operand goes into modrm.r/m
    // if there's a segment/control/debug register, it goes into reg.

    // default argument encoding order is as follows:
    // no encoding flag: m, rm, rvm, rvim
    // ENC_MR:              mr, rmv, rvmi
    // ENC_VM:              vm, mvr
    // these can also be chosen based on the location of a memory argument (except for vm)

    let mut memarg = None;
    let mut regarg = None;
    let mut regs = Vec::new();
    let mut immediates = Vec::new();

    for (arg, (c, _)) in args.into_iter().zip(FormatStringIterator::new(fmt.args)) {
        match c {
            b'm' | b'u' | b'v' | b'w' | b'k' | b'l'  => if memarg.is_some() {
                panic!("multiple memory arguments in format string");
            } else {
                memarg = Some(regs.len());
                regs.push(arg)
            },
            b'f' | b'x' | b'r' | b'y' => regs.push(arg),
            b'c' | b'd' | b's'        => if regarg.is_some() {
                panic!("multiple segment, debug or control registers in format string");
            } else {
                regarg = Some(regs.len());
                regs.push(arg)
            },
            b'i' | b'o' => immediates.push(arg),
            _ => () // hardcoded regs don't have to be encoded
        }
    }

    let len = regs.len();
    if len > 4 {
        panic!("too many arguments");
    }
    let mut regs = regs.drain(..).fuse();

    let mut m = None;
    let mut r = None;
    let mut v = None;
    let mut i = None;

    if let Some(i) = regarg {
        if i == 0 {
            r = regs.next();
            m = regs.next();
        } else {
            m = regs.next();
            r = regs.next();
        }
    } else if len == 1 {
        m = regs.next();
    } else if len == 2 {
        if fmt.flags.contains(ENC_MR) || memarg == Some(0) {
            m = regs.next();
            r = regs.next();
        } else if fmt.flags.contains(ENC_VM) {
            v = regs.next();
            m = regs.next();
        } else {
            r = regs.next();
            m = regs.next();
        }
    } else if len == 3 {
        if fmt.flags.contains(ENC_MR) || memarg == Some(1) {
            r = regs.next();
            m = regs.next();
            v = regs.next();
        } else if fmt.flags.contains(ENC_VM) || memarg == Some(0) {
            m = regs.next();
            v = regs.next();
            r = regs.next();
        } else {
            r = regs.next();
            v = regs.next();
            m = regs.next();
        }
    } else if len == 4 {
        if fmt.flags.contains(ENC_MR) || memarg == Some(2) {
            r = regs.next();
            v = regs.next();
            m = regs.next();
            i = regs.next();
        } else {
            r = regs.next();
            v = regs.next();
            i = regs.next();
            m = regs.next();
        }
    }

    (m, r, v, i, immediates)
}

fn compile_rex(ecx: &ExtCtxt, buffer: &mut StmtBuffer, rex_w: bool, reg: &Option<Arg>, rm: &Option<Arg>) {
    let mut reg_k   = RegKind::from_number(0);
    let mut index_k = RegKind::from_number(0);
    let mut base_k  = RegKind::from_number(0);

    if let Some(Arg::Direct(ref reg)) = *reg {
        reg_k = reg.node.kind.clone();
    }
    if let Some(Arg::Direct(ref rm)) = *rm {
        base_k = rm.node.kind.clone();
    }
    if let Some(Arg::Indirect(MemoryRef {ref base, ref index, ..} )) = *rm {
        if let Some(ref base) = *base {
            base_k = base.kind.clone();
        }
        if let Some(ref index) = *index {
            index_k = index.kind.clone();
        }
    }

    let rex = 0x40 | (rex_w          as u8) << 3 |
                     (reg_k.encode()   & 8) >> 1 |
                     (index_k.encode() & 8) >> 2 |
                     (base_k.encode()  & 8) >> 3 ;
    if !reg_k.is_dynamic() && !index_k.is_dynamic() && !base_k.is_dynamic() {
        buffer.push(Stmt::Const(rex));
        return;
    }

    let mut rex = ecx.expr_lit(ecx.call_site(), ast::LitKind::Byte(rex));

    if let RegKind::Dynamic(_, expr) = reg_k {
        rex = or_mask_shift_expr(ecx, rex, expr, 8, -1);
    }
    if let RegKind::Dynamic(_, expr) = index_k {
        rex = or_mask_shift_expr(ecx, rex, expr, 8, -2);
    }
    if let RegKind::Dynamic(_, expr) = base_k {
        rex = or_mask_shift_expr(ecx, rex, expr, 8, -3);
    }
    buffer.push(Stmt::ExprConst(rex));
}

fn compile_vex_xop(ecx: &ExtCtxt, buffer: &mut StmtBuffer, data: &'static Opdata, reg: &Option<Arg>,
rm: &Option<Arg>, map_sel: u8, rex_w: bool, vvvv: &Option<Arg>, vex_l: bool, prefix: u8) {
    let mut reg_k   = RegKind::from_number(0);
    let mut index_k = RegKind::from_number(0);
    let mut base_k  = RegKind::from_number(0);
    let mut vvvv_k  = RegKind::from_number(0);

    if let Some(Arg::Direct(ref reg)) = *reg {
        reg_k = reg.node.kind.clone();
    }
    if let Some(Arg::Direct(ref rm)) = *rm {
        base_k = rm.node.kind.clone();
    }
    if let Some(Arg::Indirect(MemoryRef {ref base, ref index, ..} )) = *rm {
        if let Some(ref base) = *base {
            base_k = base.kind.clone();
        }
        if let Some(ref index) = *index {
            index_k = index.kind.clone();
        }
    }
    if let Some(Arg::Direct(ref vvvv)) = *vvvv {
        vvvv_k = vvvv.node.kind.clone();
    }

    let byte1 = (map_sel        & 0x1F)      |
                (!reg_k.encode()   & 8) << 4 |
                (!index_k.encode() & 8) << 3 |
                (!base_k.encode()  & 8) << 2 ;

    let byte2 = (prefix           & 0x3)      |
                (rex_w            as u8) << 7 |
                (!vvvv_k.encode() & 0xF) << 3 |
                (vex_l            as u8) << 2 ;

    if data.flags.contains(VEX_OP) && (byte1 & 0x7F) == 0x61 && (byte2 & 0x80) == 0 && !index_k.is_dynamic() && !base_k.is_dynamic() {
        // 2-byte vex
        buffer.push(Stmt::Const(0xC5));

        let byte1 = (byte1 & 0x80) | (byte2 & 0x7F);
        if !reg_k.is_dynamic() {
            buffer.push(Stmt::Const(byte1));
            return;
        }

        let mut byte1 = ecx.expr_lit(ecx.call_site(), ast::LitKind::Byte(byte1));
        if let RegKind::Dynamic(_, expr) = reg_k {
            let expr = ecx.expr_unary(expr.span, ast::UnOp::Not, expr);
            byte1 = or_mask_shift_expr(ecx, byte1, expr, 8, 4)
        }
        buffer.push(Stmt::ExprConst(byte1));
        return;
    }

    buffer.push(Stmt::Const(if data.flags.contains(VEX_OP) {0xC4} else {0x8F}));

    if reg_k.is_dynamic() || index_k.is_dynamic() || base_k.is_dynamic() {
        let mut byte1 = ecx.expr_lit(ecx.call_site(), ast::LitKind::Byte(byte1));

        if let RegKind::Dynamic(_, expr) = reg_k {
            let expr = ecx.expr_unary(expr.span, ast::UnOp::Not, expr);
            byte1 = or_mask_shift_expr(ecx, byte1, expr, 8, 4);
        }
        if let RegKind::Dynamic(_, expr) = index_k {
            let expr = ecx.expr_unary(expr.span, ast::UnOp::Not, expr);
            byte1 = or_mask_shift_expr(ecx, byte1, expr, 8, 3);
        }
        if let RegKind::Dynamic(_, expr) = base_k {
            let expr = ecx.expr_unary(expr.span, ast::UnOp::Not, expr);
            byte1 = or_mask_shift_expr(ecx, byte1, expr, 8, 2);
        }
        buffer.push(Stmt::ExprConst(byte1));
    } else {
        buffer.push(Stmt::Const(byte1));
    }

    if vvvv_k.is_dynamic() {
        let mut byte2 = ecx.expr_lit(ecx.call_site(), ast::LitKind::Byte(byte2));

        if let RegKind::Dynamic(_, expr) = vvvv_k {
            let expr = ecx.expr_unary(expr.span, ast::UnOp::Not, expr);
            byte2 = or_mask_shift_expr(ecx, byte2, expr, 0xF, 3)
        }
        buffer.push(Stmt::ExprConst(byte2));
    } else {
        buffer.push(Stmt::Const(byte2));
    }
}

fn compile_modrm_sib(ecx: &ExtCtxt, buffer: &mut StmtBuffer, mode: u8, reg1: RegKind, reg2: RegKind) {
    let byte = mode                << 6 |
              (reg1.encode()  & 7) << 3 |
              (reg2.encode()  & 7)     ;

    if !reg1.is_dynamic() && !reg2.is_dynamic() {
        buffer.push(Stmt::Const(byte));
        return;
    }

    let mut byte = ecx.expr_lit(ecx.call_site(), ast::LitKind::Byte(byte));
    if let RegKind::Dynamic(_, expr) = reg1 {
        byte = or_mask_shift_expr(ecx, byte, expr, 7, 3);
    }
    if let RegKind::Dynamic(_, expr) = reg2 {
        byte = or_mask_shift_expr(ecx, byte, expr, 7, 0);
    }
    buffer.push(Stmt::ExprConst(byte));
}

fn compile_sib_dynscale(ecx: &ExtCtxt, buffer: &mut StmtBuffer, scale: P<ast::Expr>, reg1: RegKind, reg2: RegKind) {
    let byte = (reg1.encode()  & 7) << 3 |
               (reg2.encode()  & 7)      ;

    let mut byte = ecx.expr_lit(ecx.call_site(), ast::LitKind::Byte(byte));

    if let RegKind::Dynamic(_, expr) = reg1 {
        byte = or_mask_shift_expr(ecx, byte, expr, 7, 3);
    }
    if let RegKind::Dynamic(_, expr) = reg2 {
        byte = or_mask_shift_expr(ecx, byte, expr, 7, 0);
    }
    buffer.push(Stmt::DynScale(scale, byte));
}
