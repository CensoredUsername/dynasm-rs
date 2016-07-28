use syntax::ext::build::AstBuilder;
use syntax::ext::base::ExtCtxt;
use syntax::parse::parser::Parser;
use syntax::parse::token;
use syntax::parse::PResult;
use syntax::ast;
use syntax::ptr::P;
use syntax::codemap::{Spanned, Span};

use std::collections::HashMap;
use std::cmp::PartialEq;

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
    IndirectJumpTarget(JumpType, Option<Size>), // indirect jump target i.e. rip-relative displacement
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
    Global(Ident),         // -> label
    Backward(Ident),       //  > label
    Forward(Ident),        //  < label
    Dynamic(P<ast::Expr>), // => expr
}

// encoding of this:
// lower byte indicates which register it is
// upper byte is used to indicate which size group it falls under.

#[derive(Debug, Clone)]
pub struct Register {
    pub size: Size,
    pub kind: RegKind
}

#[derive(Debug, Clone)]
pub enum RegKind {
    Static(RegId),
    Dynamic(P<ast::Expr>)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RegId {
    RAX = 0x00, RCX = 0x01, RDX = 0x02, RBX = 0x03,
    RSP = 0x04, RBP = 0x05, RSI = 0x06, RDI = 0x07,
    R8  = 0x08, R9  = 0x09, R10 = 0x0A, R11 = 0x0B,
    R12 = 0x0C, R13 = 0x0D, R14 = 0x0E, R15 = 0x0F,

    AH = 0x14, CH = 0x15, DH = 0x16, BH = 0x17,

    RIP = 0x25
}

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone, Copy)]
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
    pub fn new_static(size: Size, id: RegId) -> Register {
        Register {size: size, kind: RegKind::Static(id) }
    }

    pub fn new_dynamic(size: Size, id: P<ast::Expr>) -> Register {
        Register {size: size, kind: RegKind::Dynamic(id) }
    }

    pub fn size(&self) -> Size {
        self.size
    }
}

impl RegKind {
    pub fn code(&self) -> Option<u8> {
        match *self {
            RegKind::Static(code) => Some(code.code()),
            RegKind::Dynamic(_) => None
        }
    }

    pub fn is_dynamic(&self) -> bool {
        match *self {
            RegKind::Static(_) => false,
            RegKind::Dynamic(_) => true
        }
    }

    pub fn is_extended(&self) -> bool {
        self.code().unwrap_or(8) > 7
    }

    pub fn encode(&self) -> u8 {
        self.code().unwrap_or(0)
    }

    pub fn from_number(id: u8) -> RegKind {
        RegKind::Static(RegId::from_number(id))
    }
}

impl PartialEq<RegId> for Register {
    fn eq(&self, other: &RegId) -> bool {
        self.kind == *other
    }
}

impl PartialEq<RegId> for RegKind {
    fn eq(&self, other: &RegId) -> bool {
        match *self {
            RegKind::Static(id) => id == *other,
            RegKind::Dynamic(_) => false
        }
    }
}

// workarounds to mask an impl<A, B> PartialEq<B> for Option<A: PartialEq<B>>
impl PartialEq<RegId> for Option<Register> {
    fn eq(&self, other: &RegId) -> bool {
        match *self {
            Some(ref a) => a == other,
            None => false
        }
    }
}

impl PartialEq<RegId> for Option<RegKind> {
    fn eq(&self, other: &RegId) -> bool {
        match *self {
            Some(ref a) => a == other,
            None => false
        }
    }
}

impl RegId {
    pub fn code(&self) -> u8 {
        *self as u8 & 0xF
    }

