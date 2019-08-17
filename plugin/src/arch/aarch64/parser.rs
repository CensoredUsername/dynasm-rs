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
        args.push(RawArg::Dot { span } );

        let arg: syn::Ident = input.parse()?;
        args.push(RawArg::Lit {
            ident: arg
        });
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
            type_: jump
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
            span: span,
            items: items,
            bang: bang
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
                span: span,
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
                span: span,
                items: items,
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
            modifier: modifier
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
            modifier: modifier
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

            if let Some(repl) = ctx.state.file_data.aliases.get(&ident) {
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
            kind: kind,
            size: size
        })))
    } else {
        // parse possible vector trailers

        // parse the elementsize/lanes specifier ("." [BHSDQ][124816])
        // note: actual ARM register width specifiers put the lane count before
        // the size, but this cannot be parsed by the rust tokenizer
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
                    "H" | "h" => Size::WORD,
                    "S" | "s" => Size::DWORD,
                    "D" | "d" => Size::QWORD,
                    "Q" | "q" => Size::OWORD,
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
            return Err(cursor.error("Expected a width/lane specifier"));
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
        let mut h = HashMap::new();

        h.insert("x0" , (X0 , Some(QWORD)));
        h.insert("x1" , (X1 , Some(QWORD)));
        h.insert("x2" , (X2 , Some(QWORD)));
        h.insert("x3" , (X3 , Some(QWORD)));
        h.insert("x4" , (X4 , Some(QWORD)));
        h.insert("x5" , (X5 , Some(QWORD)));
        h.insert("x6" , (X6 , Some(QWORD)));
        h.insert("x7" , (X7 , Some(QWORD)));
        h.insert("x8" , (X8 , Some(QWORD)));
        h.insert("x9" , (X9 , Some(QWORD)));
        h.insert("x10", (X10, Some(QWORD)));
        h.insert("x11", (X11, Some(QWORD)));
        h.insert("x12", (X12, Some(QWORD)));
        h.insert("x13", (X13, Some(QWORD)));
        h.insert("x14", (X14, Some(QWORD)));
        h.insert("x15", (X15, Some(QWORD)));
        h.insert("x16", (X16, Some(QWORD)));
        h.insert("x17", (X17, Some(QWORD)));
        h.insert("x18", (X18, Some(QWORD)));
        h.insert("x19", (X19, Some(QWORD)));
        h.insert("x20", (X20, Some(QWORD)));
        h.insert("x21", (X21, Some(QWORD)));
        h.insert("x22", (X22, Some(QWORD)));
        h.insert("x23", (X23, Some(QWORD)));
        h.insert("x24", (X24, Some(QWORD)));
        h.insert("x25", (X25, Some(QWORD)));
        h.insert("x26", (X26, Some(QWORD)));
        h.insert("x27", (X27, Some(QWORD)));
        h.insert("x28", (X28, Some(QWORD)));
        h.insert("x29", (X29, Some(QWORD)));
        h.insert("x30", (X30, Some(QWORD)));

        h.insert("w0" , (X0 , Some(DWORD)));
        h.insert("w1" , (X1 , Some(DWORD)));
        h.insert("w2" , (X2 , Some(DWORD)));
        h.insert("w3" , (X3 , Some(DWORD)));
        h.insert("w4" , (X4 , Some(DWORD)));
        h.insert("w5" , (X5 , Some(DWORD)));
        h.insert("w6" , (X6 , Some(DWORD)));
        h.insert("w7" , (X7 , Some(DWORD)));
        h.insert("w8" , (X8 , Some(DWORD)));
        h.insert("w9" , (X9 , Some(DWORD)));
        h.insert("w10", (X10, Some(DWORD)));
        h.insert("w11", (X11, Some(DWORD)));
        h.insert("w12", (X12, Some(DWORD)));
        h.insert("w13", (X13, Some(DWORD)));
        h.insert("w14", (X14, Some(DWORD)));
        h.insert("w15", (X15, Some(DWORD)));
        h.insert("w16", (X16, Some(DWORD)));
        h.insert("w17", (X17, Some(DWORD)));
        h.insert("w18", (X18, Some(DWORD)));
        h.insert("w19", (X19, Some(DWORD)));
        h.insert("w20", (X20, Some(DWORD)));
        h.insert("w21", (X21, Some(DWORD)));
        h.insert("w22", (X22, Some(DWORD)));
        h.insert("w23", (X23, Some(DWORD)));
        h.insert("w24", (X24, Some(DWORD)));
        h.insert("w25", (X25, Some(DWORD)));
        h.insert("w26", (X26, Some(DWORD)));
        h.insert("w27", (X27, Some(DWORD)));
        h.insert("w28", (X28, Some(DWORD)));
        h.insert("w29", (X29, Some(DWORD)));
        h.insert("w30", (X30, Some(DWORD)));

        h.insert("sp",  (SP,  Some(QWORD)));
        h.insert("wsp", (SP,  Some(DWORD)));

        h.insert("xzr", (XZR, Some(QWORD)));
        h.insert("wzr", (XZR, Some(DWORD)));

        h.insert("b0" , (V0 , Some(BYTE)));
        h.insert("b1" , (V1 , Some(BYTE)));
        h.insert("b2" , (V2 , Some(BYTE)));
        h.insert("b3" , (V3 , Some(BYTE)));
        h.insert("b4" , (V4 , Some(BYTE)));
        h.insert("b5" , (V5 , Some(BYTE)));
        h.insert("b6" , (V6 , Some(BYTE)));
        h.insert("b7" , (V7 , Some(BYTE)));
        h.insert("b8" , (V8 , Some(BYTE)));
        h.insert("b9" , (V9 , Some(BYTE)));
        h.insert("b10", (V10, Some(BYTE)));
        h.insert("b11", (V11, Some(BYTE)));
        h.insert("b12", (V12, Some(BYTE)));
        h.insert("b13", (V13, Some(BYTE)));
        h.insert("b14", (V14, Some(BYTE)));
        h.insert("b15", (V15, Some(BYTE)));
        h.insert("b16", (V16, Some(BYTE)));
        h.insert("b17", (V17, Some(BYTE)));
        h.insert("b18", (V18, Some(BYTE)));
        h.insert("b19", (V19, Some(BYTE)));
        h.insert("b20", (V20, Some(BYTE)));
        h.insert("b21", (V21, Some(BYTE)));
        h.insert("b22", (V22, Some(BYTE)));
        h.insert("b23", (V23, Some(BYTE)));
        h.insert("b24", (V24, Some(BYTE)));
        h.insert("b25", (V25, Some(BYTE)));
        h.insert("b26", (V26, Some(BYTE)));
        h.insert("b27", (V27, Some(BYTE)));
        h.insert("b28", (V28, Some(BYTE)));
        h.insert("b29", (V29, Some(BYTE)));
        h.insert("b30", (V30, Some(BYTE)));
        h.insert("b31", (V31, Some(BYTE)));

        h.insert("h0" , (V0 , Some(WORD)));
        h.insert("h1" , (V1 , Some(WORD)));
        h.insert("h2" , (V2 , Some(WORD)));
        h.insert("h3" , (V3 , Some(WORD)));
        h.insert("h4" , (V4 , Some(WORD)));
        h.insert("h5" , (V5 , Some(WORD)));
        h.insert("h6" , (V6 , Some(WORD)));
        h.insert("h7" , (V7 , Some(WORD)));
        h.insert("h8" , (V8 , Some(WORD)));
        h.insert("h9" , (V9 , Some(WORD)));
        h.insert("h10", (V10, Some(WORD)));
        h.insert("h11", (V11, Some(WORD)));
        h.insert("h12", (V12, Some(WORD)));
        h.insert("h13", (V13, Some(WORD)));
        h.insert("h14", (V14, Some(WORD)));
        h.insert("h15", (V15, Some(WORD)));
        h.insert("h16", (V16, Some(WORD)));
        h.insert("h17", (V17, Some(WORD)));
        h.insert("h18", (V18, Some(WORD)));
        h.insert("h19", (V19, Some(WORD)));
        h.insert("h20", (V20, Some(WORD)));
        h.insert("h21", (V21, Some(WORD)));
        h.insert("h22", (V22, Some(WORD)));
        h.insert("h23", (V23, Some(WORD)));
        h.insert("h24", (V24, Some(WORD)));
        h.insert("h25", (V25, Some(WORD)));
        h.insert("h26", (V26, Some(WORD)));
        h.insert("h27", (V27, Some(WORD)));
        h.insert("h28", (V28, Some(WORD)));
        h.insert("h29", (V29, Some(WORD)));
        h.insert("h30", (V30, Some(WORD)));
        h.insert("h31", (V31, Some(WORD)));

        h.insert("s0" , (V0 , Some(DWORD)));
        h.insert("s1" , (V1 , Some(DWORD)));
        h.insert("s2" , (V2 , Some(DWORD)));
        h.insert("s3" , (V3 , Some(DWORD)));
        h.insert("s4" , (V4 , Some(DWORD)));
        h.insert("s5" , (V5 , Some(DWORD)));
        h.insert("s6" , (V6 , Some(DWORD)));
        h.insert("s7" , (V7 , Some(DWORD)));
        h.insert("s8" , (V8 , Some(DWORD)));
        h.insert("s9" , (V9 , Some(DWORD)));
        h.insert("s10", (V10, Some(DWORD)));
        h.insert("s11", (V11, Some(DWORD)));
        h.insert("s12", (V12, Some(DWORD)));
        h.insert("s13", (V13, Some(DWORD)));
        h.insert("s14", (V14, Some(DWORD)));
        h.insert("s15", (V15, Some(DWORD)));
        h.insert("s16", (V16, Some(DWORD)));
        h.insert("s17", (V17, Some(DWORD)));
        h.insert("s18", (V18, Some(DWORD)));
        h.insert("s19", (V19, Some(DWORD)));
        h.insert("s20", (V20, Some(DWORD)));
        h.insert("s21", (V21, Some(DWORD)));
        h.insert("s22", (V22, Some(DWORD)));
        h.insert("s23", (V23, Some(DWORD)));
        h.insert("s24", (V24, Some(DWORD)));
        h.insert("s25", (V25, Some(DWORD)));
        h.insert("s26", (V26, Some(DWORD)));
        h.insert("s27", (V27, Some(DWORD)));
        h.insert("s28", (V28, Some(DWORD)));
        h.insert("s29", (V29, Some(DWORD)));
        h.insert("s30", (V30, Some(DWORD)));
        h.insert("s31", (V31, Some(DWORD)));

        h.insert("d0" , (V0 , Some(QWORD)));
        h.insert("d1" , (V1 , Some(QWORD)));
        h.insert("d2" , (V2 , Some(QWORD)));
        h.insert("d3" , (V3 , Some(QWORD)));
        h.insert("d4" , (V4 , Some(QWORD)));
        h.insert("d5" , (V5 , Some(QWORD)));
        h.insert("d6" , (V6 , Some(QWORD)));
        h.insert("d7" , (V7 , Some(QWORD)));
        h.insert("d8" , (V8 , Some(QWORD)));
        h.insert("d9" , (V9 , Some(QWORD)));
        h.insert("d10", (V10, Some(QWORD)));
        h.insert("d11", (V11, Some(QWORD)));
        h.insert("d12", (V12, Some(QWORD)));
        h.insert("d13", (V13, Some(QWORD)));
        h.insert("d14", (V14, Some(QWORD)));
        h.insert("d15", (V15, Some(QWORD)));
        h.insert("d16", (V16, Some(QWORD)));
        h.insert("d17", (V17, Some(QWORD)));
        h.insert("d18", (V18, Some(QWORD)));
        h.insert("d19", (V19, Some(QWORD)));
        h.insert("d20", (V20, Some(QWORD)));
        h.insert("d21", (V21, Some(QWORD)));
        h.insert("d22", (V22, Some(QWORD)));
        h.insert("d23", (V23, Some(QWORD)));
        h.insert("d24", (V24, Some(QWORD)));
        h.insert("d25", (V25, Some(QWORD)));
        h.insert("d26", (V26, Some(QWORD)));
        h.insert("d27", (V27, Some(QWORD)));
        h.insert("d28", (V28, Some(QWORD)));
        h.insert("d29", (V29, Some(QWORD)));
        h.insert("d30", (V30, Some(QWORD)));
        h.insert("d31", (V31, Some(QWORD)));

        h.insert("q0" , (V0 , Some(OWORD)));
        h.insert("q1" , (V1 , Some(OWORD)));
        h.insert("q2" , (V2 , Some(OWORD)));
        h.insert("q3" , (V3 , Some(OWORD)));
        h.insert("q4" , (V4 , Some(OWORD)));
        h.insert("q5" , (V5 , Some(OWORD)));
        h.insert("q6" , (V6 , Some(OWORD)));
        h.insert("q7" , (V7 , Some(OWORD)));
        h.insert("q8" , (V8 , Some(OWORD)));
        h.insert("q9" , (V9 , Some(OWORD)));
        h.insert("q10", (V10, Some(OWORD)));
        h.insert("q11", (V11, Some(OWORD)));
        h.insert("q12", (V12, Some(OWORD)));
        h.insert("q13", (V13, Some(OWORD)));
        h.insert("q14", (V14, Some(OWORD)));
        h.insert("q15", (V15, Some(OWORD)));
        h.insert("q16", (V16, Some(OWORD)));
        h.insert("q17", (V17, Some(OWORD)));
        h.insert("q18", (V18, Some(OWORD)));
        h.insert("q19", (V19, Some(OWORD)));
        h.insert("q20", (V20, Some(OWORD)));
        h.insert("q21", (V21, Some(OWORD)));
        h.insert("q22", (V22, Some(OWORD)));
        h.insert("q23", (V23, Some(OWORD)));
        h.insert("q24", (V24, Some(OWORD)));
        h.insert("q25", (V25, Some(OWORD)));
        h.insert("q26", (V26, Some(OWORD)));
        h.insert("q27", (V27, Some(OWORD)));
        h.insert("q28", (V28, Some(OWORD)));
        h.insert("q29", (V29, Some(OWORD)));
        h.insert("q30", (V30, Some(OWORD)));
        h.insert("q31", (V31, Some(OWORD)));

        h.insert("v0" , (V0 , None));
        h.insert("v1" , (V1 , None));
        h.insert("v2" , (V2 , None));
        h.insert("v3" , (V3 , None));
        h.insert("v4" , (V4 , None));
        h.insert("v5" , (V5 , None));
        h.insert("v6" , (V6 , None));
        h.insert("v7" , (V7 , None));
        h.insert("v8" , (V8 , None));
        h.insert("v9" , (V9 , None));
        h.insert("v10", (V10, None));
        h.insert("v11", (V11, None));
        h.insert("v12", (V12, None));
        h.insert("v13", (V13, None));
        h.insert("v14", (V14, None));
        h.insert("v15", (V15, None));
        h.insert("v16", (V16, None));
        h.insert("v17", (V17, None));
        h.insert("v18", (V18, None));
        h.insert("v19", (V19, None));
        h.insert("v20", (V20, None));
        h.insert("v21", (V21, None));
        h.insert("v22", (V22, None));
        h.insert("v23", (V23, None));
        h.insert("v24", (V24, None));
        h.insert("v25", (V25, None));
        h.insert("v26", (V26, None));
        h.insert("v27", (V27, None));
        h.insert("v28", (V28, None));
        h.insert("v29", (V29, None));
        h.insert("v30", (V30, None));
        h.insert("v31", (V31, None));
        h
    };

    static ref AARCH64_FAMILIES: HashMap<&'static str, (RegFamily, Option<Size>)> = {
        let mut h = HashMap::new();
        h.insert("X",   (RegFamily::INTEGER,   Some(Size::QWORD)));
        h.insert("W",   (RegFamily::INTEGER,   Some(Size::DWORD)));
        h.insert("XSP", (RegFamily::INTEGERSP, Some(Size::QWORD)));
        h.insert("WSP", (RegFamily::INTEGERSP, Some(Size::DWORD)));

        h.insert("B", (RegFamily::SIMD, Some(Size::BYTE)));
        h.insert("H", (RegFamily::SIMD, Some(Size::WORD)));
        h.insert("S", (RegFamily::SIMD, Some(Size::DWORD)));
        h.insert("D", (RegFamily::SIMD, Some(Size::QWORD)));
        h.insert("Q", (RegFamily::SIMD, Some(Size::OWORD)));

        h.insert("V", (RegFamily::SIMD, None));
        h
    };
}
