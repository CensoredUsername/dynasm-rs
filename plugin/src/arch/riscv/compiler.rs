use super::Context;
use super::riscvdata::{Template, Command, Relocation, ROUNDMODE_MAP, FENCESPEC_MAP, CSR_MAP, FP_IMM_IDENT_MAP, FP_IMM_VALUE_MAP};
use super::ast::{MatchData, FlatArg, RegListFlat, Register};

use syn::spanned::Spanned;
use quote::{quote, quote_spanned};
use proc_macro2::{TokenStream, Span};
use proc_macro_error2::emit_error;

use crate::parse_helpers::{as_signed_number, as_ident, as_float};
use crate::common::{Stmt, Size, delimited, bitmask};

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

    // Because of the sheer amount of split immediates we handle these specially
    // this generates significantly less terrible code
    let mut imm_encoder = ImmediateEncoder::new();

    for command in data.data.commands.iter() {
        // meta commands
        match *command {

            Command::Repeat => {
                cursor -= 1;
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
                    _ => panic!("Invalid argument processor")
                };

                statics.push((offset, u32::from(code)));
            },

            FlatArg::Register { span, reg: Register::Dynamic(_, ref expr) } => match *command {
                Command::R(offset) => {
                    dynamics.push((offset, quote_spanned!{ span=>
                        (#expr & 0x1F)
                    }));
                },
                Command::Reven(offset) => {
                    dynamics.push((offset, quote_spanned!{ span=>
                        {
                            let _dyn_reg = #expr;
                            if _dyn_reg & 0x1 != 0x0 {
                                ::dynasmrt::riscv64::invalid_register(_dyn_reg);
                            }
                            _dyn_reg & 0x1E
                        }
                    }));
                },
                Command::Rno0(offset) => {
                    dynamics.push((offset, quote_spanned!{ span=>
                        {
                            let _dyn_reg = #expr;
                            if _dyn_reg == 0x0 {
                                ::dynasmrt::riscv64::invalid_register(_dyn_reg);
                            }
                            _dyn_reg & 0x1F
                        }
                    }));
                },
                Command::Rno02(offset) => {
                    dynamics.push((offset, quote_spanned!{ span=>
                        {
                            let _dyn_reg = #expr;
                            if _dyn_reg == 0x0 || _dyn_reg == 0x2 {
                                ::dynasmrt::riscv64::invalid_register(_dyn_reg);
                            }
                            _dyn_reg & 0x1F
                        }
                    }));
                },
                Command::Rpop(offset) => {
                    dynamics.push((offset, quote_spanned!{ span=>
                        {
                            let _dyn_reg = #expr;
                            if _dyn_reg & 0x18 != 0x8 {
                                ::dynasmrt::riscv64::invalid_register(_dyn_reg);
                            }
                            _dyn_reg & 0x7
                        }
                    }));
                },
                Command::Rpops(offset) => {
                    dynamics.push((offset, quote_spanned!{ span=>
                        {
                            let _dyn_reg = #expr;
                            if (1u32 << (_dyn_reg & 0x1F)) & 0x00_FC_03_00 == 0 {
                                ::dynasmrt::riscv64::invalid_register(_dyn_reg);
                            }
                            _dyn_reg & 0x7
                        }
                    }));
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
                            dynamics.push((offset, quote_spanned!{ span=>
                                {
                                    let _dyn_reg = #expr;
                                    if _dyn_reg == 11 || _dyn_reg > 12 {
                                        ::dynasmrt::riscv64::invalid_register(_dyn_reg);
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

                    match imm_encoder.init(value, false) {
                        Some(static_value) => {
                            static_range_check(static_value, 0, range, scaling, span)?;
                        },
                        None => {
                            if scaling == 0 {
                                imm_encoder.set_check(quote_spanned!{ span =>
                                    _dyn_imm > #range
                                });
                            } else {
                                let zeromask: u32 = bitmask(scaling);
                                imm_encoder.set_check(quote_spanned!{ span =>
                                    _dyn_imm > #range || _dyn_imm & #zeromask != 0u32
                                });
                            }
                        }
                    }
                },
                Command::SImm(bits, scaling) => {
                    let span = value.span();
                    let range = bitmask(bits);
                    let min: i32 = (-1) << (bits - 1);

                    match imm_encoder.init(value, true) {
                        Some(static_value) => {
                            static_range_check(static_value, min, range, scaling, span)?;
                        },
                        None => {
                            if scaling == 0 {
                                imm_encoder.set_check(quote_spanned!{ span =>
                                    _dyn_imm.wrapping_sub(#min) as u32 > #range
                                });
                            } else {
                                let zeromask = bitmask(scaling) as i32;
                                imm_encoder.set_check(quote_spanned!{ span =>
                                    _dyn_imm.wrapping_sub(#min) as u32 > #range || _dyn_imm & #zeromask != 0i32
                                });
                            }
                        }
                    }
                },
                Command::NImm(bits, scaling) => {
                    let span = value.span();
                    let range = bitmask(bits);

                    match imm_encoder.init(value, true) {
                        Some(static_value) => {
                            static_range_check(-static_value, 0, range, scaling, span)?;
                        },
                        None => {
                            if scaling == 0 {
                                imm_encoder.set_check(quote_spanned!{ span =>
                                    (-_dyn_imm) as u32 > #range
                                });
                            } else {
                                let zeromask = bitmask(scaling) as i32;
                                imm_encoder.set_check(quote_spanned!{ span =>
                                    (-_dyn_imm) as u32 > #range || (-_dyn_imm) & #zeromask != 0i32
                                });
                            }
                        }
                    }
                },
                Command::UImmNo0(bits, scaling) => {
                    let span = value.span();
                    let range = bitmask(bits);

                    match imm_encoder.init(value, false) {
                        Some(static_value) => {
                            static_range_check(static_value, 0, range, scaling, span)?;
                            if static_value == 0 {
                                emit_error!(span, "Immediate cannot be zero");
                                return Err(None);
                            }
                        },
                        None => {
                            if scaling == 0 {
                                imm_encoder.set_check(quote_spanned!{ span =>
                                    _dyn_imm > #range || _dyn_imm == 0u32
                                });
                            } else {
                                let zeromask: u32 = bitmask(scaling);
                                imm_encoder.set_check(quote_spanned!{ span =>
                                    _dyn_imm > #range || _dyn_imm & #zeromask != 0u32 || _dyn_imm == 0u32
                                });
                            }
                        },
                    }
                },
                Command::SImmNo0(bits, scaling) => {
                    let span = value.span();
                    let range = bitmask(bits);
                    let min: i32 = (-1) << (bits - 1);

                    match imm_encoder.init(value, true) {
                        Some(static_value) => {
                            static_range_check(static_value, min, range, scaling, span)?;
                            if static_value == 0 {
                                emit_error!(span, "Immediate cannot be zero");
                                return Err(None);
                            }
                        },
                        None => {
                            if scaling == 0 {
                                imm_encoder.set_check(quote_spanned!{ span =>
                                    _dyn_imm.wrapping_sub(#min) as u32 > #range || _dyn_imm == 0i32
                                });
                            } else {
                                let zeromask = bitmask(scaling) as i32;
                                imm_encoder.set_check(quote_spanned!{ span =>
                                    _dyn_imm.wrapping_sub(#min) as u32 > #range || _dyn_imm & #zeromask != 0i32 || _dyn_imm == 0i32
                                });
                            }
                        }
                    }
                },
                Command::UImmOdd(bits, scaling) => {
                    let span = value.span();
                    let range = bitmask(bits);
                                let zeromask: u32 = bitmask(scaling);

                    match imm_encoder.init(value, false) {
                        Some(static_value) => {
                            let biased = static_range_check(static_value, 0, range, 0, span)?;
                            if biased & zeromask != zeromask {
                                emit_error!(span, "Unrepresentable immediate");
                                return Err(None);
                            }
                        },
                        None => {
                            if scaling == 0 {
                                imm_encoder.set_check(quote_spanned!{ span =>
                                    _dyn_imm > #range
                                });
                            } else {
                                imm_encoder.set_check(quote_spanned!{ span =>
                                    _dyn_imm > #range || _dyn_imm & #zeromask != #zeromask
                                });
                            }
                        }
                    }
                },
                Command::UImmRange(min, max) => {
                    let span = value.span();

                    match imm_encoder.init(value, false) {
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
                            imm_encoder.set_check(quote_spanned!{ span =>
                                _dyn_imm < #min || _dyn_imm > #max
                            });
                        }
                    }
                },

                // integer encoding commands
                Command::BitRange(offset, bits, scaling) => {
                    match imm_encoder.static_value {
                        Some(static_value) => {
                            let mask = bitmask(bits);
                            let slice = ((static_value as u32) >> scaling) & mask;
                            statics.push((offset, slice));
                        },
                        None => {
                            let span = value.span();
                            let mask = bitmask(bits);
                            imm_encoder.add_field(offset, quote_spanned!{ span => 
                                (((_dyn_imm as u32) >> #scaling) & #mask)
                            });
                        }
                    }
                },
                Command::NBitRange(offset, bits, scaling) => {
                    match imm_encoder.static_value {
                        Some(static_value) => {
                            let mask = bitmask(bits);
                            let slice = (((-static_value) as u32) >> scaling) & mask;
                            statics.push((offset, slice));
                        },
                        None => {
                            let span = value.span();
                            let mask = bitmask(bits);
                            imm_encoder.add_field(offset, quote_spanned!{ span => 
                                ((((-_dyn_imm) as u32) >> #scaling) & #mask)
                            });
                        }
                    }
                },

                // finish up encoding the current integer
                Command::Next => {
                    if let Some(dynamic) = imm_encoder.finalize(value) {
                        dynamics.push((0, dynamic));
                    }
                }

                // offsets can accept immediates too
                Command::Offset(relocation_type) => {
                    let bits;
                    let scaling;
                    let ranges: &'static [(u8, u8, u8)];
                    match relocation_type {
                        // 12 bits, 2-bit scaled. Equivalent to
                        // BitRange(31, 1, 12), BitRange(25, 6, 5), BitRange(8, 4, 1), BitRange(7, 1, 11)
                        Relocation::B => {
                            bits = 12;
                            scaling = 1;
                            ranges = &[(31, 1, 12), (25, 6, 5), (8, 4, 1), (7, 1, 11)];
                        },
                        // 20 bits, 2-bit scaled
                        Relocation::J => {
                            bits = 20;
                            scaling = 1;
                            ranges = &[(31, 1, 20), (21, 10, 1), (20, 1, 11), (12, 8, 12)];
                        },
                        // 9 bits, 2-bit scaled
                        Relocation::BC => {
                            bits = 9;
                            scaling = 1;
                            ranges = &[(12, 1, 8), (10, 2, 3), (5, 2, 6), (3, 2, 1), (2, 1, 5)];
                        },
                        // 12 bits, 2-bit scaled
                        Relocation::JC => {
                            bits = 12;
                            scaling = 1;
                            ranges = &[(12, 1, 11), (11, 1, 4), (9, 2, 8), (7, 1, 10), (6, 1, 6), (5, 1, 7), (3, 3, 1), (2, 1, 5)]; // why
                        },
                        // 32 bits, 12-bit scaled
                        Relocation::AUIPC => {
                            bits = 32;
                            scaling = 11;
                            ranges = &[(12, 20, 12)];
                        },
                        // 12 bits, no scaling
                        Relocation::JALR => {
                            bits = 12;
                            scaling = 1;
                            ranges = &[(20, 12, 0)];
                        },
                    }

                    let span = value.span();
                    let range = bitmask(bits);
                    let min: i32 = (-1) << (bits - 1);

                    match imm_encoder.init(value, true) {
                        Some(static_value) => {
                            static_range_check(static_value, min, range, scaling, span)?;

                            for &(offset, count, scaling) in ranges.iter() {
                                let slice = ((static_value as u32) >> scaling) & bitmask(count);
                                statics.push((offset, slice))
                            }
                        },
                        None => {
                            if scaling == 0 {
                                imm_encoder.set_check(quote_spanned!{ span =>
                                    _dyn_imm.wrapping_sub(#min) as u32 > #range
                                });
                            } else {
                                let zeromask = bitmask(scaling) as i32;
                                imm_encoder.set_check(quote_spanned!{ span =>
                                    _dyn_imm.wrapping_sub(#min) as u32 > #range || _dyn_imm & #zeromask != 0i32
                                });
                            }

                            for &(offset, count, scaling) in ranges.iter() {
                                let mask = bitmask(bits);
                                imm_encoder.add_field(offset, quote_spanned!{ span => 
                                    (((_dyn_imm as u32) >> #scaling) & #mask)
                                });
                            }
                        }
                    }

                    if let Some(dynamic) = imm_encoder.finalize(value) {
                        dynamics.push((0, dynamic));
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

                    match imm_encoder.init(value, false) {
                        Some(static_value) => {
                            static_range_check(static_value, 0, range, 0, span)?;
                            statics.push((offset, static_value as u32));
                        },
                        None => {
                            imm_encoder.set_check(quote_spanned!{ span =>
                                _dyn_imm > #range
                            });
                            imm_encoder.add_field(offset, quote_spanned!{ span => 
                                (_dyn_imm & 0xFFFu32)
                            });
                        }
                    }

                    if let Some(dynamic) = imm_encoder.finalize(value) {
                        dynamics.push((0, dynamic));
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
                _ => panic!("Invalid argument processor")
            },

            FlatArg::JumpTarget { ref jump } => match *command {
                Command::Offset( relocation ) => {
                    // encode the complete relocation. Always starts at the begin of the instruction, and also relative to that
                    // TODO: handle variable instruction width
                    let stmt = jump.clone().encode(4, 4, &[relocation.to_id()]);

                    relocations.push(stmt);
                    unimplemented!();
                },
                _ => panic!("Invalid argument processor")
            }
        }

        // figure out how far the cursor has to be advanced.
        match *command {
            Command::UImm(_, _)
            | Command::SImm(_, _)
            | Command::NImm(_, _)
            | Command::UImmNo0(_, _)
            | Command::SImmNo0(_, _)
            | Command::UImmOdd(_, _)
            | Command::UImmRange(_, _)
            | Command::BitRange(_, _, _)
            | Command::NBitRange(_, _, _) => (),
            _ => cursor += 1
        }
    }

    // sanity
    dbg!(cursor);
    if cursor != data.args.len() {
        panic!("Not enough command processors");
    }

    // for convenience sake we operate in 32 bits width, even for compressed instructions
    let mut bits = match data.data.template {
        Template::Compressed(val) => u32::from(val),
        Template::Single(val) => val
    };

    // apply all statics to template
    for (offset, value) in statics {
        bits |= value << offset;
    }

    // generate dynamic code
    if !dynamics.is_empty() {
        let mut res = quote!{
            #bits
        };
        for (offset, expr) in dynamics {
            if offset == 0 {
                res = quote!{
                    #res | #expr
                };
            } else {
                res = quote!{
                    #res | (#expr << #offset)
                };
            }
        }

        match data.data.template {
            Template::Compressed(_) => {
                res = quote!{ (#res) as u16 };
                ctx.state.stmts.push(Stmt::ExprUnsigned(delimited(res), Size::B_2));
            },
            Template::Single(_) => {
                ctx.state.stmts.push(Stmt::ExprUnsigned(delimited(res), Size::B_4));
            }
        }
    } else {
        match data.data.template {
            Template::Compressed(_) => {
                ctx.state.stmts.push(Stmt::Const(u64::from(bits & 0xFFFF), Size::B_2));
            },
            Template::Single(_) => {
                ctx.state.stmts.push(Stmt::Const(u64::from(bits), Size::B_4));
            }
        }
    }

    ctx.state.stmts.extend(relocations);

    Ok(())
}


// how to do integer encoding properly so we don't redo the same work n times
// so work is split between pure validation commands that don't anything
// and encoding commands that don't do any validation

// general workflow should be like this
// a new integer arg is encountered
// step 1: determine if it is statically evaluatable
// step 2: if static: check value. if dynamic: encode rangechecks
// step 3: if static: add bits to static template. if dynamic: add bits to encoding expression
// step 4: if we're done with this arg, emit these from the builder
struct ImmediateEncoder {
    // if this is None, we haven't initialized
    pub is_signed: Option<bool>,
    pub static_value: Option<i64>,
    pub encodes: Vec<(u8, TokenStream)>, // encoding_offset, expression
    pub check: Option<TokenStream>
}

impl ImmediateEncoder {
    pub fn new() -> ImmediateEncoder {
        ImmediateEncoder {
            is_signed: None,
            static_value: None,
            check: None,
            encodes: Vec::new()
        }
    }

    pub fn init(&mut self, dynamic_value: &syn::Expr, is_signed: bool) -> Option<i64> {
        #![allow(unexpected_cfgs)]

        if self.is_signed.is_none() {
            self.is_signed = Some(is_signed);
            self.static_value = as_signed_number(dynamic_value);
        }

        // this allows turning off static checks for testing purposes
        #[cfg(disable_static_checks="1")]
        {
            self.static_value = None;
        }

        self.static_value
    }

    pub fn add_field(&mut self, encoding_offset: u8, expr: TokenStream) {
        self.encodes.push((encoding_offset, expr));
    }

    pub fn set_check(&mut self, check: TokenStream) {
        self.check = Some(check);
    }

    pub fn finalize(&mut self, dynamic_value: &syn::Expr) -> Option<TokenStream> {
        // this should only be called if we have been initialized
        let signed = self.is_signed.expect("bad encoding data");

        // if the value was statically known, there is no need to encode something dynamically
        if self.static_value.is_some() {
            self.is_signed = None;
            self.static_value = None;
            self.check = None;
            self.encodes.clear();

            return None;
        }

        let span = dynamic_value.span();

        let load_expr = if signed {
            quote_spanned!{ span=>
                u32::from(#dynamic_value);
            }
        } else {
            quote_spanned!{ span=>
                i32::from(#dynamic_value);
            }
        };

        // assemble encoding chunks
        let mut iter = self.encodes.drain(..);

        let mut encodes = match iter.next() {
            Some((offset, check)) => quote_spanned!{ span=>
                #check << #offset
            },
            None => quote_spanned!{ span=> 0 }
        };

        for (offset, check) in iter {
            let parenthesized = delimited(encodes);
            encodes = quote_spanned!{ span=>
                #parenthesized | (#check << #offset)
            };
        }


        let check = self.check.take().expect("Integer range check was not configured for dynamic value");

        if signed {
            Some(quote_spanned!{ span=>
                {
                    let _dyn_imm = i32::from(#dynamic_value);

                    if #check {
                        ::dynasmrt::riscv64::immediate_out_of_range_signed_32(_dyn_imm);
                    }

                    #encodes
                }
            })
        } else {
            Some(quote_spanned!{ span=>
                {
                    let _dyn_imm = u32::from(#dynamic_value);

                    if #check {
                        ::dynasmrt::riscv64::immediate_out_of_range_unsigned_32(_dyn_imm);
                    }

                    #encodes
                }
            })
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

    if scale != 0 {
        if biased & bitmask(scale) != 0 {
            emit_error!(span, "Unrepresentable immediate");
            return Err(None);
        }
    }

    Ok(biased)
}