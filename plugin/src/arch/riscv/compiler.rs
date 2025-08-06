use super::Context;
use super::riscvdata::{Template, Command, Relocation, ROUNDMODE_MAP, FENCESPEC_MAP, CSR_MAP, FP_IMM_IDENT_MAP, FP_IMM_VALUE_MAP};
use super::ast::{MatchData, FlatArg, RegListFlat, Register};

use syn::spanned::Spanned;
use quote::{quote, quote_spanned};
use proc_macro2::{TokenStream, Span, Literal};
use proc_macro_error2::emit_error;

use crate::parse_helpers::{as_signed_number, as_ident, as_float};
use crate::common::{Stmt, Size, delimited, bitmask, bitmask64};

/// Compile a single instruction. Input is taken from `data`, containing both the arguments
/// and the encoding template and commands.
/// Output is written to ctx.state
/// Errors can be emitted to either the whole instruction span (by returning Err(Some(errormsg)))
/// or emited specifically using emit_error! and returning Err(None)
pub(super) fn compile_instruction(ctx: &mut Context, data: MatchData) -> Result<(), Option<String>> {
    // argument cursor
    let mut cursor = 0usize;

    // All static bitfields (compile-time constant) will be encoded into this map of (offset, bitfield)
    let mut statics = Vec::new();
    // All dynamic bitfields (run-time determined) will be encoded into this map of (offset, TokenStream)
    let mut dynamics = Vec::new();
    // Any relocation will be encoded in this list
    let mut relocations = Vec::new();

    for (i, command) in data.data.commands.iter().enumerate() {
        // meta commands
        match *command {

            Command::Repeat => {
                cursor -= 1;
                continue;
            },
            Command::Next => {
                cursor += 1;
                continue;
            },
            _ => ()
        }

        let arg = data.args.get(cursor).expect("Invalid encoding data, tried to process more arguments than given");

        match *arg {
            FlatArg::Register { span, reg: Register::Static(id) } => {
                let mut code = id.code();

                let offset = match *command {
                    Command::R(offset) => offset,
                    Command::Reven(offset) => {
                        if code & 0x1 != 0 {
                            emit_error!(span, "This register must be even numbered");
                            return Err(None);
                        }
                        offset
                    },
                    Command::Rno0(offset) => {
                        if code == 0 {
                            emit_error!(span, "This register must not be x0");
                            return Err(None);
                        }
                        offset
                    },
                    Command::Rno02(offset) => {
                        if code == 0 || code == 2 {
                            emit_error!(span, "This register must not be x0 or x2");
                            return Err(None);
                        }
                        offset
                    },
                    Command::Rpop(offset) => {
                        if code < 8 || code > 15 {
                            emit_error!(span, "This register must be one of x8-x15");
                            return Err(None);
                        }
                        code &= 7;
                        offset
                    },
                    Command::Rpops(offset) => {
                        if ((1 << code) & 0x00_FC_03_00) == 0 {
                            emit_error!(span, "This register must be one of s0-s7 (x8, x9, x18-x23");
                            return Err(None);
                        }
                        code &= 7;
                        offset
                    },
                    Command::Rpops2(offset) => {
                        match data.args.get(cursor - 1) {
                            Some(FlatArg::Register { reg: Register::Static(id2), .. } ) => {
                                if *id2 == id {
                                    emit_error!(span, "This register cannot be identical to the previous argument");
                                    return Err(None);
                                }
                            },
                            Some(FlatArg::Register { reg: Register::Dynamic(_, ref expr), .. }) => {
                                dynamics.push((0, quote_spanned!{ span=>
                                    {
                                        let _dyn_reg: u8 = #expr.into();
                                        if _dyn_reg == #code {
                                            ::dynasmrt::riscv::invalid_register(#code);
                                        }
                                        0u32
                                    }
                                }));
                            },
                            _ => panic!("Invalid encoding data, expected a register before")
                        }

                        if ((1 << code) & 0x00_FC_03_00) == 0 {
                            emit_error!(span, "This register must be one of s0-s7 (x8, x9, x18-x23");
                            return Err(None);
                        }
                        code &= 7;
                        offset
                    },
                    _ => panic!("Invalid argument processor")
                };

                statics.push((offset, u32::from(code)));
            },

            FlatArg::Register { span, reg: Register::Dynamic(_, ref expr) } => match *command {
                Command::R(offset) => {
                    dynamics.push((offset, quote_spanned!{ span=>
                        {
                            let _dyn_reg: u8 = #expr.into();
                            (_dyn_reg & 0x1F) as u32
                        }
                    }));
                },
                Command::Reven(offset) => {
                    let invalid_reg_mask: u8 = if ctx.target.is_embedded() { 0xF0 } else { 0xE0 };
                    dynamics.push((offset, quote_spanned!{ span=>
                        {
                            let _dyn_reg: u8 = #expr.into();
                            if _dyn_reg & 0x1 != 0x0 || (_dyn_reg & #invalid_reg_mask) != 0 {
                                ::dynasmrt::riscv::invalid_register(_dyn_reg);
                            }
                            (_dyn_reg & 0x1E) as u32
                        }
                    }));
                },
                Command::Rno0(offset) => {
                    let invalid_reg_mask: u8 = if ctx.target.is_embedded() { 0xF0 } else { 0xE0 };
                    dynamics.push((offset, quote_spanned!{ span=>
                        {
                            let _dyn_reg: u8 = #expr.into();
                            if _dyn_reg == 0x0 || (_dyn_reg & #invalid_reg_mask) != 0 {
                                ::dynasmrt::riscv::invalid_register(_dyn_reg);
                            }
                            (_dyn_reg & 0x1F) as u32
                        }
                    }));
                },
                Command::Rno02(offset) => {
                    let invalid_reg_mask: u8 = if ctx.target.is_embedded() { 0xF0 } else { 0xE0 };
                    dynamics.push((offset, quote_spanned!{ span=>
                        {
                            let _dyn_reg: u8 = #expr.into();
                            if _dyn_reg == 0x0 || _dyn_reg == 0x2 || (_dyn_reg & #invalid_reg_mask) != 0 {
                                ::dynasmrt::riscv::invalid_register(_dyn_reg);
                            }
                            (_dyn_reg & 0x1F) as u32
                        }
                    }));
                },
                Command::Rpop(offset) => {
                    let invalid_reg_mask: u8 = if ctx.target.is_embedded() { 0xF0 } else { 0xE0 };
                    dynamics.push((offset, quote_spanned!{ span=>
                        {
                            let _dyn_reg: u8 = #expr.into();
                            if _dyn_reg & 0x18 != 0x8 || (_dyn_reg & #invalid_reg_mask) != 0 {
                                ::dynasmrt::riscv::invalid_register(_dyn_reg);
                            }
                            (_dyn_reg & 0x7) as u32
                        }
                    }));
                },
                Command::Rpops(offset) => {
                    let invalid_reg_mask: u8 = if ctx.target.is_embedded() { 0xF0 } else { 0xE0 };
                    dynamics.push((offset, quote_spanned!{ span=>
                        {
                            let _dyn_reg: u8 = #expr.into();
                            if (1u32 << (_dyn_reg & 0x1F)) & 0x00_FC_03_00 == 0 || (_dyn_reg & #invalid_reg_mask) != 0 {
                                ::dynasmrt::riscv::invalid_register(_dyn_reg);
                            }
                            (_dyn_reg & 0x7) as u32
                        }
                    }));
                },
                Command::Rpops2(offset) => match data.args.get(cursor - 1) {
                    Some(FlatArg::Register { reg: Register::Static(id2), .. } ) => {
                        let code: u8 = id2.code();
                        let invalid_reg_mask: u8 = if ctx.target.is_embedded() { 0xF0 } else { 0xE0 };
                        dynamics.push((offset, quote_spanned!{ span=>
                            {
                                let _dyn_reg: u8 = #expr.into();
                                if (_dyn_reg == #code) || ((1u32 << (_dyn_reg & 0x1F)) & 0x00_FC_03_00 == 0) || (_dyn_reg & #invalid_reg_mask) != 0 {
                                    ::dynasmrt::riscv::invalid_register(_dyn_reg);
                                }
                                (_dyn_reg & 0x7) as u32
                            }
                        }));
                    },
                    Some(FlatArg::Register { reg: Register::Dynamic(_, ref expr2), .. }) => {
                        let invalid_reg_mask: u8 = if ctx.target.is_embedded() { 0xF0 } else { 0xE0 };
                        dynamics.push((offset, quote_spanned!{ span=>
                            {
                                let _dyn_reg: u8 = #expr.into();
                                let _dyn_reg_prev: u8 = #expr2.into();
                                if (_dyn_reg == _dyn_reg_prev) || ((1u32 << (_dyn_reg & 0x1F)) & 0x00_FC_03_00 == 0) || (_dyn_reg & #invalid_reg_mask) != 0 {
                                    ::dynasmrt::riscv::invalid_register(_dyn_reg);
                                }
                                (_dyn_reg & 0x7) as u32
                            }
                        }));
                    },
                    _ => panic!("Invalid encoding data, expected a register before")
                },
                _ => panic!("Invalid argument processor")
            },

            FlatArg::RegisterList { span, ref count } => match *command {
                Command::Rlist(offset) => match count {
                    RegListFlat::Static(c) => {
                        statics.push((offset, u32::from(*c)));
                    },
                    RegListFlat::Dynamic(expr) => {
                        if let Some(static_value) = as_signed_number(expr) {
                            if static_value < 0 || (static_value > 10 && static_value != 12) {
                                emit_error!(expr, "Impossible register list");
                                return Err(None);
                            }

                            let c = if static_value == 12 {
                                15
                            } else {
                                (static_value + 4) as u32
                            };

                            statics.push((offset, c));

                        } else {
                            // okay, so we're given an expression here, with legal input values
                            // being 0-10 and 12, which should be mapped to 4-14 and 15
                            // note: this isn't quite the right register error
                            dynamics.push((offset, quote_spanned!{ span=>
                                {
                                    let _dyn_reg: u32 = #expr;
                                    if _dyn_reg == 11 || _dyn_reg > 12 {
                                        ::dynasmrt::riscv::invalid_register(_dyn_reg as u8);
                                    }
                                    (_dyn_reg + if (_dyn_reg == 12) { 3 } else { 4 }) & 0xF
                                }
                            }));
                        }
                    }
                },
                _ => panic!("Invalid argument processor")
            },

            FlatArg::Default => match *command {
                // Default is only emitted for a RefOffset where no offset was provided, i.e. it is 0
                // This practically means we just don't have to encode anything.
                Command::UImm(_, _)
                | Command::SImm(_, _)
                | Command::BitRange(_, _, _) 
                | Command::Next => (),
                _ => panic!("Invalid argument processor")
            },

            FlatArg::Immediate { ref value } => match *command {
                // integer verification commands
                Command::UImm(bits, scaling) => {
                    let span = value.span();
                    let range: u32 = bitmask(bits);

                    let mut imm_encoder = ImmediateEncoder::new(value);
                    imm_encoder.gather_fields(data.data.commands, i + 1, &mut statics);

                    match imm_encoder.static_value {
                        Some(static_value) => {
                            static_range_check(static_value, 0, range, scaling, span)?;
                        },
                        None => {
                            let check = if scaling == 0 {
                                quote_spanned!{ span =>
                                    _dyn_imm > #range
                                }
                            } else {
                                let zeromask: u32 = bitmask(scaling);
                                quote_spanned!{ span =>
                                    _dyn_imm > #range || _dyn_imm & #zeromask != 0u32
                                }
                            };
                            imm_encoder.emit_dynamic(false, false, check, &mut dynamics);
                        }
                    }
                },
                Command::SImm(bits, scaling) => {
                    let span = value.span();
                    let range = bitmask(bits);
                    let min: i32 = (-1) << (bits - 1);

                    let mut imm_encoder = ImmediateEncoder::new(value);
                    imm_encoder.gather_fields(data.data.commands, i + 1, &mut statics);

                    match imm_encoder.static_value {
                        Some(static_value) => {
                            static_range_check(static_value, min, range, scaling, span)?;
                        },
                        None => {
                            let check = if scaling == 0 {
                                quote_spanned!{ span =>
                                    _dyn_imm.wrapping_sub(#min) as u32 > #range
                                }
                            } else {
                                let zeromask = bitmask(scaling) as i32;
                                quote_spanned!{ span =>
                                    _dyn_imm.wrapping_sub(#min) as u32 > #range || _dyn_imm & #zeromask != 0i32
                                }
                            };
                            imm_encoder.emit_dynamic(true, false, check, &mut dynamics);
                        }
                    }
                },
                Command::BigImm(bits) => {
                    let span = value.span();
                    let range = bitmask64(bits);
                    let min: i64 = (-1) << (bits - 1);

                    let mut imm_encoder = ImmediateEncoder::new(value);
                    imm_encoder.gather_fields(data.data.commands, i + 1, &mut statics);

                    match imm_encoder.static_value {
                        Some(static_value) => {
                            if static_value < min {
                                emit_error!(span, "Immediate too low");
                                return Err(None);
                            }
                            if static_value.wrapping_sub(min) as u64 > range {
                                emit_error!(span, "Immediate too high");
                                return Err(None);
                            }
                        },
                        None => {
                            let check = quote_spanned!{ span =>
                                _dyn_imm.wrapping_sub(#min) as u64 > #range
                            };
                            imm_encoder.emit_dynamic(true, true, check, &mut dynamics);
                        }
                    }
                },
                Command::UImmNo0(bits, scaling) => {
                    let span = value.span();
                    let range = bitmask(bits);

                    let mut imm_encoder = ImmediateEncoder::new(value);
                    imm_encoder.gather_fields(data.data.commands, i + 1, &mut statics);

                    match imm_encoder.static_value {
                        Some(static_value) => {
                            static_range_check(static_value, 0, range, scaling, span)?;
                            if static_value == 0 {
                                emit_error!(span, "Immediate cannot be zero");
                                return Err(None);
                            }
                        },
                        None => {
                            let check = if scaling == 0 {
                                quote_spanned!{ span =>
                                    _dyn_imm > #range || _dyn_imm == 0u32
                                }
                            } else {
                                let zeromask: u32 = bitmask(scaling);
                                quote_spanned!{ span =>
                                    _dyn_imm > #range || _dyn_imm & #zeromask != 0u32 || _dyn_imm == 0u32
                                }
                            };
                            imm_encoder.emit_dynamic(false, false, check, &mut dynamics);
                        },
                    }
                },
                Command::SImmNo0(bits, scaling) => {
                    let span = value.span();
                    let range = bitmask(bits);
                    let min: i32 = (-1) << (bits - 1);

                    let mut imm_encoder = ImmediateEncoder::new(value);
                    imm_encoder.gather_fields(data.data.commands, i + 1, &mut statics);

                    match imm_encoder.static_value {
                        Some(static_value) => {
                            static_range_check(static_value, min, range, scaling, span)?;
                            if static_value == 0 {
                                emit_error!(span, "Immediate cannot be zero");
                                return Err(None);
                            }
                        },
                        None => {
                            let check = if scaling == 0 {
                                quote_spanned!{ span =>
                                    _dyn_imm.wrapping_sub(#min) as u32 > #range || _dyn_imm == 0i32
                                }
                            } else {
                                let zeromask = bitmask(scaling) as i32;
                                quote_spanned!{ span =>
                                    _dyn_imm.wrapping_sub(#min) as u32 > #range || _dyn_imm & #zeromask != 0i32 || _dyn_imm == 0i32
                                }
                            };
                            imm_encoder.emit_dynamic(true, false, check, &mut dynamics);
                        }
                    }
                },
                Command::UImmOdd(bits, scaling) => {
                    let span = value.span();
                    let range = bitmask(bits);
                    let zeromask: u32 = bitmask(scaling);

                    let mut imm_encoder = ImmediateEncoder::new(value);
                    imm_encoder.gather_fields(data.data.commands, i + 1, &mut statics);

                    match imm_encoder.static_value {
                        Some(static_value) => {
                            let biased = static_range_check(static_value, 0, range, 0, span)?;
                            if biased & zeromask != zeromask {
                                emit_error!(span, "Unrepresentable immediate");
                                return Err(None);
                            }
                        },
                        None => {
                            let check = if scaling == 0 {
                                quote_spanned!{ span =>
                                    _dyn_imm > #range
                                }
                            } else {
                                quote_spanned!{ span =>
                                    _dyn_imm > #range || _dyn_imm & #zeromask != #zeromask
                                }
                            };
                            imm_encoder.emit_dynamic(false, false, check, &mut dynamics);
                        }
                    }
                },
                Command::UImmRange(min, max) => {
                    let span = value.span();
                    let min = u32::from(min);
                    let max = u32::from(max);

                    let mut imm_encoder = ImmediateEncoder::new(value);
                    imm_encoder.gather_fields(data.data.commands, i + 1, &mut statics);

                    match imm_encoder.static_value {
                        Some(static_value) => {
                            if static_value < i64::from(min) {
                                emit_error!(span, "Immediate too low");
                                return Err(None);
                            }
                            if static_value > i64::from(max) {
                                emit_error!(span, "Immediate too high");
                                return Err(None);
                            }
                        },
                        None => {
                            let check = quote_spanned!{ span=>
                                _dyn_imm < #min || _dyn_imm > #max
                            };
                            imm_encoder.emit_dynamic(false, false, check, &mut dynamics);
                        }
                    }
                },

                // integer encoding commands
                Command::BitRange(offset, bits, scaling) => (),
                Command::RBitRange(offset, bits, scaling) => (),

                // offsets can accept immediates too
                Command::Offset(relocation_type) => {
                    let bits;
                    let scaling;
                    let commands: &'static [Command];

                    // equivalent bitrange encodings for offsets
                    match relocation_type {
                        // 12 bits, 2-bit scaled. Equivalent to
                        Relocation::B => {
                            bits = 12;
                            scaling = 1;
                            commands = &[
                                Command::BitRange(31, 1, 12),
                                Command::BitRange(25, 6, 5),
                                Command::BitRange(8, 4, 1),
                                Command::BitRange(7, 1, 11),
                                Command::Next
                            ];
                        },
                        // 20 bits, 2-bit scaled
                        Relocation::J => {
                            bits = 20;
                            scaling = 1;
                            commands = &[
                                Command::BitRange(31, 1, 20),
                                Command::BitRange(21, 10, 1),
                                Command::BitRange(20, 1, 11),
                                Command::BitRange(12, 8, 12),
                                Command::Next
                            ];
                        },
                        // 9 bits, 2-bit scaled
                        Relocation::BC => {
                            bits = 9;
                            scaling = 1;
                            commands = &[
                                Command::BitRange(12, 1, 8),
                                Command::BitRange(10, 2, 3),
                                Command::BitRange(5, 2, 6),
                                Command::BitRange(3, 2, 1),
                                Command::BitRange(2, 1, 5),
                                Command::Next
                            ];
                        },
                        // 12 bits, 2-bit scaled
                        Relocation::JC => {
                            bits = 12;
                            scaling = 1;
                            commands = &[
                                Command::BitRange(12, 1, 11),
                                Command::BitRange(11, 1, 4),
                                Command::BitRange(9, 2, 8),
                                Command::BitRange(8, 1, 10),
                                Command::BitRange(7, 1, 6),
                                Command::BitRange(6, 1, 7),
                                Command::BitRange(3, 3, 1),
                                Command::BitRange(2, 1, 5),
                                Command::Next
                            ]; // why
                        },
                        // 32 bits, 12-bit scaled, offset by 0x800
                        Relocation::HI20 => {
                            bits = 32;
                            scaling = 0;
                            commands = &[
                                Command::RBitRange(12, 20, 12),
                                Command::Next
                            ];
                        },
                        // 12 bits, no scaling
                        Relocation::LO12 => {
                            bits = 12;
                            scaling = 0;
                            commands = &[
                                Command::BitRange(20, 12, 0),
                                Command::Next
                            ];
                        },
                        // 12 bits, no scaling
                        Relocation::LO12S => {
                            bits = 12;
                            scaling = 0;
                            commands = &[
                                Command::BitRange(7, 5, 0),
                                Command::BitRange(25, 7, 5),
                                Command::Next
                            ];
                        },
                        // 32 bits, no scaling, offset by 0x800
                        Relocation::SPLIT32 => {
                            bits = 32;
                            scaling = 0;
                            commands = &[
                                Command::RBitRange(12, 20, 12),
                                Command::BitRange(20+32, 12, 0),
                                Command::Next
                            ];
                        },
                        // 32 bits, no scaling, offset by 0x800
                        Relocation::SPLIT32S => {
                            bits = 32;
                            scaling = 0;
                            commands = &[
                                Command::RBitRange(12, 20, 12),
                                Command::BitRange(7+32, 5, 0),
                                Command::BitRange(25+32, 7, 5),
                                Command::Next
                            ];
                        },
                        Relocation::LITERAL8
                        | Relocation::LITERAL16
                        | Relocation::LITERAL32
                        | Relocation::LITERAL64 => panic!("Literal relocation in instruction"),
                    }

                    let span = value.span();
                    let mut range = bitmask(bits);
                    let min: i32 = (-1) << (bits - 1);

                    // special case: the 32-bit AUIPC-based offsets don't actually
                    // range from -0x8000_0000 to 0x7FFF_FFFF on RV64 due to how
                    // sign extension interacts between them, they range from
                    // -0x8000_0800 to 0x7FFF_F7FF. But on RV32 they do span
                    // from -0x8000_0000 to 0x7FFF_FFFF.
                    // neither of these limits will ever occur in practical code,
                    // so for sanity's sake we just clamp to between -0x8000_0000 and
                    // 0x7FFF_F7FF
                    if bits == 32 {
                        range = 0xFFFF_F7FF;
                    }

                    let mut imm_encoder = ImmediateEncoder::new(value);
                    imm_encoder.gather_fields(commands, 0, &mut statics);

                    match imm_encoder.static_value {
                        Some(static_value) => {
                            static_range_check(static_value, min, range, scaling, span)?;
                        },
                        None => {
                            let check = if scaling == 0 {
                                quote_spanned!{ span =>
                                    _dyn_imm.wrapping_sub(#min) as u32 > #range
                                }
                            } else {
                                let zeromask = bitmask(scaling) as i32;
                                quote_spanned!{ span =>
                                    _dyn_imm.wrapping_sub(#min) as u32 > #range || _dyn_imm & #zeromask != 0i32
                                }
                            };

                            imm_encoder.emit_dynamic(true, false, check, &mut dynamics);
                        }
                    }
                },

                // Non-integer immediate commands
                Command::RoundingMode(offset) => {
                    let name = as_ident(value).expect("bad command data").to_string();
                    if let Some(&bits) = ROUNDMODE_MAP.get(&&*name) {
                        statics.push((offset, u32::from(bits)));
                    } else {
                        emit_error!(value, "Unknown literal");
                        return Err(None);
                    }
                },
                Command::FenceSpec(offset) => {
                    let name = as_ident(value).expect("bad command data").to_string();
                    if let Some(&bits) = FENCESPEC_MAP.get(&&*name) {
                        statics.push((offset, u32::from(bits)));
                    } else {
                        emit_error!(value, "Unknown literal");
                        return Err(None);
                    }
                },
                Command::Csr(offset) => 'csr: {
                    // Csr is a bit special as it allows both immediates and names
                    // because we cannot differentiate from those earlier, we handle that here.
                    if let Some(name) = as_ident(value) {
                        let name = name.to_string();
                        if let Some(&bits) = CSR_MAP.get(&&*name) {
                            statics.push((offset, u32::from(bits)));
                            break 'csr;
                        }
                    }

                    let span = value.span();
                    let range = 0xFFF;
                    let commands = &[
                        Command::BitRange(offset, 12, 0),
                        Command::Next
                    ];

                    let mut imm_encoder = ImmediateEncoder::new(value);
                    imm_encoder.gather_fields(commands, 0, &mut statics);

                    match imm_encoder.static_value {
                        Some(static_value) => {
                            static_range_check(static_value, 0, range, 0, span)?;
                        },
                        None => {
                            let check = quote_spanned!{ span =>
                                _dyn_imm > #range
                            };
                            imm_encoder.emit_dynamic(false, false, check, &mut dynamics);
                        }
                    }
                },
                Command::FloatingPointImmediate(offset) => 'fpimm: {
                    // this one has several special cases, and is very hard to do at runtime
                    // firstly, min, inf and nan are not actually values, but idents
                    // (min depends on what kind of floating point register the instruction targets)
                    // next to that, we need to parse 29 floating point values
                    if let Some(name) = as_ident(value) {
                        let name = name.to_string();
                        if let Some(&bits) = FP_IMM_IDENT_MAP.get(&&*name) {
                            statics.push((offset, u32::from(bits)));
                            break 'fpimm;
                        }
                    }

                    if let Some(value) = as_float(value) {
                        let value = value as f32;

                        if let Some(&(_, bits)) = FP_IMM_VALUE_MAP.iter().find(|(val, bits)| val == &value) {
                            statics.push((offset, u32::from(bits)));
                            break 'fpimm;
                        }
                    }

                    emit_error!(value, "Invalid floating point immediate.");
                    return Err(None);
                },
                Command::SPImm(offset, negated) => 'spimm: {
                    // the value we need to encode here depends on the count of registers in the
                    // register list, and depends on the target architecture as well.
                    let count = match data.args.get(cursor - 1) {
                        Some(FlatArg::RegisterList { count, .. }) => count,
                        _ => panic!("Invalid encoding data, expected a register list before")
                    };

                    let span = value.span();

                    // either statically encode it, or return an expression for the register list bias
                    let bias_expr = match count {
                        RegListFlat::Static(code) => {
                            let code = if *code == 15 {16} else {*code};

                            let bias = if ctx.target.is_32_bit() {
                                i32::from(code / 4 * 16)
                            } else {
                                i32::from(code / 2 * 16 - 16)
                            };

                            if let Some(mut static_value) = as_signed_number(value) {
                                // statically encode everything
                                if negated {
                                    static_value = static_value.saturating_neg();
                                }

                                let bits = static_range_check(static_value, bias, 48, 4, span)? >> 4;
                                statics.push((offset, bits));

                                break 'spimm;

                            } else {
                                quote_spanned!{ span=>
                                    let _reglist_bias: i32 = #bias;
                                }
                            }
                        },
                        RegListFlat::Dynamic(expr) => {
                            if let Some(mut static_value) = as_signed_number(value) {
                                // just some sanity checks at compile time
                                if negated {
                                    static_value = static_value.saturating_neg();
                                }
                                if ctx.target.is_32_bit() {
                                    static_range_check(static_value, 16, 96, 4, span)?;
                                } else {
                                    static_range_check(static_value, 16, 144, 4, span)?;
                                }
                            }
                            if ctx.target.is_32_bit() {
                                quote_spanned!{ span=>
                                    let _reglist_expr: u8 = #expr;
                                    let _reglist_bias: i32 = (_reglist_expr as i32) / 4 * 16 + 16;
                                }
                            } else {
                                quote_spanned!{ span=>
                                    let _reglist_expr: u8 = #expr;
                                    let _reglist_bias: i32 = (_reglist_expr as i32) / 2 * 16 + 16;
                                }
                            }
                        }
                    };

                    let imm_expr = if negated {
                        let value = delimited(value);
                        quote_spanned!{ span=>
                            let _dyn_imm: i32 = -#value;
                        }
                    } else {
                        quote_spanned!{ span=>
                            let _dyn_imm: i32 = #value;
                        }
                    };

                    dynamics.push((offset, quote_spanned!{ span=>
                        {
                            #imm_expr
                            #bias_expr
                            if (_dyn_imm < _reglist_bias) || ((_dyn_imm - _reglist_bias) > 48) || ((_dyn_imm & 15) != 0) {
                                ::dynasmrt::riscv::immediate_out_of_range_signed_32(_dyn_imm);
                            }
                            (_dyn_imm - _reglist_bias) as u32 >> 4
                        }
                    }));
                },
                _ => panic!("Invalid argument processor")
            },

            FlatArg::JumpTarget { ref jump } => match *command {
                Command::Offset( relocation ) => {
                    // encode the complete relocation. Always starts at the begin of the instruction(s), and also relative to that
                    let stmt = jump.clone().encode(relocation.size(), relocation.size(), &[relocation.to_id()]);

                    relocations.push(stmt);
                },
                _ => panic!("Invalid argument processor")
            }
        }

        // figure out how far the cursor has to be advanced.
        match *command {
            Command::UImm(_, _)
            | Command::SImm(_, _)
            | Command::BigImm(_)
            | Command::UImmNo0(_, _)
            | Command::SImmNo0(_, _)
            | Command::UImmOdd(_, _)
            | Command::UImmRange(_, _)
            | Command::BitRange(_, _, _)
            | Command::RBitRange(_, _, _) => (),
            _ => cursor += 1
        }
    }

    // sanity
    if cursor != data.args.len() {
        panic!("Not enough command processors");
    }

    let mut templates = [0u32; 8];
    let mut exprs = [None, None, None, None, None, None, None, None];

    // for convenience sake we operate in 32 bits width, even for compressed instructions
    match data.data.template {
        Template::Compressed(val) => templates[0] = u32::from(val),
        Template::Single(val) => templates[0] = val,
        Template::Double(val1, val2) => {
            templates[0] = val1;
            templates[1] = val2;
        },
        Template::Many(values) => {
            templates[ .. values.len()].copy_from_slice(values);
        }
    };

    // apply all statics to templates
    for (offset, value) in statics {
        templates[(offset >> 5) as usize] |= value << (offset & 0x1F);
    }

    // and process all dynamics
    for (offset, expr) in dynamics {
        let index = usize::from(offset >> 5);
        let offset = offset & 0x1F;

        exprs[index] = match exprs[index].take() {
            Some(prev_expr) => {
                Some(if offset == 0 {
                    quote!{ #prev_expr | #expr }
                } else {
                    quote!{ #prev_expr | (#expr << #offset) }
                })
            },
            None => {
                let bits = templates[index];
                Some(if offset == 0 {
                    quote!{ #bits | #expr }
                } else {
                    quote!{ #bits | (#expr << #offset) }
                })
            }
        }
    }

    match data.data.template {
        Template::Compressed(_) => if let Some(d) = exprs[0].take() {
            let res = quote!{ (#d) as u16 };
            ctx.state.stmts.push(Stmt::ExprUnsigned(delimited(res), Size::B_2));
        } else {
            ctx.state.stmts.push(Stmt::Const(u64::from(templates[0] & 0xFFFF), Size::B_2));
        },
        Template::Single(_) => if let Some(d) = exprs[0].take() {
            ctx.state.stmts.push(Stmt::ExprUnsigned(delimited(d), Size::B_4));
        } else {
            ctx.state.stmts.push(Stmt::Const(u64::from(templates[0]), Size::B_4));
        },
        Template::Double(_, _) => {
            for i in 0 .. 2 {
                if let Some(d) = exprs[i].take() {
                    ctx.state.stmts.push(Stmt::ExprUnsigned(delimited(d), Size::B_4));
                } else {
                    ctx.state.stmts.push(Stmt::Const(u64::from(templates[i]), Size::B_4));
                }
            }
        },
        Template::Many(c) => {
            for i in 0 .. c.len() {
                if let Some(d) = exprs[i].take() {
                    ctx.state.stmts.push(Stmt::ExprUnsigned(delimited(d), Size::B_4));
                } else {
                    ctx.state.stmts.push(Stmt::Const(u64::from(templates[i]), Size::B_4));
                }
            }
        }
    }

    ctx.state.stmts.extend(relocations);

    Ok(())
}


/// Handles the encoding of immediates in a somewhat efficient fashion.
struct ImmediateEncoder<'a> {
    pub dynamic_value: &'a syn::Expr,
    pub static_value: Option<i64>,
    pub encodes: Vec<(u8, TokenStream)>, // encoding_offset, expression
    pub span: Span
}

impl<'a> ImmediateEncoder<'a> {
    pub fn new(dynamic_value: &syn::Expr) -> ImmediateEncoder {
        #![allow(unexpected_cfgs)]
        let static_value;

        // this allows turning off static checks for testing purposes
        #[cfg(not(disable_static_checks="1"))]
        {
            static_value = as_signed_number(dynamic_value);
        }
        #[cfg(disable_static_checks="1")]
        {
            static_value = None;
        }

        let span = dynamic_value.span();

        ImmediateEncoder {
            dynamic_value,
            static_value,
            encodes: Vec::new(),
            span
        }
    }

    pub fn gather_fields(&mut self, commands: &[Command], mut index: usize, statics: &mut Vec<(u8, u32)>) {
        loop {
            match commands.get(index) {
                Some(&Command::BitRange(offset, bits, scaling)) => {
                    let mask = bitmask(bits);

                    if let Some(v) = self.static_value {
                        let slice = (v >> scaling) as u32 & mask;
                        statics.push((offset, slice));

                    } else {
                        self.encodes.push((offset, quote_spanned!{ self.span=>
                            ((_dyn_imm >> #scaling) as u32 & #mask)
                        }));
                    }
                },
                Some(&Command::RBitRange(offset, bits, scaling)) => {
                    let mask = bitmask(bits);
                    let round_offset: i64 =  1 << (scaling - 1);

                    if let Some(v) = self.static_value {
                        let slice = (v.wrapping_add(round_offset) >> scaling) as u32 & mask;
                        statics.push((offset, slice));

                    } else {
                        // ensure we emit an unsuffixed literal for this so it works with all
                        // types of number
                        let round_offset = Literal::i64_unsuffixed(round_offset);
                        self.encodes.push((offset, quote_spanned!{ self.span=>
                            ((_dyn_imm.wrapping_add(#round_offset) >> #scaling) as u32 & #mask)
                        }));
                    }
                },
                Some(Command::Next) => break,
                Some(_)
                | None => panic!("Bad encoding data, integer field sequence is not terminated"),
            }
            index += 1;
        }
    }

    pub fn emit_dynamic(mut self, is_signed: bool, is_64bit: bool, check: TokenStream, dynamics: &mut Vec<(u8, TokenStream)>) {
        // assemble encoding chunks
        let mut exprs = [None, None, None, None, None, None, None, None];
        let dynamic_value = self.dynamic_value;
        let span = self.span;

        for (offset, expr) in self.encodes.drain(..) {
            let index = usize::from(offset >> 5);
            let offset = offset & 0x1F;

            exprs[index] = match exprs[index].take() {
                Some(prev_expr) => {
                    let parenthesized = delimited(prev_expr);

                    Some(if offset == 0 {
                        let expr = delimited(expr);
                        quote!{ #parenthesized | #expr }
                    } else {
                        quote!{ #parenthesized | (#expr << #offset) }
                    })
                },
                None => {
                    Some(if offset == 0 {
                        quote!{ #expr }
                    } else {
                        quote!{ #expr << #offset }
                    })
                }
            }
        }

        let imm_ty = match (is_64bit, is_signed) {
            (false, false) => quote_spanned!{ span=> u32 },
            (false, true)  => quote_spanned!{ span=> i32 },
            (true, false)  => quote_spanned!{ span=> u64 },
            (true, true)   => quote_spanned!{ span=> i64 }
        };

        let mut first = true;
        for (i, expr) in exprs.into_iter().enumerate() {
            let encodes = if let Some(encodes) = expr { encodes } else {
                continue
            };
            let offset = (i * 32) as u8;

            if first {
                first = false;
                let error_expr = match (is_64bit, is_signed) {
                    (false, false) => quote_spanned!{ span=>
                        ::dynasmrt::riscv::immediate_out_of_range_unsigned_32
                    },
                    (false, true)  => quote_spanned!{ span=>
                        ::dynasmrt::riscv::immediate_out_of_range_signed_32
                    },
                    (true, false)  => quote_spanned!{ span=>
                        ::dynasmrt::riscv::immediate_out_of_range_unsigned_64
                    },
                    (true, true)   => quote_spanned!{ span=>
                        ::dynasmrt::riscv::immediate_out_of_range_signed_64
                    }
                };

                dynamics.push((offset, quote_spanned!{ span=>
                    {
                        let _dyn_imm: #imm_ty = #dynamic_value;

                        if #check {
                            #error_expr(_dyn_imm);
                        }

                        #encodes
                    }
                }));

            } else {
                dynamics.push((offset, quote_spanned!{ span=>
                    {
                        let _dyn_imm: #imm_ty = #dynamic_value;
                        #encodes
                    }
                }));
            }
        }
    }
}

/// Checks the following things
/// value >= min
/// (value - min) <= range
/// ((value - min) & bitmask(scale)) == 0
/// returning (value - min) on success.
fn static_range_check(value: i64, min: i32, range: u32, scale: u8, span: Span) -> Result<u32, Option<String>> {
    if value < i64::from(min) {
        emit_error!(span, "Immediate too low");
        return Err(None);
    }

    let biased = value - i64::from(min);

    if biased > i64::from(range) {
        emit_error!(span, "Immediate too high");
        return Err(None);
    }

    let biased = biased as u32;

    if scale != 0 && (biased & bitmask(scale)) != 0 {
        emit_error!(span, "Unrepresentable immediate");
        return Err(None);
    }

    Ok(biased)
}