    pub fn from_number(id: u8) -> RegId {
        match id {
            0  => RegId::RAX,
            1  => RegId::RCX,
            2  => RegId::RDX,
            3  => RegId::RBX,
            4  => RegId::RSP,
            5  => RegId::RBP,
            6  => RegId::RSI,
            7  => RegId::RDI,
            8  => RegId::R8,
            9  => RegId::R9,
            10 => RegId::R10,
            11 => RegId::R11,
            12 => RegId::R12,
            13 => RegId::R13,
            14 => RegId::R14,
            15 => RegId::R15,
            _ => panic!("invalid register code")
        }
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

        // possible prefix symbols: => (dynamic label), -> (global label), . (directive)

        if parser.eat(&token::FatArrow) {
            // dynamic label branch
            let expr = try!(parser.parse_expr());

            ins.push(Item::Label(LabelType::Dynamic(expr)));
            // note: we explicitly do not try to parse a : here as it is a valid symbol inside of an expression
            continue;

        } else if parser.eat(&token::RArrow) {
            // global label branch
            let name = Spanned {node: try!(parser.parse_ident()), span: startspan};

            ins.push(Item::Label(LabelType::Global(name)));
            try!(parser.expect(&token::Colon));
            continue;

        }

        let is_directive = parser.eat(&token::Dot);
        // parse the first part of an op or a label

        let mut span = parser.span;
        let mut op = Spanned {node: try!(parser.parse_ident()), span: span};

        // parse a colon indicating we were in a label

        if parser.eat(&token::Colon) {
            ins.push(Item::Label(LabelType::Local(op)));
            continue;
        }

        // if we're parsing an instruction, read prefixes

        let mut ops = Vec::new();
        if !is_directive {
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

        if is_directive {
            ins.push(Item::Directive(op, args, span));
        } else {
            ops.push(op);
            ins.push(Item::Instruction(ops, args, span));
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

    let in_bracket = parser.check(&token::OpenDelim(token::Bracket));
    if in_bracket && parser.look_ahead(1, |x| match x {
            &token::RArrow |
            &token::Gt     |
            &token::Lt     |
            &token::FatArrow => true,
            _ => false
        }) {
        parser.bump();
    }

    macro_rules! label_return {
        ($jump:expr, $size:expr) => {
            return Ok(if in_bracket {
                try!(parser.expect(&token::CloseDelim(token::Bracket)));
                Arg::IndirectJumpTarget($jump, $size)
            } else {
                Arg::JumpTarget($jump, $size)
            });
        }
    }

    // global label
    if parser.eat(&token::RArrow) {
        let name = try!(parser.parse_ident());
        let jump = JumpType::Global(
            Ident {node: name, span: Span {hi: parser.last_span.hi, ..start} }
        );
        label_return!(jump, size);
    // forward local label
    } else if parser.eat(&token::Gt) {
        let name = try!(parser.parse_ident());
        let jump = JumpType::Forward(
            Ident {node: name, span: Span {hi: parser.last_span.hi, ..start} }
        );
        label_return!(jump, size);
    // forward global label
    } else if parser.eat(&token::Lt) {
        let name = try!(parser.parse_ident());
        let jump = JumpType::Backward(
            Ident {node: name, span: Span {hi: parser.last_span.hi, ..start} }
        );
        label_return!(jump, size);
    // dynamic label
    } else if parser.eat(&token::FatArrow) {
        let id = try!(parser.parse_expr());
        let jump = JumpType::Dynamic(id);
        label_return!(jump, size);
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

        // as dynamic regs aren't hashable (and we don't combine them), we count them separate at first
        let mut regs: Vec<(Register, isize)> = Vec::new();
        let mut static_regs: HashMap<(RegId, Size), isize> = HashMap::new();
        let mut immediates = Vec::new();

        // static reg combiner. we do not combine dynamic regs as the equation used to construct them might have side effects.
        for node in added {
            // simple reg
            if let Some(Spanned {node: reg, ..} ) = parse_reg(&node) {
                match reg.kind {
                    RegKind::Static(id) => *static_regs.entry((id, reg.size)).or_insert(0) += 1 as isize,
                    RegKind::Dynamic(_) => regs.push((reg, 1))
                }
                continue;
            }
            if let ast::Expr {node: ExprKind::Binary(ast::BinOp {node: ast::BinOpKind::Mul, ..}, ref left, ref right), ..} = node {
                // reg * const
                if let Some(Spanned {node: reg, ..} ) = parse_reg(&**left) {
                    if let ast::Expr {node: ExprKind::Lit(ref scale), ..} = **right {
                        if let ast::LitKind::Int(value, _) = scale.node {
                            match reg.kind {
                                RegKind::Static(id) => *static_regs.entry((id, reg.size)).or_insert(0) += value as isize,
                                RegKind::Dynamic(_) => regs.push((reg, value as isize))
                            }
                            continue;
                        }
                    }
                // const * reg
                } else if let Some(Spanned {node: reg, ..} ) = parse_reg(&**right) {
                    if let ast::Expr {node: ExprKind::Lit(ref scale), ..} = **left {
                        if let ast::LitKind::Int(value, _) = scale.node {
                            match reg.kind {
                                RegKind::Static(id) => *static_regs.entry((id, reg.size)).or_insert(0) += value as isize,
                                RegKind::Dynamic(_) => regs.push((reg, value as isize))
                            }
                            continue;
                        }
                    }
                }
            }
            immediates.push(node);
        }

        // flush combined static regs
        regs.extend(static_regs.drain().map(|x| {
            let ((id, size), amount) = x;
            (Register::new_static(size, id), amount)
        }));

        // can only have two regs at most
        if regs.len() > 2 {
            ecx.span_err(span, "Invalid memory reference: too many registers");
            return Ok(Arg::Invalid);
        }

        let mut drain = regs.drain(..);
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
    if let Some(path) = as_simple_name(expr) {
        // static register names
        use self::RegId::*;
        use self::Size::*;
        let (reg, size) = match &*path.node.name.as_str() {
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

            "ah" => (AH, BYTE), "ch" => (CH, BYTE), "dh" => (DH, BYTE), "bh" => (BH, BYTE),

            "rip"  => (RIP, QWORD),
            _ => return None
        };

        Some(Spanned {
            node: Register::new_static(size, reg),
            span: path.span
        })

    } else if let &ast::Expr {node: ast::ExprKind::Call(ref called, ref args), span, ..} = expr {
        // dynamically chosen registers
        if args.len() != 1 {
            return None;
        }

        let called = if let Some(called) = as_simple_name(called) {
            called
        } else {
            return None;
        };

        let size = match &*called.node.name.as_str() {
            "Rb" => Size::BYTE,
            "Rw" => Size::WORD,
            "Rd" => Size::DWORD,
            "Rq" => Size::QWORD,
            _ => return None
        };

        Some(Spanned {
            node: Register::new_dynamic(size, args[0].clone()),
            span: span
        })
    } else {
        None
    }
}

fn parse_adds(node: ast::Expr, collection: &mut Vec<ast::Expr>) {
    if let ast::Expr{node: ast::ExprKind::Binary(ast::BinOp {node: ast::BinOpKind::Add, ..}, left, right), ..} = node {
        parse_adds(left.unwrap(), collection);
        parse_adds(right.unwrap(), collection);
    } else {
        collection.push(node);
    }
}
