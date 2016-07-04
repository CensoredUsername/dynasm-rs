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

#[derive(Debug)]
pub enum Item {
    Instruction(Vec<Ident>, Vec<Arg>, Span),
    Label(Ident)
}

#[derive(Debug)]
pub enum Arg {
    Indirect(MemoryRef), // indirect memory reference supporting scale, index, base and displacement.
    Direct(Spanned<Register>), // a bare register (rax, ...)
    Immediate(P<ast::Expr>, Option<SizeHint>), // an expression that evaluates to a value. basically, anything that ain't the other two.
    Invalid // placeholder value
}

#[derive(Debug)]
pub struct MemoryRef {
    pub index: Option<Register>,
    pub scale: isize,
    pub base:  Option<Register>,
    pub disp:  Option<P<ast::Expr>>,
    pub size:  Option<SizeHint>,
    pub span:  Span
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

impl Register {
    pub fn size(&self) -> SizeHint {
        match (*self as u16) & 0x0300 {
            0x0000 => SizeHint::BYTE,
            0x0100 => SizeHint::WORD,
            0x0200 => SizeHint::DWORD,
            0x0300 => SizeHint::QWORD,
            _ => unreachable!()
        }
    }

    pub fn code(&self) -> u8 {
        *self as u8
    }
} 

#[derive(PartialOrd, PartialEq, Ord, Eq, Debug, Clone, Copy)]
pub enum SizeHint {
    BYTE  = 0,
    WORD  = 1,
    DWORD = 2,
    QWORD = 3
}

// tokentree is a list of tokens and delimited lists of tokens.
// this means we don't have to figure out nesting via []'s by ourselves.
// syntax for a single op: PREFIX* ident (SIZE? expr ("," SIZE? expr)*)? ";"

pub fn parse<'a>(ecx: &ExtCtxt, parser: &'a mut Parser) -> PResult<'a, (Ident, Vec<Item>)> {
    let span = parser.span;
    let name = Spanned {node: try!(parser.parse_ident()), span: span};

    let mut ins = Vec::new();

    while !parser.check(&token::Eof) {

        try!(parser.expect(&token::Semi));

        let startspan = parser.span;

        // parse PREFIXES + 
        let mut span = parser.span;
        let mut op = Spanned {node: try!(parser.parse_ident()), span: span};
        let mut ops = Vec::new();
        while is_prefix(op) {
            ops.push(op);
            span = parser.span;
            op = Spanned {node: try!(parser.parse_ident()), span: span};
        }
        ops.push(op);

        // parse (expr),*
        let mut args = Vec::new();

        if !parser.check(&token::Semi) && !parser.check(&token::Eof) {
            let sizehint = eat_size_hint(parser);
            args.push((try!(parser.parse_expr()), sizehint));

            while parser.eat(&token::Comma) {
                let sizehint = eat_size_hint(parser);
                args.push((try!(parser.parse_expr()), sizehint));
            }
        }

        let args: Vec<Arg> = args.into_iter().map(|x| parse_arg(ecx, x.0, x.1)).collect();

        let span = Span {hi: parser.last_span.hi, ..startspan};

        ins.push(Item::Instruction(ops, args, span));
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

const SIZES:    [(&'static str, SizeHint); 4] = [
    ("BYTE", SizeHint::BYTE),
    ("WORD", SizeHint::WORD),
    ("DWORD", SizeHint::DWORD),
    ("QWORD", SizeHint::QWORD)
];
fn eat_size_hint(parser: &mut Parser) -> Option<SizeHint> {
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

fn parse_arg<'a>(ecx: &ExtCtxt, arg: P<ast::Expr>, size: Option<SizeHint>) -> Arg {
    use syntax::ast::ExprKind;

    let arg = arg.unwrap();

    // direct register reference
    if let Some(reg) = parse_reg(&arg) {
        if size.is_some() {
            ecx.span_err(arg.span, "size hint with direct register");
        }
        return Arg::Direct(reg)
    }

    // memory location
    if let ast::Expr {node: ExprKind::Vec(mut items), span, ..} = arg {
        if items.len() != 1 {
            ecx.span_err(span, "Comma in memory reference");
            return Arg::Invalid;
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
            return Arg::Invalid;
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
                return Arg::Invalid;
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
        return Arg::Indirect(MemoryRef {
            index: index,
            scale: scale,
            base:  base,
            disp:  disp,
            size:  size,
            span:  span
        });
    }

    // immediate
    Arg::Immediate(P(arg), size)
}

fn parse_reg(expr: &ast::Expr) -> Option<Spanned<Register>> {
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

    use self::Register::*;
    let reg = match &*segment.identifier.name.as_str() {
        "rax"  => RAX,  "rcx"  => RCX,  "rdx"  => RDX,  "rbx"  => RBX,
        "rsp"  => RSP,  "rbp"  => RBP,  "rsi"  => RSI,  "rdi"  => RDI,
        "r8"   => R8,   "r9"   => R9,   "r10"  => R10,  "r11"  => R11,
        "r12"  => R12,  "r13"  => R13,  "r14"  => R14,  "r15"  => R15,

        "eax"  => EAX,  "ecx"  => ECX,  "edx"  => EDX,  "ebx"  => EBX,
        "esp"  => ESP,  "ebp"  => EBP,  "esi"  => ESI,  "edi"  => EDI,
        "r8d"  => R8D,  "r9d"  => R9D,  "r10d" => R10D, "r11d" => R11D,
        "r12d" => R12D, "r13d" => R13D, "r14d" => R14D, "r15d" => R15D,

        "ax"   => AX,   "cx"   => CX,   "dx"   => DX,   "bx"   => BX, 
        "sp"   => SP,   "bp"   => BP,   "si"   => SI,   "di"   => DI, 
        "r8w"  => R8W,  "r9w"  => R9W,  "r10w" => R10W, "r11w" => R11W,
        "r12w" => R12W, "r13w" => R13W, "r14w" => R14W, "r15w" => R15W,

        "al"   => AL,   "cl"   => CL,   "dl"   => DL,   "bl"   => BL, 
        "spl"  => SPL,  "bpl"  => BPL,  "sil"  => SIL,  "dil"  => DIL,
        "r8b"  => R8B,  "r9b"  => R9B,  "r10b" => R10B, "r11b" => R11B,
        "r12b" => R12B, "r13b" => R13B, "r14b" => R14B, "r15b" => R15B,

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
