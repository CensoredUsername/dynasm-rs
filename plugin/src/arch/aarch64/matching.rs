
use proc_macro2::Span;

use super::Context;
use super::ast::{Instruction, RawArg, CleanArg, FlatArg, RefItem, Register, RegFamily, RefKind, Modifier};
use super::aarch64data::{Opdata, Matcher, COND_MAP, get_mnemonic_data};

use crate::common::{Size, JumpType, emit_error_at};
use crate::parse_helpers::{as_ident, as_number, as_float};

/// Try finding an appropriate definition that matches the given instruction / arguments. 
pub(super) fn match_instruction(_ctx: &mut Context, instruction: &Instruction, args: Vec<RawArg>) -> Result<MatchData, Option<String>> {
    // sanitize our arg list to remove any structures that cannot be matched on
    let args = sanitize_args(args)?;

    // get the possible matchers
    let name = instruction.ident.to_string();
    let opdata = if let Some(o) = get_mnemonic_data(&name) {
        o
    } else {
        return Err(Some(format!("Unknown instruction mnemonic '{}'", name)));
    };

    // matching loop
    for data in opdata {
        if let Some(mut ctx) = match_args(&args, data) {

            // flatten the arg list for the encoding vm
            flatten_args(args, data, &mut ctx);

            return Ok(ctx);
        }
    }

    Err(Some("Unknown instruction format".into()))
}

