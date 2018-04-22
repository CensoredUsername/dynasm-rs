use syntax::ext::build::AstBuilder;
use syntax::ext::base::ExtCtxt;
use syntax::parse::parser::{Parser, PathStyle};
use syntax::parse::token;
use syntax::parse::PResult;
use syntax::ast;
use syntax::ptr::P;
use syntax::codemap::{Spanned};


use super::{Context, X86Mode};
use serialize::{Size, Ident};

use super::ast::{Instruction, RawArg, JumpType, Register, RegId, RegFamily, MemoryRefItem};

use std::collections::HashMap;

/*
 * Code
 */

// tokentree is a list of tokens and delimited lists of tokens.
// this means we don't have to figure out nesting via []'s by ourselves.
// syntax for a single op: PREFIX* ident (SIZE? expr ("," SIZE? expr)*)? ";"

pub fn parse_instruction<'a>(ctx: &mut Context, ecx: &ExtCtxt, parser: &mut Parser<'a>) -> PResult<'a, (Instruction, Vec<RawArg>)> {
    let startspan = parser.span;

    let mut ops = Vec::new();
    let mut span = parser.span;
    let mut op = Spanned {node: parse_ident_or_rust_keyword(parser)?, span: span};

    // read prefixes
    while is_prefix(op) {
        ops.push(op);
        span = parser.span;
        op = Spanned {node: parse_ident_or_rust_keyword(parser)?, span: span};
    }

    // parse (sizehint? expr),*
    let mut args = Vec::new();

    if !parser.check(&token::Semi) && !parser.check(&token::Eof) {
        args.push(parse_arg(ctx, ecx, parser)?);

        while parser.eat(&token::Comma) {
            args.push(parse_arg(ctx, ecx, parser)?);
        }
    }

    let span = startspan.with_hi(parser.prev_span.hi());

    ops.push(op);
    Ok((
        Instruction {
            idents: ops,
            span: span
        },
        args
    ))
}

const PREFIXES: [&'static str; 12] = [
    "lock",
    "rep", "repe", "repz",
    "repne", "repnz",
    "ss", "cs", "ds", "es", "fs", "gs"
];
fn is_prefix(token: Ident) -> bool {
    PREFIXES.contains(&&*token.node.name.as_str())
}

const X86_SIZES: [(&'static str, Size); 9] = [
    ("BYTE", Size::BYTE),
    ("WORD", Size::WORD),
    ("DWORD", Size::DWORD),
    ("AWORD", Size::DWORD),
    ("FWORD", Size::FWORD),
    ("QWORD", Size::QWORD),
    ("TWORD", Size::PWORD),
    ("OWORD", Size::OWORD),
    ("YWORD", Size::HWORD)
];
const X64_SIZES: [(&'static str, Size); 9] = [
    ("BYTE", Size::BYTE),
    ("WORD", Size::WORD),
    ("DWORD", Size::DWORD),
    ("FWORD", Size::FWORD),
    ("AWORD", Size::QWORD),
    ("QWORD", Size::QWORD),
    ("TWORD", Size::PWORD),
    ("OWORD", Size::OWORD),
    ("YWORD", Size::HWORD)
];
fn eat_size_hint(ctx: &Context, parser: &mut Parser) -> Option<Size> {
    let sizes = match ctx.mode {
        X86Mode::Protected => &X86_SIZES,
        X86Mode::Long      => &X64_SIZES
    };
    for &(kw, size) in sizes {
        if eat_pseudo_keyword(parser, kw) {
            return Some(size);
        }
    }
    None
}

fn eat_pseudo_keyword(parser: &mut Parser, kw: &str) -> bool {
    match parser.token {
        token::Ident(ast::Ident {ref name, ..}, _) if &*name.as_str() == kw => (),
        _ => return false
    }
    parser.bump();
    true
}

fn parse_ident_or_rust_keyword<'a>(parser: &mut Parser<'a>) -> PResult<'a, ast::Ident> {
    if let token::Ident(i, _) = parser.token {
        parser.bump();
        Ok(i)
    } else {
        // technically we could generate the error here directly, but
        // that way this error branch could diverge in behaviour from
        // the normal parse_ident.
        parser.parse_ident()
    }
}

