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

        let arg: syn::Expr = input.parse()?;
        args.push(RawArg::Immediate {
            prefixed: false,
            value: arg
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
                    "B" => Size::BYTE,
                    "H" => Size::WORD,
                    "S" => Size::DWORD,
                    "D" => Size::QWORD,
                    "Q" => Size::OWORD,
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
                    "LSL"  => Modifier::LSL,
                    "LSR"  => Modifier::LSR,
                    "ASR"  => Modifier::ASR,
                    "ROR"  => Modifier::ROR,
                    "SXTX" => Modifier::SXTX,
                    "SXTW" => Modifier::SXTW,
                    "SXTH" => Modifier::SXTH,
                    "SXTB" => Modifier::SXTB,
                    "UXTX" => Modifier::UXTX,
                    "UXTW" => Modifier::UXTW,
                    "UXTH" => Modifier::UXTH,
                    "UXTB" => Modifier::UXTB,
                    "MSL"  => Modifier::MSL,
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

        h.insert("X0" , (X0 , Some(QWORD)));
        h.insert("X1" , (X1 , Some(QWORD)));
        h.insert("X2" , (X2 , Some(QWORD)));
        h.insert("X3" , (X3 , Some(QWORD)));
        h.insert("X4" , (X4 , Some(QWORD)));
        h.insert("X5" , (X5 , Some(QWORD)));
        h.insert("X6" , (X6 , Some(QWORD)));
        h.insert("X7" , (X7 , Some(QWORD)));
        h.insert("X8" , (X8 , Some(QWORD)));
        h.insert("X9" , (X9 , Some(QWORD)));
        h.insert("X10", (X10, Some(QWORD)));
        h.insert("X11", (X11, Some(QWORD)));
        h.insert("X12", (X12, Some(QWORD)));
        h.insert("X13", (X13, Some(QWORD)));
        h.insert("X14", (X14, Some(QWORD)));
        h.insert("X15", (X15, Some(QWORD)));
        h.insert("X16", (X16, Some(QWORD)));
        h.insert("X17", (X17, Some(QWORD)));
        h.insert("X18", (X18, Some(QWORD)));
        h.insert("X19", (X19, Some(QWORD)));
        h.insert("X20", (X20, Some(QWORD)));
        h.insert("X21", (X21, Some(QWORD)));
        h.insert("X22", (X22, Some(QWORD)));
        h.insert("X23", (X23, Some(QWORD)));
        h.insert("X24", (X24, Some(QWORD)));
        h.insert("X25", (X25, Some(QWORD)));
        h.insert("X26", (X26, Some(QWORD)));
        h.insert("X27", (X27, Some(QWORD)));
        h.insert("X28", (X28, Some(QWORD)));
        h.insert("X29", (X29, Some(QWORD)));
        h.insert("X30", (X30, Some(QWORD)));

        h.insert("W0" , (X0 , Some(DWORD)));
        h.insert("W1" , (X1 , Some(DWORD)));
        h.insert("W2" , (X2 , Some(DWORD)));
        h.insert("W3" , (X3 , Some(DWORD)));
        h.insert("W4" , (X4 , Some(DWORD)));
        h.insert("W5" , (X5 , Some(DWORD)));
        h.insert("W6" , (X6 , Some(DWORD)));
        h.insert("W7" , (X7 , Some(DWORD)));
        h.insert("W8" , (X8 , Some(DWORD)));
        h.insert("W9" , (X9 , Some(DWORD)));
        h.insert("W10", (X10, Some(DWORD)));
        h.insert("W11", (X11, Some(DWORD)));
        h.insert("W12", (X12, Some(DWORD)));
        h.insert("W13", (X13, Some(DWORD)));
        h.insert("W14", (X14, Some(DWORD)));
        h.insert("W15", (X15, Some(DWORD)));
        h.insert("W16", (X16, Some(DWORD)));
        h.insert("W17", (X17, Some(DWORD)));
        h.insert("W18", (X18, Some(DWORD)));
        h.insert("W19", (X19, Some(DWORD)));
        h.insert("W20", (X20, Some(DWORD)));
        h.insert("W21", (X21, Some(DWORD)));
        h.insert("W22", (X22, Some(DWORD)));
        h.insert("W23", (X23, Some(DWORD)));
        h.insert("W24", (X24, Some(DWORD)));
        h.insert("W25", (X25, Some(DWORD)));
        h.insert("W26", (X26, Some(DWORD)));
        h.insert("W27", (X27, Some(DWORD)));
        h.insert("W28", (X28, Some(DWORD)));
        h.insert("W29", (X29, Some(DWORD)));
        h.insert("W30", (X30, Some(DWORD)));

        h.insert("SP",  (SP,  Some(QWORD)));
        h.insert("WSP", (SP,  Some(DWORD)));

        h.insert("XZR", (XZR, Some(QWORD)));
        h.insert("WZR", (XZR, Some(DWORD)));

        h.insert("B0" , (V0 , Some(BYTE)));
        h.insert("B1" , (V1 , Some(BYTE)));
        h.insert("B2" , (V2 , Some(BYTE)));
        h.insert("B3" , (V3 , Some(BYTE)));
        h.insert("B4" , (V4 , Some(BYTE)));
        h.insert("B5" , (V5 , Some(BYTE)));
        h.insert("B6" , (V6 , Some(BYTE)));
        h.insert("B7" , (V7 , Some(BYTE)));
        h.insert("B8" , (V8 , Some(BYTE)));
        h.insert("B9" , (V9 , Some(BYTE)));
        h.insert("B10", (V10, Some(BYTE)));
        h.insert("B11", (V11, Some(BYTE)));
        h.insert("B12", (V12, Some(BYTE)));
        h.insert("B13", (V13, Some(BYTE)));
        h.insert("B14", (V14, Some(BYTE)));
        h.insert("B15", (V15, Some(BYTE)));
        h.insert("B16", (V16, Some(BYTE)));
        h.insert("B17", (V17, Some(BYTE)));
        h.insert("B18", (V18, Some(BYTE)));
        h.insert("B19", (V19, Some(BYTE)));
        h.insert("B20", (V20, Some(BYTE)));
        h.insert("B21", (V21, Some(BYTE)));
        h.insert("B22", (V22, Some(BYTE)));
        h.insert("B23", (V23, Some(BYTE)));
        h.insert("B24", (V24, Some(BYTE)));
        h.insert("B25", (V25, Some(BYTE)));
        h.insert("B26", (V26, Some(BYTE)));
        h.insert("B27", (V27, Some(BYTE)));
        h.insert("B28", (V28, Some(BYTE)));
        h.insert("B29", (V29, Some(BYTE)));
        h.insert("B30", (V30, Some(BYTE)));
        h.insert("B31", (V31, Some(BYTE)));

        h.insert("H0" , (V0 , Some(WORD)));
        h.insert("H1" , (V1 , Some(WORD)));
        h.insert("H2" , (V2 , Some(WORD)));
        h.insert("H3" , (V3 , Some(WORD)));
        h.insert("H4" , (V4 , Some(WORD)));
        h.insert("H5" , (V5 , Some(WORD)));
        h.insert("H6" , (V6 , Some(WORD)));
        h.insert("H7" , (V7 , Some(WORD)));
        h.insert("H8" , (V8 , Some(WORD)));
        h.insert("H9" , (V9 , Some(WORD)));
        h.insert("H10", (V10, Some(WORD)));
        h.insert("H11", (V11, Some(WORD)));
        h.insert("H12", (V12, Some(WORD)));
        h.insert("H13", (V13, Some(WORD)));
        h.insert("H14", (V14, Some(WORD)));
        h.insert("H15", (V15, Some(WORD)));
        h.insert("H16", (V16, Some(WORD)));
        h.insert("H17", (V17, Some(WORD)));
        h.insert("H18", (V18, Some(WORD)));
        h.insert("H19", (V19, Some(WORD)));
        h.insert("H20", (V20, Some(WORD)));
        h.insert("H21", (V21, Some(WORD)));
        h.insert("H22", (V22, Some(WORD)));
        h.insert("H23", (V23, Some(WORD)));
        h.insert("H24", (V24, Some(WORD)));
        h.insert("H25", (V25, Some(WORD)));
        h.insert("H26", (V26, Some(WORD)));
        h.insert("H27", (V27, Some(WORD)));
        h.insert("H28", (V28, Some(WORD)));
        h.insert("H29", (V29, Some(WORD)));
        h.insert("H30", (V30, Some(WORD)));
        h.insert("H31", (V31, Some(WORD)));

        h.insert("S0" , (V0 , Some(DWORD)));
        h.insert("S1" , (V1 , Some(DWORD)));
        h.insert("S2" , (V2 , Some(DWORD)));
        h.insert("S3" , (V3 , Some(DWORD)));
        h.insert("S4" , (V4 , Some(DWORD)));
        h.insert("S5" , (V5 , Some(DWORD)));
        h.insert("S6" , (V6 , Some(DWORD)));
        h.insert("S7" , (V7 , Some(DWORD)));
        h.insert("S8" , (V8 , Some(DWORD)));
        h.insert("S9" , (V9 , Some(DWORD)));
        h.insert("S10", (V10, Some(DWORD)));
        h.insert("S11", (V11, Some(DWORD)));
        h.insert("S12", (V12, Some(DWORD)));
        h.insert("S13", (V13, Some(DWORD)));
        h.insert("S14", (V14, Some(DWORD)));
        h.insert("S15", (V15, Some(DWORD)));
        h.insert("S16", (V16, Some(DWORD)));
        h.insert("S17", (V17, Some(DWORD)));
        h.insert("S18", (V18, Some(DWORD)));
        h.insert("S19", (V19, Some(DWORD)));
        h.insert("S20", (V20, Some(DWORD)));
        h.insert("S21", (V21, Some(DWORD)));
        h.insert("S22", (V22, Some(DWORD)));
        h.insert("S23", (V23, Some(DWORD)));
        h.insert("S24", (V24, Some(DWORD)));
        h.insert("S25", (V25, Some(DWORD)));
        h.insert("S26", (V26, Some(DWORD)));
        h.insert("S27", (V27, Some(DWORD)));
        h.insert("S28", (V28, Some(DWORD)));
        h.insert("S29", (V29, Some(DWORD)));
        h.insert("S30", (V30, Some(DWORD)));
        h.insert("S31", (V31, Some(DWORD)));

        h.insert("D0" , (V0 , Some(QWORD)));
        h.insert("D1" , (V1 , Some(QWORD)));
        h.insert("D2" , (V2 , Some(QWORD)));
        h.insert("D3" , (V3 , Some(QWORD)));
        h.insert("D4" , (V4 , Some(QWORD)));
        h.insert("D5" , (V5 , Some(QWORD)));
        h.insert("D6" , (V6 , Some(QWORD)));
        h.insert("D7" , (V7 , Some(QWORD)));
        h.insert("D8" , (V8 , Some(QWORD)));
        h.insert("D9" , (V9 , Some(QWORD)));
        h.insert("D10", (V10, Some(QWORD)));
        h.insert("D11", (V11, Some(QWORD)));
        h.insert("D12", (V12, Some(QWORD)));
        h.insert("D13", (V13, Some(QWORD)));
        h.insert("D14", (V14, Some(QWORD)));
        h.insert("D15", (V15, Some(QWORD)));
        h.insert("D16", (V16, Some(QWORD)));
        h.insert("D17", (V17, Some(QWORD)));
        h.insert("D18", (V18, Some(QWORD)));
        h.insert("D19", (V19, Some(QWORD)));
        h.insert("D20", (V20, Some(QWORD)));
        h.insert("D21", (V21, Some(QWORD)));
        h.insert("D22", (V22, Some(QWORD)));
        h.insert("D23", (V23, Some(QWORD)));
        h.insert("D24", (V24, Some(QWORD)));
        h.insert("D25", (V25, Some(QWORD)));
        h.insert("D26", (V26, Some(QWORD)));
        h.insert("D27", (V27, Some(QWORD)));
        h.insert("D28", (V28, Some(QWORD)));
        h.insert("D29", (V29, Some(QWORD)));
        h.insert("D30", (V30, Some(QWORD)));
        h.insert("D31", (V31, Some(QWORD)));

        h.insert("Q0" , (V0 , Some(OWORD)));
        h.insert("Q1" , (V1 , Some(OWORD)));
        h.insert("Q2" , (V2 , Some(OWORD)));
        h.insert("Q3" , (V3 , Some(OWORD)));
        h.insert("Q4" , (V4 , Some(OWORD)));
        h.insert("Q5" , (V5 , Some(OWORD)));
        h.insert("Q6" , (V6 , Some(OWORD)));
        h.insert("Q7" , (V7 , Some(OWORD)));
        h.insert("Q8" , (V8 , Some(OWORD)));
        h.insert("Q9" , (V9 , Some(OWORD)));
        h.insert("Q10", (V10, Some(OWORD)));
        h.insert("Q11", (V11, Some(OWORD)));
        h.insert("Q12", (V12, Some(OWORD)));
        h.insert("Q13", (V13, Some(OWORD)));
        h.insert("Q14", (V14, Some(OWORD)));
        h.insert("Q15", (V15, Some(OWORD)));
        h.insert("Q16", (V16, Some(OWORD)));
        h.insert("Q17", (V17, Some(OWORD)));
        h.insert("Q18", (V18, Some(OWORD)));
        h.insert("Q19", (V19, Some(OWORD)));
        h.insert("Q20", (V20, Some(OWORD)));
        h.insert("Q21", (V21, Some(OWORD)));
        h.insert("Q22", (V22, Some(OWORD)));
        h.insert("Q23", (V23, Some(OWORD)));
        h.insert("Q24", (V24, Some(OWORD)));
        h.insert("Q25", (V25, Some(OWORD)));
        h.insert("Q26", (V26, Some(OWORD)));
        h.insert("Q27", (V27, Some(OWORD)));
        h.insert("Q28", (V28, Some(OWORD)));
        h.insert("Q29", (V29, Some(OWORD)));
        h.insert("Q30", (V30, Some(OWORD)));
        h.insert("Q31", (V31, Some(OWORD)));

        h.insert("V0" , (V0 , None));
        h.insert("V1" , (V1 , None));
        h.insert("V2" , (V2 , None));
        h.insert("V3" , (V3 , None));
        h.insert("V4" , (V4 , None));
        h.insert("V5" , (V5 , None));
        h.insert("V6" , (V6 , None));
        h.insert("V7" , (V7 , None));
        h.insert("V8" , (V8 , None));
        h.insert("V9" , (V9 , None));
        h.insert("V10", (V10, None));
        h.insert("V11", (V11, None));
        h.insert("V12", (V12, None));
        h.insert("V13", (V13, None));
        h.insert("V14", (V14, None));
        h.insert("V15", (V15, None));
        h.insert("V16", (V16, None));
        h.insert("V17", (V17, None));
        h.insert("V18", (V18, None));
        h.insert("V19", (V19, None));
        h.insert("V20", (V20, None));
        h.insert("V21", (V21, None));
        h.insert("V22", (V22, None));
        h.insert("V23", (V23, None));
        h.insert("V24", (V24, None));
        h.insert("V25", (V25, None));
        h.insert("V26", (V26, None));
        h.insert("V27", (V27, None));
        h.insert("V28", (V28, None));
        h.insert("V29", (V29, None));
        h.insert("V30", (V30, None));
        h.insert("V31", (V31, None));
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
