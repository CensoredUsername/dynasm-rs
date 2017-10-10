use syntax::ext::build::AstBuilder;
use syntax::ext::base::ExtCtxt;
use syntax::parse::parser::{Parser, PathStyle};
use syntax::parse::token;
use syntax::parse::PResult;
use syntax::ast;
use syntax::ptr::P;
use syntax::codemap::{Spanned, Span};

use std::cmp::PartialEq;

use ::State;
use serialize::{Size, Ident};

/**
 * collections
 */

#[derive(Debug)]
pub struct Instruction(pub Vec<Ident>, pub Vec<Arg>, pub Span);

#[derive(Debug)]
pub enum Arg {
    // unprocessed typemapped argument
    TypeMappedRaw {
        span: Span,
        base_reg: Register,
        scale: ast::Path,
        value_size: Option<Size>,
        nosplit: bool,
        disp_size: Option<Size>,
        scaled_items: Vec<MemoryRefItem>,
        attribute: Option<ast::Ident>,
    },
    // unprocessed memory reference argument
    IndirectRaw {
        span: Span,
        value_size: Option<Size>,
        nosplit: bool,
        disp_size: Option<Size>,
        items: Vec<MemoryRefItem>,
    },
    Indirect(MemoryRef), // indirect memory reference supporting scale, index, base and displacement.
    Direct(Spanned<Register>), // a bare register (rax, ...)
    JumpTarget(JumpType, Option<Size>), // jump target.
    IndirectJumpTarget(JumpType, Option<Size>), // indirect jump target i.e. rip-relative displacement
    Immediate(P<ast::Expr>, Option<Size>), // an expression that evaluates to a value. basically, anything that ain't the other three
    Invalid // placeholder value
}

#[derive(Debug)]
pub enum MemoryRefItem {
    ScaledRegister(Register, isize),
    Register(Register),
    Displacement(P<ast::Expr>)
}

#[derive(Debug)]
pub struct MemoryRef {
    pub span:       Span,
    pub size:       Option<Size>,
    pub nosplit:    bool,
    pub disp_size:  Option<Size>,
    pub base:       Option<Register>,
    pub index:      Option<(Register, isize, Option<P<ast::Expr>>)>,
    pub disp:       Option<P<ast::Expr>>
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
    Dynamic(RegFamily, P<ast::Expr>)
}

// this map identifies the different registers that exist. some of these can be referred to as different sizes
// but they share the same ID here (think AL/AX/EAX/RAX, XMM/YMM)
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RegId {
    // size: 1, 2, 4 or 8 bytes
    RAX = 0x00, RCX = 0x01, RDX = 0x02, RBX = 0x03,
    RSP = 0x04, RBP = 0x05, RSI = 0x06, RDI = 0x07,
    R8  = 0x08, R9  = 0x09, R10 = 0x0A, R11 = 0x0B,
    R12 = 0x0C, R13 = 0x0D, R14 = 0x0E, R15 = 0x0F,

    // size: 8 bytes
    RIP = 0x15,

    // size: 1 byte
    AH = 0x24, CH = 0x25, DH = 0x26, BH = 0x27,

    // size: 10 bytes
    ST0 = 0x30, ST1 = 0x31, ST2 = 0x32, ST3 = 0x33,
    ST4 = 0x34, ST5 = 0x35, ST6 = 0x36, ST7 = 0x37,

    // size: 8 bytes. alternative encoding exists
    MMX0 = 0x40, MMX1 = 0x41, MMX2 = 0x42, MMX3 = 0x43,
    MMX4 = 0x44, MMX5 = 0x45, MMX6 = 0x46, MMX7 = 0x47,

    // size: 16 bytes or 32 bytes
    XMM0  = 0x50, XMM1  = 0x51, XMM2  = 0x52, XMM3  = 0x53,
    XMM4  = 0x54, XMM5  = 0x55, XMM6  = 0x56, XMM7  = 0x57,
    XMM8  = 0x58, XMM9  = 0x59, XMM10 = 0x5A, XMM11 = 0x5B,
    XMM12 = 0x5C, XMM13 = 0x5D, XMM14 = 0x5E, XMM15 = 0x5F,

    // size: 2 bytes. alternative encoding exists
    ES = 0x60, CS = 0x61, SS = 0x62, DS = 0x63,
    FS = 0x64, GS = 0x65,

    // size: 4 bytes
    CR0  = 0x70, CR1  = 0x71, CR2  = 0x72, CR3  = 0x73,
    CR4  = 0x74, CR5  = 0x75, CR6  = 0x76, CR7  = 0x77,
    CR8  = 0x78, CR9  = 0x79, CR10 = 0x7A, CR11 = 0x7B,
    CR12 = 0x7C, CR13 = 0x7D, CR14 = 0x7E, CR15 = 0x7F,

    // size: 4 bytes
    DR0  = 0x80, DR1  = 0x81, DR2  = 0x82, DR3  = 0x83,
    DR4  = 0x84, DR5  = 0x85, DR6  = 0x86, DR7  = 0x87,
    DR8  = 0x88, DR9  = 0x89, DR10 = 0x8A, DR11 = 0x8B,
    DR12 = 0x8C, DR13 = 0x8D, DR14 = 0x8E, DR15 = 0x8F,

    // size: 16 bytes
    BND0 = 0x90, BND1 = 0x91, BND2 = 0x92, BND3 = 0x93
}

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone, Copy)]
pub enum RegFamily {
    LEGACY = 0,
    RIP = 1,
    HIGHBYTE = 2,
    FP = 3,
    MMX = 4,
    XMM = 5,
    SEGMENT = 6,
    CONTROL = 7,
    DEBUG = 8,
    BOUND = 9
}

