use super::matching::MatchData;
use super::aarch64data::{Command, COND_MAP, SPECIAL_IDENT_MAP, SpecialComm, Relocation};
use super::Context;
use super::ast::{FlatArg, RegKind, RegId, Modifier};
use super::encoding_helpers;

use crate::common::{Stmt, Size, delimited, bitmask};
use crate::parse_helpers::{as_ident, as_unsigned_number, as_float, as_signed_number};

use syn::spanned::Spanned;
use quote::{quote, quote_spanned};
use proc_macro2::{TokenStream, Span};
use proc_macro_error2::emit_error;

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
                    statics.push((offset, u32::from(id.code())));
                },
                Command::REven(offset) => {
                    if id.code() & 1 != 0 {
                        emit_error!(span, "Field only supports even registers");
                        return Err(None);
                    }
                    statics.push((offset, u32::from(id.code())));
                },
                Command::RNoZr(offset) => {
                    if id.code() == 31 {
                        emit_error!(span, "Field does not support register the zr/sp register");
                        return Err(None);
                    }
                    statics.push((offset, u32::from(id.code())));
                },
                Command::R4(offset) => {
                    if id.code() >= 16 {
                        emit_error!(span, "Field only supports register numbers 0-15");
                        return Err(None);
                    }
                    statics.push((offset, u32::from(id.code())));
                },
                Command::RNext => {
                    if let Some(FlatArg::Direct { span: _prevspan, reg: ref prevreg } ) = data.args.get(cursor - 1) {
                        match prevreg {
                            RegKind::Static(previd) => if id.code() != ((previd.code() + 1) % 32) {
                                emit_error!(span, "Invalid register. This register has to be the register after the previous argument.");
                                return Err(None);
                            },
                            RegKind::Dynamic(_, _) => if id != RegId::XZR {
                                emit_error!(span, "Please use XZR here to indicate that it should be the register after the previous argument.");
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
                    emit_error!(span, "This register is constrained to be the register after the previous argument's register. As such, it does not support dynamic registers. Please substitute it with XZR to indicate this");
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
                    statics.push((offset, u32::from(bits)))
                },
                Command::CondInv(offset) => {
                    let name = as_ident(value).expect("bad command data").to_string();
                    let bits = *COND_MAP.get(&&*name).expect("bad command data");
                    statics.push((offset, u32::from(bits) ^ 1))
                },
                Command::LitList(offset, listname) => {
                    let name = as_ident(value).expect("bad command data").to_string();
                    let list = SPECIAL_IDENT_MAP.get(listname).expect("bad command data");
                    if let Some(&bits) = list.get(&&*name) {
                        statics.push((offset, bits));
                    } else {
                        emit_error!(value, "Unknown literal");
                        return Err(None);
                    }
                },

                // unsigned immediate encodings

                Command::Ubits(offset, bitlen) => {
                    let mask = bitmask(bitlen);

                    if let Some((biased, _)) = static_range_check(value, 0, mask, 0)? {
                        statics.push((offset, biased));

                    } else {
                        let check = dynamic_range_check_unsigned(value.span(), 0, mask, 0);

                        dynamics.push((offset, quote_spanned!{ value.span()=>
                            { let _dyn_imm: u32 = #value; #check; _dyn_imm & #mask }
                        }));
                    }
                },
                Command::Uscaled(offset, bitlen, shift) => {
                    let mask = bitmask(bitlen);

                    if let Some((biased, _)) = static_range_check(value, 0, mask, shift)? {
                        statics.push((offset, biased));

                    } else {
                        let check = dynamic_range_check_unsigned(value.span(), 0, mask, shift);

                        dynamics.push((offset, quote_spanned!{ value.span()=>
                            { let _dyn_imm: u32 = #value; #check; (_dyn_imm >> #shift) & #mask }
                        }));
                    }
                },
                Command::Uslice(offset, bitlen, shift) => {
                    let mask = bitmask(bitlen);

                    if let Some(value) = as_unsigned_number(value) {
                        statics.push((offset, ((value as u32) >> shift) & mask));

                    } else {
                        dynamics.push((offset, quote_spanned!{ value.span()=>
                            { let _dyn_imm: u32 = #value; (#value >> #shift) & #mask }
                        }));
                    }
                },
                Command::Ulist(offset, options) => {
                    if let Some(number) = as_unsigned_number(value) {
                        if let Some(i) = options.iter().rposition(|&n| u64::from(n) == number) {
                            statics.push((offset, i as u32));

                        } else {
                            emit_error!(value, "Invalid immediate '{}'", number);
                            return Err(None);
                        }
                    } else {
                        dynamics.push((offset, quote_spanned!{ value.span()=>
                            {
                                let _dyn_imm = #value;
                                [#(#options),*].iter().rposition(|&n| n as u32 == _dyn_imm)
                                    .unwrap_or_else(|| ::dynasmrt::aarch64::immediate_out_of_range_unsigned_32(_dyn_imm)) as u32
                            }
                        }));
                    }
                },
                Command::Urange(offset, min, max) => {
                    let range = u32::from(max - min);
                    let min = u32::from(min);
                    if let Some((biased, _)) = static_range_check(value, min as i32, range, 0)? {
                        statics.push((offset, biased));

                    } else {
                        let check = dynamic_range_check_unsigned(value.span(), min, range, 0);

                        let mask = (range + 1).next_power_of_two() - 1;

                        dynamics.push((offset, quote_spanned!{ value.span()=>
                            { let _dyn_imm: u32 = #value; #check; (_dyn_imm - #min) & #mask }
                        }));
                    }
                },
                Command::Usubone(offset, bitlen) => {
                    let mask = bitmask(bitlen);

                    if let Some((biased, _)) = static_range_check(value, 1, mask, 0)? {
                        statics.push((offset, mask - biased));

                    } else {
                        let check = dynamic_range_check_unsigned(value.span(), 1, mask, 0);

                        let top = mask + 1;
                        dynamics.push((offset, quote_spanned!{ value.span()=>
                            { let _dyn_imm: u32 = #value; #check; (#top - _dyn_imm) & #mask }
                        }));
                    }
                },
                Command::Usubzero(offset, bitlen) => {
                    let mask = bitmask(bitlen);

                    if let Some((biased, _)) = static_range_check(value, 0, mask, 0)? {
                        statics.push((offset, mask - biased));

                    } else {
                        let check = dynamic_range_check_unsigned(value.span(), 0, mask, 0);

                        dynamics.push((offset, quote_spanned!{ value.span()=>
                            { let _dyn_imm: u32 = #value; #check; (#mask - _dyn_imm) & #mask }
                        }));
                    }
                },
                Command::Usubmod(offset, bitlen) => {
                    let mask = bitmask(bitlen);

                    if let Some((biased, _)) = static_range_check(value, 0, mask, 0)? {
                        statics.push((offset, biased.wrapping_neg() & mask));

                    } else {
                        let check = dynamic_range_check_unsigned(value.span(), 0, mask, 0);

                        dynamics.push((offset, quote_spanned!{ value.span()=>
                            { let _dyn_imm: u32 = #value; #check; _dyn_imm.wrapping_neg() & #mask }
                        }));
                    }
                },
                Command::Usum(offset, bitlen) => {
                    let mask = bitmask(bitlen);
                    let prev_value = if let Some(FlatArg::Immediate {value: prev_value } ) = data.args.get(cursor - 1) {
                        prev_value
                    } else {
                        panic!("Bad encoding data, previous argument was not an immediate");
                    };

                    let number = if let Some(prev_number) = as_unsigned_number(prev_value) {
                        if prev_number > mask as u64 {
                            emit_error!(prev_value, "Impossible immediate combination");
                            return Err(None);
                        };

                        if let Some((biased, _)) = static_range_check(value, 1, mask - (prev_number as u32), 0)? {
                            Some(biased + (prev_number as u32))
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    if let Some(number) = number {
                        statics.push((offset, number & mask));
                    } else {
                        let check = quote_spanned!{ value.span()=>
                            if (#value - 1u32) > (#mask - #prev_value) { ::dynasmrt::aarch64::immediate_out_of_range_unsigned_32(#value); }
                        };

                        dynamics.push((offset, quote_spanned!{ value.span()=>
                            { let _dyn_imm: u32 = #value; #check; (#prev_value + _dyn_imm - 1) & #mask }
                        }));
                    }
                },
                Command::Ufields(bitfields) => {
                    let mask = bitmask(bitfields.len() as u8);

                    if let Some((biased, _)) = static_range_check(value, 0, mask, 0)? {
                        for (i, &field) in bitfields.iter().rev().enumerate() {
                            statics.push((field as u8, (biased >> i) & 1));
                        }

                    } else {
                        let check = dynamic_range_check_unsigned(value.span(), 0, mask, 0);

                        for (i, &field) in bitfields.iter().rev().enumerate() {
                            if i == 0 {
                                dynamics.push((field as u8, quote_spanned!{ value.span()=>
                                    { let _dyn_imm: u32 = #value; #check; (_dyn_imm >> #i) & 1 }
                                }));

                            } else {
                                dynamics.push((field as u8, quote_spanned!{ value.span()=>
                                    (#value >> #i) & 1
                                }));
                            }
                        }
                    }
                },

                // signed immediate encoding

                Command::Sbits(offset, bitlen) => {
                    let mask = bitmask(bitlen);
                    let half = -1i32 << (bitlen - 1);

                    if let Some((_, scaled)) = static_range_check(value, half, mask, 0)? {
                        statics.push((offset, scaled & mask));

                    } else {
                        let check = dynamic_range_check_signed(value.span(), half, mask, 0);

                        dynamics.push((offset, quote_spanned!{ value.span()=>
                            { let _dyn_imm: i32 = #value; #check; (_dyn_imm as u32) & #mask }
                        }));
                    }
                },
                Command::Sscaled(offset, bitlen, shift) => {
                    let mask = bitmask(bitlen);
                    let half = -1i32 << (bitlen - 1);

                    if let Some((_, scaled)) = static_range_check(value, half, mask, shift)? {
                        statics.push((offset, scaled & mask));

                    } else {
                        let check = dynamic_range_check_signed(value.span(), half, mask, shift);

                        dynamics.push((offset, quote_spanned!{ value.span()=>
                            { let _dyn_imm: i32 = #value; #check; ((_dyn_imm >> #shift) as u32) & #mask }
                        }));
                    }
                },
                Command::Sslice(offset, bitlen, shift) => {
                    let mask = bitmask(bitlen);

                    if let Some(value) = as_signed_number(value) {
                        statics.push((offset, ((value >> shift) as u32) & mask));

                    } else {
                        dynamics.push((offset, quote_spanned!{ value.span()=>
                            { let _dyn_imm: i32 = #value; ((_dyn_imm >> #shift) as u32) & #mask }
                        }));
                    }
                },

                // pure validation encodings.

                Command::CUbits(bitlen) => {
                    let mask = bitmask(bitlen);

                    if static_range_check(value, 0, mask, 0)?.is_none() {
                        let check = dynamic_range_check_unsigned(value.span(), 0, mask, 0);

                        dynamics.push((0, quote_spanned! { value.span()=>
                            { let _dyn_imm: u32 = #value; #check; 0 }
                        }));
                    }
                },
                Command::CUsum(bitlen) => {
                    let mask = bitmask(bitlen);

                    let prev_value = if let Some(FlatArg::Immediate {value: prev_value } ) = data.args.get(cursor - 1) {
                        prev_value
                    } else {
                        panic!("Bad encoding data, previous argument was not an immediate");
                    };

                    let check = if let Some(prev_number) = as_unsigned_number(prev_value) {
                        if prev_number > mask as u64 {
                            emit_error!(prev_value, "Impossible immediate combination");
                            return Err(None);
                        };

                        static_range_check(value, 1, mask - (prev_number as u32), 0)?
                    } else {
                        None
                    };

                    if check.is_none() {

                        let check = quote_spanned!{ value.span()=>
                            if (#value - 1u32) > (#mask - #prev_value) { ::dynasmrt::aarch64::immediate_out_of_range_unsigned_32(#value); }
                        };

                        dynamics.push((0, quote_spanned! { value.span()=>
                            { #check; 0}
                        }));
                    }
                },
                Command::CSscaled(bitlen, shift) => {
                    let mask = bitmask(bitlen);
                    let half = -1i32 << (bitlen - 1);

                    if static_range_check(value, half, mask, shift)?.is_none() {
                        let check = dynamic_range_check_signed(value.span(), half, mask, shift);
                        dynamics.push((0, quote_spanned! { value.span()=>
                            { let _dyn_imm: i32 = #value; #check; 0 }
                        }));
                    }
                },
                Command::CUrange(min, max) => {
                    let range = u32::from(max - min);
                    let min = u32::from(min);

                    if static_range_check(value, min as i32, range, 0)?.is_none() {
                        let check = dynamic_range_check_unsigned(value.span(), min, range, 0);
                        dynamics.push((0, quote_spanned! { value.span()=>
                            { let _dyn_imm: u32 = #value; #check; 0 }
                        }));
                    }
                },

                // specials. These have some more involved code.
                Command::Special(offset, special) => handle_special_immediates(offset, special, value, &mut statics, &mut dynamics)?,

                // jump targets also accept immediates
                Command::Offset(relocation) => match relocation {
                     // b, bl 26 bits, dword aligned
                    Relocation::B => {
                        let bits = 26;
                        let mask = bitmask(bits);
                        let half = -1i32 << (bits - 1);
                        if let Some((_, scaled)) = static_range_check(value, half, mask, 2)? {
                            statics.push((0, scaled & mask));

                        } else {
                            let check = dynamic_range_check_signed(value.span(), half, mask, 2);

                            dynamics.push((0, quote_spanned!{ value.span()=>
                                { let _dyn_imm: i32 = #value; #check; ((_dyn_imm >> 2u8) as u32) & #mask }
                            }));
                        }
                    },
                    // b.cond, cbnz, cbz, ldr, ldrsw, prfm: 19 bits, dword aligned
                    Relocation::BCOND => {
                        let bits = 19;
                        let mask = bitmask(bits);
                        let half = -1i32 << (bits - 1);
                        if let Some((_, scaled)) = static_range_check(value, half, mask, 2)? {
                            statics.push((5, scaled & mask));

                        } else {
                            let check = dynamic_range_check_signed(value.span(), half, mask, 2);

                            dynamics.push((5, quote_spanned!{ value.span()=>
                                { let _dyn_imm: i32 = #value; #check; ((_dyn_imm >> 2u8) as u32) & #mask }
                            }));
                        }
                    },
                    // adr split 21 bit, byte aligned
                    Relocation::ADR => {
                        let bits = 21;
                        let mask = bitmask(bits);
                        let half = -1i32 << (bits - 1);
                        if let Some((_, scaled)) = static_range_check(value, half, mask, 0)? {
                            statics.push((5, (scaled >> 2u8) & 0x7FFFFu32));
                            statics.push((29, scaled & 3u32));

                        } else {
                            let check = dynamic_range_check_signed(value.span(), half, mask, 0);
                            dynamics.push((5, quote_spanned!{ value.span()=>
                                { let _dyn_imm: i32 = #value; #check; ((_dyn_imm >> 2u8) as u32) & 0x7FFFFu32 }
                            }));
                            dynamics.push((29, quote_spanned!{ value.span()=>
                                (#value as u32) & 3u32
                            }));
                        }
                    },
                    // adrp split 21 bit, 4096-byte aligned
                    Relocation::ADRP => {
                        let bits = 21;
                        let mask = bitmask(bits);
                        let half = -1i32 << (bits - 1);
                        if let Some((_, scaled)) = static_range_check(value, half, mask, 12)? {
                            statics.push((5, scaled & 0x7FFFF));
                            statics.push((29, scaled & 3));

                        } else {
                            let check = dynamic_range_check_signed(value.span(), half, mask, 12);
                            dynamics.push((5, quote_spanned!{ value.span()=>
                                { let _dyn_imm: i32 = #value; #check; ((_dyn_imm >> 14u8) as u32) & 0x7FFFFu32 }
                            }));
                            dynamics.push((29, quote_spanned!{ value.span()=>
                                ((#value >> 12u8) as u32) & 3u32
                            }));
                        }
                    },
                    // tbnz, tbz: 14 bits, dword aligned
                    Relocation::TBZ => {
                        let bits = 14;
                        let mask = bitmask(bits);
                        let half = -1i32 << (bits - 1);
                        if let Some((_, scaled)) = static_range_check(value, half, mask, 2)? {
                            statics.push((5, scaled & mask));

                        } else {
                            let check = dynamic_range_check_signed(value.span(), half, mask, 2);
                            dynamics.push((5, quote_spanned!{ value.span()=>
                                { let _dyn_imm: i32 = #value; #check; ((_dyn_imm >> 2) as u32) & #mask }
                            }));
                        }
                    },
                    Relocation::LITERAL8
                    | Relocation::LITERAL16
                    | Relocation::LITERAL32
                    | Relocation::LITERAL64 => ()
                },

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
                Command::CUbits(_) |
                Command::CSscaled(_, _) => (),

                _ => panic!("Invalid argument processor")
            },
            FlatArg::JumpTarget { ref jump } => match *command {
                Command::Offset(relocation) => {
                    // encode the complete relocation. Always starts at the begin of the instruction, and also relative to that
                    let stmt = jump.clone().encode(4, 4, &[relocation.to_id()]);

                    relocations.push(stmt);
                },
                _ => panic!("Invalid argument processor")
            },
            FlatArg::Lit { ref ident } => match *command {

                // Condition codes, literals
                Command::Cond(offset) => {
                    let name = ident.to_string();
                    let bits = *COND_MAP.get(&&*name).expect("bad command data");
                    statics.push((offset, u32::from(bits)))
                },
                Command::CondInv(offset) => {
                    let name = ident.to_string();
                    let bits = *COND_MAP.get(&&*name).expect("bad command data");
                    statics.push((offset, u32::from(bits) ^ 1))
                },
                Command::LitList(offset, listname) => {
                    let name = ident.to_string();
                    let list = SPECIAL_IDENT_MAP.get(listname).expect("bad command data");
                    if let Some(&bits) = list.get(&&*name) {
                        statics.push((offset, bits));
                    } else {
                        emit_error!(ident, "Unknown literal");
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
            Command::CUbits(_) |
            Command::CUsum(_) |
            Command::CSscaled(_, _) |
            Command::CUrange(_, _) => (),
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
        ctx.state.stmts.push(Stmt::ExprUnsigned(delimited(res), Size::B_4));
    } else {
        ctx.state.stmts.push(Stmt::Const(u64::from(bits), Size::B_4));
    }

    // generate code to be emitted for relocations
    ctx.state.stmts.extend(relocations);

    Ok(())
}

fn handle_special_immediates(offset: u8, special: SpecialComm, imm: &syn::Expr, statics: &mut Vec<(u8, u32)>, dynamics: &mut Vec<(u8, TokenStream)>) -> Result<(), Option<String>> {
    match special {
        SpecialComm::INVERTED_WIDE_IMMEDIATE_X => if let Some(number) = None::<u64> { // as_unsigned_number(imm) {
            if let Some(encoded) = encoding_helpers::encode_wide_immediate_64bit(!number) {
                statics.push((offset, encoded as u32));
                return Ok(());
            }
        } else {
            dynamics.push((offset, quote_spanned!{ imm.span()=>
                {
                    let value: u64 = !#imm;
                    let offset = value.trailing_zeros() & 0b110000;

                    if (value & !(0xFFFFu64 << offset)) != 0 {
                        ::dynasmrt::aarch64::immediate_out_of_range_unsigned_64(!value);
                    }

                    ((0xFFFFu64 & (value >> offset)) as u32) | (offset << 12)
                }
            }));
            return Ok(());
        },
        SpecialComm::INVERTED_WIDE_IMMEDIATE_W => if let Some(number) = as_unsigned_number(imm) {
            if number <= u64::from(u32::MAX) {
                if let Some(encoded) = encoding_helpers::encode_wide_immediate_32bit(!(number as u32)) {
                    statics.push((offset, encoded as u32));
                    return Ok(());
                }
            }
        } else {
            dynamics.push((offset, quote_spanned!{ imm.span()=>
                {
                    let value: u32 = !#imm;
                    let offset = value.trailing_zeros() & 0b10000;

                    if (value & !(0xFFFFu32 << offset)) != 0 {
                        ::dynasmrt::aarch64::immediate_out_of_range_unsigned_32(!value);
                    }

                    (0xFFFFu32 & (value >> offset)) | (offset << 12)
                }
            }));
            return Ok(());
        },
        SpecialComm::WIDE_IMMEDIATE_X => if let Some(number) = as_unsigned_number(imm) {
            if let Some(encoded) = encoding_helpers::encode_wide_immediate_64bit(number) {
                statics.push((offset, encoded as u32));
                return Ok(());
            }
        } else {
            dynamics.push((offset, quote_spanned!{ imm.span()=>
                {
                    let value: u64 = #imm;
                    let offset = value.trailing_zeros() & 0b110000;

                    if (value & !(0xFFFFu64 << offset)) != 0 {
                        ::dynasmrt::aarch64::immediate_out_of_range_unsigned_64(value);
                    }

                    ((0xFFFFu64 & (value >> offset)) as u32) | (offset << 12)
                }
            }));
            return Ok(());
        },
        SpecialComm::WIDE_IMMEDIATE_W => if let Some(number) = as_unsigned_number(imm) {
            if number <= u64::from(u32::MAX) {
                if let Some(encoded) = encoding_helpers::encode_wide_immediate_32bit(number as u32) {
                    statics.push((offset, encoded as u32));
                    return Ok(());
                }
            }
        } else {
            dynamics.push((offset, quote_spanned!{ imm.span()=>
                {
                    let value: u32 = #imm;
                    let offset = value.trailing_zeros() & 0b10000;

                    if (value & !(0xFFFFu32 << offset)) != 0 {
                        ::dynasmrt::aarch64::immediate_out_of_range_unsigned_32(value);
                    }

                    (0xFFFFu32 & (value >> offset)) | (offset << 12)
                }
            }));
            return Ok(());
        },
        SpecialComm::STRETCHED_IMMEDIATE => if let Some(number) = as_unsigned_number(imm) {
            if let Some(encoded) = encoding_helpers::encode_stretched_immediate(number) {
                statics.push((offset, encoded & 0x1F as u32));
                statics.push((offset + 6, encoded & 0xE0 as u32));
                return Ok(());
            }
        } else {
            dynamics.push((offset, quote_spanned!{ imm.span()=>
                {
                    let value: u64 = #imm;
                    let mut test = value & 0x0101_0101_0101_0101;
                    test |= test << 1;
                    test |= test << 2;
                    test |= test << 4;
                    if test != value {
                        ::dynasmrt::aarch64::immediate_out_of_range_unsigned_64(value);
                    }
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
        SpecialComm::LOGICAL_IMMEDIATE_W => if let Some(number) = as_unsigned_number(imm) {
            if number <= u64::from(u32::MAX) {
                if let Some(encoded) = encoding_helpers::encode_logical_immediate_32bit(number as u32) {
                    statics.push((offset, u32::from(encoded)));
                    return Ok(());
                }
            }
        } else {
            dynamics.push((offset, quote_spanned!{ imm.span()=>
                dynasmrt::aarch64::encode_logical_immediate_32bit(#imm).unwrap_or_else(
                    || ::dynasmrt::aarch64::immediate_out_of_range_unsigned_32(#imm)) as u32
            }));
            return Ok(());
        },
        SpecialComm::LOGICAL_IMMEDIATE_X => if let Some(number) = as_unsigned_number(imm) {
            if let Some(encoded) = encoding_helpers::encode_logical_immediate_64bit(number) {
                statics.push((offset, u32::from(encoded)));
                return Ok(());
            }
        } else {
            dynamics.push((offset, quote_spanned!{ imm.span()=>
                dynasmrt::aarch64::encode_logical_immediate_64bit(#imm).unwrap_or_else(
                    || ::dynasmrt::aarch64::immediate_out_of_range_unsigned_64(#imm)) as u32
            }));
            return Ok(());
        },
        SpecialComm::FLOAT_IMMEDIATE => if let Some(number) = as_float(imm) {
            if let Some(encoded) = encoding_helpers::encode_floating_point_immediate(number as f32) {
                statics.push((offset, u32::from(encoded)));
                return Ok(());
            }
        } else {
            dynamics.push((offset, quote_spanned!{ imm.span()=>
                {
                    let value: f32 = #imm;
                    let bits = value.to_bits();

                    let check = (bits >> 25) & 0x3F;
                    if (check != 0b10_0000 && check != 0b01_1111) || (bits & 0x7_FFFF) != 0 {
                        ::dynasmrt::aarch64::immediate_out_of_range_unsigned_f32(value);
                    }

                    ((bits >> 24) & 0x80) | ((bits >> 19) & 0x7F)
                }
            }));
            return Ok(());
        },
        SpecialComm::SPLIT_FLOAT_IMMEDIATE => if let Some(number) = as_float(imm) {
            if let Some(encoded) = encoding_helpers::encode_floating_point_immediate(number as f32) {
                statics.push((offset, u32::from(encoded & 0x1F)));
                statics.push((offset + 6, u32::from(encoded & 0xE0)));
                return Ok(());
            }
        } else {
            dynamics.push((offset, quote_spanned!{ imm.span()=>
                {
                    let value: f32 = #imm;
                    let bits = value.to_bits();

                    let check = (bits >> 25) & 0x3F;
                    if (check != 0b10_0000 && check != 0b01_1111) || (bits & 0x7_FFFF) != 0 {
                        ::dynasmrt::aarch64::immediate_out_of_range_unsigned_f32(value);
                    }

                    ((bits >> 18) & 0x20_00) | ((bits >> 13) & 0x18_00) | ((bits >> 19) & 0x1F)
                }
            }));
            return Ok(());
        },
    }

    emit_error!(imm, "Unrepresentable immediate");
    Err(None)
}

/// statically checks the value of expr.
/// the check performed is effectively ((expr >> scale) + bias) <= range
/// as well as that ((expr >> scale) << scale) == expr
/// this function operates only on 32-bit numbers, both signed and unsigned
/// `bias` specifies the lowest value that the expression can have, while `range`
/// specifies the highest value, after bias has been subtracted.
/// if the expression cannot be statically evaluated, `Ok(None) is returned`
/// if it can be evaluated, `Ok(Some(biased, unbiased))` is returned if it meets the range check.
/// else, an `Err` is returned.
fn static_range_check(expr: &syn::Expr, bias: i32, range: u32, scale: u8) -> Result<Option<(u32, u32)>, Option<String>> {
    #![allow(unexpected_cfgs)]

    // signed 64-bit parse is always safe for 32-bit numbers
    let value = match as_signed_number(expr) {
        Some(v) => v,
        None => return Ok(None)
    };

    // this allows turning off static checks for testing purposes
    #[cfg(disable_static_checks="1")]
    return Ok(None);


    // arithmetic right shift
    let scaled: i64 = value >> scale;
    if scaled << scale != value {
        emit_error!(expr, "Unrepresentable immediate");
        return Err(None);
    }

    let biased = scaled - i64::from(bias);
    if biased < 0 {
        emit_error!(expr, "Immediate too small");
        Err(None)
    } else if biased > i64::from(range) {
        emit_error!(expr, "Immediate too large");
        Err(None)
    } else {
        // this cast is always safe
        Ok(Some((biased as u32, scaled as u32)))
    }
}

/// emits the code for a range check on an unsigned immediate.
fn dynamic_range_check_unsigned(span: Span, bias: u32, range: u32, scale: u8) -> TokenStream {
    let check = if scale == 0 {
        if bias == 0 {
            quote_spanned!{ span=> _dyn_imm > #range }
        } else {
            quote_spanned!{ span=> _dyn_imm.wrapping_sub(#bias) > #range }
        }
    } else {
        let mask = bitmask(scale);

        if bias == 0 {
            quote_spanned!{ span=> ((_dyn_imm & #mask) != 0) || (_dyn_imm >> #scale) > #range }
        } else {
            quote_spanned!{ span=> ((_dyn_imm & #mask) != 0) || (_dyn_imm >> #scale).wrapping_sub(#bias) > #range }
        }
    };

    quote_spanned!{ span => if #check { ::dynasmrt::aarch64::immediate_out_of_range_unsigned_32(_dyn_imm); }}
}

/// emits the code for a range check on a signed immediate.
fn dynamic_range_check_signed(span: Span, bias: i32, range: u32, scale: u8) -> TokenStream {
    let bias = -bias;

    let check = if scale == 0 {
        if bias == 0 {
            quote_spanned!{ span => (_dyn_imm as u32) > #range }
        } else {
            quote_spanned!{ span => (_dyn_imm.wrapping_add(#bias) as u32) > #range }
        }
    } else {
        let mask = bitmask(scale) as i32;

        if bias == 0 {
            quote_spanned!{ span=> ((_dyn_imm & #mask) != 0) || ((_dyn_imm >> #scale) as u32) > #range }
        } else {
            quote_spanned!{ span=> ((_dyn_imm & #mask) != 0) || ((_dyn_imm >> #scale).wrapping_add(#bias) as u32) > #range }
        }
    };

    quote_spanned!{ span => if #check { ::dynasmrt::aarch64::immediate_out_of_range_signed_32(_dyn_imm); }}
}
