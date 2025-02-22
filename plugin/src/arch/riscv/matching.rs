use proc_macro_error2::emit_error;
use proc_macro2::Span;

use super::{Context, RiscVTarget};
use super::ast::{ParsedInstruction, RawArg, RegListCount, MatchData, FlatArg, RegListFlat, Register, RegId, RegFamily};
use super::riscvdata::{Opdata, Matcher, ISAFlags, get_mnemonic_data};

use crate::common::JumpKind;
use crate::parse_helpers::{as_ident, as_signed_number};

/// Try finding an appropriate instruction definition that matches the given instruction / arguments.
pub(super) fn match_instruction(ctx: &mut Context, mut instruction: ParsedInstruction) -> Result<MatchData, Option<String>> {
    // sanitize Raw args from parsing for any impossible constructs
    sanitize_args(&mut instruction.args, &ctx.target)?;

    let opdata = get_mnemonic_data(&instruction.name).ok_or_else(|| Some(format!("Unknown instruction mnemonic '{}'", instruction.name)))?;

    // iterate through the supported instruction formats. If one matches, lower the args to
    // FlatArgs and return the combined Matchdata
    for data in opdata {
        // skip data not intended for our target
        if ctx.target.is_32_bit() && !data.isa_flags.contains(ISAFlags::RV32) {
            continue;
        }
        if ctx.target.is_64_bit() && !data.isa_flags.contains(ISAFlags::RV64) {
            continue;
        }

        // TODO: give errors for missing features here

        if let Some(mut match_data) = match_args(&instruction.args, data) {
            flatten_args(instruction.args, &mut match_data);

            return Ok(match_data)
        }
    }

    // TODO: implement proper debug suggestions
    Err(Some(
        format!("'{}': instruction format mismatch, expected one of the following forms:\nTODO", instruction.name)
    ))
}

/// Sanitizes arguments, ensuring that
/// register lists always contain the expected {ra, s0-s11} registers
/// Extern relocations are not allowed
/// No registers above 15 are used on E-profile RISCV
/// Canonicalize `0(register)` references as without offset (like `(register)`)
fn sanitize_args(args: &mut [RawArg], target: &RiscVTarget) -> Result<(), Option<String>> {
    for arg in args {
        match arg {
            RawArg::Register { reg, span } => sanitize_register(reg, *span, target)?,
            RawArg::Reference { base, span, offset } => {
                sanitize_register(base, *span, target)?;
                if let Some(o) = offset.as_ref() {
                    if as_signed_number(o) == Some(0) {
                        *offset = None
                    }
                }
            },
            RawArg::JumpTarget { jump } => {
                if let JumpKind::Bare(_) = jump.kind {
                    emit_error!(jump.span(), "Extern relocations are not allowed in aarch64");
                    return Err(None);
                }
            },
            RawArg::RegisterList { first, count, span } => {
                if first.as_id() != Some(RegId::X1) {
                    emit_error!(span, "The first item in a register list should be 'ra' (x1)");
                    return Err(None);
                }

                match count {
                    RegListCount::Static(_) => (),
                    RegListCount::Dynamic(_) => (),
                    RegListCount::Single(first) => {
                        if first.as_id() != Some(RegId::X8) {
                            emit_error!(span, "The second item in a register list should be 's0' (x8)");
                            return Err(None);
                        }
                        *count = RegListCount::Static(5);
                    },
                    RegListCount::Double(first, last) => {
                        if first.as_id() != Some(RegId::X8) {
                            emit_error!(span, "The second item in a register list should be 's0' (x8)");
                            return Err(None);
                        }
                        let amount = match last {
                            Register::Dynamic(_, _) => {
                                emit_error!(span, "Please use the {ra; amount} format to specify the amount of registers in a list dynamically");
                                return Err(None);
                            },
                            Register::Static(RegId::X9) => 6,
                            Register::Static(RegId::X18) => 7,
                            Register::Static(RegId::X19) => 8,
                            Register::Static(RegId::X20) => 9,
                            Register::Static(RegId::X21) => 10,
                            Register::Static(RegId::X22) => 11,
                            Register::Static(RegId::X23) => 12,
                            Register::Static(RegId::X24) => 13,
                            Register::Static(RegId::X25) => 14,
                            Register::Static(RegId::X27) => 15,
                            Register::Static(_) => {
                                emit_error!(span, "Cannot end a register list on this register. The last register should be a saved register that is not s10 or s0");
                                return Err(None);
                            }
                        };
                        if target.is_embedded() && amount > 6 {
                            emit_error!(span, "Registers above x15 cannot be used on RV-E profiles");
                            return Err(None)
                        }

                        *count = RegListCount::Static(amount);
                    }
                }
            },
            _ => ()
        }
    }

    Ok(())
}