/// Sanitizes arguments, ensuring that:
/// Register lists contain only vector registers without element specifiers
/// Vector register size specifications are possible (1B 2B 4B 8B 16B 1H 2H 4H 8H 1S 2S 4S 1D 2D)
/// References obey the allowed formats and use only normal registers
/// Reference modifiers are in the allowed set of modifiers
fn sanitize_args(args: Vec<RawArg>) -> Result<Vec<CleanArg>, Option<String>> {
    let mut args = args.into_iter().peekable();
    let mut res = Vec::new();

    while let Some(arg) = args.next() {
        match arg {
            // direct register arguments: Validate vector register element size / lane count combination is possible
            RawArg::Direct { span, reg } => {
                sanitize_register(span, &reg)?;
                res.push(CleanArg::Direct { span, reg });
            },
            // offsets: validate that only relative jumps are allowed (no extern relocations)
            RawArg::JumpTarget { type_ } => {
                if let JumpType::Bare(_) = type_ {
                    emit_error_at(type_.span(), "Extern relocations are not allowed in aarch64".into());
                    return Err(None);
                }
                res.push(CleanArg::JumpTarget { type_ });
            },
            // parsing error: just raise it
            RawArg::Invalid => {
                return Err(None);
            },
            // modifier: LSL LSR ASR ROR and MSL require an immediate.
            RawArg::Modifier { span, modifier } => {
                if modifier.expr.is_none() {
                    match modifier.op {
                        Modifier::LSL | Modifier::LSR | Modifier::ASR | Modifier::ROR | Modifier::MSL => {
                            emit_error_at(span, "LSL, LSR, ASR, ROR and MSL modifiers require a shift immediate.".into());
                            return Err(None);
                        }
                        _ => ()
                    }
                }

                res.push(CleanArg::Modifier { span, modifier });
            },
            // dot: passthrough
            RawArg::Dot { span } => {
                res.push(CleanArg::Dot { span } );
            },
            // lit: passthrough
            RawArg::Lit { ident } => {
                res.push(CleanArg::Lit { ident } );
            },
            // immediate: pass through
            RawArg::Immediate { value, prefixed } => {
                res.push(CleanArg::Immediate { value, prefixed })
            },
            // reference: first, assert the used indexing mode (base, offset, pre-indexed, or register-indexed)
            // then, verify that the base register is always an XSP register
            // for the register-indexed mode, additionally verify that the index register is either an W or an X register
            // and that the appropriate extend mode is used (UXTW/SXTW for W, LSL/SXTX for X)
            RawArg::Reference { span, items, bang } => {
                let mut items = items.into_iter();
                let mut hit_end = false;
                let mut kind = RefKind::Base;

                // first item in items has to be a register and is the base.
                let base = match items.next() {
                    Some(RefItem::Direct { reg, .. }) => reg,
                    Some(_) => {
                        emit_error_at(span, "First item in a reference list has to be a register".into());
                        return Err(None);
                    },
                    None => unreachable!("Cannot create empty references in the parser")
                };

                // second item is either a register or an offset
                match items.next() {
                    Some(RefItem::Direct { reg, ..}) => {
                        kind = RefKind::Indexed(reg, None);
                    },
                    Some(RefItem::Immediate { value }) => {
                        kind = RefKind::Offset(value);
                    },
                    Some(RefItem::Modifier { .. }) => {
                        emit_error_at(span, "Cannot have a modifier without index register or offset".into());
                        return Err(None);
                    },
                    None => hit_end = true
                }

                // if the second item was a register, there could be a modifier
                if let RefKind::Indexed(_, ref mut modifier) = kind {
                    match items.next() {
                        Some(RefItem::Modifier { modifier: m, ..}) => {
                            *modifier = Some(m)
                        },
                        Some(_) => {
                            emit_error_at(span, "Too many items in reference list".into());
                            return Err(None);
                        },
                        None => hit_end = true
                    }
                }

                // there should not be any more items in the reference
                if !hit_end && items.next().is_some() {
                    emit_error_at(span, "Too many items in reference list".into());
                    return Err(None);
                }

                // determine the mode. Currently post-indexed is just handled by parsing said arg at match time.
                if bang {
                    if let RefKind::Offset(offset) = kind {
                        kind = RefKind::PreIndexed(offset);
                    } else {
                        emit_error_at(span, "Cannot use pre-indexed addressing without an immediate offset.".into());
                        return Err(None);
                    }
                }

                // sanitizaiton
                // base can only be a Xn|SP reg
                if !(base.size() == Size::QWORD && (base.family() == RegFamily::INTEGERSP || (base.family() == RegFamily::INTEGER && !base.kind().is_zero_reg()))) {
                    emit_error_at(span, "Base register can only be a Xn|SP register".into());
                    return Err(None);
                }

                // index can only be a Xn or Wn reg
                if let RefKind::Indexed(ref index, ref modifier) = kind {
                    if index.family() != RegFamily::INTEGER {
                        emit_error_at(span, "Index register can only be a Xn or Wn register".into());
                        return Err(None);
                    }

                    // limited set of allowed modifiers.
                    if let Some(ref m) = modifier {
                        if if index.size() == Size::QWORD {m.op != Modifier::LSL && m.op != Modifier::SXTX} else {m.op != Modifier::SXTW && m.op != Modifier::UXTW} {
                            emit_error_at(span, "Invalid modifier for the selected base register type".into());
                            return Err(None);
                        }

                        // LSL requires a stated immediate
                        if m.op == Modifier::LSL && m.expr.is_none() {
                            emit_error_at(span, "LSL reference modifier requires an immediate".into());
                            return Err(None);
                        }
                    }
                }

                res.push(CleanArg::Reference {
                    span,
                    base,
                    kind
                });
            },
            // registerlist in dash notation: verify that all used registers have the same element size / lane count.
            // then, canonicalize it to first register / count and confirm it is a valid bare vector register
            RawArg::DashList { span, first, last, element } => {
                let mut s = ListSanitizer::new();
                s.sanitize(span, &first)?;
                s.sanitize(span, &last)?;

                let first_code = first.kind().encode();
                let last_code = last.kind().encode();
                let amount = if last_code <= first_code {
                    last_code + 32 - first_code
                } else {
                    last_code - first_code
                };

                res.push(CleanArg::RegList {
                    span,
                    first,
                    amount,
                    element
                })

            },
            // registerlist in comma notation: verify that all used registers have the same element size / lane count.
            // then, canonicalize it to first register / count and confirm it is a valid bare vector register
            RawArg::CommaList { span, items, element } => {
                if items.len() > 32 {
                    emit_error_at(span, "Too many registers in register list.".into());
                    return Err(None);
                }
                let amount = items.len() as u8;

                let mut items = items.into_iter();
                let first = items.next().unwrap();

                let mut s = ListSanitizer::new();
                s.sanitize(span, &first)?;
                let code = first.kind().encode();
                let mut next_code = code;


                for item in items {
                    s.sanitize(span, &item)?;
                    next_code = (next_code + 1) % 32;
                    if item.kind().encode() != next_code {
                        emit_error_at(span, "Registers in register list are not monotonically incrementing".into());
                        return Err(None);
                    }
                }

                res.push(CleanArg::RegList {
                    span,
                    first,
                    amount,
                    element,
                })
            },
            // registerlist in amount notation: verify the register and confirm it is a valid bare vector register
            RawArg::AmountList { span, first, amount, element } => {
                sanitize_register(span, &first)?;
                if let Register::Vector(v) = &first {
                    if v.element.is_some() {
                        emit_error_at(span, "Cannot use element specifiers inside of register lists.".into());
                        return Err(None);
                    }
                } else {
                    emit_error_at(span, "Can only use vector registers in register lists.".into());
                    return Err(None);
                }

                // ensure amount is a constant usize
                let amount = if let Some(amount) = as_number(&amount) {
                    if amount > 32 {
                        emit_error_at(span, "Too many registers in register list.".into());
                        return Err(None);
                    }
                    amount as u8
                } else {
                    emit_error_at(span, "Register list requires a contant amount of registers specified".into());
                    return Err(None);
                };

                res.push(CleanArg::RegList {
                    span,
                    first,
                    amount,
                    element,
                })
            }
        }
    }

    Ok(res)
}