fn parse_arg<'a>(ctx: &mut Context, ecx: &ExtCtxt, parser: &mut Parser<'a>) -> PResult<'a, RawArg> {
    // sizehint
    let size = eat_size_hint(ctx, parser);

    let start = parser.span;

    let in_bracket = parser.check(&token::OpenDelim(token::Bracket));
    if in_bracket && parser.look_ahead(1, |x| match *x {
            token::RArrow |
            token::Gt     |
            token::Lt     |
            token::FatArrow => true,
            _ => false
        }) {
        parser.bump();
    }

    macro_rules! label_return {
        ($jump:expr, $size:expr) => {
            return Ok(if in_bracket {
                parser.expect(&token::CloseDelim(token::Bracket))?;
                RawArg::IndirectJumpTarget {
                    type_: $jump,
                    size: $size
                }
            } else {
                RawArg::JumpTarget {
                    type_: $jump,
                    size: $size
                }
            });
        }
    }

    // global label
    if parser.eat(&token::RArrow) {
        let name = parser.parse_ident()?;
        let jump = JumpType::Global(
            Ident {node: name, span: start.with_hi(parser.prev_span.hi()) }
        );
        label_return!(jump, size);
    // forward local label
    } else if parser.eat(&token::Gt) {
        let name = parser.parse_ident()?;
        let jump = JumpType::Forward(
            Ident {node: name, span: start.with_hi(parser.prev_span.hi()) }
        );
        label_return!(jump, size);
    // forward global label
    } else if parser.eat(&token::Lt) {
        let name = parser.parse_ident()?;
        let jump = JumpType::Backward(
            Ident {node: name, span: start.with_hi(parser.prev_span.hi()) }
        );
        label_return!(jump, size);
    // dynamic label
    } else if parser.eat(&token::FatArrow) {
        let id = parser.parse_expr()?;
        let jump = JumpType::Dynamic(id);
        label_return!(jump, size);
    // extern location
    } else if eat_pseudo_keyword(parser, "extern"){
        let location = parser.parse_expr()?;
        let jump = JumpType::Bare(location);
        label_return!(jump, size);
    }


    // memory location
    if parser.eat(&token::OpenDelim(token::DelimToken::Bracket)) {
        let span = parser.span;
        let nosplit = eat_pseudo_keyword(parser, "NOSPLIT");
        let disp_size = eat_size_hint(ctx, parser);
        let expr = parser.parse_expr()?;
        let span = expr.span.with_lo(span.lo());
        parser.expect(&token::CloseDelim(token::DelimToken::Bracket))?;

        let items = parse_adds(ctx, ecx, expr);

        // assemble the memory location
        return Ok(RawArg::IndirectRaw {
            span: span,
            nosplit: nosplit,
            value_size: size,
            disp_size: disp_size,
            items: items
        });
    }

    // it's a normal (register/immediate/typemapped) operand
    parser.parse_expr()?.and_then(|arg| {
        // typemapped
        if parser.eat(&token::FatArrow) {
            let base = parse_reg(ctx, &arg);
            let base = if let Some(base) = base { base } else {
                ecx.span_err(arg.span, "Expected register");
                return Ok(RawArg::Invalid);
            };

            let ty = parser.parse_path(PathStyle::Type)?;

            // any attribute, register as index and immediate in index
            let mut nosplit = false;
            let mut disp_size = None;
            let items;

            if parser.eat(&token::OpenDelim(token::DelimToken::Bracket)) {
                nosplit = eat_pseudo_keyword(parser, "NOSPLIT");
                disp_size = eat_size_hint(ctx, parser);
                let index_expr = parser.parse_expr()?;

                parser.expect(&token::CloseDelim(token::DelimToken::Bracket))?;
                items = parse_adds(ctx, ecx, index_expr);
            } else {
                items = Vec::new();
            }

            let attr = if parser.eat(&token::Dot) {
                Some(parser.parse_ident()?)
            } else {
                None
            };

            return Ok(RawArg::TypeMappedRaw {
                span: start.with_hi(parser.prev_span.hi()),
                nosplit: nosplit,
                value_size: size,
                disp_size: disp_size,
                base_reg: base.node,
                scale: ty,
                scaled_items: items,
                attribute: attr,
            });
        }

        // direct register reference
        if let Some(reg) = parse_reg(ctx, &arg) {
            if size.is_some() {
                ecx.span_err(arg.span, "size hint with direct register");
            }
            return Ok(RawArg::Direct {
                reg: reg.node,
                span: reg.span
            })
        }

        // immediate
        Ok(RawArg::Immediate {
            value: P(arg),
            size: size
        })
    })
}

