use syntax::ext::build::AstBuilder;
use syntax::ext::base::ExtCtxt;
use syntax::parse::parser::Parser;
use syntax::parse::token;
use syntax::parse::PResult;
use syntax::ast;
use syntax::ptr::P;
use syntax::codemap::{Spanned, Span};

use std::collections::HashMap;

pub type Ident = Spanned<ast::Ident>;

/**
 * collections
 */

#[derive(Debug)]
pub enum Item {
    Instruction(Vec<Ident>, Vec<Arg>, Span),
    Label(LabelType),
    Directive(Ident, Vec<Arg>, Span)
}

#[derive(Debug)]
pub enum Arg {
    Indirect(MemoryRef), // indirect memory reference supporting scale, index, base and displacement.
    Direct(Spanned<Register>), // a bare register (rax, ...)
    JumpTarget(JumpType, Option<Size>), // jump target.
    Immediate(P<ast::Expr>, Option<Size>), // an expression that evaluates to a value. basically, anything that ain't the other three
    Invalid // placeholder value
}

#[derive(Debug)]
pub struct MemoryRef {
    pub index: Option<Register>,
    pub scale: isize,
    pub base:  Option<Register>,
    pub disp:  Option<P<ast::Expr>>,
    pub size:  Option<Size>,
    pub span:  Span
}

#[derive(Debug)]
pub enum LabelType {
    Global(Ident),         // . label :
    Local(Ident),          // label :
    Dynamic(P<ast::Expr>), // => expr :
}

#[derive(Debug)]
pub enum JumpType {
    // note: these symbol choices try to avoid stuff that is a valid starting symbol for parse_expr
    // in order to allow the full range of expressions to be used. the only currently existing ambiguity is 
    // with the symbol <, as this symbol is also the starting symbol for the universal calling syntax <Type as Trait>.method(args)
    Global(Ident),         // . label
    Backward(Ident),       // > label
    Forward(Ident),        // < label
    Dynamic(P<ast::Expr>), // => expr
}

// encoding of this:
// lower byte indicates which register it is
// upper byte is used to indicate which size group it falls under.

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Register {
    RAX  = 0x0300, RCX  = 0x0301, RDX  = 0x0302, RBX  = 0x0303,
    RSP  = 0x0304, RBP  = 0x0305, RSI  = 0x0306, RDI  = 0x0307,
    R8   = 0x0308, R9   = 0x0309, R10  = 0x030A, R11  = 0x030B,
    R12  = 0x030C, R13  = 0x030D, R14  = 0x030E, R15  = 0x030F,

    EAX  = 0x0200, ECX  = 0x0201, EDX  = 0x0202, EBX  = 0x0203,
    ESP  = 0x0204, EBP  = 0x0205, ESI  = 0x0206, EDI  = 0x0207,
    R8D  = 0x0208, R9D  = 0x0209, R10D = 0x020A, R11D = 0x020B,
    R12D = 0x020C, R13D = 0x020D, R14D = 0x020E, R15D = 0x020F,

    AX   = 0x0100, CX   = 0x0101, DX   = 0x0102, BX   = 0x0103,
    SP   = 0x0104, BP   = 0x0105, SI   = 0x0106, DI   = 0x0107,
    R8W  = 0x0108, R9W  = 0x0109, R10W = 0x010A, R11W = 0x010B,
    R12W = 0x010C, R13W = 0x010D, R14W = 0x010E, R15W = 0x010F,

    AL   = 0x0000, CL   = 0x0001, DL   = 0x0002, BL   = 0x0003,
    SPL  = 0x0004, BPL  = 0x0005, SIL  = 0x0006, DIL  = 0x0007,
    R8B  = 0x0008, R9B  = 0x0009, R10B = 0x000A, R11B = 0x000B,
    R12B = 0x000C, R13B = 0x000D, R14B = 0x000E, R15B = 0x000F,

    RIP = 0x0310,
}