struct ListSanitizer {
    pub element_size: Option<Size>,
    pub lanes: Option<Option<u8>>
}

impl ListSanitizer {
    fn new() -> ListSanitizer {
        ListSanitizer {
            element_size: None,
            lanes: None
        }
    }

    // check if this register spec is valid in a register list
    fn sanitize(&mut self, span: Span, register: &Register) -> Result<(), Option<String>> {
        sanitize_register(span, register)?;
        if let Register::Vector(v) = register {
            if v.element.is_some() {
                emit_error_at(span, "Cannot use element specifiers inside of register lists.".into());
                return Err(None);
            }

            if v.kind.is_dynamic() {
                emit_error_at(span, "Cannot use dynamic registers inside of a comma/dash register list.".into());
                return Err(None);
            }

            if let Some(size) = self.element_size {
                if size != v.element_size {
                    emit_error_at(span, "Inconsistent element sizes.".into());
                    return Err(None);
                }
            } else {
                self.element_size = Some(v.element_size)
            }

            if let Some(lanes) = self.lanes {
                if lanes != v.lanes {
                    emit_error_at(span, "Inconsistent lane count.".into());
                    return Err(None);
                }
            } else {
                self.lanes = Some(v.lanes);
            }
        } else {
            emit_error_at(span, "Can only use vector registers in register lists.".into());
            return Err(None);
        }
        Ok(())
    }
}

// check that the register spec is possible
fn sanitize_register(span: Span, register: &Register) -> Result<(), Option<String>> {
    if let Register::Vector(v) = register {
        if let Some(total) = v.full_size() {
            if total > 16 {
                emit_error_at(span, "Overly wide vector register.".into());
                return Err(None)
            }
        }
    }
    Ok(())
}


/// struct containing information found during a match
#[derive(Debug)]
pub struct MatchData {
    pub simd_full_width: Option<bool>,
    pub data: &'static Opdata,
    pub args: Vec<FlatArg>,
}

impl MatchData {
    pub fn new(data: &'static Opdata) -> MatchData {
        MatchData {
            simd_full_width: None,
            data,
            args: Vec::new()
        }
    }
}


