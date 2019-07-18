
use proc_macro2::Span;

use super::Context;
use super::ast::{Instruction, RawArg, CleanArg, FlatArg, JumpType, RefItem, Register, RegFamily, RegKind, RegId, RefKind, Modifier};
use super::aarch64data::{Opdata, Matcher, COND_MAP, get_mnemonic_data};
use ::emit_error_at;
use serialize::Size;
use parse_helpers::{as_ident, as_number, as_float};


pub(super) fn match_instruction(_ctx: &mut Context, instruction: &Instruction, args: Vec<RawArg>) -> Result<(&'static Opdata, Vec<FlatArg>), Option<String>> {
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
        if matches_args(&args, data) {
            // keep only necessary operands for encoding
            let args = filter_args(args, data);

            // flatten the arg list for the encoding vm
            let args = flatten_args(args);

            return Ok((data, args));
        }
    }

    Err(Some("Unknown instruction format".into()))
}

/// Sanitizes arguments, ensuring that:
/// Register lists contain only vector registers without element specifiers
/// Vector register size specifications are possible
/// References obey the allowed formats and use only normal registers
/// Reference modifiers are in the allowed set of modifiers
/// modifiers and their respective immediates have been split up
fn sanitize_args(args: Vec<RawArg>) -> Result<Vec<CleanArg>, Option<String>> {
    let mut args = args.into_iter().peekable();
    let mut res = Vec::new();

    while let Some(arg) = args.next() {
        match arg {
            RawArg::Direct { span, reg } => {
                res.push(CleanArg::Direct { span, reg });
            },
            RawArg::JumpTarget { type_ } => {
                if let JumpType::Bare(_) = type_ {
                    return Err(Some("Extern relocations are not allowed in aarch64".into()));
                }
                res.push(CleanArg::JumpTarget { type_ });
            },
            RawArg::Invalid => {
                return Err(None);
            },
            RawArg::Modifier { span, modifier } => {
                if modifier.expr.is_none() && (modifier.op == Modifier::LSL || modifier.op == Modifier::LSR || modifier.op == Modifier::ASR || modifier.op == Modifier::ROR) {
                    emit_error_at(span, "LSL, LSR, ASR and ROR modifiers require a shift immediate.".into());
                    return Err(None);
                }

                res.push(CleanArg::Modifier { span, modifier });
            },
            RawArg::Dot { span } => {
                res.push(CleanArg::Dot { span } );
            },
            RawArg::Immediate { value, prefixed } => {
                res.push(CleanArg::Immediate { value, prefixed })
            },
            RawArg::Reference { span, items, bang } => {

                let mut items = items.into_iter();

                // first item in items has to be a register and is the base.
                let base = match items.next() {
                    Some(RefItem::Direct { reg, .. }) => reg,
                    Some(_) => {
                        emit_error_at(span, "First item in a reference list has to be a register".into());
                        return Err(None);
                    },
                    None => unreachable!("Cannot create empty references in the parser")
                };

                let mut hit_end = false;
                let mut kind = RefKind::Base;

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

                // we don't know about anything else in here
                if !hit_end {
                    if let Some(_) = items.next() {
                        emit_error_at(span, "Too many items in reference list".into());
                        return Err(None);   
                    }
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
                if !(base.size() == Size::QWORD || (base.kind().family() == RegFamily::SCALARSP || 
                        (!base.is_dynamic() && base.family() == RegFamily::SCALAR && base.kind() != &RegKind::Static(RegId::XZR)))) {
                    emit_error_at(span, "Base register can only be a Xn|SP register".into());
                    return Err(None);
                }

                // index can only be a Xn or Wn reg
                if let RefKind::Indexed(ref index, ref modifier) = kind {
                    if index.kind().family() != RegFamily::SCALAR {
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
                let amount = if let Some(amount) = ::parse_helpers::as_number(&amount) {
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
                    emit_error_at(span, "Inconsistent element sizes.".into());
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
        if let Some(lanes) = v.lanes {
            if let Some(total) = lanes.checked_mul(v.element_size.in_bytes()) {
                if total > 16 {
                    emit_error_at(span, "Overly wide vector register.".into());
                    return Err(None)
                }
            }
        }
    }
    Ok(())
}


impl Matcher {
    /// Returns if this matcher can match the given argument.
    pub fn matches(&self, arg: &CleanArg) -> bool {
        match arg {
            CleanArg::Reference { kind, .. } => {
                match kind {
                    RefKind::Base => match self {
                        Matcher::RefBase | Matcher::RefOffset => true,
                        _ => false
                    },
                    RefKind::Offset(_) => *self == Matcher::RefOffset,
                    RefKind::PreIndexed(_) => *self == Matcher::RefPre,
                    RefKind::Indexed(_, _) => *self == Matcher::RefIndex,
                }
            },
            CleanArg::RegList { amount, element, first, .. } => {
                match self {
                    Matcher::RegList(a) => a == amount && element.is_none(),
                    Matcher::RegListSized(a, e, s) => a == amount && element.is_none() && first.size() == *e && first.assume_vector().lanes == Some(*s),
                    Matcher::RegListLanes(a, e) => a == amount && element.is_some() && first.size() == *e,
                    _ => false
                }
            },
            CleanArg::Direct { reg, .. } => {
                if let Register::Vector(ref v) = reg {
                    match self {
                        Matcher::VWild => v.element.is_none(),
                        Matcher::VSized(size) => *size == v.element_size && v.element.is_none(),
                        Matcher::VSizedStatic(size, lanes) => *size == v.element_size && v.lanes == Some(*lanes),
                        Matcher::VLanes(size) => *size == v.element_size && v.element.is_some(),
                        Matcher::VLanesStatic(size, element) => *size == v.element_size && v.element.as_ref().and_then(as_number) == Some(*element as u64),
                        _ => false
                    }
                } else {
                    match self {
                        Matcher::W => reg.size() == Size::DWORD && reg.family() == RegFamily::SCALAR,
                        Matcher::X => reg.size() == Size::QWORD && reg.family() == RegFamily::SCALAR,
                        Matcher::WSP => reg.size() == Size::DWORD && (reg.family() == RegFamily::SCALARSP ||
                            (!reg.is_dynamic() && reg.family() == RegFamily::SCALAR && reg.kind() != &RegKind::Static(RegId::XZR))),
                        Matcher::XSP => reg.size() == Size::QWORD && (reg.family() == RegFamily::SCALARSP ||
                            (!reg.is_dynamic() && reg.family() == RegFamily::SCALAR && reg.kind() != &RegKind::Static(RegId::XZR))),
                        Matcher::B => reg.family() == RegFamily::VECTOR && reg.size() == Size::BYTE,
                        Matcher::H => reg.family() == RegFamily::VECTOR && reg.size() == Size::WORD,
                        Matcher::S => reg.family() == RegFamily::VECTOR && reg.size() == Size::DWORD,
                        Matcher::D => reg.family() == RegFamily::VECTOR && reg.size() == Size::QWORD,
                        Matcher::Q => reg.family() == RegFamily::VECTOR && reg.size() == Size::OWORD,
                        Matcher::V => reg.family() == RegFamily::VECTOR,
                        _ => false
                    }
                }
            },
            CleanArg::JumpTarget { .. } => *self == Matcher::Offset,
            CleanArg::Immediate { prefixed, value } => {
                if *prefixed {
                    *self == Matcher::Imm
                } else {
                    match self {
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
                    }
                }
            },
            CleanArg::Modifier { modifier, .. } => {
                if let Matcher::Mod(list) = self {
                    list.iter().any(|m| m == &modifier.op)
                } else {
                    false
                }
            },
            CleanArg::Dot { .. } => *self == Matcher::Dot
        }
    }

    /// Returns if this matcher consumes the argument (i.e. it does not need to be encoded)
    pub fn consumes(&self) -> bool {
        match self {
            Matcher::Dot => true,
            Matcher::Lit(_) => true,
            Matcher::LitInt(_) => true,
            Matcher::LitFloat(_) => true,
            Matcher::Ident => false,
            Matcher::Cond => false,
            Matcher::Imm => false,
            Matcher::W |
            Matcher::X |
            Matcher::WSP |
            Matcher::XSP |
            Matcher::B |
            Matcher::H |
            Matcher::S |
            Matcher::D |
            Matcher::Q => false,
            Matcher::V => false,
            Matcher::VWild |
            Matcher::VSized(_) |
            Matcher::VSizedStatic(_, _) |
            Matcher::VLanes(_) |
            Matcher::VLanesStatic(_, _) => false,
            Matcher::RegList(_) |
            Matcher::RegListLanes(_, _) |
            Matcher::RegListSized(_, _, _) => false,
            Matcher::Offset => false,
            Matcher::RefBase |
            Matcher::RefOffset |
            Matcher::RefPre |
            Matcher::RefIndex => false,
            Matcher::Mod(_) => false,

            // this is special anyway
            Matcher::End => panic!("Should never query if End is a consuming matcher"),
            Matcher::Unimp => panic!("Should never query if Unimp is a consuming matcher")
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

pub fn matches_args(args: &[CleanArg], data: &Opdata) -> bool {
    let mut matchers = data.matchers.iter();

    for arg in args {
        match matchers.next() {
            Some(Matcher::End) => continue,
            Some(matcher) => if !matcher.matches(arg) {
                return false;
            },
            None => return false
        }
    }
    match matchers.next() {
        Some(Matcher::End) => true,
        None => true,
        Some(_) => false
    }
}

pub fn filter_args(args: Vec<CleanArg>, data: &Opdata) -> impl Iterator<Item=CleanArg> {
    args.into_iter()
        .zip(data.matchers.iter().filter(|m| m.is_end()))
        .filter_map(|(a, m)| if m.consumes() { None } else { Some(a) } )
}

/// flatten the arg list into a linear sequence of encodable elements
fn flatten_args(args: impl Iterator<Item=CleanArg>) -> Vec<FlatArg> {
    let mut res = Vec::new();

    for arg in args.into_iter() {
        match arg {
            CleanArg::Reference { span, base, kind} => {
                res.push(FlatArg::Direct { span, reg: base } );
                match kind {
                    RefKind::Base => (),
                    RefKind::Offset(value) => res.push(FlatArg::Immediate { value } ),
                    RefKind::PreIndexed(value) => res.push(FlatArg::Immediate { value } ),
                    RefKind::Indexed(index, modifier) => {
                        res.push(FlatArg::Direct { span, reg: index } );
                        if let Some(modifier) = modifier {
                            res.push(FlatArg::Modifier { span, modifier: modifier.op } );
                            if let Some(expr) = modifier.expr {
                                res.push(FlatArg::Immediate { value: expr } );
                            }
                        }
                    }
                }
            },
            CleanArg::RegList { span, first, element, .. } => {
                res.push(FlatArg::Direct { span, reg: first } );
                if let Some(element) = element {
                    res.push(FlatArg::Immediate { value: element } );
                }
            },
            CleanArg::Direct { span, reg } => {
                res.push(FlatArg::Direct { span, reg } );
            },
            CleanArg::JumpTarget { type_ } => {
                res.push(FlatArg::JumpTarget { type_ } );
            },
            CleanArg::Immediate { value, .. } => {
                res.push(FlatArg::Immediate { value } );
            },
            CleanArg::Modifier { span, modifier } => {
                res.push(FlatArg::Modifier { span, modifier: modifier.op } );
                if let Some(expr) = modifier.expr {
                    res.push(FlatArg::Immediate { value: expr });
                }
            },
            CleanArg::Dot { .. } => ()
        }
    }

    res
}