#[derive(PartialOrd, PartialEq, Ord, Eq, Debug, Clone, Copy)]
pub enum Size {
    BYTE  = 1,
    WORD  = 2,
    DWORD = 4,
    QWORD = 8
}

/*
 * impls
 */

impl Register {
    pub fn size(&self) -> Size {
        match (*self as u16) & 0x0300 {
            0x0000 => Size::BYTE,
            0x0100 => Size::WORD,
            0x0200 => Size::DWORD,
            0x0300 => Size::QWORD,
            _ => unreachable!()
        }
    }

    pub fn code(&self) -> u8 {
        *self as u8
    }
} 

impl Size {
    pub fn in_bytes(&self) -> u8 {
        *self as u8
    }
}

/*
 * Code
 */

// tokentree is a list of tokens and delimited lists of tokens.
// this means we don't have to figure out nesting via []'s by ourselves.
// syntax for a single op: PREFIX* ident (SIZE? expr ("," SIZE? expr)*)? ";"

pub fn parse<'a>(ecx: &ExtCtxt, parser: &mut Parser<'a>) -> PResult<'a, (Ident, Vec<Item>)> {
    let span = parser.span;
    let name = Spanned {node: try!(parser.parse_ident()), span: span};

    let mut ins = Vec::new();

    while !parser.check(&token::Eof) {

        try!(parser.expect(&token::Semi));

        let startspan = parser.span;

        // parse . or => indicating possible label/directive or label

        let has_dot = parser.eat(&token::Dot);

        if !has_dot && parser.eat(&token::FatArrow) {
            // dynamic label branch
            let expr = try!(parser.parse_expr());

            ins.push(Item::Label(LabelType::Dynamic(expr)));
            continue;
        }

        // parse the first part of an op or a label

        let mut span = parser.span;
        let mut op = Spanned {node: try!(parser.parse_ident()), span: span};

        // parse a colon indicating we were in a label

        if parser.eat(&token::Colon) {
            ins.push(Item::Label(if has_dot {
                LabelType::Global(op)
            } else {
                LabelType::Local(op)
            }));
            continue;
        }

        // if op was a prefix, continue parsing ops

        let mut ops = Vec::new();
        if !has_dot {
            while is_prefix(op) {
                ops.push(op);
                span = parser.span;
                op = Spanned {node: try!(parser.parse_ident()), span: span};
            }
        }

        // parse (sizehint? expr),*
        let mut args = Vec::new();

        if !parser.check(&token::Semi) && !parser.check(&token::Eof) {
            args.push(try!(parse_arg(ecx, parser)));

            while parser.eat(&token::Comma) {
                args.push(try!(parse_arg(ecx, parser)));
            }
        }

        let span = Span {hi: parser.last_span.hi, ..startspan};

        if !has_dot {
            ops.push(op);
            ins.push(Item::Instruction(ops, args, span));
        } else {
            ins.push(Item::Directive(op, args, span));
        }
    }

    Ok((name, ins))
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