impl Matcher {
    /// Returns if this matcher matches the given argument
    pub fn matches(&self, arg: &CleanArg, ctx: &mut MatchData) -> bool {
        match arg {
            CleanArg::Reference { kind, .. } => {
                match kind {
                    RefKind::Base => *self == Matcher::RefBase || *self == Matcher::RefOffset,
                    RefKind::Offset(_) => *self == Matcher::RefOffset,
                    RefKind::PreIndexed(_) => *self == Matcher::RefPre,
                    RefKind::Indexed(_, _) => *self == Matcher::RefIndex,
                }
            },
            CleanArg::RegList { amount, element, first, .. } => {
                let first = first.assume_vector();
                match self {
                    Matcher::RegList(m_amount, element_size) => {
                        if m_amount != amount || *element_size != first.element_size() || element.is_some() {
                            return false;
                        }

                        if let Some(bytes) = first.full_size() {
                            let full_width = match bytes {
                                8 => false,
                                16 => true,
                                _ => return false
                            };
                            match ctx.simd_full_width {
                                None => {
                                    ctx.simd_full_width = Some(full_width);
                                    true
                                }
                                Some(f) => f == full_width
                            }
                        } else {
                            false
                        }
                    },
                    Matcher::RegListStatic(m_amount, element_size, lanecount) =>
                        m_amount == amount && *element_size == first.element_size() && element.is_none() && first.lanes == Some(*lanecount),
                    Matcher::RegListElement(m_amount, element_size) =>
                        m_amount == amount && *element_size == first.element_size() && element.is_some(),
                    _ => false
                }
            },
            CleanArg::Direct { reg, .. } => {
                match reg {
                    Register::Vector(ref v) => match self {
                        Matcher::V(size) => {
                            if *size != v.element_size || v.element.is_some() {
                                return false;
                            }
                            if let Some(bytes) = v.full_size() {
                                let full_width = match bytes {
                                    8 => false,
                                    16 => true,
                                    _ => return false
                                };
                                match ctx.simd_full_width {
                                    None => {
                                        ctx.simd_full_width = Some(full_width);
                                        true
                                    }
                                    Some(f) => f == full_width
                                }
                            } else {
                                false
                            }
                        },
                        Matcher::VStatic(size, lanes) =>
                            *size == v.element_size && v.element.is_none() && v.lanes == Some(*lanes),
                        Matcher::VElement(size) =>
                            *size == v.element_size && v.element.is_some(),
                        Matcher::VElementStatic(size, element) =>
                            *size == v.element_size && v.element.as_ref().and_then(as_number) == Some(*element as u64),
                        _ => false
                    },
                    Register::Scalar(ref s) => match self {
                        Matcher::W => s.size() == Size::DWORD && s.kind.family() == RegFamily::INTEGER,
                        Matcher::X => s.size() == Size::QWORD && s.kind.family() == RegFamily::INTEGER,
                        Matcher::WSP => s.size() == Size::DWORD && (s.kind.family() == RegFamily::INTEGERSP || (s.kind.family() == RegFamily::INTEGER && !s.kind.is_zero_reg())),
                        Matcher::XSP => s.size() == Size::QWORD && (s.kind.family() == RegFamily::INTEGERSP || (s.kind.family() == RegFamily::INTEGER && !s.kind.is_zero_reg())),
                        Matcher::B => s.size() == Size::BYTE && s.kind.family() == RegFamily::SIMD,
                        Matcher::H => s.size() == Size::WORD && s.kind.family() == RegFamily::SIMD,
                        Matcher::S => s.size() == Size::DWORD && s.kind.family() == RegFamily::SIMD,
                        Matcher::D => s.size() == Size::QWORD && s.kind.family() == RegFamily::SIMD,
                        Matcher::Q => s.size() == Size::OWORD && s.kind.family() == RegFamily::SIMD,
                        _ => false
                    }
                }
            },
            CleanArg::JumpTarget { .. } => *self == Matcher::Offset,
            CleanArg::Immediate { prefixed: true, .. } => *self == Matcher::Imm,
            CleanArg::Immediate { prefixed: false, value} => match self {
                Matcher::Imm => true,
                Matcher::Ident => as_ident(value).is_some(),
                Matcher::Cond => if let Some(i) = as_ident(value) {
                    COND_MAP.contains_key(&&*i.to_string())
                } else {
                    false
                },
                Matcher::Lit(s) => if let Some(i) = as_ident(value) {
                    i.to_string() == *s
                } else {
                    false
                },
                Matcher::LitInt(v) => as_number(value) == Some(*v as u64),
                Matcher::LitFloat(v) => as_float(value) == Some(*v as f64),
                _ => false
            },
            CleanArg::Modifier { modifier, .. } => {
                if let Matcher::Mod(list) = self {
                    list.iter().any(|m| m == &modifier.op)
                } else {
                    false
                }
            },
            CleanArg::Dot { .. } => *self == Matcher::Dot,
            CleanArg::Lit { ident } => match self {
                Matcher::Ident => true,
                Matcher::Cond => COND_MAP.contains_key(&&*ident.to_string()),
                Matcher::Lit(s) => ident.to_string() == *s,
                _ => false
            }
        }
    }

