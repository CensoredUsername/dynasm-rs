use syn::{parse, Token};
use syn::spanned::Spanned;
use proc_macro2::Span;
use proc_macro_error::emit_error;

use lazy_static::lazy_static;

use crate::common::Size;
use crate::parse_helpers::{eat_pseudo_keyword, parse_ident_or_rust_keyword, as_ident, ParseOptExt};

use super::{Context, X86Mode};
use super::ast::{Instruction, RawArg, Register, RegId, RegFamily, MemoryRefItem};

use std::collections::HashMap;

/*
 * Code
 */

// parses a full instruction
// syntax for a single op: PREFIX* ident (SIZE? expr ("," SIZE? expr)*)? ";"
pub(super) fn parse_instruction(ctx: &mut Context, input: parse::ParseStream) -> parse::Result<(Instruction, Vec<RawArg>)> {
    let span = input.cursor().span();

    let mut ops = Vec::new();

    // read prefixes + op
    let mut op = parse_ident_or_rust_keyword(input)?;
    while is_prefix(&op) {
        ops.push(op);
        op = parse_ident_or_rust_keyword(input)?;
    }
    ops.push(op);

    // parse (sizehint? expr),*
    let mut args = Vec::new();

    if !(input.is_empty() || input.peek(Token![;])) {
        args.push(parse_arg(ctx, input)?);

        while input.peek(Token![,]) {
            let _: Token![,] = input.parse()?;

            args.push(parse_arg(ctx, input)?);
        }
    }

    // let span = span.join(input.cursor().span()); // FIXME can't join spans ATM

    Ok((
        Instruction {
            idents: ops,
            span
        },
        args
    ))
}

/// checks if the given ident is a valid x86 prefix
fn is_prefix(ident: &syn::Ident) -> bool {
    const PREFIXES: [&str; 12] = [
        "lock",
        "rep", "repe", "repz",
        "repne", "repnz",
        "ss", "cs", "ds", "es", "fs", "gs"
    ];

    PREFIXES.contains(&ident.to_string().as_str())
}

/// if a size hint is present in the parse stream, returning the indicated size
fn eat_size_hint(ctx: &Context, input: parse::ParseStream) -> Option<Size> {
    const X86_SIZES: [(&str, Size); 9] = [
        ("BYTE", Size::BYTE),
        ("WORD", Size::B_2),
        ("DWORD", Size::B_4),
        ("AWORD", Size::B_4),
        ("FWORD", Size::B_6),
        ("QWORD", Size::B_8),
        ("TWORD", Size::B_10),
        ("OWORD", Size::B_16),
        ("YWORD", Size::B_32)
    ];
    const X64_SIZES: [(&str, Size); 9] = [
        ("BYTE", Size::BYTE),
        ("WORD", Size::B_2),
        ("DWORD", Size::B_4),
        ("FWORD", Size::B_6),
        ("AWORD", Size::B_8),
        ("QWORD", Size::B_8),
        ("TWORD", Size::B_10),
        ("OWORD", Size::B_16),
        ("YWORD", Size::B_32)
    ];

    let sizes = match ctx.mode {
        X86Mode::Protected => &X86_SIZES,
        X86Mode::Long      => &X64_SIZES
    };
    for &(kw, size) in sizes {
        if eat_pseudo_keyword(input, kw) {
            return Some(size);
        }
    }
    None
}

