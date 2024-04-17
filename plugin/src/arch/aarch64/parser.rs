use syn::{parse, Token};

use lazy_static::lazy_static;

use crate::parse_helpers::{parse_ident_or_rust_keyword, ParseOpt, ParseOptExt};
use crate::common::Size;

use super::Context;
use super::ast::{Instruction, RawArg, Register, RegId, RegKind, RegScalar, RegVector, RegFamily, RefItem, Modifier, ModifyExpr};

use std::collections::HashMap;

// parses a full instruction
// syntax for a single op: ident ("." expr)* (arg ("," arg)*)? ";"
pub(super) fn parse_instruction(ctx: &mut Context, input: parse::ParseStream) -> parse::Result<(Instruction, Vec<RawArg>)> {
    let span = input.cursor().span();

    // read the full dot-separated op
    let op = parse_ident_or_rust_keyword(input)?;

    let mut args = Vec::new();

    // parse any dot args
    while input.peek(Token![.]) {
        let span = input.cursor().span();
        let _: Token![.] = input.parse()?;
        let arg: syn::Ident = input.parse()?;

        args.push(RawArg::Dot { span } );
        args.push(RawArg::Lit { ident: arg });
    }

    // parse 0 or more comma-separated args
    if !(input.is_empty() || input.peek(Token![;])) {
        args.push(parse_arg(ctx, input)?);

        while input.peek(Token![,]) {
            let _: Token![,] = input.parse()?;

            args.push(parse_arg(ctx, input)?);
        }
    }

    // let span = span.join(input.cursor().span()); // FIXME can't join spans ATM

    Ok((
        Instruction {
            ident: op,
            span
        },
        args
    ))
}