    pub fn flatarg_count(&self) -> usize {
        match self {
            Matcher::Dot => 0,
            Matcher::Lit(_) => 0,
            Matcher::LitInt(_) => 0,
            Matcher::LitFloat(_) => 0,
            Matcher::Ident => 1,
            Matcher::Cond => 1,
            Matcher::Imm => 1,
            Matcher::W |
            Matcher::X |
            Matcher::WSP |
            Matcher::XSP |
            Matcher::B |
            Matcher::H |
            Matcher::S |
            Matcher::D |
            Matcher::Q => 1,
            Matcher::V(_) |
            Matcher::VStatic(_, _) => 1,
            Matcher::VElement(_) => 2,
            Matcher::VElementStatic(_, _) => 1,
            Matcher::RegList(_, _) |
            Matcher::RegListStatic(_, _, _) => 1,
            Matcher::RegListElement(_, _) => 2,
            Matcher::Offset => 1,
            Matcher::RefBase => 1,
            Matcher::RefOffset => 2,
            Matcher::RefPre => 2,
            Matcher::RefIndex => 4,
            Matcher::Mod(_) => 2,

            // this is special anyway
            Matcher::End => 0,
        }
    }

    /// Returns if this matcher is the End matcher
    pub fn is_end(&self) -> bool {
        match self {
            Matcher::End => true,
            _ => false,
        }
    }
}

/// Check if the args string matches the data matching template
pub fn match_args(args: &[CleanArg], data: &'static Opdata) -> Option<MatchData> {
    let mut ctx = MatchData::new(data);

    let mut args = args.iter().peekable();

    for matcher in data.matchers {
        match matcher {
            Matcher::End => if args.peek().is_some() {
                continue;
            } else {
                return Some(ctx);
            },
            matcher => if let Some(arg) = args.next() {
                if !matcher.matches(arg, &mut ctx) {
                    return None;
                }
            } else {
                return None;
            },
        }
    }

    if args.next().is_some() {
        None
    } else {
        Some(ctx)
    }
}

/// flatten the arg list into a linear sequence of encodable elements
fn flatten_args(args: Vec<CleanArg>, data: &Opdata, ctx: &mut MatchData) {
    let mut source_args = args.into_iter();
    let mut new_args = Vec::new();

    for matcher in data.matchers {
        let arg_count = match matcher {
            Matcher::End => continue,
            matcher => matcher.flatarg_count()
        };

        if let Some(arg) = source_args.next() {
            match arg {
                CleanArg::Reference { span, base, kind} => {
                    new_args.push(FlatArg::Direct { span, reg: base.kind_owned() } );
                    match kind {
                        RefKind::Base => (),
                        RefKind::Offset(value) =>
                            new_args.push(FlatArg::Immediate { value } ),
                        RefKind::PreIndexed(value) =>
                            new_args.push(FlatArg::Immediate { value } ),
                        RefKind::Indexed(index, modifier) => {
                            new_args.push(FlatArg::Direct { span, reg: index.kind_owned() } );
                            if let Some(modifier) = modifier {
                                new_args.push(FlatArg::Modifier { span, modifier: modifier.op } );
                                if let Some(expr) = modifier.expr {
                                    new_args.push(FlatArg::Immediate { value: expr } );
                                }
                            }
                        }
                    }
                },
                CleanArg::RegList { span, first, element, .. } => {
                    new_args.push(FlatArg::Direct { span, reg: first.kind_owned() } );
                    if let Some(element) = element {
                        new_args.push(FlatArg::Immediate { value: element } );
                    }
                },
                CleanArg::Direct { span, reg } => {
                    match reg {
                        Register::Scalar(s) => {
                            new_args.push(FlatArg::Direct { span, reg: s.kind });
                        },
                        Register::Vector(v) => {
                            new_args.push(FlatArg::Direct { span, reg: v.kind });
                            if let Some(element) = v.element {
                                new_args.push(FlatArg::Immediate { value: element });
                            }
                        }
                    }
                },
                CleanArg::JumpTarget { type_ } => {
                    new_args.push(FlatArg::JumpTarget { type_ } );
                },
                CleanArg::Immediate { value, .. } => {
                    new_args.push(FlatArg::Immediate { value } );
                },
                CleanArg::Modifier { span, modifier } => {
                    new_args.push(FlatArg::Modifier { span, modifier: modifier.op } );
                    if let Some(expr) = modifier.expr {
                        new_args.push(FlatArg::Immediate { value: expr });
                    }
                },
                CleanArg::Dot { .. } => (),
                CleanArg::Lit { ident } => {
                    new_args.push(FlatArg::Lit { ident });
                }
            }
        }

        new_args.resize_with(arg_count, || FlatArg::Default);

        ctx.args.extend(new_args.drain(..))
    }
}