/// tries to parse a full arg definition
fn parse_arg(ctx: &mut Context, input: parse::ParseStream) -> parse::Result<RawArg> {
    // sizehint
    let size = eat_size_hint(ctx, input);

    let _start = input.cursor().span(); // FIXME can't join spans yet

    // bare label
    if let Some(jump) = input.parse_opt()? {
        return Ok(RawArg::JumpTarget {
            jump,
            size
        })
    }

    // indirect
    if input.peek(syn::token::Bracket) {
        let span = input.cursor().span();
        let inner;
        let _ = syn::bracketed!(inner in input);
        let inner = &inner;

        // label
        if let Some(jump) = inner.parse_opt()? {
            return Ok(RawArg::IndirectJumpTarget {
                jump,
                size
            })
        }

        // memory reference
        let nosplit = eat_pseudo_keyword(inner, "NOSPLIT");
        let disp_size = eat_size_hint(ctx, inner);
        let expr: syn::Expr = inner.parse()?;

        // split the expression into the different (displacement, register, scaled register) components
        let items = parse_adds(ctx, expr);

        return Ok(RawArg::IndirectRaw {
            span,
            nosplit,
            value_size: size,
            disp_size,
            items
        })

    }

    // it's a normal (register/immediate/typemapped) operand
    let arg: syn::Expr = input.parse()?;

    // typemapped: expr => type [expr] . ident
    if input.peek(Token![=>]) {
        let _: Token![=>] = input.parse()?;

        let base = if let Some((_, base)) = parse_reg(ctx, &arg) {
            base
        } else {
            emit_error!(arg, "Expected register");
            return Ok(RawArg::Invalid);
        };

        let ty: syn::Path = input.parse()?;

        // any attribute, register as index and immediate in index
        let mut nosplit = false;
        let mut disp_size = None;

        let items = if input.peek(syn::token::Bracket) {
            let inner;
            let _brackets = syn::bracketed!(inner in input);
            let inner = &inner;

            nosplit = eat_pseudo_keyword(inner, "NOSPLIT");
            disp_size = eat_size_hint(ctx, inner);
            let index_expr: syn::Expr = inner.parse()?;

            parse_adds(ctx, index_expr)
        } else {
            Vec::new()
        };

        let attr: Option<syn::Ident> = if input.peek(Token![.]) {
            let _: Token![.] = input.parse()?;
            Some(input.parse()?)
        } else {
            None
        };

        return Ok(RawArg::TypeMappedRaw {
            span: arg.span(), // FIXME can't join spans yet
            nosplit,
            value_size: size,
            disp_size,
            base_reg: base,
            scale: ty,
            scaled_items: items,
            attribute: attr,
        });
    }

    // direct register
    if let Some((span, reg)) = parse_reg(ctx, &arg) {
        if size.is_some() {
            emit_error!(span, "size hint with direct register");
        }
        return Ok(RawArg::Direct {
            reg,
            span
        })
    }

    // immediate
    Ok(RawArg::Immediate {
        value: arg,
        size
    })
}

/// checks if an expression is interpretable as a register reference.
fn parse_reg(ctx: &Context, expr: &syn::Expr) -> Option<(Span, Register)> {
    if let Some(path) = as_ident(expr) {
        // static register names

        let name = path.to_string();
        let mut name = name.as_str();
        if let Some(x) = ctx.state.invocation_context.aliases.get(name) {
            name = x;
        }

        let (reg, size) = match ctx.mode {
            X86Mode::Long      => X64_REGISTERS.get(&name).cloned(),
            X86Mode::Protected => X86_REGISTERS.get(&name).cloned()
        }?;

        Some((
            path.span(),
            Register::new_static(size, reg)
        ))

    } else if let syn::Expr::Call(syn::ExprCall {ref func, ref args, ..}) = expr {
        // dynamically chosen registers ( ident(expr) )
        if args.len() != 1 {
            return None;
        }

        let called = if let Some(called) = as_ident(&&*func) {
            called
        } else {
            return None;
        };

        let name = called.to_string();
        let name = name.as_str();
        let (size, family) = match ctx.mode {
            X86Mode::Long      => X64_FAMILIES.get(&name).cloned(),
            X86Mode::Protected => X86_FAMILIES.get(&name).cloned()
        }?;

        Some((
            expr.span(), // FIXME:can't join spans atm
            Register::new_dynamic(size, family, args[0].clone())
        ))
    } else {
        None
    }
}

/// splits an expression into different components of a memory reference.
fn parse_adds(ctx: &Context, expr: syn::Expr) -> Vec<MemoryRefItem> {
    let mut adds = Vec::new();
    collect_adds(expr, &mut adds);

    let mut items = Vec::new();

    // detect what kind of equation we're dealing with
    for node in adds {
        // simple reg
        if let Some((_, reg)) = parse_reg(ctx, &node) {
            items.push(MemoryRefItem::Register(reg));
            continue;
        }
        if let syn::Expr::Binary(syn::ExprBinary { op: syn::BinOp::Mul(_), ref left, ref right, .. } ) = node {
            // reg * const
            if let Some((_, reg)) = parse_reg(ctx, left) {
                if let syn::Expr::Lit(syn::ExprLit {ref lit, ..}) = **right {
                    if let syn::Lit::Int(lit) = lit {
                        if let Ok(value) = lit.base10_parse::<isize>() {
                            items.push(MemoryRefItem::ScaledRegister(reg, value));
                            continue;
                        }
                    }
                }
            } // const * reg
            if let Some((_, reg)) = parse_reg(ctx, right) {
                if let syn::Expr::Lit(syn::ExprLit {ref lit, ..}) = **left {
                    if let syn::Lit::Int(lit) = lit {
                        if let Ok(value) = lit.base10_parse::<isize>() {
                            items.push(MemoryRefItem::ScaledRegister(reg, value));
                            continue;
                        }
                    }
                }
            }
        }
        items.push(MemoryRefItem::Displacement(node));
    }

    items
}

