use std::collections::hash_map::Entry;

use syn::parse;
use syn::Token;
use syn::spanned::Spanned;
use quote::quote_spanned;
use proc_macro2::{TokenTree, Literal, Span};
use proc_macro_error3::emit_error;

use crate::common::{Stmt, Size, delimited, RelocationEncoding, JumpTarget};
use crate::arch;
use crate::DynasmContext;
use crate::parse_helpers::ParseOptExt;

pub(crate) fn evaluate_directive(invocation_context: &mut DynasmContext, stmts: &mut Vec<Stmt>, input: parse::ParseStream) -> parse::Result<()> {
    let directive: syn::Ident = input.parse()?;
    let span = directive.span();

    match directive.to_string().as_str() {
        // TODO: oword, qword, float, double, long double

        "arch" => {
            // ; .arch ident
            let arch: syn::Ident = input.parse()?;
            if let Some(a) = arch::from_str(arch.to_string().as_str()) {
                invocation_context.current_arch = a;
            } else {
                emit_error!(arch, "Unknown architecture '{}'", arch);
            }
        },
        "feature" => {
            // ; .feature ident ("," ident) *
            let mut features = Vec::new();
            let ident: syn::Ident = input.parse()?;
            features.push(ident);

            while input.peek(Token![,]) {
                let _: Token![,] = input.parse()?;
                let ident: syn::Ident = input.parse()?;
                features.push(ident);
            }

            // ;.feature none  cancels all features
            if features.len() == 1 && features[0] == "none" {
                features.pop();
            }
            invocation_context.current_arch.set_features(&features);
        },
        // ; .u8 (expr ("," expr)*)?
        "u8"  => directive_unsigned(stmts, input, Size::BYTE)?,
        "u16" => directive_unsigned(stmts, input, Size::B_2)?,
        "u32" => directive_unsigned(stmts, input, Size::B_4)?,
        "u64" => directive_unsigned(stmts, input, Size::B_8)?,
        "i8"  => directive_signed(stmts, input, Size::BYTE)?,
        "i16" => directive_signed(stmts, input, Size::B_2)?,
        "i32" => directive_signed(stmts, input, Size::B_4)?,
        "i64" => directive_signed(stmts, input, Size::B_8)?,
        "f32" => directive_float(stmts, input, Size::B_4)?,
        "f64" => directive_float(stmts, input, Size::B_8)?,
        // ; .rel8 label
        "rel8"  => directive_reference(span, stmts, input, Size::BYTE, true)?,
        "rel16" => directive_reference(span, stmts, input, Size::B_2, true)?,
        "rel32" => directive_reference(span, stmts, input, Size::B_4, true)?,
        "rel64" => directive_reference(span, stmts, input, Size::B_8, true)?,
        // ; .abs32 label
        "abs32" => directive_reference(span, stmts, input, Size::B_4, false)?,
        "abs64" => directive_reference(span, stmts, input, Size::B_8, false)?,
        "bytes" => {
            // ; .bytes expr
            let iterator: syn::Expr = input.parse()?;
            stmts.push(Stmt::ExprExtend(delimited(iterator)));
        },
        "align" => {
            // ; .align expr ("," expr)
            // this might need to be architecture dependent
            let value: syn::Expr = input.parse()?;

            let with = if input.peek(Token![,]) {
                let _: Token![,] = input.parse()?;
                let with: syn::Expr = input.parse()?;
                delimited(with)
            } else {
                let with = invocation_context.current_arch.default_align();
                TokenTree::Literal(Literal::u8_unsuffixed(with))
            };

            stmts.push(Stmt::Align(delimited(value), with));
        },
        "alias" => {
            // ; .alias ident, ident
            // consider changing this to ; .alias ident = ident next breaking change
            let alias = input.parse::<syn::Ident>()?;
            let _: Token![,] = input.parse()?;
            let reg = input.parse::<syn::Ident>()?;

            let alias_name = alias.to_string();

            match invocation_context.aliases.entry(alias_name) {
                Entry::Occupied(_) => {
                    emit_error!(alias, "Duplicate alias definition, alias '{}' was already defined", alias);
                },
                Entry::Vacant(v) => {
                    v.insert(reg.to_string());
                }
            }
        },
        // these are deprecated, but to prevent bad error messages handle them explicitly
        // I'd like to provide a warning instead, but proc-macro-error2 emit_warning! seems to not work.
        "byte"  => emit_error!(span, "Directive .byte is deprecated, please use .i8 or .u8 instead."),
        "word"  => emit_error!(span, "Directive .word is deprecated, please use .i16 or .u16 instead."),
        "dword" => emit_error!(span, "Directive .dword is deprecated, please use .i32 or .u32 instead."),
        "qword" => emit_error!(span, "Directive .qword is deprecated, please use .i64 or .u64 instead."),
        d => {
            // unknown directive. skip ahead until we hit a ; so the parser can recover
            emit_error!(directive, "unknown directive '{}'", d);
            skip_until_semicolon(input);
        }
    }

    Ok(())
}

