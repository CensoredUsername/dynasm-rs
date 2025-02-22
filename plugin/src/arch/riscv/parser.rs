use std::collections::HashMap;

use syn::{parse, Token};
use syn::spanned::Spanned;


use lazy_static::lazy_static;

use crate::parse_helpers::{parse_ident_or_rust_keyword, ParseOptExt};

use super::{Context, ast};

// TODO: implement register lists, and References without offset

// syntax for a single op: ident ("." ident)* (arg ("," arg)*)? ";"
pub(super) fn parse_instruction(ctx: &mut Context, input: parse::ParseStream) -> parse::Result<ast::ParsedInstruction> {
    let span = input.cursor().span();

    // read the full dot-separated op
    let mut name = parse_ident_or_rust_keyword(input)?.to_string();

    while input.peek(Token![.]) {
        let _: Token![.] = input.parse()?;
        name.push('.');
        name.push_str(&parse_ident_or_rust_keyword(input)?.to_string());
    }

    let mut args = Vec::new();

    // parse 0 or more comma-separated args
    if !(input.is_empty() || input.peek(Token![;])) {
        args.push(parse_arg(ctx, input)?);

        while input.peek(Token![,]) {
            let _: Token![,] = input.parse()?;

            args.push(parse_arg(ctx, input)?);
        }
    }

    // let span = span.join(input.cursor().span()); // FIXME can't join spans ATM

    Ok(ast::ParsedInstruction {
        name,
        span,
        args
    })
}


/// tries to parse a full arg definition
fn parse_arg(ctx: &mut Context, input: parse::ParseStream) -> parse::Result<ast::RawArg> {
    let start = input.cursor().span(); // FIXME can't join spans yet

    // a label
    if let Some(jump) = input.parse_opt()? {
        return Ok(ast::RawArg::JumpTarget {
            jump
        });
    }

    // register. we could also parse this from an expression, but this
    // would lead to worse error messages.
    if let Some(reg) = parse_reg(ctx, input)? {
        return Ok(ast::RawArg::Register {
            reg,
            span: start
        })
    }

    // expression
    let expr: syn::Expr = input.parse()?;

    // the hard part: a RISC-V style memory reference will get parsed as a normal function call by
    // rustc. but we need to allow the offset to also be an arbitrary expression. so we need to
    // check if the topmost ast node is a call expression with a single argument that is a register.
    match &expr {
        syn::Expr::Call(exprcall) => {
            if exprcall.args.len() == 1 {
                if let Some(reg) = parse_reg_from_expression(ctx, &exprcall.args[0])? {
                    return Ok(ast::RawArg::Reference {
                        span: exprcall.span(),
                        base: reg,
                        offset: Some(*exprcall.func.clone())
                    })
                }
            }
        },
        _ => ()
    }

    Ok(ast::RawArg::Immediate { value: expr })
}

/// Checks if the given expression could be a valid RISC-V register reference
/// This can be a simple register name (like `x5`)
/// an alias (any simple name that is registered, like `base`)
/// or a dynamic register (like `X(expr)`)
/// this is needed because the notation RISC-V uses for memory references is ambiguous, so we
/// can't distinguish easily at parse time.
fn parse_reg_from_expression(ctx: &mut Context, expr: &syn::Expr) -> parse::Result<Option<ast::Register>> {
    Ok(match expr {
        syn::Expr::Call(exprcall) => {
            let name = match &*exprcall.func {
                syn::Expr::Path(exprpath) => match exprpath.path.get_ident() {
                    Some(ident) => ident.to_string(),
                    None => return Ok(None)
                }
                _ => return Ok(None)
            };

            if exprcall.args.len() != 1 {
                return Err(parse::Error::new(
                    expr.span(),
                    "Too many arguments in register family expression"
                ));
            }

            if let Some(&family) = RISCV_FAMILIES.get(&*name) {
                Some(ast::Register::Dynamic(family, exprcall.args[0].clone()))
            } else {
                None
            }
        },
        syn::Expr::Path(exprpath) => {
            let mut name = match exprpath.path.get_ident() {
                Some(ident) => ident.to_string(),
                None => return Ok(None)
            };

            // fail if it is a family reference without call expression
            if RISCV_FAMILIES.contains_key(&*name) {
                return Err(parse::Error::new(
                    exprpath.path.span(),
                    "Register family reference without dynamic register id"
                ));
            }

            // check if it is an alias
            if let Some(repl) = ctx.state.invocation_context.aliases.get(&name) {
                name = repl.clone();
            }

            // resolve normal register references
            RISCV_REGISTERS.get(&*name).cloned().map(ast::Register::Static)
        },
        _ =>  None
    })
}

/// Parses a single register, if present
/// This can be a simple register name (like `x5`)
/// an alias (any simple name that is registered, like `base`)
/// or a dynamic register (like `X(expr)`)
fn parse_reg(ctx: &mut Context, input: parse::ParseStream) -> parse::Result<Option<ast::Register>> {
    // we need to consume an ident, but only if it's one of the many we care about
    // so use a step parser to figure it out.
    let name = input.step(|cursor| {
        if let Some((ident, rest)) = cursor.ident() {
            let mut ident = ident.to_string();

            // first, parse known register families
            if RISCV_FAMILIES.contains_key(&*ident) {
                return Ok((ident, rest));
            }

            // otherwise, see if this is an alias
            if let Some(repl) = ctx.state.invocation_context.aliases.get(&ident) {
                ident = repl.clone();
            }

            // resolve normal register references
            if RISCV_REGISTERS.contains_key(&*ident) {
                return Ok((ident, rest));
            }
        }
        Err(cursor.error("expected identifier"))
    });

    let name = match name {
        Ok(name) => name,
        Err(_) => return Ok(None)
    };

    // we know we have a register reference now, try to resolve it.
    let register = if let Some(&id) = RISCV_REGISTERS.get(&*name) {
        ast::Register::Static(id)
    } else if let Some(&family) = RISCV_FAMILIES.get(&*name) {
        // need to parse the trailing `( expr )`
        let inner;
        let _ = syn::parenthesized!(inner in input);
        let inner = &inner;

        let expr: syn::Expr = inner.parse()?;

        ast::Register::Dynamic(family, expr)
    } else {
        unreachable!()
    };

    Ok(Some(register))
}