const SIZES:    [(&'static str, Size); 4] = [
    ("BYTE", Size::BYTE),
    ("WORD", Size::WORD),
    ("DWORD", Size::DWORD),
    ("QWORD", Size::QWORD)
];
fn eat_size_hint(parser: &mut Parser) -> Option<Size> {
    for &(kw, size) in SIZES.iter() {
        if eat_pseudo_keyword(parser, kw) {
            return Some(size);
        }
    }
    None
}

fn eat_pseudo_keyword(parser: &mut Parser, kw: &str) -> bool {
    match parser.token {
        token::Token::Ident(ast::Ident {ref name, ..}) if &*name.as_str() == kw => (),
        _ => return false
    }
    parser.bump();
    true
}

fn parse_arg<'a>(ecx: &ExtCtxt, parser: &mut Parser<'a>) -> PResult<'a, Arg> {
    use syntax::ast::ExprKind;

    // sizehint
    let size = eat_size_hint(parser);

    let start = parser.span;

    // global label
    if parser.eat(&token::Dot) {
        let name = try!(parser.parse_ident());
        return Ok(Arg::JumpTarget(JumpType::Global(
            Ident {node: name, span: Span {hi: parser.last_span.hi, ..start} }
        ), size));

    // forward local label
    } else if parser.eat(&token::Gt) {
        let name = try!(parser.parse_ident());
        return Ok(Arg::JumpTarget(JumpType::Forward(
            Ident {node: name, span: Span {hi: parser.last_span.hi, ..start} }
        ), size));

    // forward global label
    } else if parser.eat(&token::Lt) {
        let name = try!(parser.parse_ident());
        return Ok(Arg::JumpTarget(JumpType::Backward(
            Ident {node: name, span: Span {hi: parser.last_span.hi, ..start} }
        ), size));

    // dynamic label
    } else if parser.eat(&token::FatArrow) {
        let id = try!(parser.parse_expr());
        return Ok(Arg::JumpTarget(JumpType::Dynamic(id), size));

    }

    // it's a normal (register/immediate/memoryref) operand
    let arg = try!(parser.parse_expr()).unwrap();

    // direct register reference
    if let Some(reg) = parse_reg(&arg) {
        if size.is_some() {
            ecx.span_err(arg.span, "size hint with direct register");
        }
        return Ok(Arg::Direct(reg))
    }

    // memory location
    if let ast::Expr {node: ExprKind::Vec(mut items), span, ..} = arg {
        if items.len() != 1 {
            ecx.span_err(span, "Comma in memory reference");
            return Ok(Arg::Invalid);
        }

        let mut added = Vec::new();
        parse_adds(items.pop().unwrap().unwrap(), &mut added);

        let mut regs: HashMap<Register, isize> = HashMap::new();
        let mut immediates = Vec::new();

        for node in added {
            if let Some(reg) = parse_reg(&node) {
                *regs.entry(reg.node).or_insert(0) += 1;
                continue;
            }
            if let ast::Expr {node: ExprKind::Binary(ast::BinOp {node: ast::BinOpKind::Mul, ..}, ref left, ref right), ..} = node {

                if let Some(reg) = parse_reg(&**left) {
                    if let ast::Expr {node: ExprKind::Lit(ref scale), ..} = **right {
                        if let ast::LitKind::Int(value, _) = scale.node {
                            *regs.entry(reg.node).or_insert(0) += value as isize;
                            continue;
                        }
                    }
                } else if let Some(reg) = parse_reg(&**right) {
                    if let ast::Expr {node: ExprKind::Lit(ref scale), ..} = **left {
                        if let ast::LitKind::Int(value, _) = scale.node {
                            *regs.entry(reg.node).or_insert(0) += value as isize;
                            continue;
                        }
                    }
                }
            }
            immediates.push(node);
        }

        // scale, index, base
        if regs.len() > 2 {
            ecx.span_err(span, "Invalid memory reference: too many registers");
            return Ok(Arg::Invalid);
        }
        let mut drain = regs.drain();
        let (index, scale, base) = match (drain.next(), drain.next()) {
            (None,                  None)                 => (None, 0, None),
            (Some((index, scale)),  None)                 | 
            (None,                  Some((index, scale))) | 
            (Some((index, scale)),  Some((_, 0)))         | 
            (Some((_, 0)),          Some((index, scale))) => if scale == 1 {(None, 0, Some(index))} else {(Some(index), scale, None)},
            (Some((base, 1)),       Some((index, scale))) |
            (Some((index, scale)),  Some((base, 1)))      => (Some(index), scale, Some(base)),
            _ => {
                ecx.span_err(span, "Invalid memory reference: only one register can be scaled");
                return Ok(Arg::Invalid);
            }
        };

        // reconstruct immediates
        let mut immediates = immediates.drain(..);
        let disp = if let Some(disp) = immediates.next() {
            let mut disp = P(disp);
            for immediate in immediates {
                disp = ecx.expr_binary(span, ast::BinOpKind::Add, disp, P(immediate));
            }
            Some(disp)
        } else {
            None
        };

        // assemble the memory location
        return Ok(Arg::Indirect(MemoryRef {
            index: index,
            scale: scale,
            base:  base,
            disp:  disp,
            size:  size,
            span:  span
        }));
    }

    // immediate
    Ok(Arg::Immediate(P(arg), size))
}