fn directive_signed(stmts: &mut Vec<Stmt>, input: parse::ParseStream, size: Size) -> parse::Result<()> {
    // FIXME: this could be replaced by a Punctuated parser?
    // parse (expr (, expr)*)?

    if input.is_empty() || input.peek(Token![;]) {
        return Ok(())
    }

    if let Some(jump) = input.parse_opt::<JumpTarget>()? {
        emit_error!(jump.span(), "Labels in data directives are deprecated, please use the .absX or .relX directives instead.");
    } else {
        let expr: syn::Expr = input.parse()?;
        stmts.push(Stmt::ExprSigned(delimited(expr), size));
    }


    while input.peek(Token![,]) {
        let _: Token![,] = input.parse()?;

        if let Some(jump) = input.parse_opt::<JumpTarget>()? {
            emit_error!(jump.span(), "Labels in data directives are deprecated, please use the .absX or .relX directives instead.");
        } else {
            let expr: syn::Expr = input.parse()?;
            stmts.push(Stmt::ExprSigned(delimited(expr), size));
        }
    }

    Ok(())
}

fn directive_unsigned(stmts: &mut Vec<Stmt>, input: parse::ParseStream, size: Size) -> parse::Result<()> {
    // FIXME: this could be replaced by a Punctuated parser?
    // parse (expr (, expr)*)?

    if input.is_empty() || input.peek(Token![;]) {
        return Ok(())
    }

    if let Some(jump) = input.parse_opt::<JumpTarget>()? {
        emit_error!(jump.span(), "Labels in data directives are deprecated, please use the .absX or .relX directives instead.");
    } else {
        let expr: syn::Expr = input.parse()?;
        stmts.push(Stmt::ExprUnsigned(delimited(expr), size));
    }


    while input.peek(Token![,]) {
        let _: Token![,] = input.parse()?;

        if let Some(jump) = input.parse_opt::<JumpTarget>()? {
            emit_error!(jump.span(), "Labels in data directives are deprecated, please use the .absX or .relX directives instead.");
        } else {
            let expr: syn::Expr = input.parse()?;
            stmts.push(Stmt::ExprUnsigned(delimited(expr), size));
        }
    }

    Ok(())
}

fn directive_float(stmts: &mut Vec<Stmt>, input: parse::ParseStream, size: Size) -> parse::Result<()> {
    // FIXME: this could be replaced by a Punctuated parser?
    // parse (expr (, expr)*)?

    if input.is_empty() || input.peek(Token![;]) {
        return Ok(())
    }

    let expr: syn::Expr = input.parse()?;
    let expr = match size {
        Size::B_4 => quote_spanned! {expr.span() => f32::to_bits( #expr ) },
        Size::B_8 => quote_spanned! {expr.span() => f64::to_bits( #expr ) },
        _ => unreachable!()
    };
    stmts.push(Stmt::ExprUnsigned(delimited(expr), size));

    while input.peek(Token![,]) {
        let _: Token![,] = input.parse()?;

        let expr: syn::Expr = input.parse()?;
        let expr = match size {
            Size::B_4 => quote_spanned! {expr.span() => f32::to_bits( #expr ) },
            Size::B_8 => quote_spanned! {expr.span() => f64::to_bits( #expr ) },
            _ => unreachable!()
        };
        stmts.push(Stmt::ExprUnsigned(delimited(expr), size));
    }

    Ok(())
}

fn directive_reference(span: Span, stmts: &mut Vec<Stmt>, input: parse::ParseStream, size: Size, relative_encoding: bool) -> parse::Result<()> {
    // parses a single label and encodes a reference to it.

    if let Some(jump) = input.parse_opt::<JumpTarget>()? {
        stmts.push(Stmt::Const(0, size));
        stmts.push(jump.encode(size.in_bytes(), size.in_bytes(), relative_encoding, RelocationEncoding::Simple(size)));
    } else {
        emit_error!(span, "Expected a label.");
    }

    Ok(())
}

/// In case a directive is unknown, try to skip up to the next ; and resume parsing.
fn skip_until_semicolon(input: parse::ParseStream) {
    let _ = input.step(|cursor| {
        let mut rest = *cursor;
        while let Some((tt, next)) = rest.token_tree() {
            match tt {
                ::proc_macro2::TokenTree::Punct(ref punct) if punct.as_char() == ';' => {
                    return Ok(((), rest));
                }
                _ => rest = next,
            }
        }
        Ok(((), rest))
    });
}