/// Sanitize a single register, checking that on RV32/64E only the top 16 registers are used.
fn sanitize_register(register: &Register, span: Span, target: &RiscVTarget) -> Result<(), Option<String>> {
    // RV32E/RV64E only define 16 integer registers.
    // (They do still define 32 floating point registers if F is used)
    if target.is_embedded() && register.family() == RegFamily::INTEGER {
        if let Some(code) = register.code() {
            if code >= 16 {
                emit_error!(span, "The second item in a register list should be 's0' (x8)");
                return Err(None);
            }
        }
    }
    Ok(())
}


impl MatchData {
    pub fn new(data: &'static Opdata) -> MatchData {
        MatchData {
            data,
            args: Vec::new()
        }
    }
}


impl Matcher {
    /// Returns if this matcher matches the given argument
    pub fn matches(&self, arg: &RawArg) -> bool {
        match arg {
            RawArg::Immediate { value } => match self {
                Matcher::Imm => true,
                Matcher::Offset => true,
                Matcher::Ident => as_ident(value).is_some(),
                _ => false
            },
            RawArg::JumpTarget { jump } => *self == Matcher::Offset,
            RawArg::Register { reg, .. }=> match self {
                Matcher::X => reg.family() == RegFamily::INTEGER,
                Matcher::F => reg.family() == RegFamily::FP,
                _ => false,
            },
            RawArg::Reference { offset, base, .. } => match self {
                Matcher::Ref => offset.is_none(),
                Matcher::RefOffset => true,
                _ => false,
            },
            RawArg::RegisterList { first, count, .. } => *self == Matcher::Xlist,
        }
    }
}


/// Check if the parsed instruction arguments match the data matching template
pub fn match_args(args: &[RawArg], data: &'static Opdata) -> Option<MatchData> {
    let mut args = args.iter();

    // check if each matcher matches an appropriate arg
    for matcher in data.matchers {
        if let Some(arg) = args.next() {
            if !matcher.matches(arg) {
                return None;
            }
        } else {
            return None;
        }
    }

    // and return success if there's no more args remaining to match
    if args.next().is_some() {
        None
    } else {
        Some(MatchData::new(data))
    }
}


/// Populate MatchData with FlatArgs
fn flatten_args(args: Vec<RawArg>, data: &mut MatchData) {
    for (arg, matcher) in args.into_iter().zip(data.data.matchers.iter()) {
        match arg {
            RawArg::Immediate { value } => {
                data.args.push(FlatArg::Immediate { value });
            },
            RawArg::JumpTarget { jump } => {
                data.args.push(FlatArg::JumpTarget { jump });
            },
            RawArg::Register { span, reg } => {
                data.args.push(FlatArg::Register { span, reg });
            },
            RawArg::Reference { span, offset, base } => {
                data.args.push(FlatArg::Register { span, reg: base });
                if let Matcher::RefOffset = matcher {
                    if let Some(offset) = offset {
                        data.args.push(FlatArg::Immediate { value: offset });
                    } else {
                        data.args.push(FlatArg::Default);
                    }
                }
            },
            RawArg::RegisterList {span, first, count } => {
                let count = match count {
                    RegListCount::Static(c) => RegListFlat::Static(c),
                    RegListCount::Dynamic(expr) => RegListFlat::Dynamic(expr),
                    _ => unreachable!("RegListCount ought to be sanitized at this point.")
                };
                data.args.push(FlatArg::RegisterList { span, count });
            }
        }
    }
}
