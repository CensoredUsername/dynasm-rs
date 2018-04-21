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

const SIZES: [(&'static str, Size); 8] = [
    ("BYTE", Size::BYTE),
    ("WORD", Size::WORD),
    ("DWORD", Size::DWORD),
    ("AWORD", Size::QWORD),
    ("QWORD", Size::QWORD),
    ("TWORD", Size::PWORD),
    ("OWORD", Size::OWORD),
    ("YWORD", Size::HWORD)
];
fn eat_size_hint(parser: &mut Parser) -> Option<Size> {
    for &(kw, size) in &SIZES {
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
    let size = eat_size_hint(parser);

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
    }

    // memory location
    if parser.eat(&token::OpenDelim(token::DelimToken::Bracket)) {
        let span = parser.span;
        let nosplit = eat_pseudo_keyword(parser, "NOSPLIT");
        let disp_size = eat_size_hint(parser);
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
                disp_size = eat_size_hint(parser);
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

        use self::RegId::*;
        use serialize::Size::*;
        let (reg, size) = if let X86Mode::Long = ctx.mode {
            // TODO: I wouldn't be surprised if this is a performance bottleneck currently. Maybe factor this out into a hashmap at one point
            match name {
                "rax"|"r0" => (RAX, QWORD), "rcx"|"r1" => (RCX, QWORD), "rdx"|"r2" => (RDX, QWORD), "rbx"|"r3" => (RBX, QWORD),
                "rsp"|"r4" => (RSP, QWORD), "rbp"|"r5" => (RBP, QWORD), "rsi"|"r6" => (RSI, QWORD), "rdi"|"r7" => (RDI, QWORD),
                "r8"       => (R8,  QWORD), "r9"       => (R9,  QWORD), "r10"      => (R10, QWORD), "r11"      => (R11, QWORD),
                "r12"      => (R12, QWORD), "r13"      => (R13, QWORD), "r14"      => (R14, QWORD), "r15"      => (R15, QWORD),

                "eax"|"r0d" => (RAX, DWORD), "ecx"|"r1d" => (RCX, DWORD), "edx"|"r2d" => (RDX, DWORD), "ebx"|"r3d" => (RBX, DWORD),
                "esp"|"r4d" => (RSP, DWORD), "ebp"|"r5d" => (RBP, DWORD), "esi"|"r6d" => (RSI, DWORD), "edi"|"r7d" => (RDI, DWORD),
                "r8d"       => (R8,  DWORD), "r9d"       => (R9,  DWORD), "r10d"      => (R10, DWORD), "r11d"      => (R11, DWORD),
                "r12d"      => (R12, DWORD), "r13d"      => (R13, DWORD), "r14d"      => (R14, DWORD), "r15d"      => (R15, DWORD),

                "ax"|"r0w" => (RAX, WORD), "cx"|"r1w" => (RCX, WORD), "dx"|"r2w" => (RDX, WORD), "bx"|"r3w" => (RBX, WORD),
                "sp"|"r4w" => (RSP, WORD), "bp"|"r5w" => (RBP, WORD), "si"|"r6w" => (RSI, WORD), "di"|"r7w" => (RDI, WORD),
                "r8w"      => (R8,  WORD), "r9w"      => (R9,  WORD), "r10w"     => (R10, WORD), "r11w"     => (R11, WORD),
                "r12w"     => (R12, WORD), "r13w"     => (R13, WORD), "r14w"     => (R14, WORD), "r15w"     => (R15, WORD),

                "al"|"r0b" => (RAX, BYTE), "cl"|"r1b" => (RCX, BYTE), "dl"|"r2b" => (RDX, BYTE), "bl"|"r3b" => (RBX, BYTE),
                "spl"      => (RSP, BYTE), "bpl"      => (RBP, BYTE), "sil"      => (RSI, BYTE), "dil"      => (RDI, BYTE),
                "r8b"      => (R8,  BYTE), "r9b"      => (R9,  BYTE), "r10b"     => (R10, BYTE), "r11b"     => (R11, BYTE),
                "r12b"     => (R12, BYTE), "r13b"     => (R13, BYTE), "r14b"     => (R14, BYTE), "r15b"     => (R15, BYTE),

                "rip"  => (RIP, QWORD), "eip" => (RIP, DWORD),

                "ah" => (AH, BYTE), "ch" => (CH, BYTE), "dh" => (DH, BYTE), "bh" => (BH, BYTE),

                "st0" => (ST0, PWORD), "st1" => (ST1, PWORD), "st2" => (ST2, PWORD), "st3" => (ST3, PWORD),
                "st4" => (ST4, PWORD), "st5" => (ST5, PWORD), "st6" => (ST6, PWORD), "st7" => (ST7, PWORD),

                "mm0" => (MMX0, QWORD), "mm1" => (MMX1, QWORD), "mm2" => (MMX2, QWORD), "mm3" => (MMX3, QWORD),
                "mm4" => (MMX4, QWORD), "mm5" => (MMX5, QWORD), "mm6" => (MMX6, QWORD), "mm7" => (MMX7, QWORD),

                "xmm0"  => (XMM0 , OWORD), "xmm1"  => (XMM1 , OWORD), "xmm2"  => (XMM2 , OWORD), "xmm3"  => (XMM3 , OWORD),
                "xmm4"  => (XMM4 , OWORD), "xmm5"  => (XMM5 , OWORD), "xmm6"  => (XMM6 , OWORD), "xmm7"  => (XMM7 , OWORD),
                "xmm8"  => (XMM8 , OWORD), "xmm9"  => (XMM9 , OWORD), "xmm10" => (XMM10, OWORD), "xmm11" => (XMM11, OWORD),
                "xmm12" => (XMM12, OWORD), "xmm13" => (XMM13, OWORD), "xmm14" => (XMM14, OWORD), "xmm15" => (XMM15, OWORD),

                "ymm0"  => (XMM0 , HWORD), "ymm1"  => (XMM1 , HWORD), "ymm2"  => (XMM2 , HWORD), "ymm3"  => (XMM3 , HWORD),
                "ymm4"  => (XMM4 , HWORD), "ymm5"  => (XMM5 , HWORD), "ymm6"  => (XMM6 , HWORD), "ymm7"  => (XMM7 , HWORD),
                "ymm8"  => (XMM8 , HWORD), "ymm9"  => (XMM9 , HWORD), "ymm10" => (XMM10, HWORD), "ymm11" => (XMM11, HWORD),
                "ymm12" => (XMM12, HWORD), "ymm13" => (XMM13, HWORD), "ymm14" => (XMM14, HWORD), "ymm15" => (XMM15, HWORD),

                "es" => (ES, WORD), "cs" => (CS, WORD), "ss" => (SS, WORD), "ds" => (DS, WORD),
                "fs" => (FS, WORD), "gs" => (GS, WORD),

                "cr0"  => (CR0 , QWORD), "cr1"  => (CR1 , QWORD), "cr2"  => (CR2 , QWORD), "cr3"  => (CR3 , QWORD),
                "cr4"  => (CR4 , QWORD), "cr5"  => (CR5 , QWORD), "cr6"  => (CR6 , QWORD), "cr7"  => (CR7 , QWORD),
                "cr8"  => (CR8 , QWORD), "cr9"  => (CR9 , QWORD), "cr10" => (CR10, QWORD), "cr11" => (CR11, QWORD),
                "cr12" => (CR12, QWORD), "cr13" => (CR13, QWORD), "cr14" => (CR14, QWORD), "cr15" => (CR15, QWORD),

                "dr0"  => (DR0 , QWORD), "dr1"  => (DR1 , QWORD), "dr2"  => (DR2 , QWORD), "dr3"  => (DR3 , QWORD),
                "dr4"  => (DR4 , QWORD), "dr5"  => (DR5 , QWORD), "dr6"  => (DR6 , QWORD), "dr7"  => (DR7 , QWORD),
                "dr8"  => (DR8 , QWORD), "dr9"  => (DR9 , QWORD), "dr10" => (DR10, QWORD), "dr11" => (DR11, QWORD),
                "dr12" => (DR12, QWORD), "dr13" => (DR13, QWORD), "dr14" => (DR14, QWORD), "dr15" => (DR15, QWORD),

                "bnd0" => (BND0, OWORD), "bnd1" => (BND1, OWORD), "bnd2" => (BND2, OWORD), "bnd3" => (BND3, OWORD),

                _ => return None
            }
        } else {
            match name {
                "eax" => (RAX, DWORD), "ecx" => (RCX, DWORD), "edx" => (RDX, DWORD), "ebx" => (RBX, DWORD),
                "esp" => (RSP, DWORD), "ebp" => (RBP, DWORD), "esi" => (RSI, DWORD), "edi" => (RDI, DWORD),

                "ax" => (RAX, WORD), "cx" => (RCX, WORD), "dx" => (RDX, WORD), "bx" => (RBX, WORD),
                "sp" => (RSP, WORD), "bp" => (RBP, WORD), "si" => (RSI, WORD), "di" => (RDI, WORD),

                "al" => (RAX, BYTE), "cl" => (RCX, BYTE), "dl" => (RDX, BYTE), "bl" => (RBX, BYTE),

                "eip" => (RIP, DWORD),

                "ah" => (AH, BYTE), "ch" => (CH, BYTE), "dh" => (DH, BYTE), "bh" => (BH, BYTE),

                "st0" => (ST0, PWORD), "st1" => (ST1, PWORD), "st2" => (ST2, PWORD), "st3" => (ST3, PWORD),
                "st4" => (ST4, PWORD), "st5" => (ST5, PWORD), "st6" => (ST6, PWORD), "st7" => (ST7, PWORD),

                "mm0" => (MMX0, QWORD), "mm1" => (MMX1, QWORD), "mm2" => (MMX2, QWORD), "mm3" => (MMX3, QWORD),
                "mm4" => (MMX4, QWORD), "mm5" => (MMX5, QWORD), "mm6" => (MMX6, QWORD), "mm7" => (MMX7, QWORD),

                "xmm0"  => (XMM0 , OWORD), "xmm1"  => (XMM1 , OWORD), "xmm2"  => (XMM2 , OWORD), "xmm3"  => (XMM3 , OWORD),
                "xmm4"  => (XMM4 , OWORD), "xmm5"  => (XMM5 , OWORD), "xmm6"  => (XMM6 , OWORD), "xmm7"  => (XMM7 , OWORD),

                "ymm0"  => (XMM0 , HWORD), "ymm1"  => (XMM1 , HWORD), "ymm2"  => (XMM2 , HWORD), "ymm3"  => (XMM3 , HWORD),
                "ymm4"  => (XMM4 , HWORD), "ymm5"  => (XMM5 , HWORD), "ymm6"  => (XMM6 , HWORD), "ymm7"  => (XMM7 , HWORD),

                "es" => (ES, WORD), "cs" => (CS, WORD), "ss" => (SS, WORD), "ds" => (DS, WORD),
                "fs" => (FS, WORD), "gs" => (GS, WORD),

                "cr0"  => (CR0 , DWORD), "cr1"  => (CR1 , DWORD), "cr2"  => (CR2 , DWORD), "cr3"  => (CR3 , DWORD),
                "cr4"  => (CR4 , DWORD), "cr5"  => (CR5 , DWORD), "cr6"  => (CR6 , DWORD), "cr7"  => (CR7 , DWORD),

                "dr0"  => (DR0 , DWORD), "dr1"  => (DR1 , DWORD), "dr2"  => (DR2 , DWORD), "dr3"  => (DR3 , DWORD),
                "dr4"  => (DR4 , DWORD), "dr5"  => (DR5 , DWORD), "dr6"  => (DR6 , DWORD), "dr7"  => (DR7 , DWORD),

                "bnd0" => (BND0, OWORD), "bnd1" => (BND1, OWORD), "bnd2" => (BND2, OWORD), "bnd3" => (BND3, OWORD),

                _ => return None
            }
        };

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

        let (size, family) = if let X86Mode::Long = ctx.mode {
            match &*called.node.name.as_str() {
                "Rb" => (Size::BYTE,  RegFamily::LEGACY),
                "Rh" => (Size::BYTE,  RegFamily::HIGHBYTE),
                "Rw" => (Size::WORD,  RegFamily::LEGACY),
                "Rd" => (Size::DWORD, RegFamily::LEGACY),
                "Ra" |
                "Rq" => (Size::QWORD, RegFamily::LEGACY),
                "Rf" => (Size::PWORD, RegFamily::FP),
                "Rm" => (Size::QWORD, RegFamily::MMX),
                "Rx" => (Size::OWORD, RegFamily::XMM),
                "Ry" => (Size::HWORD, RegFamily::XMM),
                "Rs" => (Size::WORD,  RegFamily::SEGMENT),
                "RC" => (Size::QWORD, RegFamily::CONTROL),
                "RD" => (Size::QWORD, RegFamily::DEBUG),
                "RB" => (Size::OWORD, RegFamily::BOUND),
                _ => return None
            }
        } else {
            match &*called.node.name.as_str() {
                "Rb" => (Size::BYTE,  RegFamily::LEGACY),
                "Rh" => (Size::BYTE,  RegFamily::HIGHBYTE),
                "Rw" => (Size::WORD,  RegFamily::LEGACY),
                "Ra" |
                "Rd" => (Size::DWORD, RegFamily::LEGACY),
                "Rf" => (Size::PWORD, RegFamily::FP),
                "Rm" => (Size::QWORD, RegFamily::MMX),
                "Rx" => (Size::OWORD, RegFamily::XMM),
                "Ry" => (Size::HWORD, RegFamily::XMM),
                "Rs" => (Size::WORD,  RegFamily::SEGMENT),
                "RC" => (Size::DWORD, RegFamily::CONTROL),
                "RD" => (Size::DWORD, RegFamily::DEBUG),
                "RB" => (Size::OWORD, RegFamily::BOUND),
                _ => return None
            }
        };

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
