use super::matching::MatchData;
use super::aarch64data::{Command, COND_MAP, SPECIAL_IDENT_MAP, SpecialComm};
use super::Context;
use super::ast::{FlatArg, RegKind, RegId, Modifier, };
use super::encoding_helpers;

use crate::common::{Stmt, Size, delimited, emit_error_at, bitmask};
use crate::parse_helpers::{as_ident, as_number, as_float, as_signed_number};

use syn::spanned::Spanned;
use quote::{quote, quote_spanned};
use proc_macro2::TokenStream;

pub(super) fn compile_instruction(ctx: &mut Context, data: MatchData) -> Result<(), Option<String>> {
    let mut cursor = 0usize;

    // All static bitfields (compile-time constant) will be encoded into this map of (offset, bitfield)
    let mut statics = Vec::new();
    // All dynamic bitfields (run-time determined) will be encoded into this map of (offset, TokenStream)
    let mut dynamics = Vec::new();
    // Any relocations will be encoded into this list
    let mut relocations = Vec::new();

    for command in data.data.commands.iter() {
        match *command {
            // special commands that don't check the current arg
            Command::A => {
                cursor += 1;
                continue
            },
            Command::C => {
                cursor -= 1;
                continue
            },
            Command::Rwidth(offset) => {
                statics.push((offset, data.simd_full_width.unwrap_or(true) as u32));
                continue
            },

            _ => ()
        }

        let arg = data.args.get(cursor).expect("Invalid encoding data, tried to process more arguments than given");

        match *arg {
            FlatArg::Direct { span, reg: RegKind::Static(id) } => match *command {
                Command::R(offset) => {
                    statics.push((offset, id.code() as u32));
                },
                Command::REven(offset) => {
                    if id.code() & 1 != 0 {
                        emit_error_at(span, "Field only supports even registers".into());
                        return Err(None);
                    }
                    statics.push((offset, id.code() as u32));
                },
                Command::RNoZr(offset) => {
                    if id.code() == 31 {
                        emit_error_at(span, "Field does not support register the zr/sp register".into());
                        return Err(None);
                    }
                    statics.push((offset, id.code() as u32));
                },
                Command::R4(offset) => {
                    if id.code() >= 16 {
                        emit_error_at(span, "Field only supports register numbers 0-15".into());
                        return Err(None);
                    }
                    statics.push((offset, id.code() as u32));
                },
                Command::RNext => {
                    if let Some(FlatArg::Direct { span: _prevspan, reg: ref prevreg } ) = data.args.get(cursor - 1) {
                        match prevreg {
                            RegKind::Static(previd) => if id.code() != ((previd.code() + 1) % 32) {
                                emit_error_at(span, "Invalid register. This register has to be the register after the previous argument.".into());
                                return Err(None);
                            },
                            RegKind::Dynamic(_, _) => if id != RegId::XZR {
                                emit_error_at(span, "Please use XZR here to indicate that it should be the register after the previous argument.".into());
                                return Err(None);
                            }
                        }
                    } else {
                        panic!("RNext command without the previous command being a register encoder");
                    }
                },
                _ => panic!("Invalid argument processor")
            },
            FlatArg::Direct { span, reg: RegKind::Dynamic(_, ref expr) } => match *command {
                Command::R(offset)
                | Command::RNoZr(offset) => {
                    dynamics.push((offset, quote_spanned!{ span=>
                        #expr & 0x1F
                    }));
                },
                Command::REven(offset) => {
                    dynamics.push((offset, quote_spanned!{ span=>
                        #expr & 0x1E
                    }));
                },
                Command::R4(offset) => {
                    dynamics.push((offset, quote_spanned!{ span=>
                        #expr & 0xF
                    }));
                },
                Command::RNext => {
                    emit_error_at(span, "This register is constrained to be the register after the previous argument's register. As such, it does not support dynamic registers. Please substitute it with XZR to indicate this".into());
                    return Err(None);
                },
                _ => panic!("Invalid argument processor")
            },
            FlatArg::Modifier { modifier, .. } => match *command {
                Command::Rotates(offset) => match modifier {
                    Modifier::LSL => statics.push((offset, 0b00)),
                    Modifier::LSR => statics.push((offset, 0b01)),
                    Modifier::ASR => statics.push((offset, 0b10)),
                    Modifier::ROR => statics.push((offset, 0b11)),
                    _ => panic!("Unexpected modifier for argument processor")
                },
                Command::ExtendsW(offset) => match modifier {
                    Modifier::UXTB => statics.push((offset, 0b000)),
                    Modifier::UXTH => statics.push((offset, 0b001)),
                    Modifier::UXTW => statics.push((offset, 0b010)),
                    Modifier::UXTX => statics.push((offset, 0b011)),
                    Modifier::SXTB => statics.push((offset, 0b100)),
                    Modifier::SXTH => statics.push((offset, 0b101)),
                    Modifier::SXTW => statics.push((offset, 0b110)),
                    Modifier::SXTX => statics.push((offset, 0b111)),
                    Modifier::LSL  => statics.push((offset, 0b010)),
                    _ => panic!("Unexpected modifier for argument processor")
                },
                Command::ExtendsX(offset) => match modifier {
                    Modifier::UXTB => statics.push((offset, 0b000)),
                    Modifier::UXTH => statics.push((offset, 0b001)),
                    Modifier::UXTW => statics.push((offset, 0b010)),
                    Modifier::UXTX => statics.push((offset, 0b011)),
                    Modifier::SXTB => statics.push((offset, 0b100)),
                    Modifier::SXTH => statics.push((offset, 0b101)),
                    Modifier::SXTW => statics.push((offset, 0b110)),
                    Modifier::SXTX => statics.push((offset, 0b111)),
                    Modifier::LSL  => statics.push((offset, 0b011)),
                    _ => panic!("Unexpected modifier for argument processor")
                },
                _ => panic!("Invalid argument processor")
            },
            FlatArg::Immediate { ref value } => match *command {

                // Condition codes, literals
                Command::Cond(offset) => {
                    let name = as_ident(value).expect("bad command data").to_string();
                    let bits = *COND_MAP.get(&&*name).expect("bad command data");
                    statics.push((offset, bits as u32))
                },
                Command::CondInv(offset) => {
                    let name = as_ident(value).expect("bad command data").to_string();
                    let bits = *COND_MAP.get(&&*name).expect("bad command data");
                    statics.push((offset, bits as u32 ^ 1))
                },
                Command::LitList(offset, listname) => {
                    let name = as_ident(value).expect("bad command data").to_string();
                    let list = SPECIAL_IDENT_MAP.get(listname).expect("bad command data");
                    if let Some(&bits) = list.get(&&*name) {
                        statics.push((offset, bits));
                    } else {
                        emit_error_at(value.span(), "Unknown literal".into());
                        return Err(None);
                    }
                },

                // unsigned integer encodings
                Command::Ubits(offset, bitlen) => {
                    let mask = bitmask(bitlen);
                    if let Some(value) = unsigned_rangecheck(value, 0, mask, 0) {
                        statics.push((offset, value?));
                    } else {
                        dynamics.push((offset, quote_spanned!{ value.span()=>
                            #value & #mask
                        }));
                    }
                },
                Command::Uscaled(offset, bitlen, shift) => {
                    let mask = bitmask(bitlen);
                    if let Some(value) = unsigned_rangecheck(value, 0, mask, shift) {
                        statics.push((offset, value?));
                    } else {
                        dynamics.push((offset, quote_spanned!{ value.span()=>
                            (#value >> #shift) & #mask
                        }));
                    }
                },
                Command::Uslice(offset, bitlen, shift) => {
                    let mask = bitmask(bitlen);
                    if let Some(value) = as_number(value) {
                        statics.push((offset, ((value as u32) >> shift) & mask));
                    } else {
                        dynamics.push((offset, quote_spanned!{ value.span()=>
                            (#value >> #shift) & #mask
                        }));
                    }
                },
                Command::Ulist(offset, options) => {
                    if let Some(number) = as_number(value) {
                        if let Some(i) = options.iter().rposition(|&n| n as u64 == number) {
                            statics.push((offset, i as u32));
                        } else {
                            emit_error_at(value.span(), "Impossible value".into());
                            return Err(None);
                        }
                    } else {
                        dynamics.push((offset, quote_spanned!{ value.span()=>
                            [#(#options),*].iter().rposition(|&n| n as u32 == #value).expect("impossible value") as u32
                        }));
                    }
                },
                Command::Urange(offset, min, max) => {
                    let max = max as u32;
                    let min = min as u32;
                    if let Some(value) = unsigned_rangecheck(value, min, max, 0) {
                        statics.push((offset, value? - min));
                    } else {
                        let range = max - min;
                        let mask = range.next_power_of_two() - 1;
                        dynamics.push((offset, quote_spanned!{ value.span()=>
                            (#value - #min) & #mask
                        }));
                    }
                },
                Command::Usub(offset, bitlen, addval) => {
                    let mask = bitmask(bitlen);
                    let addval = addval as u32;
                    if let Some(value) = unsigned_rangecheck(value, addval - mask, addval, 0) {
                        statics.push((offset, addval - value?));
                    } else {
                        dynamics.push((offset, quote_spanned!{ value.span()=> 
                            (#addval - #value) & #mask
                        }));
                    }
                },
                Command::Unegmod(offset, bitlen) => {
                    let mask = bitmask(bitlen);
                    let addval = 1u32 << bitlen;
                    if let Some(value) = unsigned_rangecheck(value, 0, mask, 0) {
                        statics.push((offset, (addval - value?) & mask));
                    } else {
                        dynamics.push((offset, quote_spanned!{ value.span()=> 
                            (#addval - #value) & #mask
                        }));
                    }
                },
                Command::Usumdec(offset, bitlen) => {
                    let mask = bitmask(bitlen);
                    if let Some(FlatArg::Immediate {value: leftvalue } ) = data.args.get(cursor - 1) {
                        dynamics.push((offset, quote_spanned!{ value.span()=> 
                            (#leftvalue + #value - 1) & #mask
                        }));
                    } else {
                        panic!("Bad encoding data, previous argument was not an immediate");
                    }
                },
                Command::Ufields(bitfields) => {
                    let mask = bitmask(bitfields.len() as u8);
                    if let Some(value) = unsigned_rangecheck(value, 0, mask, 0) {
                        let value = value?;
                        for (i, &field) in bitfields.iter().rev().enumerate() {
                            statics.push((field as u8, (value >> i) & 1));
                        }
                    } else {
                        for (i, &field) in bitfields.iter().rev().enumerate() {
                            dynamics.push((field as u8, quote_spanned!{ value.span()=> 
                                (#value >> #i) & 1
                            }));
                        }
                    }
                },

                // signed integer encoding
                Command::Sbits(offset, bitlen) => {
                    let mask = bitmask(bitlen);
                    let half = -1i32 << (bitlen - 1);
                    if let Some(value) = signed_rangecheck(value, half, mask as i32 + half, 0) {
                        statics.push((offset, (value? as u32) & mask));
                    } else {
                        dynamics.push((offset, quote_spanned!{ value.span()=>
                            (#value as u32) & #mask
                        }));
                    }
                },
                Command::Sscaled(offset, bitlen, shift) => {
                    let mask = bitmask(bitlen);
                    let half = -1i32 << (bitlen - 1);
                    if let Some(value) = signed_rangecheck(value, half, mask as i32 - half, shift) {
                        statics.push((offset, (value? as u32) & mask));
                    } else {
                        dynamics.push((offset, quote_spanned!{ value.span()=>
                            ((#value >> #shift) as u32) & #mask
                        }));
                    }
                },
                Command::Sslice(offset, bitlen, shift) => {
                    let mask = bitmask(bitlen);
                    if let Some(value) = as_signed_number(value) {
                        statics.push((offset, ((value >> shift) as u32) & mask));
                    } else {
                        dynamics.push((offset, quote_spanned!{ value.span()=>
                            ((#value >> #shift) as u32) & #mask
                        }));
                    }
                },

                // nonconsuming integer checks
                Command::BUbits(bitlen) => {
                    let mask = bitmask(bitlen);
                    if let Some(value) = unsigned_rangecheck(value, 0, mask, 0) {
                        value?;
                    }
                },
                Command::BUsum(bitlen) => {
                    let prev = if let Some(FlatArg::Immediate {value: leftvalue } ) = data.args.get(cursor - 1) {
                        leftvalue
                    } else {
                        panic!("Bad encoding data, previous argument was not an immediate");
                    };
                    let mut max = 1u32 << bitlen;
                    if let Some(value) = as_number(prev) {
                        max -= value as u32;
                    }
                    if let Some(value) = unsigned_rangecheck(value, 1, max, 0) {
                        value?;
                    }
                },
                Command::BSscaled(bitlen, shift) => {
                    let mask = bitmask(bitlen);
                    let half = -1i32 << (bitlen - 1);
                    if let Some(value) = signed_rangecheck(value, half, mask as i32 + half, shift) {
                        value?;
                    }
                },
                Command::BUrange(min, max) => {
                    let min = min as u32;
                    let max = max as u32;
                    if let Some(value) = unsigned_rangecheck(value, min, max, 0) {
                        value?;
                    }
                },

                // specials. These have some more involved code.
                Command::Special(offset, special) => handle_special_immediates(offset, special, value, &mut statics, &mut dynamics)?,

                _ => panic!("Invalid argument processor")
            },
            FlatArg::Default => match *command {
                // Registers default to R31
                Command::R(offset) => {
                    statics.push((offset, 0b11111u32));
                },

                // modifiers to LSL
                Command::Rotates(offset) => {
                    statics.push((offset, 0b00));
                },
                Command::ExtendsW(offset) => {
                    statics.push((offset, 0b010));
                },
                Command::ExtendsX(offset) => {
                    statics.push((offset, 0b011));
                },

                // normal integer encodings default to 0 (i.e. not doing anything)
                // however encoders for which 0 is not necessarily a valid value cannot match default
                Command::Ubits(_, _) |
                Command::Uscaled(_, _, _) |
                Command::Uslice(_, _, _) |
                Command::Urange(_, _, _) |
                Command::Ulist(_, _) |
                Command::Ufields(_) |
                Command::Sbits(_, _) |
                Command::Sscaled(_, _, _) |
                Command::Sslice(_, _, _) => (),

                // integer checks don't have anything to check
                Command::BUbits(_) |
                Command::BSscaled(_, _) => (),

                _ => panic!("Invalid argument processor")
            },
            FlatArg::JumpTarget { type_: ref target } => match *command {
                Command::Offset(relocation) => {
                    // what kind of relocation is it
                    let data = [relocation.to_id()];

                    // encode the complete relocation
                    let stmt = target.clone().encode(&data);

                    relocations.push(stmt);
                }
                _ => panic!("Invalid argument processor")
            },
            FlatArg::Lit { ref ident } => match *command {

                // Condition codes, literals
                Command::Cond(offset) => {
                    let name = ident.to_string();
                    let bits = *COND_MAP.get(&&*name).expect("bad command data");
                    statics.push((offset, bits as u32))
                },
                Command::CondInv(offset) => {
                    let name = ident.to_string();
                    let bits = *COND_MAP.get(&&*name).expect("bad command data");
                    statics.push((offset, bits as u32 ^ 1))
                },
                Command::LitList(offset, listname) => {
                    let name = ident.to_string();
                    let list = SPECIAL_IDENT_MAP.get(listname).expect("bad command data");
                    if let Some(&bits) = list.get(&&*name) {
                        statics.push((offset, bits));
                    } else {
                        emit_error_at(ident.span(), "Unknown literal".into());
                        return Err(None);
                    }
                },
                _ => panic!("Invalid argument processor")
            }
        }

        // figure out how far the cursor has to be advanced.
        match *command {
            Command::Uslice(_, _, _) |
            Command::Sslice(_, _, _) => (),
            Command::BUbits(_) |
            Command::BUsum(_) |
            Command::BSscaled(_, _) |
            Command::BUrange(_, _) => (),
            _ => cursor += 1
        }
    }

    // sanity
    if cursor != data.args.len() {
        panic!("Not enough command processors");
    }

    // apply all statics to bits
    let mut bits = data.data.base;
    for (offset, value) in statics {
        bits |= value << offset;
    }

    // generate code to be emitted for dynamics
    if !dynamics.is_empty() {
        let mut res = quote!{
            #bits
        };
        for (offset, expr) in dynamics {
            res = quote!{
                #res | ((#expr) << #offset)
            };
        }
        ctx.state.stmts.push(Stmt::ExprUnsigned(delimited(res), Size::DWORD));
    } else {
        ctx.state.stmts.push(Stmt::Const(bits as u64, Size::DWORD));
    }

    // generate code to be emitted for relocations
    ctx.state.stmts.extend(relocations);

    Ok(())
}

fn handle_special_immediates(offset: u8, special: SpecialComm, imm: &syn::Expr, statics: &mut Vec<(u8, u32)>, dynamics: &mut Vec<(u8, TokenStream)>) -> Result<(), Option<String>> {
    match special {
        SpecialComm::INVERTED_WIDE_IMMEDIATE_X => if let Some(number) = as_number(imm) {
            if let Some(encoded) = encoding_helpers::encode_wide_immediate_64bit(!number) {
                statics.push((offset, encoded as u32));
                return Ok(());
            }
        } else {
            dynamics.push((offset, quote_spanned!{ imm.span()=>
                {
                    let value: u64 = !#imm;
                    let offset = value.trailing_zeros() & 0b110000;
                    ((0xFFFFu64 & (value >> offset)) as u32) | (offset << 12)
                }
            }));
            return Ok(());
        },
        SpecialComm::INVERTED_WIDE_IMMEDIATE_W => if let Some(number) = as_number(imm) {
            if number <= std::u32::MAX as u64 {
                if let Some(encoded) = encoding_helpers::encode_wide_immediate_32bit(!(number as u32)) {
                    statics.push((offset, encoded as u32));
                    return Ok(());
                }
            }
        } else {
            dynamics.push((offset, quote_spanned!{ imm.span()=>
                {
                    let value: u64 = !#imm;
                    let offset = value.trailing_zeros() & 0b10000;
                    ((0xFFFFu64 & (value >> offset)) as u32) | (offset << 12)
                }
            }));
            return Ok(());
        },
        SpecialComm::WIDE_IMMEDIATE_X => if let Some(number) = as_number(imm) {
            if let Some(encoded) = encoding_helpers::encode_wide_immediate_64bit(number) {
                statics.push((offset, encoded as u32));
                return Ok(());
            }
        } else {
            dynamics.push((offset, quote_spanned!{ imm.span()=>
                {
                    let value: u64 = #imm;
                    let offset = value.trailing_zeros() & 0b110000;
                    ((0xFFFFu64 & (value >> offset)) as u32) | (offset << 12)
                }
            }));
            return Ok(());
        },
        SpecialComm::WIDE_IMMEDIATE_W => if let Some(number) = as_number(imm) {
            if number <= std::u32::MAX as u64 {
                if let Some(encoded) = encoding_helpers::encode_wide_immediate_32bit(number as u32) {
                    statics.push((offset, encoded as u32));
                    return Ok(());
                }
            }
        } else {
            dynamics.push((offset, quote_spanned!{ imm.span()=>
                {
                    let value: u64 = #imm;
                    let offset = value.trailing_zeros() & 0b10000;
                    ((0xFFFFu64 & (value >> offset)) as u32) | (offset << 12)
                }
            }));
            return Ok(());
        },
        SpecialComm::STRETCHED_IMMEDIATE => if let Some(number) = as_number(imm) {
            if let Some(encoded) = encoding_helpers::encode_stretched_immediate(number) {
                statics.push((offset, encoded & 0x1F as u32));
                statics.push((offset + 6, encoded & 0xE0 as u32));
                return Ok(());
            }
        } else {
            dynamics.push((offset, quote_spanned!{ imm.span()=>
                {
                    let value: u64 = #imm;
                    let mut masked = value & 0x8040201008040201;
                    masked |= masked >> 32;
                    masked |= masked >> 16;
                    masked |= masked >> 8;
                    let masked = masked as u32;
                    ((masked & 0xE0) << 6) | (masked & 0x1F) 
                }
            }));
            return Ok(());
        },
        SpecialComm::LOGICAL_IMMEDIATE_W => if let Some(number) = as_number(imm) {
            if number <= std::u32::MAX as u64 {
                if let Some(encoded) = encoding_helpers::encode_logical_immediate_32bit(number as u32) {
                    statics.push((offset, encoded as u32));
                    return Ok(());
                }
            }
        } else {
            dynamics.push((offset, quote_spanned!{ imm.span()=>
                dynasmrt::aarch64::encode_logical_immediate_32bit(#imm).expect("Impossible logical immediate") as u32
            }));
            return Ok(());
        },
        SpecialComm::LOGICAL_IMMEDIATE_X => if let Some(number) = as_number(imm) {
            if let Some(encoded) = encoding_helpers::encode_logical_immediate_64bit(number) {
                statics.push((offset, encoded as u32));
                return Ok(());
            }
        } else {
            dynamics.push((offset, quote_spanned!{ imm.span()=>
                dynasmrt::aarch64::encode_logical_immediate_64bit(#imm).expect("Impossible logical immediate") as u32
            }));
            return Ok(());
        },
        SpecialComm::FLOAT_IMMEDIATE => if let Some(number) = as_float(imm) {
            if let Some(encoded) = encoding_helpers::encode_floating_point_immediate(number as f32) {
                statics.push((offset, encoded as u32));
                return Ok(());
            }
        } else {
            dynamics.push((offset, quote_spanned!{ imm.span()=>
                {
                    let value: f32 = #imm;
                    let bits = value.to_bits();
                    ((bits >> 24) & 0x80) | ((bits >> 19) & 0x7F)
                }
            }));
            return Ok(());
        },
        SpecialComm::SPLIT_FLOAT_IMMEDIATE => if let Some(number) = as_float(imm) {
            if let Some(encoded) = encoding_helpers::encode_floating_point_immediate(number as f32) {
                statics.push((offset, (encoded & 0x1F) as u32));
                statics.push((offset + 6, (encoded & 0xE0) as u32));
                return Ok(());
            }
        } else {
            dynamics.push((offset, quote_spanned!{ imm.span()=>
                {
                    let value: f32 = #imm;
                    let bits = value.to_bits();
                    ((bits >> 18) & 0x80) | ((bits >> 13) & 0x60) | ((bits >> 19) & 0x1F)
                }
            }));
            return Ok(());
        },
    }

    emit_error_at(imm.span(), "Impossible to encode immediate".into());
    return Err(None);
}

fn unsigned_rangecheck(expr: &syn::Expr, min: u32, max: u32, scale: u8) -> Option<Result<u32, Option<String>>> {
    let value = as_number(expr)?;
    let scaled = value >> scale;

    Some(if (scaled << scale) != value {
        emit_error_at(expr.span(), "Unrepresentable value".into());
        Err(None)
    } else if scaled > max as u64 {
        emit_error_at(expr.span(), "Value too large".into());
        Err(None)
    } else if scaled < min as u64 {
        emit_error_at(expr.span(), "Value too small".into());
        Err(None)
    } else {
        Ok(scaled as u32)
    })
}

fn signed_rangecheck(expr: &syn::Expr, min: i32, max: i32, scale: u8) -> Option<Result<i32, Option<String>>> {
    let value = as_signed_number(expr)?;
    let scaled = value >> scale;

    Some(if (scaled << scale) != value {
        emit_error_at(expr.span(), "Unrepresentable value".into());
        Err(None)
    } else if scaled > max as i64 {
        emit_error_at(expr.span(), "Value too large".into());
        Err(None)
    } else if scaled < min as i64 {
        emit_error_at(expr.span(), "Value too small".into());
        Err(None)
    } else {
        Ok(scaled as i32)
    })
}
