use std::ops::Deref;
use std::iter::Extend;

use syntax::ext::build::AstBuilder;
use syntax::ext::base::ExtCtxt;
use syntax::ast;
use syntax::ptr::P;
use syntax::codemap::Spanned;

use parser::{self, Item, Arg, Ident, MemoryRef, Register, Size};
use x64data::get_mnemnonic_data;

/*
 * Compilation output
 */

#[derive(Clone, Debug)]
pub struct StmtBuffer {
    buf: Vec<Stmt>,
    labels: Vec<(Ident, usize)>,
    pos: usize
}

impl StmtBuffer {
    pub fn new() -> StmtBuffer {
        StmtBuffer {buf: Vec::new(), labels: Vec::new(), pos: 0}
    }

    pub fn push(&mut self, s: Stmt) {
        self.pos += match s {
            Stmt::Const(_) => 1,
            Stmt::Var(_, s) => s.in_bytes() as usize
        };
        self.buf.push(s);
    }

    pub fn make_label(&mut self, ident: Ident) {
        self.labels.push((ident, self.pos));
    }

    pub fn into_vec(self) -> (Vec<Stmt>, Vec<(Ident, usize)>) {
        (self.buf, self.labels)
    }

    pub fn offset(&self) -> usize {
        self.pos
    }
}

impl Deref for StmtBuffer {
    type Target = Vec<Stmt>;

    fn deref(&self) -> &Vec<Stmt> {
        &self.buf
    }
}

impl Extend<Stmt> for StmtBuffer {
    fn extend<T: IntoIterator<Item=Stmt>>(&mut self, iter: T) {
        for i in iter {
            self.push(i)
        }
    }
}

#[derive(Clone, Debug)]
pub enum Stmt {
    Const(u8),
    Var(P<ast::Expr>, Size)
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
    pub const MISMATCHING_SIZES : u16 = 0x0040; // the operand sizes of the opcodes don't match up
    pub const EAX_ONLY          : u16 = 0x0080; // some instructions only operate on al/ax/eax
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
 * Instruction encoding macros
 */

macro_rules! rex {
    ($size:expr, $reg:expr, $ind:expr, $base:expr) => 
        (Stmt::Const(0x40 | (($size as u8 & 1) << 3) | (($reg as u8 & 0x8) >> 1) |
                            (($ind as u8 & 0x8) >> 2) | (($base as u8 & 0x8) >> 3)));

    ($size:expr, $reg:expr, $rm:expr) => 
        (Stmt::Const(0x40 | (($size as u8 & 1) << 3) | (($reg as u8 & 0x8) >> 1) | 
                            (($rm as u8 & 0x8) >> 3)));
}

const MOD_DIRECT: u8 = 0b11;
const MOD_NODISP: u8 = 0b00;
const MOD_DISP8:  u8 = 0b01;
const MOD_DISP32: u8 = 0b10;

macro_rules! modrm {
    ($mo:expr, $reg:expr, $rm:expr) => 
        (Stmt::Const(($rm as u8 & 7) | (($reg as u8 & 7) << 3) | ($mo << 6)));
}

macro_rules! sib {
    ($sc:expr, $ind:expr, $base:expr) => 
        (Stmt::Const(($base as u8 & 7) | (($ind as u8 & 7) << 3) | (($sc as u8) << 6)));
}

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
            Item::Label(ident) => stmts.make_label(ident)
        }
    }

    if successful {
        Ok(stmts)
    } else {
        Err(())
    }
}