/*
 * impls
 */

impl Register {
    pub fn new_static(size: Size, id: RegId) -> Register {
        Register {size: size, kind: RegKind::Static(id) }
    }

    pub fn new_dynamic(size: Size, family: RegFamily, id: P<ast::Expr>) -> Register {
        Register {size: size, kind: RegKind::Dynamic(family, id) }
    }

    pub fn size(&self) -> Size {
        self.size
    }
}

impl RegKind {
    pub fn code(&self) -> Option<u8> {
        match *self {
            RegKind::Static(code) => Some(code.code()),
            RegKind::Dynamic(_, _) => None
        }
    }

    pub fn family(&self) -> RegFamily {
        match *self {
            RegKind::Static(code) => code.family(),
            RegKind::Dynamic(family, _) => family
        }
    }

    pub fn is_dynamic(&self) -> bool {
        match *self {
            RegKind::Static(_) => false,
            RegKind::Dynamic(_, _) => true
        }
    }

    pub fn is_extended(&self) -> bool {
        match self.family() {
            RegFamily::LEGACY  |
            RegFamily::XMM     |
            RegFamily::CONTROL |
            RegFamily::DEBUG   => self.code().unwrap_or(8) > 7,
            _ => false
        }
    }

    pub fn encode(&self) -> u8 {
        self.code().unwrap_or(0)
    }

    pub fn from_number(id: u8) -> RegKind {
        RegKind::Static(RegId::from_number(id))
    }
}

impl PartialEq<Register> for Register {
    fn eq(&self, other: &Register) -> bool {
        if self.size == other.size {
            if let RegKind::Static(code) = self.kind {
                if let RegKind::Static(other_code) = other.kind {
                    return code == other_code
                }
            }
        }
        return false;
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
            RegKind::Dynamic(_, _) => false
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

    pub fn family(&self) -> RegFamily {
        match *self as u8 >> 4 {
            0 => RegFamily::LEGACY,
            1 => RegFamily::RIP,
            2 => RegFamily::HIGHBYTE,
            3 => RegFamily::FP,
            4 => RegFamily::MMX,
            5 => RegFamily::XMM,
            6 => RegFamily::SEGMENT,
            7 => RegFamily::CONTROL,
            8 => RegFamily::DEBUG,
            9 => RegFamily::BOUND,
            _ => unreachable!()
        }
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
            _ => panic!("invalid register code {:?}", id)
        }
    }
}

/*
 * Code
 */

// tokentree is a list of tokens and delimited lists of tokens.
// this means we don't have to figure out nesting via []'s by ourselves.
// syntax for a single op: PREFIX* ident (SIZE? expr ("," SIZE? expr)*)? ";"