/// Takes an expression and splits all added (and subtracted) components
fn collect_adds(node: syn::Expr, collection: &mut Vec<syn::Expr>) {
    if let syn::Expr::Binary(syn::ExprBinary { op: syn::BinOp::Add(_), left, right, .. } ) = node {
        collect_adds(*left, collection);
        collect_adds(*right, collection);
    } else if let syn::Expr::Binary(syn::ExprBinary { op: syn::BinOp::Sub(sub), left, right, .. } ) = node {
        collect_adds(*left, collection);
        collection.push(syn::Expr::Unary(syn::ExprUnary { op: syn::UnOp::Neg(sub), expr: right, attrs: Vec::new() } ));
    } else {
        collection.push(node);
    }
}

// why
lazy_static!{
    static ref X64_REGISTERS: HashMap<&'static str, (RegId, Size)> = {
        use self::RegId::*;
        use crate::common::Size::*;

        static MAP: &[(&str, (RegId, Size))] = &[
            ("rax", (RAX, B_8)),
            ("r0" , (RAX, B_8)),
            ("rcx", (RCX, B_8)),
            ("r1" , (RCX, B_8)),
            ("rdx", (RDX, B_8)),
            ("r2" , (RDX, B_8)),
            ("rbx", (RBX, B_8)),
            ("r3" , (RBX, B_8)),
            ("rsp", (RSP, B_8)),
            ("r4" , (RSP, B_8)),
            ("rbp", (RBP, B_8)),
            ("r5" , (RBP, B_8)),
            ("rsi", (RSI, B_8)),
            ("r6" , (RSI, B_8)),
            ("rdi", (RDI, B_8)),
            ("r7" , (RDI, B_8)),
            ("r8" , (R8,  B_8)),
            ("r9" , (R9,  B_8)),
            ("r10", (R10, B_8)),
            ("r11", (R11, B_8)),
            ("r12", (R12, B_8)),
            ("r13", (R13, B_8)),
            ("r14", (R14, B_8)),
            ("r15", (R15, B_8)),

            ("eax" , (RAX, B_4)),
            ("r0d" , (RAX, B_4)),
            ("ecx" , (RCX, B_4)),
            ("r1d" , (RCX, B_4)),
            ("edx" , (RDX, B_4)),
            ("r2d" , (RDX, B_4)),
            ("ebx" , (RBX, B_4)),
            ("r3d" , (RBX, B_4)),
            ("esp" , (RSP, B_4)),
            ("r4d" , (RSP, B_4)),
            ("ebp" , (RBP, B_4)),
            ("r5d" , (RBP, B_4)),
            ("esi" , (RSI, B_4)),
            ("r6d" , (RSI, B_4)),
            ("edi" , (RDI, B_4)),
            ("r7d" , (RDI, B_4)),
            ("r8d" , (R8,  B_4)),
            ("r9d" , (R9,  B_4)),
            ("r10d", (R10, B_4)),
            ("r11d", (R11, B_4)),
            ("r12d", (R12, B_4)),
            ("r13d", (R13, B_4)),
            ("r14d", (R14, B_4)),
            ("r15d", (R15, B_4)),

            ("ax"  , (RAX, B_2)),
            ("r0w" , (RAX, B_2)),
            ("cx"  , (RCX, B_2)),
            ("r1w" , (RCX, B_2)),
            ("dx"  , (RDX, B_2)),
            ("r2w" , (RDX, B_2)),
            ("bx"  , (RBX, B_2)),
            ("r3w" , (RBX, B_2)),
            ("sp"  , (RSP, B_2)),
            ("r4w" , (RSP, B_2)),
            ("bp"  , (RBP, B_2)),
            ("r5w" , (RBP, B_2)),
            ("si"  , (RSI, B_2)),
            ("r6w" , (RSI, B_2)),
            ("di"  , (RDI, B_2)),
            ("r7w" , (RDI, B_2)),
            ("r8w" , (R8,  B_2)),
            ("r9w" , (R9,  B_2)),
            ("r10w", (R10, B_2)),
            ("r11w", (R11, B_2)),
            ("r12w", (R12, B_2)),
            ("r13w", (R13, B_2)),
            ("r14w", (R14, B_2)),
            ("r15w", (R15, B_2)),

            ("al"  , (RAX, BYTE)),
            ("r0b" , (RAX, BYTE)),
            ("cl"  , (RCX, BYTE)),
            ("r1b" , (RCX, BYTE)),
            ("dl"  , (RDX, BYTE)),
            ("r2b" , (RDX, BYTE)),
            ("bl"  , (RBX, BYTE)),
            ("r3b" , (RBX, BYTE)),
            ("spl" , (RSP, BYTE)),
            ("r4b" , (RSP, BYTE)),
            ("bpl" , (RBP, BYTE)),
            ("r5b" , (RBP, BYTE)),
            ("sil" , (RSI, BYTE)),
            ("r6b" , (RSI, BYTE)),
            ("dil" , (RDI, BYTE)),
            ("r7b" , (RDI, BYTE)),
            ("r8b" , (R8,  BYTE)),
            ("r9b" , (R9,  BYTE)),
            ("r10b", (R10, BYTE)),
            ("r11b", (R11, BYTE)),
            ("r12b", (R12, BYTE)),
            ("r13b", (R13, BYTE)),
            ("r14b", (R14, BYTE)),
            ("r15b", (R15, BYTE)),

            ("rip", (RIP, B_8)),
            ("eip", (RIP, B_4)),

            ("ah", (AH, BYTE)),
            ("ch", (CH, BYTE)),
            ("dh", (DH, BYTE)),
            ("bh", (BH, BYTE)),

            ("st0", (ST0, B_10)),
            ("st1", (ST1, B_10)),
            ("st2", (ST2, B_10)),
            ("st3", (ST3, B_10)),
            ("st4", (ST4, B_10)),
            ("st5", (ST5, B_10)),
            ("st6", (ST6, B_10)),
            ("st7", (ST7, B_10)),

            ("mm0", (MMX0, B_8)),
            ("mm1", (MMX1, B_8)),
            ("mm2", (MMX2, B_8)),
            ("mm3", (MMX3, B_8)),
            ("mm4", (MMX4, B_8)),
            ("mm5", (MMX5, B_8)),
            ("mm6", (MMX6, B_8)),
            ("mm7", (MMX7, B_8)),

            ("xmm0" , (XMM0 , B_16)),
            ("xmm1" , (XMM1 , B_16)),
            ("xmm2" , (XMM2 , B_16)),
            ("xmm3" , (XMM3 , B_16)),
            ("xmm4" , (XMM4 , B_16)),
            ("xmm5" , (XMM5 , B_16)),
            ("xmm6" , (XMM6 , B_16)),
            ("xmm7" , (XMM7 , B_16)),
            ("xmm8" , (XMM8 , B_16)),
            ("xmm9" , (XMM9 , B_16)),
            ("xmm10", (XMM10, B_16)),
            ("xmm11", (XMM11, B_16)),
            ("xmm12", (XMM12, B_16)),
            ("xmm13", (XMM13, B_16)),
            ("xmm14", (XMM14, B_16)),
            ("xmm15", (XMM15, B_16)),

            ("ymm0" , (XMM0 , B_32)),
            ("ymm1" , (XMM1 , B_32)),
            ("ymm2" , (XMM2 , B_32)),
            ("ymm3" , (XMM3 , B_32)),
            ("ymm4" , (XMM4 , B_32)),
            ("ymm5" , (XMM5 , B_32)),
            ("ymm6" , (XMM6 , B_32)),
            ("ymm7" , (XMM7 , B_32)),
            ("ymm8" , (XMM8 , B_32)),
            ("ymm9" , (XMM9 , B_32)),
            ("ymm10", (XMM10, B_32)),
            ("ymm11", (XMM11, B_32)),
            ("ymm12", (XMM12, B_32)),
            ("ymm13", (XMM13, B_32)),
            ("ymm14", (XMM14, B_32)),
            ("ymm15", (XMM15, B_32)),

            ("es", (ES, B_2)),
            ("cs", (CS, B_2)),
            ("ss", (SS, B_2)),
            ("ds", (DS, B_2)),
            ("fs", (FS, B_2)),
            ("gs", (GS, B_2)),

            ("cr0" , (CR0 , B_8)),
            ("cr1" , (CR1 , B_8)),
            ("cr2" , (CR2 , B_8)),
            ("cr3" , (CR3 , B_8)),
            ("cr4" , (CR4 , B_8)),
            ("cr5" , (CR5 , B_8)),
            ("cr6" , (CR6 , B_8)),
            ("cr7" , (CR7 , B_8)),
            ("cr8" , (CR8 , B_8)),
            ("cr9" , (CR9 , B_8)),
            ("cr10", (CR10, B_8)),
            ("cr11", (CR11, B_8)),
            ("cr12", (CR12, B_8)),
            ("cr13", (CR13, B_8)),
            ("cr14", (CR14, B_8)),
            ("cr15", (CR15, B_8)),

            ("dr0" , (DR0 , B_8)),
            ("dr1" , (DR1 , B_8)),
            ("dr2" , (DR2 , B_8)),
            ("dr3" , (DR3 , B_8)),
            ("dr4" , (DR4 , B_8)),
            ("dr5" , (DR5 , B_8)),
            ("dr6" , (DR6 , B_8)),
            ("dr7" , (DR7 , B_8)),
            ("dr8" , (DR8 , B_8)),
            ("dr9" , (DR9 , B_8)),
            ("dr10", (DR10, B_8)),
            ("dr11", (DR11, B_8)),
            ("dr12", (DR12, B_8)),
            ("dr13", (DR13, B_8)),
            ("dr14", (DR14, B_8)),
            ("dr15", (DR15, B_8)),

            ("bnd0", (BND0, B_16)),
            ("bnd1", (BND1, B_16)),
            ("bnd2", (BND2, B_16)),
            ("bnd3", (BND3, B_16)),
        ];
        MAP.iter().cloned().collect()
    };
    static ref X86_REGISTERS: HashMap<&'static str, (RegId, Size)> = {
        use self::RegId::*;
        use crate::common::Size::*;

        static MAP: &[(&str, (RegId, Size))] = &[
            ("eax", (RAX, B_4)),
            ("ecx", (RCX, B_4)),
            ("edx", (RDX, B_4)),
            ("ebx", (RBX, B_4)),
            ("esp", (RSP, B_4)),
            ("ebp", (RBP, B_4)),
            ("esi", (RSI, B_4)),
            ("edi", (RDI, B_4)),

            ("ax", (RAX, B_2)),
            ("cx", (RCX, B_2)),
            ("dx", (RDX, B_2)),
            ("bx", (RBX, B_2)),
            ("sp", (RSP, B_2)),
            ("bp", (RBP, B_2)),
            ("si", (RSI, B_2)),
            ("di", (RDI, B_2)),

            ("al", (RAX, BYTE)),
            ("cl", (RCX, BYTE)),
            ("dl", (RDX, BYTE)),
            ("bl", (RBX, BYTE)),

            ("eip", (RIP, B_4)),

            ("ah", (AH, BYTE)),
            ("ch", (CH, BYTE)),
            ("dh", (DH, BYTE)),
            ("bh", (BH, BYTE)),

            ("st0", (ST0, B_10)),
            ("st1", (ST1, B_10)),
            ("st2", (ST2, B_10)),
            ("st3", (ST3, B_10)),
            ("st4", (ST4, B_10)),
            ("st5", (ST5, B_10)),
            ("st6", (ST6, B_10)),
            ("st7", (ST7, B_10)),

            ("mm0", (MMX0, B_8)),
            ("mm1", (MMX1, B_8)),
            ("mm2", (MMX2, B_8)),
            ("mm3", (MMX3, B_8)),
            ("mm4", (MMX4, B_8)),
            ("mm5", (MMX5, B_8)),
            ("mm6", (MMX6, B_8)),
            ("mm7", (MMX7, B_8)),

            ("xmm0", (XMM0, B_16)),
            ("xmm1", (XMM1, B_16)),
            ("xmm2", (XMM2, B_16)),
            ("xmm3", (XMM3, B_16)),
            ("xmm4", (XMM4, B_16)),
            ("xmm5", (XMM5, B_16)),
            ("xmm6", (XMM6, B_16)),
            ("xmm7", (XMM7, B_16)),

            ("ymm0", (XMM0, B_32)),
            ("ymm1", (XMM1, B_32)),
            ("ymm2", (XMM2, B_32)),
            ("ymm3", (XMM3, B_32)),
            ("ymm4", (XMM4, B_32)),
            ("ymm5", (XMM5, B_32)),
            ("ymm6", (XMM6, B_32)),
            ("ymm7", (XMM7, B_32)),

            ("es", (ES, B_2)),
            ("cs", (CS, B_2)),
            ("ss", (SS, B_2)),
            ("ds", (DS, B_2)),
            ("fs", (FS, B_2)),
            ("gs", (GS, B_2)),

            ("cr0", (CR0, B_4)),
            ("cr1", (CR1, B_4)),
            ("cr2", (CR2, B_4)),
            ("cr3", (CR3, B_4)),
            ("cr4", (CR4, B_4)),
            ("cr5", (CR5, B_4)),
            ("cr6", (CR6, B_4)),
            ("cr7", (CR7, B_4)),

            ("dr0", (DR0, B_4)),
            ("dr1", (DR1, B_4)),
            ("dr2", (DR2, B_4)),
            ("dr3", (DR3, B_4)),
            ("dr4", (DR4, B_4)),
            ("dr5", (DR5, B_4)),
            ("dr6", (DR6, B_4)),
            ("dr7", (DR7, B_4)),

            ("bnd0", (BND0, B_16)),
            ("bnd1", (BND1, B_16)),
            ("bnd2", (BND2, B_16)),
            ("bnd3", (BND3, B_16)),
        ];
        MAP.iter().cloned().collect()
    };
    static ref X64_FAMILIES:  HashMap<&'static str, (Size, RegFamily)> = {
        static MAP: &[(&str, (Size, RegFamily))] = &[
            ("Rb", (Size::BYTE,  RegFamily::LEGACY)),
            ("Rh", (Size::BYTE,  RegFamily::HIGHBYTE)),
            ("Rw", (Size::B_2,  RegFamily::LEGACY)),
            ("Rd", (Size::B_4, RegFamily::LEGACY)),
            ("Ra", (Size::B_8, RegFamily::LEGACY)),
            ("Rq", (Size::B_8, RegFamily::LEGACY)),
            ("Rf", (Size::B_10, RegFamily::FP)),
            ("Rm", (Size::B_8, RegFamily::MMX)),
            ("Rx", (Size::B_16, RegFamily::XMM)),
            ("Ry", (Size::B_32, RegFamily::XMM)),
            ("Rs", (Size::B_2,  RegFamily::SEGMENT)),
            ("RC", (Size::B_8, RegFamily::CONTROL)),
            ("RD", (Size::B_8, RegFamily::DEBUG)),
            ("RB", (Size::B_16, RegFamily::BOUND)),
        ];
        MAP.iter().cloned().collect()
    };
    static ref X86_FAMILIES:  HashMap<&'static str, (Size, RegFamily)> = {
        static MAP: &[(&str, (Size, RegFamily))] = &[
            ("Rb",(Size::BYTE,  RegFamily::LEGACY)),
            ("Rh",(Size::BYTE,  RegFamily::HIGHBYTE)),
            ("Rw",(Size::B_2,  RegFamily::LEGACY)),
            ("Ra",(Size::B_4, RegFamily::LEGACY)),
            ("Rd",(Size::B_4, RegFamily::LEGACY)),
            ("Rf",(Size::B_10, RegFamily::FP)),
            ("Rm",(Size::B_8, RegFamily::MMX)),
            ("Rx",(Size::B_16, RegFamily::XMM)),
            ("Ry",(Size::B_32, RegFamily::XMM)),
            ("Rs",(Size::B_2,  RegFamily::SEGMENT)),
            ("RC",(Size::B_4, RegFamily::CONTROL)),
            ("RD",(Size::B_4, RegFamily::DEBUG)),
            ("RB",(Size::B_16, RegFamily::BOUND)),
        ];
        MAP.iter().cloned().collect()
    };
}