fn as_simple_name(expr: &ast::Expr) -> Option<Ident> {
    let path = match expr {
        &ast::Expr {node: ast::ExprKind::Path(None, ref path) , ..} => path,
        _ => return None
    };

    if path.global || path.segments.len() != 1 {
        return None;
    }

    let segment = &path.segments[0];
    if !segment.parameters.is_empty() {
        return None;
    }

    Some(Ident {node: segment.identifier, span: path.span})
}

fn parse_reg(expr: &ast::Expr) -> Option<Spanned<Register>> {
    let path = if let Some(path) = as_simple_name(expr) {
        path
    } else {
        return None
    };

    use self::Register::*;
    let reg = match &*path.node.name.as_str() {
        "rax"|"r0" => RAX, "rcx"|"r1" => RCX, "rdx"|"r2" => RDX, "rbx"|"r3" => RBX,
        "rsp"|"r4" => RSP, "rbp"|"r5" => RBP, "rsi"|"r6" => RSI, "rdi"|"r7" => RDI,
        "r8"       => R8,  "r9"       => R9,  "r10"      => R10, "r11"      => R11,
        "r12"      => R12, "r13"      => R13, "r14"      => R14, "r15"      => R15,

        "eax"|"r0d" => EAX, "ecx"|"r1d" => ECX, "edx"|"r2d" => EDX, "ebx"|"r3d" => EBX,
        "esp"|"r4d" => ESP, "ebp"|"r5d" => EBP, "esi"|"r6d" => ESI, "edi"|"r7d" => EDI,
        "r8d"       => R8D, "r9d"       => R9D, "r10d"      => R10D,"r11d"      => R11D,
        "r12d"      => R12D,"r13d"      => R13D,"r14d"      => R14D,"r15d"      => R15D,

        "ax"|"r0w" => AX,  "cx"|"r1w" => CX,  "dx"|"r2w" => DX,  "bx"|"r3w" => BX, 
        "sp"|"r4w" => SP,  "bp"|"r5w" => BP,  "si"|"r6w" => SI,  "di"|"r7w" => DI, 
        "r8w"      => R8W, "r9w"      => R9W, "r10w"     => R10W,"r11w"     => R11W,
        "r12w"     => R12W,"r13w"     => R13W,"r14w"     => R14W,"r15w"     => R15W,

        "al"|"r0b" => AL,  "cl"|"r1b" => CL,  "dl"|"r2b" => DL,  "bl"|"r3b" => BL, 
        "spl"      => SPL, "bpl"      => BPL, "sil"      => SIL, "dil"      => DIL,
        "r8b"      => R8B, "r9b"      => R9B, "r10b"     => R10B,"r11b"     => R11B,
        "r12b"     => R12B,"r13b"     => R13B,"r14b"     => R14B,"r15b"     => R15B,

        // not encodable yet: AH, CH, DH, BH

        "rip"  => RIP,
        _ => return None
    };

    Some(Spanned {
        node: reg,
        span: path.span
    })
}

fn parse_adds(node: ast::Expr, collection: &mut Vec<ast::Expr>) {
    if let ast::Expr{node: ast::ExprKind::Binary(ast::BinOp {node: ast::BinOpKind::Add, ..}, left, right), ..} = node {
        parse_adds(left.unwrap(), collection);
        parse_adds(right.unwrap(), collection);
    } else {
        collection.push(node);
    }
}