pub fn as_simple_name(expr: &ast::Expr) -> Option<Ident> {
    let path = match *expr {
        ast::Expr {node: ast::ExprKind::Path(None, ref path) , ..} => path,
        _ => return None
    };

    if path.is_global() || path.segments.len() != 1 {
        return None;
    }

    let segment = &path.segments[0];
    if !segment.parameters.is_none() {
        return None;
    }

    Some(Ident {node: segment.ident, span: path.span})
}

fn parse_reg(ctx: &Context, expr: &ast::Expr) -> Option<Spanned<Register>> {
    if let Some(path) = as_simple_name(expr) {
        // static register names

        let mut name = &*path.node.name.as_str();
        if let Some(x) = ctx.state.crate_data.aliases.get(name) {
            name = x;
        }

        let (reg, size) = match ctx.mode {
            X86Mode::Long      => X64_REGISTERS.get(&name).cloned(),
            X86Mode::Protected => X86_REGISTERS.get(&name).cloned()
        }?;

        Some(Spanned {
            node: Register::new_static(size, reg),
            span: path.span
        })

    } else if let ast::Expr {node: ast::ExprKind::Call(ref called, ref args), span, ..} = *expr {
        // dynamically chosen registers
        if args.len() != 1 {
            return None;
        }

        let called = if let Some(called) = as_simple_name(called) {
            called
        } else {
            return None;
        };

        let name = &*called.node.name.as_str();
        let (size, family) = match ctx.mode {
            X86Mode::Long      => X64_FAMILIES.get(&name).cloned(),
            X86Mode::Protected => X86_FAMILIES.get(&name).cloned()
        }?;

        Some(Spanned {
            node: Register::new_dynamic(size, family, args[0].clone()),
            span: span
        })
    } else {
        None
    }
}

fn parse_adds(ctx: &Context, ecx: &ExtCtxt, expr: P<ast::Expr>) -> Vec<MemoryRefItem> {
    use syntax::ast::ExprKind;

    let mut adds = Vec::new();
    collect_adds(ecx, expr, &mut adds);

    let mut items = Vec::new();

    // detect what kind of equation we're dealing with
    for node in adds {
        // simple reg
        if let Some(Spanned {node: reg, ..} ) = parse_reg(ctx, &node) {
            items.push(MemoryRefItem::Register(reg));
            continue;
        }
        if let ast::Expr {node: ExprKind::Binary(ast::BinOp {node: ast::BinOpKind::Mul, ..}, ref left, ref right), ..} = *node {
            // reg * const
            if let Some(Spanned {node: reg, ..} ) = parse_reg(ctx, left) {
                if let ast::Expr {node: ExprKind::Lit(ref scale), ..} = **right {
                    if let ast::LitKind::Int(value, _) = scale.node {
                        items.push(MemoryRefItem::ScaledRegister(reg, value as isize));
                        continue;
                    }
                }
            } // const * reg
            if let Some(Spanned {node: reg, ..} ) = parse_reg(ctx, right) {
                if let ast::Expr {node: ExprKind::Lit(ref scale), ..} = **left {
                    if let ast::LitKind::Int(value, _) = scale.node {
                        items.push(MemoryRefItem::ScaledRegister(reg, value as isize));
                        continue;
                    }
                }
            }
        }
        items.push(MemoryRefItem::Displacement(node));
    }

    items
}

fn collect_adds(ecx: &ExtCtxt, node: P<ast::Expr>, collection: &mut Vec<P<ast::Expr>>) {
    node.and_then(|node| {
        if let ast::Expr {node: ast::ExprKind::Binary(ast::BinOp {node: ast::BinOpKind::Add, ..}, left, right), ..} = node {
            collect_adds(ecx, left, collection);
            collect_adds(ecx, right, collection);
        } else if let ast::Expr {node: ast::ExprKind::Binary(ast::BinOp {node: ast::BinOpKind::Sub, ..}, left, right), ..} = node {
            collect_adds(ecx, left, collection);
            let span = right.span;
            collection.push(ecx.expr_unary(span, ast::UnOp::Neg, right));
        } else {
            collection.push(P(node));
        }
    })
}