fn compile_op(ecx: &ExtCtxt, buffer: &mut StmtBuffer, op: Ident, prefixes: Vec<Ident>, mut args: Vec<Arg>) -> Result<(), Option<String>> {
    // this call also inserts more size information in the AST if applicable.
    let data = try!(match_op_format(ecx, op, &mut args));

    // determine operand size if not overridden
    let mut op_size = if (data.flags & flags::MISMATCHING_SIZES) == 0 {
        try!(get_operand_size(ecx, &args))
    } else {
        Size::DWORD
    };

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
    let prefixes = try!(get_legacy_prefixes(ecx, data, prefixes, op_size));
    buffer.extend(prefixes.into_iter().filter_map(|x| x.clone()));

    let (reg, rm, args) = extract_args(data, args);

    // do we need to encode a REX byte
    let need_rexsize = op_size == Size::QWORD || (data.flags & flags::REQUIRES_REXSIZE) != 0;

    // no register args
    if rm.is_none() {

        // rex prefix
        if need_rexsize {
            buffer.push(rex!(need_rexsize, 0, 0, 0));
        }
        // opcode
        buffer.extend(data.ops.iter().map(|x| Stmt::Const(*x)));

    // register in opcode byte
    } else if (data.flags & flags::REGISTER_IN_OPCODE) != 0 {

        let regcode = if let Some(Arg::Direct(reg)) = rm {
            try!(guard_impossible_regs(ecx, data, reg))
        } else {
            panic!("bad encoding data, expected register");
        };

        // rex prefix
        if need_rexsize || regcode > 7 {
            buffer.push(rex!(need_rexsize, regcode, 0));
        }
        // opcode
        let (last, rest) = data.ops.split_last().expect("bad encoding data, no opcode specified");
        buffer.extend(rest.into_iter().map(|x| Stmt::Const(*x)));
        buffer.push(Stmt::Const(last + (regcode & 7)));

    // ModRM byte
    } else if let Some(Arg::Direct(rm)) = rm {

        let regcode = if let Some(Arg::Direct(reg)) = reg {
            try!(guard_impossible_regs(ecx, data, reg))
        } else { // reg is given by the instruction encoding
            data.reg
        };
        let rmcode = try!(guard_impossible_regs(ecx, data, rm));

        // rex prefix
        if need_rexsize || regcode > 7 || rmcode > 7 {
            buffer.push(rex!(need_rexsize, regcode, rmcode));
        }
        // opcode
        buffer.extend(data.ops.iter().map(|x| Stmt::Const(*x)));
        // ModRM
        buffer.push(modrm!(MOD_DIRECT, regcode, rmcode));

    // ModRM and SIB byte
    } else if let Some(Arg::Indirect(mut mem)) = rm {

        let regcode = if let Some(Arg::Direct(reg)) = reg {
            try!(guard_impossible_regs(ecx, data, reg))
        } else { // reg is given by the instruction encoding
            data.reg
        };

        // detect impossible to encode memoryrefs. also stops RIP in odd places.
        try!(sanitize_memoryref(ecx, &mut mem));

        // TODO: if the arg is constant we should be able to optimize to MOD_DISP8.

        // RBP can only be encoded as base if a displacement is present.
        let mode = if mem.disp == None && mem.base == Some(Register::RBP) {
            MOD_DISP8
        // mode_nodisp has to be selected if RIP is encoded, or if no base is to be encoded. note that in these scenarions the disp should actually be encoded
        } else if mem.disp == None || mem.base == Some(Register::RIP) || mem.base == None {
            MOD_NODISP
        } else {
            MOD_DISP32
        };

        // if there's an index we need to escape into the SIB byte
        if let Some(index) = mem.index {

            let indexcode = index.code();
            // to encode the lack of a base we encode RBP
            let basecode = if let Some(base) = mem.base {
                base
            } else {
                Register::RBP
            }.code();

            // rex prefix
            if need_rexsize || regcode > 7 || indexcode > 7 || basecode > 7 {
                buffer.push(rex!(need_rexsize, regcode, indexcode, basecode));
            }
            // opcode
            buffer.extend(data.ops.iter().map(|x| Stmt::Const(*x)));
            // ModRM
            buffer.push(modrm!(mode, regcode, Register::RSP.code()));
            // SIB
            buffer.push(sib!(mem.scale, indexcode, basecode));

        // no index, only a base.
        } else if let Some(base) = mem.base {

            // RBP at MOD_NODISP is used to encode RBP
            let basecode = if base == Register::RIP {
                Register::RBP
            } else {
                base
            }.code();

            // rex prefix
            if need_rexsize || regcode > 7 || basecode > 7 {
                buffer.push(rex!(need_rexsize, regcode, basecode));
            }
            // opcode
            buffer.extend(data.ops.iter().map(|x| Stmt::Const(*x)));
            // ModRM
            buffer.push(modrm!(mode, regcode, basecode));

        } else {

            let basecode = Register::RBP.code();
            let indexcode = Register::RSP.code();
            // no base, no index. only disp
            if need_rexsize || regcode > 7 || indexcode > 7 || basecode > 7 {
                buffer.push(rex!(need_rexsize, regcode, indexcode, basecode));
            }
            // opcode
            buffer.extend(data.ops.iter().map(|x| Stmt::Const(*x)));
            // ModRM
            buffer.push(modrm!(mode, regcode, basecode));
            // SIB
            buffer.push(sib!(0, indexcode, basecode));

        }

        // Disp
        if let Some(disp) = mem.disp {
            buffer.push(Stmt::Var(disp, Size::DWORD));
        } else if mem.base == None || mem.base == Some(Register::RIP) {
            buffer.push(Stmt::Const(0));
            buffer.push(Stmt::Const(0));
            buffer.push(Stmt::Const(0));
            buffer.push(Stmt::Const(0));
        } else if mem.base == Some(Register::RBP) {
            buffer.push(Stmt::Const(0));
        }
    }

    // immediates
    for arg in args {
        let stmt = match arg {
            Arg::Immediate(expr, Some(size), false) => Stmt::Var(expr, size),
            Arg::Immediate(expr, None,       false) => Stmt::Var(expr, if op_size != Size::QWORD {op_size} else {Size::DWORD}),
            Arg::Immediate(expr, size,       true)  => {
                let size = if let Some(size) = size {
                    size
                } else if op_size != Size::QWORD {
                    op_size
                } else {
                    Size::DWORD
                };

                // we can safely do this as there's a guarantee of only one immediate if it's a code offset
                let offset = buffer.offset() + size.in_bytes() as usize;
                let span = expr.span;

                // generate the expression to subtract this offset
                let value = ast::LitKind::Int(offset as u64, ast::LitIntType::Unsuffixed);
                let expr = ecx.expr_binary(span, ast::BinOpKind::Sub, expr, ecx.expr_lit(span, value));

                Stmt::Var(expr, size)
            },
            _ => continue 
        };
        buffer.push(stmt);
    }

    Ok(())
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

fn guard_impossible_regs(ecx: &ExtCtxt, fmt: &'static Opdata, reg: Spanned<Register>) -> Result<u8, Option<String>> {
    if reg.node == Register::RIP {
        ecx.span_err(reg.span, "'rip' can only be used as a memory offset");
        Err(None)
    } else if (fmt.flags & flags::EAX_ONLY) != 0 && !(reg.node == Register::AL || 
                                                      reg.node == Register::AX ||
                                                      reg.node == Register::EAX) {
        ecx.span_err(reg.span, "this instruction only allows AL, AX or EAX to be used as argument");
        Err(None)
    } else {
        Ok(reg.node.code())
    }
}

fn sanitize_memoryref(ecx: &ExtCtxt, mem: &mut MemoryRef) -> Result<(), Option<String>> {
    // sort out impossible scales
    mem.scale = match (mem.scale, mem.base) {
        (0, _   ) => 0, // no index
        (1, _   ) => 0,
        (2, _   ) => 1,
        (4, _   ) => 2,
        (8, _   ) => 3,
        (3, ref mut base @ None) => {
            *base = mem.index;
            1
        },
        (5, ref mut base @ None) => {
            *base = mem.index;
            2
        },
        (9, ref mut base @ None) => {
            *base = mem.index;
            3
        },
        (scale, _) => {
            ecx.span_err(mem.span, &format!("Scale '{}' cannot be encoded", scale));
            return Err(None);
        }
    };

    // RSP as index field can not be represented.
    if mem.index == Some(Register::RSP) {
        // as we always fill the base field first this is impossible to satisfy
        ecx.span_err(mem.span, "'rsp' cannot be used as index field");
        return Err(None);
    }

    // RSP as base without index (add an index so we escape into SIB)
    if mem.base == Some(Register::RSP) && mem.index.is_none() {
        mem.index = Some(Register::RSP);
        mem.scale = 1;
    }

    // RIP as index
    if mem.index == Some(Register::RIP) {
        ecx.span_err(mem.span, "'rip' cannot be used as index");
        return Err(None);
    }

    // RIP as base with index
    if mem.index == Some(Register::RIP) && mem.index.is_some() {
        ecx.span_err(mem.span, "'rip' cannot be used as base when an index is to be encoded");
        return Err(None);
    }

    // RBP as base field just requires a mandatory MOD_DISP8. we don't sort that out here.
    // same for no base, as this requires a MOD_DISP32
    Ok(())
}

fn get_operand_size(ecx: &ExtCtxt, args: &[Arg]) -> Result<Size, Option<String>> {
    // determine operand size.
    // ensures that all operands have the same size, and that the immediate size is smaller or equal.
    // if no operands are present, the immediate size is used. if no immediates are present, the default size is used

    let mut op_size = None;
    let mut im_size = None;
    let mut has_immediate = false;
    let mut has_args = false;
    let mut has_code_offset = false;

    for arg in args {
        match *arg {
            Arg::Direct(Spanned {node: reg, span}) => {
                has_args = true;
                let size = reg.size();
                if op_size.is_some() && op_size != Some(size) {
                    ecx.span_err(span, "Conflicting operand sizes");
                    return Err(None);
                } else {
                    op_size = Some(size)
                }
            },
            Arg::Indirect(MemoryRef {size, span, ..}) => {
                has_args = true;
                if let Some(size) = size {
                    if op_size.is_some() && op_size != Some(size) {
                        ecx.span_err(span, "Conflicting operand sizes");
                        return Err(None);
                    } else {
                        op_size = Some(size);
                    }
                }
            },
            Arg::Immediate(_, size, relative) => {
                if relative {
                    if has_immediate {
                        panic!("multiple code offsets in format string");
                    }
                    has_code_offset = true;
                } else if has_code_offset {
                    panic!("multiple code offsets in format string");
                }
                has_immediate = true;
                if let Some(size) = size {
                    if im_size.is_none() || im_size.unwrap() < size  {
                        im_size = Some(size);
                    }
                }
            },
            Arg::Invalid => unreachable!()
        }
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
        return Err(Some(format!("Unkonwn argument size")));
    } else if has_immediate {
        return Err(Some(format!("Unknown immediate size")));
    } else {
        Ok(Size::DWORD)
    }
}

fn get_legacy_prefixes(ecx: &ExtCtxt, fmt: &'static Opdata, idents: Vec<Ident>, op_size: Size) -> Result<[Option<Stmt>; 4], Option<String>> {
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

    if (fmt.flags & flags::REQUIRES_ADDRSIZE) != 0 {
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
                ('i', &Arg::Immediate(_, size, _)) |
                ('c', &Arg::Immediate(_, size, _)) => size,
                ('r', &Arg::Direct(Spanned {node: ref reg, ..} )) |
                ('v', &Arg::Direct(Spanned {node: ref reg, ..} )) => Some(reg.size()),
                ('m', &Arg::Indirect(MemoryRef {size, ..} )) |
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
        while let Some(code) = fmt.next() {
            let fsize = fmt.next().unwrap();
            let arg: &mut Arg = args.next().unwrap();

            match *arg {
                Arg::Immediate(_, ref mut size @ None, _) |
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

            // make any code offsets relative
            if 'c' == code {
                if let &mut Arg::Immediate(_, _, ref mut relative) = arg {
                    *relative = true
                }
            }
        }
    }

    Ok(())
}
