use syntax::ext::base::ExtCtxt;
use syntax::ext::build::AstBuilder;
use syntax::ast;
use syntax::ptr::P;
use syntax::codemap::Spanned;

use serialize::{self, Stmt, Size, Ident};
use super::{Context, X86Mode};
use super::parser::{Arg, Instruction, MemoryRef, MemoryRefItem, Register, RegKind, RegFamily, RegId, JumpType};
use super::x64data::get_mnemnonic_data;
use super::x64data::Flags;
use super::x64data::Features;
use super::debug::format_opdata_list;

use std::mem::{swap, replace};
use std::slice;
use std::iter;


/*
 * Instruction encoding data formats
 */

#[derive(Debug)]
pub struct Opdata {
    pub args:  &'static [u8],  // format string of arg format
    pub ops:   &'static [u8],
    pub reg:   u8,
    pub flags: Flags,
    pub features: Features
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


#[derive(Debug, Clone, Copy)]
enum RelocationKind {
    /// A rip-relative relocation. No need to keep track of.
    Relative,
    // An absolute offset to a rip-relative location.
    Absolute,
    // A relative offset to an absolute location,
    Extern,
}

impl RelocationKind {
    fn to_id(&self) -> u8 {
        match *self {
            RelocationKind::Relative => 0,
            RelocationKind::Absolute => 1,
            RelocationKind::Extern   => 2
        }
    }
}

/*
 * Implementation
 */

pub fn compile_instruction(ctx: &mut Context, ecx: &ExtCtxt, instruction: Instruction) -> Result<(), Option<String>> {
    let Instruction(mut ops, mut args, _) = instruction;
    let op = ops.pop().unwrap();
    let prefixes = ops;

    // sanitize memory references and determine address size
    let pref_addr = sanitize_addresses(&ctx, ecx, &mut args)?;

    // figure out immediate sizes where the immediates are constants
    size_immediates(&mut args);

    // this call also inserts more size information in the AST if applicable.
    let data = match_op_format(ctx, ecx, op, &mut args)?;

    // determine legacy prefixes
    let (mut pref_mod, pref_seg) = get_legacy_prefixes(ecx, data, prefixes)?;

    let mut pref_size = false;
    let mut rex_w = false;
    let mut vex_l = false;

    // determine if size prefixes are necessary
    if data.flags.intersects(Flags::AUTO_SIZE | Flags::AUTO_NO32 | Flags::AUTO_REXW | Flags::AUTO_VEXL) {
        // determine operand size
        let op_size = get_operand_size(data, &mut args)?;

        match ctx.mode {
            X86Mode::Protected => if op_size == Size::QWORD {
                return Err(Some(format!("'{}': Does not support 64 bit operands in 32-bit mode", &*op.node.name.as_str())));
            },
            X86Mode::Long => ()
        }

        if data.flags.contains(Flags::AUTO_NO32) {
            match (op_size, ctx.mode) {
                (Size::WORD, _) => pref_size = true,
                (Size::QWORD, X86Mode::Long) => (),
                (Size::DWORD, X86Mode::Protected) => (),
                (Size::DWORD, X86Mode::Long) => return Err(Some(format!("'{}': Does not support 32 bit operands in 64-bit mode", &*op.node.name.as_str()))),
                (_, _) => panic!("bad formatting data"),
            }
        } else if data.flags.contains(Flags::AUTO_REXW) {
            if op_size == Size::QWORD {
                rex_w = true;
            } else if op_size != Size::DWORD {
                return Err(Some(format!("'{}': Does not support 16-bit operands", &*op.node.name.as_str())));
            }
        } else if data.flags.contains(Flags::AUTO_VEXL) {
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
    let pref_size = pref_size || data.flags.contains(Flags::WORD_SIZE);
    let rex_w     = rex_w     || data.flags.contains(Flags::WITH_REXW);
    let vex_l     = vex_l     || data.flags.contains(Flags::WITH_VEXL);
    let pref_addr = pref_addr || data.flags.contains(Flags::PREF_67);

    if        data.flags.contains(Flags::PREF_F0) { pref_mod = Some(0xF0);
    } else if data.flags.contains(Flags::PREF_F2) { pref_mod = Some(0xF2);
    } else if data.flags.contains(Flags::PREF_F3) { pref_mod = Some(0xF3);
    }

    // check if this combination of args can actually be encoded and whether a rex prefix is necessary
    let need_rex = validate_args(data, &args, rex_w)?;

    // split args
    let (mut rm, reg, vvvv, ireg, mut args) = extract_args(data, args);

    // we'll need this to keep track of where relocations need to be made
    // (target, offset, size, kind)
    let mut relocations = Vec::new();

    let mut ops = data.ops;

    // deal with ops that encode the final byte in an immediate
    let immediate_opcode = if data.flags.intersects(Flags::IMM_OP) {
        let (&imm, rest) = ops.split_last().expect("bad formatting data");
        ops = rest;
        Some(imm)
    } else {
        None
    };

    // shorthand
    let buffer = &mut ctx.state.stmts;

    // legacy-only prefixes
    if let Some(pref) = pref_seg {
        buffer.push(Stmt::u8(pref));
    }
    if pref_addr {
        buffer.push(Stmt::u8(0x67));
    }

    // VEX/XOP prefixes embed the operand size prefix / modification prefixes in them.
    if data.flags.intersects(Flags::VEX_OP | Flags::XOP_OP) {
        let prefix = if pref_size        { 0b01
        } else if pref_mod == Some(0xF3) { 0b10
        } else if pref_mod == Some(0xF2) { 0b11
        } else                           { 0
        };
        // map_sel is stored in the first byte of the opcode
        let (&map_sel, tail) = ops.split_first().expect("bad formatting data");
        ops = tail;
        compile_vex_xop(ecx, buffer, data, &reg, &rm, map_sel, rex_w, &vvvv, vex_l, prefix);
    // otherwise, the size/mod prefixes have to be pushed and check if a rex prefix has to be generated.
    } else {
        if let Some(pref) = pref_mod {
            buffer.push(Stmt::u8(pref));
        }
        if pref_size {
            buffer.push(Stmt::u8(0x66));
        }
        if need_rex {
            // Certain SSE/AVX legacy encoded operations are not available in 32-bit mode
            // as they require a REX.W prefix to be encoded, which is impossible. We catch those cases here
            if ctx.mode == X86Mode::Protected {
                return Err(Some(format!("'{}': Does not support 64 bit operand size in 32-bit mode", &*op.node.name.as_str())))
            }
            compile_rex(ecx, buffer, rex_w, &reg, &rm);
        }
    }

    // if rm is embedded in the last opcode byte, push it here
    if data.flags.contains(Flags::SHORT_ARG) {
        let (last, head) = ops.split_last().expect("bad formatting data");
        ops = head;
        buffer.push(Stmt::Extend(Vec::from(ops)));

        let rm_k = if let Some(Arg::Direct(rm)) = rm.take() {
            rm.node.kind
        } else {
            panic!("bad formatting data")
        };

        if let RegKind::Dynamic(_, expr) = rm_k {
            let last = ecx.expr_lit(ecx.call_site(), ast::LitKind::Byte(*last));
            buffer.push(Stmt::ExprUnsigned(serialize::expr_mask_shift_or(ecx, last, expr, 7, 0), Size::BYTE));
        } else {
            buffer.push(Stmt::u8(last + (rm_k.encode() & 7)));
        }
    // just push the opcode
    } else {
        buffer.push(Stmt::Extend(Vec::from(ops)));
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
        match mem.index {
            // VSIB has different mode rules
            Some(_) if mem.index.as_ref().unwrap().0.kind.family() == RegFamily::XMM => {
                let (index, scale, scale_expr) = mem.index.unwrap();
                let index = index.kind;

                let (base, mode) = if let Some(base) = mem.base {
                    (base.kind, match (&mem.disp, mem.disp_size) {
                        (&Some(_), Some(Size::BYTE)) => MOD_DISP8,
                        (&Some(_), _) => MOD_DISP32,
                        (&None, _) => MOD_DISP8
                    })
                } else {
                    (RegKind::Static(RegId::RBP), MOD_NOBASE)
                };

                compile_modrm_sib(ecx, buffer, mode, reg_k, RegKind::Static(RegId::RSP));

                if let Some(expr) = scale_expr {
                    compile_sib_dynscale(ecx, buffer, &ctx.state.target, scale, expr, index, base);
                } else {
                    compile_modrm_sib(ecx, buffer, scale as u8, index, base);
                }

                if let Some(disp) = mem.disp {
                    buffer.push(Stmt::ExprSigned(disp, if mode == MOD_DISP8 {Size::BYTE} else {Size::DWORD}));
                } else if mode == MOD_DISP8 {
                    // no displacement was asked for, but we have to encode one as there's a base
                    buffer.push(Stmt::u8(0));
                } else {
                    // MODE_NOBASE requires a dword displacement, and if we got here no displacement was asked for.
                    buffer.push(Stmt::u32(0));
                }
            },
            // normal indirect addressing
            index => {
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
                } else if let Some(Size::BYTE) = mem.disp_size {
                    MOD_DISP8
                } else {
                    MOD_DISP32
                };

                // if there's an index we need to escape into the SIB byte
                if let Some((index, scale, scale_expr)) = index {
                    // to encode the lack of a base we encode RBP
                    let base = if let Some(base) = mem.base {
                        base.kind
                    } else {
                        RegKind::Static(RegId::RBP)
                    };

                    compile_modrm_sib(ecx, buffer, mode, reg_k, RegKind::Static(RegId::RSP));
                    if let Some(expr) = scale_expr {
                        compile_sib_dynscale(ecx, buffer, &ctx.state.target, scale, expr, index.kind, base);
                    } else {
                        compile_modrm_sib(ecx, buffer, scale as u8, index.kind, base);
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
                    buffer.push(Stmt::ExprSigned(disp, if mode == MOD_DISP8 {Size::BYTE} else {Size::DWORD}));
                } else if no_base || rip_relative {
                    buffer.push(Stmt::u32(0));
                } else if rbp_relative {
                    buffer.push(Stmt::u8(0));
                }
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

        buffer.push(Stmt::u32(0));
        match ctx.mode {
            X86Mode::Long      => relocations.push((target, 0, Size::DWORD, RelocationKind::Relative)),
            X86Mode::Protected => relocations.push((target, 0, Size::DWORD, RelocationKind::Absolute))
        }
    }

    // opcode encoded after the displacement
    if let Some(code) = immediate_opcode {
        buffer.push(Stmt::u8(code));

        // bump relocations
        relocations.iter_mut().for_each(|r| r.1 += 1);
    }

    // register in immediate argument
    if let Some(Arg::Direct(ireg)) = ireg {
        let ireg = ireg.node.kind;
        let byte = ireg.encode() << 4;

        let mut byte = ecx.expr_lit(ecx.call_site(), ast::LitKind::Byte(byte));
        if let RegKind::Dynamic(_, expr) = ireg {
            byte = serialize::expr_mask_shift_or(ecx, byte, expr, 0xF, 4);
        }
        // if immediates are present, the register argument will be merged into the
        // first immediate byte.
        if !args.is_empty() {
            if let Arg::Immediate(expr, Some(Size::BYTE)) = args.remove(0) {
                byte = serialize::expr_mask_shift_or(ecx, byte, expr, 0xF, 0);
            } else {
                panic!("bad formatting data")
            }
        }
        buffer.push(Stmt::ExprUnsigned(byte, Size::BYTE));

        // bump relocations
        relocations.iter_mut().for_each(|r| r.1 += 1);
    }

    // immediates
    for arg in args {
        match arg {
            Arg::Immediate(expr, Some(size)) => {
                buffer.push(Stmt::ExprSigned(expr, size));

                // bump relocations
                relocations.iter_mut().for_each(|r| r.1 += size.in_bytes());
            },
            Arg::JumpTarget(target, Some(size)) => {
                // placeholder
                buffer.push(Stmt::Const(0, size));

                // bump relocations
                relocations.iter_mut().for_each(|r| r.1 += size.in_bytes());

                // add the new relocation
                relocations.push((target, 0, size, RelocationKind::Relative));
            },
            Arg::Immediate(_, None) => panic!("Immediate with undetermined size, did get_operand_size not run?"),
            Arg::JumpTarget(_, None) => panic!("Jumptarget with undetermined size, did get_operand_size not run?"),
            _ => panic!("bad immediate data")
        };
    }

    // push relocations
    for (target, offset, size, kind) in relocations.drain(..) {
        buffer.push(match target {
            JumpType::Global(ident)   => Stmt::GlobalJumpTarget(  ident, serialize::expr_tuple_of_u8s(ecx, ident.span, &[offset, size.in_bytes(), kind.to_id()])),
            JumpType::Forward(ident)  => Stmt::ForwardJumpTarget( ident, serialize::expr_tuple_of_u8s(ecx, ident.span, &[offset, size.in_bytes(), kind.to_id()])),
            JumpType::Backward(ident) => Stmt::BackwardJumpTarget(ident, serialize::expr_tuple_of_u8s(ecx, ident.span, &[offset, size.in_bytes(), kind.to_id()])),
            JumpType::Dynamic(expr)   => {
                let span = expr.span;
                Stmt::DynamicJumpTarget(expr, serialize::expr_tuple_of_u8s(ecx, span, &[offset, size.in_bytes(), kind.to_id()]))
            },
            JumpType::Bare(expr)      => {
                let span = expr.span;
                Stmt::BareJumpTarget(expr, serialize::expr_tuple_of_u8s(ecx, span, &[offset, size.in_bytes(), kind.to_id()]))
            }
        })
    }

    Ok(())
}

fn sanitize_addresses(ctx: &Context, ecx: &ExtCtxt, args: &mut [Arg]) -> Result<bool, Option<String>> {
    // determine if an address size prefix is necessary, and sanitize the register choice for memoryrefs
    let mut addr_size = None;

    process_raw_memoryrefs(ecx, args)?;

    for arg in args.iter_mut() {
        if let Arg::Indirect(ref mut mem) = *arg {
            sanitize_memoryref(ecx, mem)?;

            if let Some(ref reg) = mem.base {
                if reg.kind.family() == RegFamily::LEGACY || reg.kind.family() == RegFamily::RIP {
                    if addr_size.is_some() && addr_size != Some(reg.size()) {
                        ecx.span_err(mem.span, "Conflicting address sizes");
                        return Err(None);
                    }
                    addr_size = Some(reg.size());
                }
            }
            if let Some((ref reg, _, _)) = mem.index {
                if reg.kind.family() == RegFamily::LEGACY || reg.kind.family() == RegFamily::RIP {
                    if addr_size.is_some() && addr_size != Some(reg.size()) {
                        ecx.span_err(mem.span, "Conflicting address sizes");
                        return Err(None);
                    }
                    addr_size = Some(reg.size());
                }
            }

            if let Some(size) = mem.disp_size {
                if mem.disp.is_none() {
                    ecx.span_err(mem.span, "Displacement size without displacement");
                }

                if size != Size::BYTE && size != Size::DWORD {
                    ecx.span_err(mem.span, "Invalid displacement size, only BYTE or DWORD are possible");
                }
            }

            if mem.disp_size.is_none() {
                if let Some(ref disp) = mem.disp {
                    match derive_size(&*disp) {
                        Some(Size::BYTE) => mem.disp_size = Some(Size::BYTE),
                        Some(_) => mem.disp_size = Some(Size::DWORD), // QWORD size should generate a compiler warning naturally
                        None => ()
                    }
                }
            }
        }
    }

    Ok(match (ctx.mode, addr_size) {
        (_, None) => false,
        (X86Mode::Long, Some(Size::QWORD)) => false,
        (X86Mode::Long, Some(Size::DWORD)) => true,
        (X86Mode::Protected, Some(Size::DWORD)) => false,
        (X86Mode::Protected, Some(Size::WORD)) => true,
        _ => return Err(Some("Impossible address size".into()))
    })
}

fn process_raw_memoryrefs(ecx: &ExtCtxt, args: &mut [Arg]) -> Result<(), Option<String>> {
    // process raw memoryref forms
    for arg in args {
        // temporarily swap the arg out so we can get the owned data
        let temparg = replace(arg, Arg::Invalid);
        *arg = match temparg {
            Arg::IndirectRaw {span, value_size, nosplit, disp_size, items} => {
                // split the ast on the memoryrefitem types
                let mut scaled = Vec::new();
                let mut regs = Vec::new();
                let mut disps = Vec::new();
                for item in items {
                    match item {
                        MemoryRefItem::Register(reg) => regs.push(reg),
                        MemoryRefItem::ScaledRegister(reg, value) => scaled.push((reg, value)),
                        MemoryRefItem::Displacement(expr) => disps.push(expr)
                    }
                }

                // figure out the base register if possible
                let mut base_reg_index = None;
                for (i, reg) in regs.iter().enumerate() {
                    if !(regs.iter().enumerate().any(|(j, other)| i != j && reg == other) ||
                         scaled.iter().any(|&(ref other, _)| reg == other)) {
                        base_reg_index = Some(i);
                        break;
                    }
                }
                let mut base = base_reg_index.map(|i| regs.remove(i));

                // join all registers
                scaled.extend(regs.into_iter().map(|r| (r, 1)));
                let mut joined_regs = Vec::new();
                for (reg, s) in scaled {
                    // does this register already have a spot?
                    if let Some(i) = joined_regs.iter().position(|&(ref other, _)| &reg == other) {
                        joined_regs[i].1 += s;
                    } else {
                        joined_regs.push((reg, s));
                    }
                }

                // identify an index candidate (and a base if one wasn't found yet)
                let index = if base.is_none() {
                    // we have to look for a base candidate first as not all index candidates
                    // are viable base candidates
                    base_reg_index = joined_regs.iter().position(|&(_, s)| s == 1);
                    base = base_reg_index.map(|i| joined_regs.remove(i).0);
                    // get the index
                    let index = joined_regs.pop();
                    // if nosplit was used, a scaled base was found but not an index, swap them
                    // as there was only one register and this register was scaled.
                    if nosplit && index.is_none() && base.is_some() {
                        base.take().map(|reg| (reg, 1))
                    } else {
                        index
                    }
                } else {
                    joined_regs.pop()
                };


                if !joined_regs.is_empty() {
                    ecx.span_err(span, "Impossible memory argument");
                    return Err(None);
                }

                // merge disps
                let disp = serialize::expr_add_many(ecx, span, disps.into_iter());

                // finalize the memoryref
                Arg::Indirect(MemoryRef {
                    span: span,
                    size: value_size,
                    nosplit: nosplit,
                    disp_size: disp_size,
                    base: base,
                    index: index.map(|(r, s)| (r, s, None)),
                    disp: disp,
                })
            },
            Arg::TypeMappedRaw {span, base_reg, scale, value_size, nosplit, disp_size, scaled_items, attribute} => {
                let base = base_reg;

                // collect registers / displacements
                let mut scaled = Vec::new();
                let mut disps = Vec::new();
                for item in scaled_items {
                    match item {
                        MemoryRefItem::Register(reg) => scaled.push((reg, 1)),
                        MemoryRefItem::ScaledRegister(reg, scale) => scaled.push((reg, scale)),
                        MemoryRefItem::Displacement(expr) => disps.push(expr)
                    }
                }

                // join all registers
                let mut joined_regs = Vec::new();
                for (reg, s) in scaled {
                    // does this register already have a spot?
                    if let Some(i) = joined_regs.iter().position(|&(ref other, _)| &reg == other) {
                        joined_regs[i].1 += s;
                    } else {
                        joined_regs.push((reg, s));
                    }
                }

                // identify an index register
                let index = joined_regs.pop();

                if !joined_regs.is_empty() {
                    ecx.span_err(span, "Impossible memory argument");
                    return Err(None);
                }

                // index = scale * index_scale * index
                // disp = scale * disps + attribute

                // Displacement size calculation intermezzo:
                // in typemaps, the following equations are the relations for the scale and displacement.
                // Now as we can't figure these out at compile time (this'd be nice), by default we'll
                // always generate a 32-bit displacement if disp_size isn't set. This means we know
                // the size of disp at this point already.
                // as for the index calculation: that doesn't change size.
                let true_disp_size = disp_size.unwrap_or(Size::DWORD);

                // merge disps [a, b, c] into (a + b + c)
                let scaled_disp = serialize::expr_add_many(ecx, span, disps.into_iter());

                // scale disps (a + b + c) * size_of<scale> as disp_size
                let scaled_disp = scaled_disp.map(|disp| serialize::expr_size_of_scale(ecx, scale.clone(), disp, true_disp_size));

                // attribute displacement offset_of(scale, attr) as disp_size
                let attr_disp = attribute.map(|attr| serialize::expr_offset_of(ecx, scale.clone(), attr, true_disp_size));

                // add displacement sources together
                let disp = if let Some(scaled_disp) = scaled_disp {
                    if let Some(attr_disp) = attr_disp {
                        Some(ecx.expr_binary(span, ast::BinOpKind::Add, attr_disp, scaled_disp))
                    } else {
                        Some(scaled_disp)
                    }
                } else {
                    attr_disp
                };

                // finalize the memoryref
                Arg::Indirect(MemoryRef {
                    span: span,
                    size: value_size,
                    nosplit: nosplit,
                    disp_size: disp_size,
                    base: Some(base),
                    index: index.map(|(r, s)| (r, s, Some(serialize::expr_size_of(ecx, scale)))),
                    disp: disp,
                })
            },
            a => a
        }
    }
    Ok(())
}

fn size_immediates(args: &mut [Arg]) {
    for arg in args {
        if let Arg::Immediate(ref expr, ref mut size @ None) = *arg {
            *size = derive_size(&*expr);
        }
    }
}

fn derive_size(expr: &ast::Expr) -> Option<Size> {
    use syntax::ast::ExprKind;

    match expr.node {
        ExprKind::Lit(ref lit) => match lit.node {
            ast::LitKind::Byte(_) => Some(Size::BYTE),
            ast::LitKind::Int(i, _) => if i < 0x80 {
                Some(Size::BYTE)
            } else if i < 0x8000 {
                Some(Size::WORD)
            } else if i < 0x8000_0000 {
                Some(Size::DWORD)
            } else {
                Some(Size::QWORD)
            },
            _ => None
        },
        ExprKind::Unary(ast::UnOp::Neg, ref expr) => match expr.node {
            ExprKind::Lit(ref lit) => match lit.node {
                ast::LitKind::Byte(_) => Some(Size::BYTE),
                ast::LitKind::Int(i, _) => if i >= 0x80 {
                    Some(Size::BYTE)
                } else if i >= 0x8000 {
                    Some(Size::WORD)
                } else if i >= 0x8000_0000 {
                    Some(Size::DWORD)
                } else {
                    Some(Size::QWORD)
                },
                _ => None
            },
            _ => None
        },
        _ => None
    }
}

fn sanitize_memoryref(ecx: &ExtCtxt, mem: &mut MemoryRef) -> Result<(), Option<String>> {
    // sort out impossible scales
    if let Some((ref index, ref mut scale, None)) = mem.index {
        *scale = match (*scale, mem.base.is_none()) {
            (1, _   ) => 0,
            (2, true) if !mem.nosplit && index.kind.family() == RegFamily::LEGACY => {
                 // size optimization. splits up [index * 2] into [index + index]
                mem.base = Some(index.clone());
                0
            },
            (2, _   ) => 1,
            (4, _   ) => 2,
            (8, _   ) => 3,
            (3, true) if !mem.nosplit && index.kind.family() == RegFamily::LEGACY => {
                mem.base = Some(index.clone());
                1
            },
            (5, true) if !mem.nosplit && index.kind.family() == RegFamily::LEGACY => {
                mem.base = Some(index.clone());
                2
            },
            (9, true) if !mem.nosplit && index.kind.family() == RegFamily::LEGACY => {
                mem.base = Some(index.clone());
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
            ref mut index @ None if !mem.nosplit => {
                // move base to index
                *index = base.take().map(|reg| (reg, 0, None));
                return Ok(());
            },
            Some((ref mut index, 0, None)) if !mem.nosplit && index.kind.family() == RegFamily::LEGACY => {
                // swap base and index.
                swap(base.as_mut().unwrap(), index);
                return Ok(());
            },
            _ => {
                ecx.span_err(mem.span, "vsib addressing requires a general purpose register as base");
                return Err(None);
            }
        },
        ref base => match mem.index {
            Some((ref index, _, _)) if index.kind.family() == RegFamily::XMM => {
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
    if mem.base.as_ref().map_or(false, |base| base.kind.family() != RegFamily::LEGACY && base.kind != RegId::RIP) {
        ecx.span_err(mem.span, "bad register type as base");
        return Err(None);
    } else if mem.index.as_ref().map_or(false, |&(ref index, _, _)| index.kind.family() != RegFamily::LEGACY) {
        ecx.span_err(mem.span, "bad register type as index");
        return Err(None);
    }

    // RIP as base with index
    if mem.base == RegId::RIP && mem.index.is_some() {
        ecx.span_err(mem.span, "'rip' cannot be used as base when an index is present");
        return Err(None);
    }

    // RSP as index field can not be represented. Check if we can swap it with base
    if let Some((index, scale, scale_expr)) = mem.index.take() {
        if index == RegId::RSP {
            if !mem.nosplit && scale == 0 && scale_expr.is_none() && mem.base != RegId::RSP {
                // swap index and base
                mem.index = mem.base.take().map(|b| (b, 0, None));
                mem.base = Some(index);
            } else {
                // as we always fill the base field first this is impossible to satisfy
                ecx.span_err(mem.span, "'rsp' cannot be used as index field");
                return Err(None);
            }
        } else {
            mem.index = Some((index, scale, scale_expr));
        }
    }

    // RSP or R12 as base without index (add an index so we escape into SIB)
    if (mem.base == RegId::RSP || mem.base == RegId::R12) && mem.index.is_none() {
        mem.index = Some((Register::new_static(Size::QWORD, RegId::RSP), 0, None));
    }

    // RBP as base field just requires a mandatory MOD_DISP8. we don't sort that out here.
    // same for no base, as this requires a MOD_DISP32
    Ok(())
}

fn match_op_format(ctx: &Context, ecx: &ExtCtxt, ident: Ident, args: &mut [Arg]) -> Result<&'static Opdata, Option<String>> {
    let name = &*ident.node.name.as_str();

    let data = if let Some(data) = get_mnemnonic_data(name) {
        data
    } else {
        ecx.span_err(ident.span, &format!("'{}' is not a valid instruction", name));
        return Err(None);
    };

    for format in data {
        if let Ok(_) = match_format_string(ctx, format, args) {
            return Ok(format)
        }
    }

    Err(Some(
        format!("'{}': argument type/size mismatch, expected one of the following forms:\n{}", name, format_opdata_list(name, data))
    ))
}

fn match_format_string(ctx: &Context, fmt: &'static Opdata, args: &mut [Arg]) -> Result<(), &'static str> {
    let fmtstr = &fmt.args;

    if ctx.mode != X86Mode::Protected && fmt.flags.intersects(Flags::X86_ONLY) {
        return Err("Not available in 32-bit mode");
    }

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
    // b : bound reg

    // v : r and m
    // u : x and m
    // w : y and m

    // A ... P: match rax - r15
    // Q ... V: match es, cs, ss, ds, fs, gs
    // W: matches CR8
    // X: matches st0

    // b, w, d, q match a byte, word, doubleword and quadword.
    // * matches all possible sizes for this operand (w/d for i, w/d/q for r/v, o/h for y/w and everything for m)
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
                (b'b', &Arg::Direct(Spanned {node: ref reg, ..} )) if
                    reg.kind.family() == RegFamily::BOUND => Some(reg.size()),

                // memory offsets
                (b'm',          &Arg::Indirect(MemoryRef {size, ref index, ..} )) |
                (b'u' ... b'w', &Arg::Indirect(MemoryRef {size, ref index, ..} )) if
                    index.is_none() || index.as_ref().unwrap().0.kind.family() != RegFamily::XMM => size,

                (b'm',          &Arg::IndirectJumpTarget(_, size)) |
                (b'u' ... b'w', &Arg::IndirectJumpTarget(_, size)) => size,

                // vsib addressing. as they have two sizes that must be checked they check one of the sizes here
                (b'k', &Arg::Indirect(MemoryRef {size, index: Some((ref index, _, _)), ..} )) if
                    (size.is_none() || size == Some(Size::DWORD)) &&
                    index.kind.family() == RegFamily::XMM => Some(index.size()),
                (b'l', &Arg::Indirect(MemoryRef {size, index: Some((ref index, _, _)), ..} )) if
                    (size.is_none() ||  size == Some(Size::QWORD)) &&
                    index.kind.family() == RegFamily::XMM => Some(index.size()),
                _ => return Err("argument type mismatch")
            };

            // if size is none it always matches (and will later be coerced to a more specific type if the match is successful)
            if let Some(size) = size {
                if !match (fsize, code) {
                    // immediates can always fit in larger slots
                    (b'w', b'i') => size <= Size::WORD,
                    (b'd', b'i') => size <= Size::DWORD,
                    (b'q', b'i') => size <= Size::QWORD,
                    (b'*', b'i') => size <= Size::DWORD,
                    // normal size matches
                    (b'b', _)    => size == Size::BYTE,
                    (b'w', _)    => size == Size::WORD,
                    (b'd', _)    => size == Size::DWORD,
                    (b'q', _)    => size == Size::QWORD,
                    (b'p', _)    => size == Size::PWORD,
                    (b'o', _)    => size == Size::OWORD,
                    (b'h', _)    => size == Size::HWORD,
                    // what is allowed for wildcards
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
            } else if fsize != b'*' && fmt.flags.contains(Flags::EXACT_SIZE) {
                // Basically, this format is a more specific version of an instruction
                // that also has more general versions. This should only be picked
                // if the size constraints are met, not if the size is unspecified
                return Err("alternate variant exists");
            }
        }
    }

    // we've found a match, update all specific constraints
    {
        let mut args = args.iter_mut();
        for (code, fsize) in FormatStringIterator::new(fmtstr) {
            let arg: &mut Arg = args.next().unwrap();

            match *arg {
                Arg::Immediate(_, ref mut size) |
                Arg::JumpTarget(_, ref mut size) |
                Arg::Indirect(MemoryRef {ref mut size, ..} ) => match (fsize, code) {
                    (b'b', _) => *size = Some(Size::BYTE),
                    (b'w', _) => *size = Some(Size::WORD),
                    (_, b'k') |
                    (b'd', _) => *size = Some(Size::DWORD),
                    (_, b'l') |
                    (b'q', _) => *size = Some(Size::QWORD),
                    (b'p', _) => *size = Some(Size::PWORD),
                    (b'o', _) => *size = Some(Size::OWORD),
                    (b'h', _) => *size = Some(Size::HWORD),
                    (b'*', _) |
                    (b'!', _) => (),
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
            "rep"   => if fmt.flags.contains(Flags::REP) {
                (&mut group1, 0xF3)
            } else {
                ecx.span_err(prefix.span, &format!("Cannot use prefix {} on this instruction", name));
                return Err(None);
            },
            "repe"  |
            "repz"  => if fmt.flags.contains(Flags::REPE) {
                (&mut group1, 0xF3)
            } else {
                ecx.span_err(prefix.span, &format!("Cannot use prefix {} on this instruction", name));
                return Err(None);
            },
            "repnz" |
            "repne" => if fmt.flags.contains(Flags::REP) {
                (&mut group1, 0xF2)
            } else {
                ecx.span_err(prefix.span, &format!("Cannot use prefix {} on this instruction", name));
                return Err(None);
            },
            "lock"  => if fmt.flags.contains(Flags::LOCK) {
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

fn get_operand_size(fmt: &'static Opdata, args: &mut [Arg]) -> Result<Size, Option<String>> {
    // determine operand size to automatically determine appropriate prefixes
    // ensures that all operands have the same size, and that the immediate size is smaller or equal.

    let mut has_args = false;
    let mut op_size = None;
    let mut im_size = None;

    // only scan args which have wildcarded size
    for (arg, (_, size)) in args.iter_mut().zip(FormatStringIterator::new(fmt.args)) {
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
                if let Some((ref reg, _, _)) = *index {
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
            Arg::Immediate(_, ref mut size)  |
            Arg::JumpTarget(_, ref mut size) => {
                if im_size.is_some() {
                    return Err(Some("Multiple immediates with indetermined size".to_string()))
                }
                im_size = Some(size);
            },
            Arg::TypeMappedRaw {..} |
            Arg::IndirectRaw {..} |
            Arg::Invalid => unreachable!()
        }
    }

    if !has_args {
        panic!("get_operand_size was invoked without wildcard size arguments {:?}", fmt);
    }
    if let Some(op_size) = op_size {
        // was an immediate provided
        if let Some(im_size) = im_size {
            // did said immediate have a set size
            if let Some(size) = *im_size {
                if size > op_size || size == Size::QWORD {
                    return Err(Some("Immediate size mismatch".to_string()));
                }
            }
            // upgrade the immediate size to the necessary size
            if op_size == Size::QWORD {
                *im_size = Some(Size::DWORD);
            } else {
                *im_size = Some(op_size);
            }
        }
        Ok(op_size)
    } else {
        Err(Some("Unknown operand size".to_string()))
    }
}

fn validate_args(fmt: &'static Opdata, args: &[Arg], rex_w: bool) -> Result<bool, Option<String>> {
    // performs checks for not encodable arg combinations
    // output arg indicates if a rex prefix can be encoded
    let mut requires_rex    = rex_w;
    let mut requires_no_rex = false;

    for (arg, (c, _)) in args.iter().zip(FormatStringIterator::new(fmt.args)) {
        // only scan args that are actually encoded
        if let b'a' ..= b'z' = c {
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
                    if let Some((ref reg, _, _)) = *index {
                        requires_rex = requires_rex || reg.kind.is_extended();
                    }
                },
                Arg::Immediate(_, _)          |
                Arg::JumpTarget(_, _)         |
                Arg::IndirectJumpTarget(_, _) => (),
                Arg::TypeMappedRaw {..} |
                Arg::IndirectRaw {..} |
                Arg::Invalid => unreachable!()
            }
        }
    }

    if requires_rex && requires_no_rex {
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
            b'f' | b'x' | b'r' | b'y' | b'b' => regs.push(arg),
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
        if fmt.flags.contains(Flags::ENC_MR) || memarg == Some(0) {
            m = regs.next();
            r = regs.next();
        } else if fmt.flags.contains(Flags::ENC_VM) {
            v = regs.next();
            m = regs.next();
        } else {
            r = regs.next();
            m = regs.next();
        }
    } else if len == 3 {
        if fmt.flags.contains(Flags::ENC_MR) || memarg == Some(1) {
            r = regs.next();
            m = regs.next();
            v = regs.next();
        } else if fmt.flags.contains(Flags::ENC_VM) || memarg == Some(0) {
            m = regs.next();
            v = regs.next();
            r = regs.next();
        } else {
            r = regs.next();
            v = regs.next();
            m = regs.next();
        }
    } else if len == 4 {
        if fmt.flags.contains(Flags::ENC_MR) || memarg == Some(2) {
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

fn compile_rex(ecx: &ExtCtxt, buffer: &mut Vec<Stmt>, rex_w: bool, reg: &Option<Arg>, rm: &Option<Arg>) {
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
        if let Some((ref index, _, _)) = *index {
            index_k = index.kind.clone();
        }
    }

    let rex = 0x40 | (rex_w          as u8) << 3 |
                     (reg_k.encode()   & 8) >> 1 |
                     (index_k.encode() & 8) >> 2 |
                     (base_k.encode()  & 8) >> 3 ;
    if !reg_k.is_dynamic() && !index_k.is_dynamic() && !base_k.is_dynamic() {
        buffer.push(Stmt::u8(rex));
        return;
    }

    let mut rex = ecx.expr_lit(ecx.call_site(), ast::LitKind::Byte(rex));

    if let RegKind::Dynamic(_, expr) = reg_k {
        rex = serialize::expr_mask_shift_or(ecx, rex, expr, 8, -1);
    }
    if let RegKind::Dynamic(_, expr) = index_k {
        rex = serialize::expr_mask_shift_or(ecx, rex, expr, 8, -2);
    }
    if let RegKind::Dynamic(_, expr) = base_k {
        rex = serialize::expr_mask_shift_or(ecx, rex, expr, 8, -3);
    }
    buffer.push(Stmt::ExprUnsigned(rex, Size::BYTE));
}

fn compile_vex_xop(ecx: &ExtCtxt, buffer: &mut Vec<Stmt>, data: &'static Opdata, reg: &Option<Arg>,
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
        if let Some((ref index, _, _)) = *index {
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

    if data.flags.contains(Flags::VEX_OP) && (byte1 & 0x7F) == 0x61 && (byte2 & 0x80) == 0 && !index_k.is_dynamic() && !base_k.is_dynamic() {
        // 2-byte vex
        buffer.push(Stmt::u8(0xC5));

        let byte1 = (byte1 & 0x80) | (byte2 & 0x7F);
        if !reg_k.is_dynamic() {
            buffer.push(Stmt::u8(byte1));
            return;
        }

        let mut byte1 = ecx.expr_lit(ecx.call_site(), ast::LitKind::Byte(byte1));
        if let RegKind::Dynamic(_, expr) = reg_k {
            let expr = ecx.expr_unary(expr.span, ast::UnOp::Not, expr);
            byte1 = serialize::expr_mask_shift_or(ecx, byte1, expr, 8, 4)
        }
        buffer.push(Stmt::ExprUnsigned(byte1, Size::BYTE));
        return;
    }

    buffer.push(Stmt::u8(if data.flags.contains(Flags::VEX_OP) {0xC4} else {0x8F}));

    if reg_k.is_dynamic() || index_k.is_dynamic() || base_k.is_dynamic() {
        let mut byte1 = ecx.expr_lit(ecx.call_site(), ast::LitKind::Byte(byte1));

        if let RegKind::Dynamic(_, expr) = reg_k {
            let expr = ecx.expr_unary(expr.span, ast::UnOp::Not, expr);
            byte1 = serialize::expr_mask_shift_or(ecx, byte1, expr, 8, 4);
        }
        if let RegKind::Dynamic(_, expr) = index_k {
            let expr = ecx.expr_unary(expr.span, ast::UnOp::Not, expr);
            byte1 = serialize::expr_mask_shift_or(ecx, byte1, expr, 8, 3);
        }
        if let RegKind::Dynamic(_, expr) = base_k {
            let expr = ecx.expr_unary(expr.span, ast::UnOp::Not, expr);
            byte1 = serialize::expr_mask_shift_or(ecx, byte1, expr, 8, 2);
        }
        buffer.push(Stmt::ExprUnsigned(byte1, Size::BYTE));
    } else {
        buffer.push(Stmt::u8(byte1));
    }

    if vvvv_k.is_dynamic() {
        let mut byte2 = ecx.expr_lit(ecx.call_site(), ast::LitKind::Byte(byte2));

        if let RegKind::Dynamic(_, expr) = vvvv_k {
            let expr = ecx.expr_unary(expr.span, ast::UnOp::Not, expr);
            byte2 = serialize::expr_mask_shift_or(ecx, byte2, expr, 0xF, 3)
        }
        buffer.push(Stmt::ExprUnsigned(byte2, Size::BYTE));
    } else {
        buffer.push(Stmt::u8(byte2));
    }
}

fn compile_modrm_sib(ecx: &ExtCtxt, buffer: &mut Vec<Stmt>, mode: u8, reg1: RegKind, reg2: RegKind) {
    let byte = mode                << 6 |
              (reg1.encode()  & 7) << 3 |
              (reg2.encode()  & 7)     ;

    if !reg1.is_dynamic() && !reg2.is_dynamic() {
        buffer.push(Stmt::u8(byte));
        return;
    }

    let mut byte = ecx.expr_lit(ecx.call_site(), ast::LitKind::Byte(byte));
    if let RegKind::Dynamic(_, expr) = reg1 {
        byte = serialize::expr_mask_shift_or(ecx, byte, expr, 7, 3);
    }
    if let RegKind::Dynamic(_, expr) = reg2 {
        byte = serialize::expr_mask_shift_or(ecx, byte, expr, 7, 0);
    }
    buffer.push(Stmt::ExprUnsigned(byte, Size::BYTE));
}

fn compile_sib_dynscale(ecx: &ExtCtxt, buffer: &mut Vec<Stmt>, target: &P<ast::Expr>, scale: isize, scale_expr: P<ast::Expr>, reg1: RegKind, reg2: RegKind) {
    let byte = (reg1.encode()  & 7) << 3 |
               (reg2.encode()  & 7)      ;

    let mut byte = ecx.expr_lit(ecx.call_site(), ast::LitKind::Byte(byte));

    if let RegKind::Dynamic(_, expr) = reg1 {
        byte = serialize::expr_mask_shift_or(ecx, byte, expr, 7, 3);
    }
    if let RegKind::Dynamic(_, expr) = reg2 {
        byte = serialize::expr_mask_shift_or(ecx, byte, expr, 7, 0);
    }
    let span = scale_expr.span;
    let (stmt, expr) = serialize::expr_dynscale(
        ecx, 
        target,
        ecx.expr_binary(
            span,
            ast::BinOpKind::Mul,
            scale_expr,
            ecx.expr_lit(span, ast::LitKind::Int(scale as u128, ast::LitIntType::Unsuffixed)),
        ),
        byte
    );
    buffer.push(Stmt::Stmt(stmt));
    buffer.push(Stmt::ExprUnsigned(expr, Size::BYTE));
}