/// tries to parse a full arg definition
fn parse_arg(ctx: &mut Context, input: parse::ParseStream) -> parse::Result<RawArg> {
    let _start = input.cursor().span(); // FIXME can't join spans yet

    // a label
    if let Some(jump) = input.parse_opt()? {
        return Ok(RawArg::JumpTarget {
            jump
        });
    }

    // reference
    if input.peek(syn::token::Bracket) {
        let span = input.cursor().span();
        let inner;
        let _ = syn::bracketed!(inner in input);
        let inner = &inner;

        // parse comma-separated inner items
        let mut items = Vec::new();
        items.push(parse_refitem(ctx, inner)?);

        while inner.peek(Token![,]) {
            let _: Token![,] = inner.parse()?;

            items.push(parse_refitem(ctx, inner)?);
        }

        // TODO: confirm we parsed everything?

        let bang = if input.peek(Token![!]) {
            let _: Token![!] = input.parse()?;
            true
        } else {
            false
        };

        return Ok(RawArg::Reference {
            span,
            items,
            bang
        });
    }

    // registerlist
    if input.peek(syn::token::Brace) {
        let span = input.cursor().span();
        let inner;
        let _ = syn::braced!(inner in input);
        let inner = &inner;

        let first = parse_reg(ctx, inner)?.ok_or_else(|| inner.error("Expected register"))?;

        // parses {reg - reg}
        let mut ast = if inner.peek(Token![-]) {
            let _: Token![-] = inner.parse()?;

            let last = parse_reg(ctx, inner)?.ok_or_else(|| inner.error("Expected register"))?;

            RawArg::DashList {
                span,
                first,
                last,
                element: None
            }

        // parses {reg, reg .. , reg}
        } else if inner.peek(Token![*]) {
            let _: Token![*] = inner.parse()?;

            let amount: syn::Expr = inner.parse()?;

            RawArg::AmountList {
                span,
                first,
                amount,
                element: None
            }

        } else {
            let mut items = Vec::new();
            items.push(first);

            while inner.peek(Token![,]) {
                let _: Token![,] = inner.parse()?;

                items.push(parse_reg(ctx, inner)?.ok_or_else(|| inner.error("Expected register"))?);
            }

            RawArg::CommaList {
                span,
                items,
                element: None
            }
        };

        // parse a trailing [element] declaration
        if input.peek(syn::token::Bracket) {
            let inner;
            let _ = syn::bracketed!(inner in input);
            let inner = &inner;

            let expr: syn::Expr = inner.parse()?;
            match ast {
                RawArg::DashList {ref mut element, ..} |
                RawArg::CommaList {ref mut element, ..} |
                RawArg::AmountList {ref mut element, ..} => *element = Some(expr),
                _ => ()
            }
        }

        return Ok(ast)
    }

    // modifier
    if let Some(modifier) = input.parse_opt()? {
        return Ok(RawArg::Modifier {
            span: _start,
            modifier
        });
    }

    // immediate (arm notation)
    if input.peek(Token![#]) {
        let _: Token![#] = input.parse()?;
        let arg: syn::Expr = input.parse()?;
        return Ok(RawArg::Immediate {
            prefixed: true,
            value: arg
        });
    }

    // register
    if let Some(reg) = parse_reg(ctx, input)? {
        return Ok(RawArg::Direct {
            reg,
            span: _start
        })
    }

    // immediate (relaxed notation)
    let arg: syn::Expr = input.parse()?;
    Ok(RawArg::Immediate {
        prefixed: false,
        value: arg
    })
}

fn parse_refitem(ctx: &mut Context, input: parse::ParseStream) -> parse::Result<RefItem> {
    let _start = input.cursor().span(); // FIXME can't join spans yet

    // modifier
    if let Some(modifier) = input.parse_opt()? {
        return Ok(RefItem::Modifier {
            span: _start,
            modifier
        });
    }

    // immediate (arm notation)
    if input.peek(Token![#]) {
        let _: Token![#] = input.parse()?;
        let arg: syn::Expr = input.parse()?;
        return Ok(RefItem::Immediate {
            value: arg
        });
    }

    // register
    if let Some(reg) = parse_reg(ctx, input)? {
        return Ok(RefItem::Direct {
            reg,
            span: _start
        })
    }

    // immediate (relaxed notation)
    let arg: syn::Expr = input.parse()?;
    Ok(RefItem::Immediate {
        value: arg
    })
}

fn parse_reg(ctx: &mut Context, input: parse::ParseStream) -> parse::Result<Option<Register>> {
    let name = match input.step(|cursor| {
        if let Some((ident, rest)) = cursor.ident() {
            let mut ident = ident.to_string();

            if AARCH64_FAMILIES.contains_key(&*ident) {
                return Ok((ident, rest));
            }

            if let Some(repl) = ctx.state.invocation_context.aliases.get(&ident) {
                ident = repl.clone();
            }

            if AARCH64_REGISTERS.contains_key(&*ident) {
                return Ok((ident, rest));
            }
        }
        Err(cursor.error("expected identifier"))
    }) {
        Ok(name) => name,
        Err(_) => return Ok(None)
    };

    let kind;
    let size;

    if let Some(&(id, s)) = AARCH64_REGISTERS.get(&*name) {
        size = s;
        kind = RegKind::Static(id);

    } else if let Some(&(family, s)) = AARCH64_FAMILIES.get(&*name) {

        // parse the dynamic register expression
        let inner;
        let _ = syn::parenthesized!(inner in input);
        let inner = &inner;

        let expr: syn::Expr = inner.parse()?;

        size = s;
        kind = RegKind::Dynamic(family, expr);
    } else {
        unreachable!();
    };

    if let Some(size) = size {
        Ok(Some(Register::Scalar(RegScalar {
            kind,
            size
        })))
    } else {
        // parse possible vector trailers

        // parse the elementsize/lanes specifier ("." [BHSDQ][124816])
        // note: actual ARM register width specifiers put the lane count before
        // the size, but this cannot be parsed by the Rust tokenizer
        let _: Token![.] = input.parse()?;
        let (element_size, lanes) = input.step(|cursor| {
            if let Some((ident, rest)) = cursor.ident() {
                let ident = ident.to_string();

                if !ident.is_char_boundary(1) {
                    return Err(cursor.error("Invalid width specifier"));
                }

                let (first, trailer) = ident.split_at(1);
                let element_size = match first {
                    "B" | "b" => Size::BYTE,
                    "H" | "h" => Size::B_2,
                    "S" | "s" => Size::B_4,
                    "D" | "d" => Size::B_8,
                    "Q" | "q" => Size::B_16,
                    _ => return Err(cursor.error("Invalid width specifier"))
                };

                let lanes = if trailer.is_empty() {
                    None
                } else {
                    Some(match trailer.parse::<u8>() {
                        Ok(1) => 1,
                        Ok(2) => 2,
                        Ok(4) => 4,
                        Ok(8) => 8,
                        Ok(16)=> 16,
                        _ => return Err(cursor.error("Invalid width specifier"))
                    })
                };

                return Ok(((element_size, lanes), rest));
            }
            Err(cursor.error("Expected a width/lane specifier"))
        })?;

        // parse the element specifier
        let element = if input.peek(syn::token::Bracket) {
            let inner;
            let _ = syn::bracketed!(inner in input);
            let inner = &inner;

            let expr: syn::Expr = inner.parse()?;
            Some(expr)
        } else {
            None
        };

        Ok(Some(Register::Vector(RegVector {
            kind,
            element_size,
            lanes,
            element
        })))
    }
}

impl ParseOpt for ModifyExpr {
    fn parse(input: parse::ParseStream) -> parse::Result<Option<Self>> {
        let modifier: Modifier = match input.parse() {
            Ok(m) => m,
            Err(_) => return Ok(None)
        };

        // valid terminating symbols
        if input.is_empty() || input.peek(Token![;]) || input.peek(Token![,]) {
            return Ok(Some(ModifyExpr::new(modifier, None)));
        }

        // parse an expr with possible leading # sign
        if input.peek(Token![#]) {
            let _: Token![#] = input.parse()?;
        }
        let expr: syn::Expr = input.parse()?;

        Ok(Some(ModifyExpr::new(modifier, Some(expr))))
    }
}

impl parse::Parse for Modifier {
    fn parse(input: parse::ParseStream) -> parse::Result<Modifier> {
        input.step(|cursor| {
            if let Some((ident, rest)) = cursor.ident() {
                let modifier = match &*ident.to_string() {
                    "LSL"  | "lsl"  => Modifier::LSL,
                    "LSR"  | "lsr"  => Modifier::LSR,
                    "ASR"  | "asr"  => Modifier::ASR,
                    "ROR"  | "ror"  => Modifier::ROR,
                    "SXTX" | "sxtx" => Modifier::SXTX,
                    "SXTW" | "sxtw" => Modifier::SXTW,
                    "SXTH" | "sxth" => Modifier::SXTH,
                    "SXTB" | "sxtb" => Modifier::SXTB,
                    "UXTX" | "uxtx" => Modifier::UXTX,
                    "UXTW" | "uxtw" => Modifier::UXTW,
                    "UXTH" | "uxth" => Modifier::UXTH,
                    "UXTB" | "uxtb" => Modifier::UXTB,
                    "MSL"  | "msl"  => Modifier::MSL,
                    _ => return Err(cursor.error("Unknown modifier"))
                };

                Ok((modifier, rest))
            } else {
                Err(cursor.error("Expected modifier"))
            }
        })
    }
}

lazy_static!{
    static ref AARCH64_REGISTERS: HashMap<&'static str, (RegId, Option<Size>)> = {
        use self::RegId::*;
        use crate::common::Size::*;

        static MAP: &[(&str, (RegId, Option<Size>))] = &[
            ("x0" , (X0 , Some(B_8))),
            ("x1" , (X1 , Some(B_8))),
            ("x2" , (X2 , Some(B_8))),
            ("x3" , (X3 , Some(B_8))),
            ("x4" , (X4 , Some(B_8))),
            ("x5" , (X5 , Some(B_8))),
            ("x6" , (X6 , Some(B_8))),
            ("x7" , (X7 , Some(B_8))),
            ("x8" , (X8 , Some(B_8))),
            ("x9" , (X9 , Some(B_8))),
            ("x10", (X10, Some(B_8))),
            ("x11", (X11, Some(B_8))),
            ("x12", (X12, Some(B_8))),
            ("x13", (X13, Some(B_8))),
            ("x14", (X14, Some(B_8))),
            ("x15", (X15, Some(B_8))),
            ("x16", (X16, Some(B_8))),
            ("x17", (X17, Some(B_8))),
            ("x18", (X18, Some(B_8))),
            ("x19", (X19, Some(B_8))),
            ("x20", (X20, Some(B_8))),
            ("x21", (X21, Some(B_8))),
            ("x22", (X22, Some(B_8))),
            ("x23", (X23, Some(B_8))),
            ("x24", (X24, Some(B_8))),
            ("x25", (X25, Some(B_8))),
            ("x26", (X26, Some(B_8))),
            ("x27", (X27, Some(B_8))),
            ("x28", (X28, Some(B_8))),
            ("x29", (X29, Some(B_8))),
            ("x30", (X30, Some(B_8))),

            ("w0" , (X0 , Some(B_4))),
            ("w1" , (X1 , Some(B_4))),
            ("w2" , (X2 , Some(B_4))),
            ("w3" , (X3 , Some(B_4))),
            ("w4" , (X4 , Some(B_4))),
            ("w5" , (X5 , Some(B_4))),
            ("w6" , (X6 , Some(B_4))),
            ("w7" , (X7 , Some(B_4))),
            ("w8" , (X8 , Some(B_4))),
            ("w9" , (X9 , Some(B_4))),
            ("w10", (X10, Some(B_4))),
            ("w11", (X11, Some(B_4))),
            ("w12", (X12, Some(B_4))),
            ("w13", (X13, Some(B_4))),
            ("w14", (X14, Some(B_4))),
            ("w15", (X15, Some(B_4))),
            ("w16", (X16, Some(B_4))),
            ("w17", (X17, Some(B_4))),
            ("w18", (X18, Some(B_4))),
            ("w19", (X19, Some(B_4))),
            ("w20", (X20, Some(B_4))),
            ("w21", (X21, Some(B_4))),
            ("w22", (X22, Some(B_4))),
            ("w23", (X23, Some(B_4))),
            ("w24", (X24, Some(B_4))),
            ("w25", (X25, Some(B_4))),
            ("w26", (X26, Some(B_4))),
            ("w27", (X27, Some(B_4))),
            ("w28", (X28, Some(B_4))),
            ("w29", (X29, Some(B_4))),
            ("w30", (X30, Some(B_4))),

            ("sp",  (SP,  Some(B_8))),
            ("wsp", (SP,  Some(B_4))),

            ("xzr", (XZR, Some(B_8))),
            ("wzr", (XZR, Some(B_4))),

            ("b0" , (V0 , Some(BYTE))),
            ("b1" , (V1 , Some(BYTE))),
            ("b2" , (V2 , Some(BYTE))),
            ("b3" , (V3 , Some(BYTE))),
            ("b4" , (V4 , Some(BYTE))),
            ("b5" , (V5 , Some(BYTE))),
            ("b6" , (V6 , Some(BYTE))),
            ("b7" , (V7 , Some(BYTE))),
            ("b8" , (V8 , Some(BYTE))),
            ("b9" , (V9 , Some(BYTE))),
            ("b10", (V10, Some(BYTE))),
            ("b11", (V11, Some(BYTE))),
            ("b12", (V12, Some(BYTE))),
            ("b13", (V13, Some(BYTE))),
            ("b14", (V14, Some(BYTE))),
            ("b15", (V15, Some(BYTE))),
            ("b16", (V16, Some(BYTE))),
            ("b17", (V17, Some(BYTE))),
            ("b18", (V18, Some(BYTE))),
            ("b19", (V19, Some(BYTE))),
            ("b20", (V20, Some(BYTE))),
            ("b21", (V21, Some(BYTE))),
            ("b22", (V22, Some(BYTE))),
            ("b23", (V23, Some(BYTE))),
            ("b24", (V24, Some(BYTE))),
            ("b25", (V25, Some(BYTE))),
            ("b26", (V26, Some(BYTE))),
            ("b27", (V27, Some(BYTE))),
            ("b28", (V28, Some(BYTE))),
            ("b29", (V29, Some(BYTE))),
            ("b30", (V30, Some(BYTE))),
            ("b31", (V31, Some(BYTE))),

            ("h0" , (V0 , Some(B_2))),
            ("h1" , (V1 , Some(B_2))),
            ("h2" , (V2 , Some(B_2))),
            ("h3" , (V3 , Some(B_2))),
            ("h4" , (V4 , Some(B_2))),
            ("h5" , (V5 , Some(B_2))),
            ("h6" , (V6 , Some(B_2))),
            ("h7" , (V7 , Some(B_2))),
            ("h8" , (V8 , Some(B_2))),
            ("h9" , (V9 , Some(B_2))),
            ("h10", (V10, Some(B_2))),
            ("h11", (V11, Some(B_2))),
            ("h12", (V12, Some(B_2))),
            ("h13", (V13, Some(B_2))),
            ("h14", (V14, Some(B_2))),
            ("h15", (V15, Some(B_2))),
            ("h16", (V16, Some(B_2))),
            ("h17", (V17, Some(B_2))),
            ("h18", (V18, Some(B_2))),
            ("h19", (V19, Some(B_2))),
            ("h20", (V20, Some(B_2))),
            ("h21", (V21, Some(B_2))),
            ("h22", (V22, Some(B_2))),
            ("h23", (V23, Some(B_2))),
            ("h24", (V24, Some(B_2))),
            ("h25", (V25, Some(B_2))),
            ("h26", (V26, Some(B_2))),
            ("h27", (V27, Some(B_2))),
            ("h28", (V28, Some(B_2))),
            ("h29", (V29, Some(B_2))),
            ("h30", (V30, Some(B_2))),
            ("h31", (V31, Some(B_2))),

            ("s0" , (V0 , Some(B_4))),
            ("s1" , (V1 , Some(B_4))),
            ("s2" , (V2 , Some(B_4))),
            ("s3" , (V3 , Some(B_4))),
            ("s4" , (V4 , Some(B_4))),
            ("s5" , (V5 , Some(B_4))),
            ("s6" , (V6 , Some(B_4))),
            ("s7" , (V7 , Some(B_4))),
            ("s8" , (V8 , Some(B_4))),
            ("s9" , (V9 , Some(B_4))),
            ("s10", (V10, Some(B_4))),
            ("s11", (V11, Some(B_4))),
            ("s12", (V12, Some(B_4))),
            ("s13", (V13, Some(B_4))),
            ("s14", (V14, Some(B_4))),
            ("s15", (V15, Some(B_4))),
            ("s16", (V16, Some(B_4))),
            ("s17", (V17, Some(B_4))),
            ("s18", (V18, Some(B_4))),
            ("s19", (V19, Some(B_4))),
            ("s20", (V20, Some(B_4))),
            ("s21", (V21, Some(B_4))),
            ("s22", (V22, Some(B_4))),
            ("s23", (V23, Some(B_4))),
            ("s24", (V24, Some(B_4))),
            ("s25", (V25, Some(B_4))),
            ("s26", (V26, Some(B_4))),
            ("s27", (V27, Some(B_4))),
            ("s28", (V28, Some(B_4))),
            ("s29", (V29, Some(B_4))),
            ("s30", (V30, Some(B_4))),
            ("s31", (V31, Some(B_4))),

            ("d0" , (V0 , Some(B_8))),
            ("d1" , (V1 , Some(B_8))),
            ("d2" , (V2 , Some(B_8))),
            ("d3" , (V3 , Some(B_8))),
            ("d4" , (V4 , Some(B_8))),
            ("d5" , (V5 , Some(B_8))),
            ("d6" , (V6 , Some(B_8))),
            ("d7" , (V7 , Some(B_8))),
            ("d8" , (V8 , Some(B_8))),
            ("d9" , (V9 , Some(B_8))),
            ("d10", (V10, Some(B_8))),
            ("d11", (V11, Some(B_8))),
            ("d12", (V12, Some(B_8))),
            ("d13", (V13, Some(B_8))),
            ("d14", (V14, Some(B_8))),
            ("d15", (V15, Some(B_8))),
            ("d16", (V16, Some(B_8))),
            ("d17", (V17, Some(B_8))),
            ("d18", (V18, Some(B_8))),
            ("d19", (V19, Some(B_8))),
            ("d20", (V20, Some(B_8))),
            ("d21", (V21, Some(B_8))),
            ("d22", (V22, Some(B_8))),
            ("d23", (V23, Some(B_8))),
            ("d24", (V24, Some(B_8))),
            ("d25", (V25, Some(B_8))),
            ("d26", (V26, Some(B_8))),
            ("d27", (V27, Some(B_8))),
            ("d28", (V28, Some(B_8))),
            ("d29", (V29, Some(B_8))),
            ("d30", (V30, Some(B_8))),
            ("d31", (V31, Some(B_8))),

            ("q0" , (V0 , Some(B_16))),
            ("q1" , (V1 , Some(B_16))),
            ("q2" , (V2 , Some(B_16))),
            ("q3" , (V3 , Some(B_16))),
            ("q4" , (V4 , Some(B_16))),
            ("q5" , (V5 , Some(B_16))),
            ("q6" , (V6 , Some(B_16))),
            ("q7" , (V7 , Some(B_16))),
            ("q8" , (V8 , Some(B_16))),
            ("q9" , (V9 , Some(B_16))),
            ("q10", (V10, Some(B_16))),
            ("q11", (V11, Some(B_16))),
            ("q12", (V12, Some(B_16))),
            ("q13", (V13, Some(B_16))),
            ("q14", (V14, Some(B_16))),
            ("q15", (V15, Some(B_16))),
            ("q16", (V16, Some(B_16))),
            ("q17", (V17, Some(B_16))),
            ("q18", (V18, Some(B_16))),
            ("q19", (V19, Some(B_16))),
            ("q20", (V20, Some(B_16))),
            ("q21", (V21, Some(B_16))),
            ("q22", (V22, Some(B_16))),
            ("q23", (V23, Some(B_16))),
            ("q24", (V24, Some(B_16))),
            ("q25", (V25, Some(B_16))),
            ("q26", (V26, Some(B_16))),
            ("q27", (V27, Some(B_16))),
            ("q28", (V28, Some(B_16))),
            ("q29", (V29, Some(B_16))),
            ("q30", (V30, Some(B_16))),
            ("q31", (V31, Some(B_16))),

            ("v0" , (V0 , None)),
            ("v1" , (V1 , None)),
            ("v2" , (V2 , None)),
            ("v3" , (V3 , None)),
            ("v4" , (V4 , None)),
            ("v5" , (V5 , None)),
            ("v6" , (V6 , None)),
            ("v7" , (V7 , None)),
            ("v8" , (V8 , None)),
            ("v9" , (V9 , None)),
            ("v10", (V10, None)),
            ("v11", (V11, None)),
            ("v12", (V12, None)),
            ("v13", (V13, None)),
            ("v14", (V14, None)),
            ("v15", (V15, None)),
            ("v16", (V16, None)),
            ("v17", (V17, None)),
            ("v18", (V18, None)),
            ("v19", (V19, None)),
            ("v20", (V20, None)),
            ("v21", (V21, None)),
            ("v22", (V22, None)),
            ("v23", (V23, None)),
            ("v24", (V24, None)),
            ("v25", (V25, None)),
            ("v26", (V26, None)),
            ("v27", (V27, None)),
            ("v28", (V28, None)),
            ("v29", (V29, None)),
            ("v30", (V30, None)),
            ("v31", (V31, None)),
        ];
        MAP.iter().cloned().collect()
    };

    static ref AARCH64_FAMILIES: HashMap<&'static str, (RegFamily, Option<Size>)> = {
        static MAP: &[(&str, (RegFamily, Option<Size>))] = &[
            ("X",   (RegFamily::INTEGER,   Some(Size::B_8))),
            ("W",   (RegFamily::INTEGER,   Some(Size::B_4))),
            ("XSP", (RegFamily::INTEGERSP, Some(Size::B_8))),
            ("WSP", (RegFamily::INTEGERSP, Some(Size::B_4))),

            ("B", (RegFamily::SIMD, Some(Size::BYTE))),
            ("H", (RegFamily::SIMD, Some(Size::B_2))),
            ("S", (RegFamily::SIMD, Some(Size::B_4))),
            ("D", (RegFamily::SIMD, Some(Size::B_8))),
            ("Q", (RegFamily::SIMD, Some(Size::B_16))),

            ("V", (RegFamily::SIMD, None)),
        ];
        MAP.iter().cloned().collect()
    };
}