lazy_static!{
    static ref RISCV_REGISTERS: HashMap<&'static str, ast::RegId> = {
        use ast::RegId::*;

        static MAP: &[(&str, ast::RegId)] = &[
            ("x0" , X0),
            ("x1" , X1),
            ("x2" , X2),
            ("x3" , X3),
            ("x4" , X4),
            ("x5" , X5),
            ("x6" , X6),
            ("x7" , X7),
            ("x8" , X8),
            ("x9" , X9),
            ("x10", X10),
            ("x11", X11),
            ("x12", X12),
            ("x13", X13),
            ("x14", X14),
            ("x15", X15),
            ("x16", X16),
            ("x17", X17),
            ("x18", X18),
            ("x19", X19),
            ("x20", X20),
            ("x21", X21),
            ("x22", X22),
            ("x23", X23),
            ("x24", X24),
            ("x25", X25),
            ("x26", X26),
            ("x27", X27),
            ("x28", X28),
            ("x29", X29),
            ("x30", X30),
            ("x31", X31),

            ("zero" , X0),
            ("ra" , X1),
            ("sp" , X2),
            ("gp" , X3),
            ("tp" , X4),
            ("t0" , X5),
            ("t1" , X6),
            ("t2" , X7),
            ("fp" , X8),
            ("s0" , X8),
            ("s1" , X9),
            ("a0", X10),
            ("a1", X11),
            ("a2", X12),
            ("a3", X13),
            ("a4", X14),
            ("a5", X15),
            ("a6", X16),
            ("a7", X17),
            ("s2", X18),
            ("s3", X19),
            ("s4", X20),
            ("s5", X21),
            ("s6", X22),
            ("s7", X23),
            ("s8", X24),
            ("s9", X25),
            ("s10", X26),
            ("s11", X27),
            ("t3", X28),
            ("t4", X29),
            ("t5", X30),
            ("t6", X31),

            ("f0" , F0),
            ("f1" , F1),
            ("f2" , F2),
            ("f3" , F3),
            ("f4" , F4),
            ("f5" , F5),
            ("f6" , F6),
            ("f7" , F7),
            ("f8" , F8),
            ("f9" , F9),
            ("f10", F10),
            ("f11", F11),
            ("f12", F12),
            ("f13", F13),
            ("f14", F14),
            ("f15", F15),
            ("f16", F16),
            ("f17", F17),
            ("f18", F18),
            ("f19", F19),
            ("f20", F20),
            ("f21", F21),
            ("f22", F22),
            ("f23", F23),
            ("f24", F24),
            ("f25", F25),
            ("f26", F26),
            ("f27", F27),
            ("f28", F28),
            ("f29", F29),
            ("f30", F30),
            ("f31", F31),

            ("ft0" , F0),
            ("ft1" , F1),
            ("ft2" , F2),
            ("ft3" , F3),
            ("ft4" , F4),
            ("ft5" , F5),
            ("ft6" , F6),
            ("ft7" , F7),
            ("fs8" , F8),
            ("fs9" , F9),
            ("fa0", F10),
            ("fa1", F11),
            ("fa2", F12),
            ("fa3", F13),
            ("fa4", F14),
            ("fa5", F15),
            ("fa6", F16),
            ("fa7", F17),
            ("fs2", F18),
            ("fs3", F19),
            ("fs4", F20),
            ("fs5", F21),
            ("fs6", F22),
            ("fs7", F23),
            ("fs8", F24),
            ("fs9", F25),
            ("fs10", F26),
            ("fs11", F27),
            ("ft8", F28),
            ("ft9", F29),
            ("ft10", F30),
            ("ft11", F31),

            ("v0" , V0),
            ("v1" , V1),
            ("v2" , V2),
            ("v3" , V3),
            ("v4" , V4),
            ("v5" , V5),
            ("v6" , V6),
            ("v7" , V7),
            ("v8" , V8),
            ("v9" , V9),
            ("v10", V10),
            ("v11", V11),
            ("v12", V12),
            ("v13", V13),
            ("v14", V14),
            ("v15", V15),
            ("v16", V16),
            ("v17", V17),
            ("v18", V18),
            ("v19", V19),
            ("v20", V20),
            ("v21", V21),
            ("v22", V22),
            ("v23", V23),
            ("v24", V24),
            ("v25", V25),
            ("v26", V26),
            ("v27", V27),
            ("v28", V28),
            ("v29", V29),
            ("v30", V30),
            ("v31", V31),
        ];
        MAP.iter().cloned().collect()
    };

    static ref RISCV_FAMILIES: HashMap<&'static str, ast::RegFamily> = {
        static MAP: &[(&str, ast::RegFamily)] = &[
            ("X", ast::RegFamily::INTEGER),
            ("F", ast::RegFamily::FP),
            ("V", ast::RegFamily::VECTOR)
        ];
        MAP.iter().cloned().collect()
    };
}