pub fn parse_instruction<'a>(state: &mut State, ecx: &ExtCtxt, parser: &mut Parser<'a>) -> PResult<'a, Instruction> {
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
        args.push(parse_arg(state, ecx, parser)?);

        while parser.eat(&token::Comma) {
            args.push(parse_arg(state, ecx, parser)?);
        }
    }

    let span = startspan.with_hi(parser.prev_span.hi());

    ops.push(op);
    Ok(Instruction(ops, args, span))
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
        token::Ident(ast::Ident {ref name, ..}) if &*name.as_str() == kw => (),
        _ => return false
    }
    parser.bump();
    true
}

fn parse_ident_or_rust_keyword<'a>(parser: &mut Parser<'a>) -> PResult<'a, ast::Ident> {
    if let token::Ident(i) = parser.token {
        parser.bump();
        Ok(i)
    } else {
        // technically we could generate the error here directly, but
        // that way this error branch could diverge in behaviour from
        // the normal parse_ident.
        parser.parse_ident()
    }
}

fn parse_arg<'a>(state: &mut State, ecx: &ExtCtxt, parser: &mut Parser<'a>) -> PResult<'a, Arg> {
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
                Arg::IndirectJumpTarget($jump, $size)
            } else {
                Arg::JumpTarget($jump, $size)
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

        let items = parse_adds(state, ecx, expr);

        // assemble the memory location
        return Ok(Arg::IndirectRaw {
            span: span,
            value_size: size,
            nosplit: nosplit,
            disp_size: disp_size,
            items: items
        });
    }

    // it's a normal (register/immediate/typemapped) operand
    parser.parse_expr()?.and_then(|arg| {
        // typemapped
        if parser.eat(&token::FatArrow) {
            let base = parse_reg(state, &arg);
            let base = if let Some(base) = base { base } else {
                ecx.span_err(arg.span, "Expected register");
                return Ok(Arg::Invalid);
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
                items = parse_adds(state, ecx, index_expr);
            } else {
                items = Vec::new();
            }

            let attr = if parser.eat(&token::Dot) {
                Some(parser.parse_ident()?)
            } else {
                None
            };

            return Ok(Arg::TypeMappedRaw {
                span: start.with_hi(parser.prev_span.hi()),
                base_reg: base.node,
                scale: ty,
                value_size: size,
                nosplit: nosplit,
                disp_size: disp_size,
                scaled_items: items,
                attribute: attr,
            });
        }

        // direct register reference
        if let Some(reg) = parse_reg(state, &arg) {
            if size.is_some() {
                ecx.span_err(arg.span, "size hint with direct register");
            }
            return Ok(Arg::Direct(reg))
        }

        // immediate
        Ok(Arg::Immediate(P(arg), size))
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

    Some(Ident {node: segment.identifier, span: path.span})
}

fn parse_reg(state: &State, expr: &ast::Expr) -> Option<Spanned<Register>> {
    if let Some(path) = as_simple_name(expr) {
        // static register names

        let mut name = &*path.node.name.as_str();
        if let Some(x) = state.crate_data.aliases.get(name) {
            name = x;
        }

        use self::RegId::*;
        use serialize::Size::*;
        let (reg, size) = match name {
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

            "rip"  => (RIP, QWORD),

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

        let (size, family) = match &*called.node.name.as_str() {
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
        };

        Some(Spanned {
            node: Register::new_dynamic(size, family, args[0].clone()),
            span: span
        })
    } else {
        None
    }
}

fn parse_adds(state: &State, ecx: &ExtCtxt, expr: P<ast::Expr>) -> Vec<MemoryRefItem> {
    use syntax::ast::ExprKind;

    let mut adds = Vec::new();
    collect_adds(ecx, expr, &mut adds);

    let mut items = Vec::new();

    // detect what kind of equation we're dealing with
    for node in adds {
        // simple reg
        if let Some(Spanned {node: reg, ..} ) = parse_reg(state, &node) {
            items.push(MemoryRefItem::Register(reg));
            continue;
        }
        if let ast::Expr {node: ExprKind::Binary(ast::BinOp {node: ast::BinOpKind::Mul, ..}, ref left, ref right), ..} = *node {
            // reg * const
            if let Some(Spanned {node: reg, ..} ) = parse_reg(state, left) {
                if let ast::Expr {node: ExprKind::Lit(ref scale), ..} = **right {
                    if let ast::LitKind::Int(value, _) = scale.node {
                        items.push(MemoryRefItem::ScaledRegister(reg, value as isize));
                        continue;
                    }
                }
            } // const * reg
            if let Some(Spanned {node: reg, ..} ) = parse_reg(state, right) {
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