lazy_static!{
    static ref X64_REGISTERS: HashMap<&'static str, (RegId, Size)> = {
        use self::RegId::*;
        use serialize::Size::*;

        let mut h = HashMap::new();
        h.insert("rax", (RAX, QWORD));
        h.insert("r0" , (RAX, QWORD));
        h.insert("rcx", (RCX, QWORD));
        h.insert("r1" , (RCX, QWORD));
        h.insert("rdx", (RDX, QWORD));
        h.insert("r2" , (RDX, QWORD));
        h.insert("rbx", (RBX, QWORD));
        h.insert("r3" , (RBX, QWORD));
        h.insert("rsp", (RSP, QWORD));
        h.insert("r4" , (RSP, QWORD));
        h.insert("rbp", (RBP, QWORD));
        h.insert("r5" , (RBP, QWORD));
        h.insert("rsi", (RSI, QWORD));
        h.insert("r6" , (RSI, QWORD));
        h.insert("rdi", (RDI, QWORD));
        h.insert("r7" , (RDI, QWORD));
        h.insert("r8" , (R8,  QWORD));
        h.insert("r9" , (R9,  QWORD));
        h.insert("r10", (R10, QWORD));
        h.insert("r11", (R11, QWORD));
        h.insert("r12", (R12, QWORD));
        h.insert("r13", (R13, QWORD));
        h.insert("r14", (R14, QWORD));
        h.insert("r15", (R15, QWORD));

        h.insert("eax" , (RAX, DWORD));
        h.insert("r0d" , (RAX, DWORD));
        h.insert("ecx" , (RCX, DWORD));
        h.insert("r1d" , (RCX, DWORD));
        h.insert("edx" , (RDX, DWORD));
        h.insert("r2d" , (RDX, DWORD));
        h.insert("ebx" , (RBX, DWORD));
        h.insert("r3d" , (RBX, DWORD));
        h.insert("esp" , (RSP, DWORD));
        h.insert("r4d" , (RSP, DWORD));
        h.insert("ebp" , (RBP, DWORD));
        h.insert("r5d" , (RBP, DWORD));
        h.insert("esi" , (RSI, DWORD));
        h.insert("r6d" , (RSI, DWORD));
        h.insert("edi" , (RDI, DWORD));
        h.insert("r7d" , (RDI, DWORD));
        h.insert("r8d" , (R8,  DWORD));
        h.insert("r9d" , (R9,  DWORD));
        h.insert("r10d", (R10, DWORD));
        h.insert("r11d", (R11, DWORD));
        h.insert("r12d", (R12, DWORD));
        h.insert("r13d", (R13, DWORD));
        h.insert("r14d", (R14, DWORD));
        h.insert("r15d", (R15, DWORD));

        h.insert("ax"  , (RAX, WORD));
        h.insert("r0w" , (RAX, WORD));
        h.insert("cx"  , (RCX, WORD));
        h.insert("r1w" , (RCX, WORD));
        h.insert("dx"  , (RDX, WORD));
        h.insert("r2w" , (RDX, WORD));
        h.insert("bx"  , (RBX, WORD));
        h.insert("r3w" , (RBX, WORD));
        h.insert("sp"  , (RSP, WORD));
        h.insert("r4w" , (RSP, WORD));
        h.insert("bp"  , (RBP, WORD));
        h.insert("r5w" , (RBP, WORD));
        h.insert("si"  , (RSI, WORD));
        h.insert("r6w" , (RSI, WORD));
        h.insert("di"  , (RDI, WORD));
        h.insert("r7w" , (RDI, WORD));
        h.insert("r8w" , (R8,  WORD));
        h.insert("r9w" , (R9,  WORD));
        h.insert("r10w", (R10, WORD));
        h.insert("r11w", (R11, WORD));
        h.insert("r12w", (R12, WORD));
        h.insert("r13w", (R13, WORD));
        h.insert("r14w", (R14, WORD));
        h.insert("r15w", (R15, WORD));

        h.insert("al"  , (RAX, BYTE));
        h.insert("r0b" , (RAX, BYTE));
        h.insert("cl"  , (RCX, BYTE));
        h.insert("r1b" , (RCX, BYTE));
        h.insert("dl"  , (RDX, BYTE));
        h.insert("r2b" , (RDX, BYTE));
        h.insert("bl"  , (RBX, BYTE));
        h.insert("r3b" , (RBX, BYTE));
        h.insert("spl" , (RSP, BYTE));
        h.insert("r4b" , (RSP, BYTE));
        h.insert("bpl" , (RBP, BYTE));
        h.insert("r5b" , (RBP, BYTE));
        h.insert("sil" , (RSI, BYTE));
        h.insert("r6b" , (RSI, BYTE));
        h.insert("dil" , (RDI, BYTE));
        h.insert("r7b" , (RDI, BYTE));
        h.insert("r8b" , (R8,  BYTE));
        h.insert("r9b" , (R9,  BYTE));
        h.insert("r10b", (R10, BYTE));
        h.insert("r11b", (R11, BYTE));
        h.insert("r12b", (R12, BYTE));
        h.insert("r13b", (R13, BYTE));
        h.insert("r14b", (R14, BYTE));
        h.insert("r15b", (R15, BYTE));

        h.insert("rip", (RIP, QWORD));
        h.insert("eip", (RIP, DWORD));

        h.insert("ah", (AH, BYTE));
        h.insert("ch", (CH, BYTE));
        h.insert("dh", (DH, BYTE));
        h.insert("bh", (BH, BYTE));

        h.insert("st0", (ST0, PWORD));
        h.insert("st1", (ST1, PWORD));
        h.insert("st2", (ST2, PWORD));
        h.insert("st3", (ST3, PWORD));
        h.insert("st4", (ST4, PWORD));
        h.insert("st5", (ST5, PWORD));
        h.insert("st6", (ST6, PWORD));
        h.insert("st7", (ST7, PWORD));

        h.insert("mm0", (MMX0, QWORD));
        h.insert("mm1", (MMX1, QWORD));
        h.insert("mm2", (MMX2, QWORD));
        h.insert("mm3", (MMX3, QWORD));
        h.insert("mm4", (MMX4, QWORD));
        h.insert("mm5", (MMX5, QWORD));
        h.insert("mm6", (MMX6, QWORD));
        h.insert("mm7", (MMX7, QWORD));

        h.insert("xmm0" , (XMM0 , OWORD));
        h.insert("xmm1" , (XMM1 , OWORD));
        h.insert("xmm2" , (XMM2 , OWORD));
        h.insert("xmm3" , (XMM3 , OWORD));
        h.insert("xmm4" , (XMM4 , OWORD));
        h.insert("xmm5" , (XMM5 , OWORD));
        h.insert("xmm6" , (XMM6 , OWORD));
        h.insert("xmm7" , (XMM7 , OWORD));
        h.insert("xmm8" , (XMM8 , OWORD));
        h.insert("xmm9" , (XMM9 , OWORD));
        h.insert("xmm10", (XMM10, OWORD));
        h.insert("xmm11", (XMM11, OWORD));
        h.insert("xmm12", (XMM12, OWORD));
        h.insert("xmm13", (XMM13, OWORD));
        h.insert("xmm14", (XMM14, OWORD));
        h.insert("xmm15", (XMM15, OWORD));

        h.insert("ymm0" , (XMM0 , HWORD));
        h.insert("ymm1" , (XMM1 , HWORD));
        h.insert("ymm2" , (XMM2 , HWORD));
        h.insert("ymm3" , (XMM3 , HWORD));
        h.insert("ymm4" , (XMM4 , HWORD));
        h.insert("ymm5" , (XMM5 , HWORD));
        h.insert("ymm6" , (XMM6 , HWORD));
        h.insert("ymm7" , (XMM7 , HWORD));
        h.insert("ymm8" , (XMM8 , HWORD));
        h.insert("ymm9" , (XMM9 , HWORD));
        h.insert("ymm10", (XMM10, HWORD));
        h.insert("ymm11", (XMM11, HWORD));
        h.insert("ymm12", (XMM12, HWORD));
        h.insert("ymm13", (XMM13, HWORD));
        h.insert("ymm14", (XMM14, HWORD));
        h.insert("ymm15", (XMM15, HWORD));

        h.insert("es", (ES, WORD));
        h.insert("cs", (CS, WORD));
        h.insert("ss", (SS, WORD));
        h.insert("ds", (DS, WORD));
        h.insert("fs", (FS, WORD));
        h.insert("gs", (GS, WORD));

        h.insert("cr0" , (CR0 , QWORD));
        h.insert("cr1" , (CR1 , QWORD));
        h.insert("cr2" , (CR2 , QWORD));
        h.insert("cr3" , (CR3 , QWORD));
        h.insert("cr4" , (CR4 , QWORD));
        h.insert("cr5" , (CR5 , QWORD));
        h.insert("cr6" , (CR6 , QWORD));
        h.insert("cr7" , (CR7 , QWORD));
        h.insert("cr8" , (CR8 , QWORD));
        h.insert("cr9" , (CR9 , QWORD));
        h.insert("cr10", (CR10, QWORD));
        h.insert("cr11", (CR11, QWORD));
        h.insert("cr12", (CR12, QWORD));
        h.insert("cr13", (CR13, QWORD));
        h.insert("cr14", (CR14, QWORD));
        h.insert("cr15", (CR15, QWORD));

        h.insert("dr0" , (DR0 , QWORD));
        h.insert("dr1" , (DR1 , QWORD));
        h.insert("dr2" , (DR2 , QWORD));
        h.insert("dr3" , (DR3 , QWORD));
        h.insert("dr4" , (DR4 , QWORD));
        h.insert("dr5" , (DR5 , QWORD));
        h.insert("dr6" , (DR6 , QWORD));
        h.insert("dr7" , (DR7 , QWORD));
        h.insert("dr8" , (DR8 , QWORD));
        h.insert("dr9" , (DR9 , QWORD));
        h.insert("dr10", (DR10, QWORD));
        h.insert("dr11", (DR11, QWORD));
        h.insert("dr12", (DR12, QWORD));
        h.insert("dr13", (DR13, QWORD));
        h.insert("dr14", (DR14, QWORD));
        h.insert("dr15", (DR15, QWORD));

        h.insert("bnd0", (BND0, OWORD));
        h.insert("bnd1", (BND1, OWORD));
        h.insert("bnd2", (BND2, OWORD));
        h.insert("bnd3", (BND3, OWORD));
        h
    };
    static ref X86_REGISTERS: HashMap<&'static str, (RegId, Size)> = {
        use self::RegId::*;
        use serialize::Size::*;

        let mut h = HashMap::new();
        h.insert("eax", (RAX, DWORD));
        h.insert("ecx", (RCX, DWORD));
        h.insert("edx", (RDX, DWORD));
        h.insert("ebx", (RBX, DWORD));
        h.insert("esp", (RSP, DWORD));
        h.insert("ebp", (RBP, DWORD));
        h.insert("esi", (RSI, DWORD));
        h.insert("edi", (RDI, DWORD));

        h.insert("ax", (RAX, WORD));
        h.insert("cx", (RCX, WORD));
        h.insert("dx", (RDX, WORD));
        h.insert("bx", (RBX, WORD));
        h.insert("sp", (RSP, WORD));
        h.insert("bp", (RBP, WORD));
        h.insert("si", (RSI, WORD));
        h.insert("di", (RDI, WORD));

        h.insert("al", (RAX, BYTE));
        h.insert("cl", (RCX, BYTE));
        h.insert("dl", (RDX, BYTE));
        h.insert("bl", (RBX, BYTE));

        h.insert("eip", (RIP, DWORD));

        h.insert("ah", (AH, BYTE));
        h.insert("ch", (CH, BYTE));
        h.insert("dh", (DH, BYTE));
        h.insert("bh", (BH, BYTE));

        h.insert("st0", (ST0, PWORD));
        h.insert("st1", (ST1, PWORD));
        h.insert("st2", (ST2, PWORD));
        h.insert("st3", (ST3, PWORD));
        h.insert("st4", (ST4, PWORD));
        h.insert("st5", (ST5, PWORD));
        h.insert("st6", (ST6, PWORD));
        h.insert("st7", (ST7, PWORD));

        h.insert("mm0", (MMX0, QWORD));
        h.insert("mm1", (MMX1, QWORD));
        h.insert("mm2", (MMX2, QWORD));
        h.insert("mm3", (MMX3, QWORD));
        h.insert("mm4", (MMX4, QWORD));
        h.insert("mm5", (MMX5, QWORD));
        h.insert("mm6", (MMX6, QWORD));
        h.insert("mm7", (MMX7, QWORD));

        h.insert("xmm0", (XMM0, OWORD));
        h.insert("xmm1", (XMM1, OWORD));
        h.insert("xmm2", (XMM2, OWORD));
        h.insert("xmm3", (XMM3, OWORD));
        h.insert("xmm4", (XMM4, OWORD));
        h.insert("xmm5", (XMM5, OWORD));
        h.insert("xmm6", (XMM6, OWORD));
        h.insert("xmm7", (XMM7, OWORD));

        h.insert("ymm0", (XMM0, HWORD));
        h.insert("ymm1", (XMM1, HWORD));
        h.insert("ymm2", (XMM2, HWORD));
        h.insert("ymm3", (XMM3, HWORD));
        h.insert("ymm4", (XMM4, HWORD));
        h.insert("ymm5", (XMM5, HWORD));
        h.insert("ymm6", (XMM6, HWORD));
        h.insert("ymm7", (XMM7, HWORD));

        h.insert("es", (ES, WORD));
        h.insert("cs", (CS, WORD));
        h.insert("ss", (SS, WORD));
        h.insert("ds", (DS, WORD));
        h.insert("fs", (FS, WORD));
        h.insert("gs", (GS, WORD));

        h.insert("cr0", (CR0, DWORD));
        h.insert("cr1", (CR1, DWORD));
        h.insert("cr2", (CR2, DWORD));
        h.insert("cr3", (CR3, DWORD));
        h.insert("cr4", (CR4, DWORD));
        h.insert("cr5", (CR5, DWORD));
        h.insert("cr6", (CR6, DWORD));
        h.insert("cr7", (CR7, DWORD));

        h.insert("dr0", (DR0, DWORD));
        h.insert("dr1", (DR1, DWORD));
        h.insert("dr2", (DR2, DWORD));
        h.insert("dr3", (DR3, DWORD));
        h.insert("dr4", (DR4, DWORD));
        h.insert("dr5", (DR5, DWORD));
        h.insert("dr6", (DR6, DWORD));
        h.insert("dr7", (DR7, DWORD));

        h.insert("bnd0", (BND0, OWORD));
        h.insert("bnd1", (BND1, OWORD));
        h.insert("bnd2", (BND2, OWORD));
        h.insert("bnd3", (BND3, OWORD));
        h
    };
    static ref X64_FAMILIES:  HashMap<&'static str, (Size, RegFamily)> = {
        let mut h = HashMap::new();
        h.insert("Rb", (Size::BYTE,  RegFamily::LEGACY));
        h.insert("Rh", (Size::BYTE,  RegFamily::HIGHBYTE));
        h.insert("Rw", (Size::WORD,  RegFamily::LEGACY));
        h.insert("Rd", (Size::DWORD, RegFamily::LEGACY));
        h.insert("Ra", (Size::QWORD, RegFamily::LEGACY));
        h.insert("Rq", (Size::QWORD, RegFamily::LEGACY));
        h.insert("Rf", (Size::PWORD, RegFamily::FP));
        h.insert("Rm", (Size::QWORD, RegFamily::MMX));
        h.insert("Rx", (Size::OWORD, RegFamily::XMM));
        h.insert("Ry", (Size::HWORD, RegFamily::XMM));
        h.insert("Rs", (Size::WORD,  RegFamily::SEGMENT));
        h.insert("RC", (Size::QWORD, RegFamily::CONTROL));
        h.insert("RD", (Size::QWORD, RegFamily::DEBUG));
        h.insert("RB", (Size::OWORD, RegFamily::BOUND));
        h
    };
    static ref X86_FAMILIES:  HashMap<&'static str, (Size, RegFamily)> = {
        let mut h = HashMap::new();
        h.insert("Rb",(Size::BYTE,  RegFamily::LEGACY));
        h.insert("Rh",(Size::BYTE,  RegFamily::HIGHBYTE));
        h.insert("Rw",(Size::WORD,  RegFamily::LEGACY));
        h.insert("Ra",(Size::DWORD, RegFamily::LEGACY));
        h.insert("Rd",(Size::DWORD, RegFamily::LEGACY));
        h.insert("Rf",(Size::PWORD, RegFamily::FP));
        h.insert("Rm",(Size::QWORD, RegFamily::MMX));
        h.insert("Rx",(Size::OWORD, RegFamily::XMM));
        h.insert("Ry",(Size::HWORD, RegFamily::XMM));
        h.insert("Rs",(Size::WORD,  RegFamily::SEGMENT));
        h.insert("RC",(Size::DWORD, RegFamily::CONTROL));
        h.insert("RD",(Size::DWORD, RegFamily::DEBUG));
        h.insert("RB",(Size::OWORD, RegFamily::BOUND));
        h
    };
}
