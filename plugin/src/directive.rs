use std::collections::hash_map::Entry;

use syn::parse;
use syn::Token;

use super::DynasmData;
use serialize::{Stmt, Size, delimited};
use arch;
use ::emit_error_at;

impl DynasmData {
    pub fn evaluate_directive(&mut self, stmts: &mut Vec<Stmt>, input: parse::ParseStream) -> parse::Result<()> {
        let directive: syn::Ident = input.parse()?;

        match directive.to_string().as_str() {
            // TODO: oword, qword, float, double, long double

            "arch" => {
                // ; .arch ident
                let arch: syn::Ident = input.parse()?;
                if let Some(a) = arch::from_str(arch.to_string().as_str()) {
                    self.current_arch = a;
                } else {
                    emit_error_at(arch.span(), format!("Unknown architecture '{}'", arch.to_string()));
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
                self.current_arch.set_features(&features);
            },
            // ; .byte (expr ("," expr)*)?
            "byte"  => Self::directive_const(stmts, input, Size::BYTE)?,
            "word"  => Self::directive_const(stmts, input, Size::WORD)?,
            "dword" => Self::directive_const(stmts, input, Size::DWORD)?,
            "qword" => Self::directive_const(stmts, input, Size::QWORD)?,
            "bytes" => {
                // ; .bytes expr
                let iterator: syn::Expr = input.parse()?;
                stmts.push(Stmt::ExprExtend(delimited(iterator)));
            },
            "align" => {
                // ; .align expr
                // this might need to be architecture dependent
                let value: syn::Expr = input.parse()?;
                stmts.push(Stmt::Align(delimited(value)));
            },
            "alias" => {
                // ; .alias ident, ident
                // consider changing this to ; .alias ident = ident next breaking change
                let alias = input.parse::<syn::Ident>()?;
                let _: Token![,] = input.parse()?;
                let reg = input.parse::<syn::Ident>()?;

                let alias_name = alias.to_string();

                match self.aliases.entry(alias_name) {
                    Entry::Occupied(_) => {
                        emit_error_at(alias.span(), format!("Duplicate alias definition, alias '{}' was already defined", alias.to_string()));
                    },
                    Entry::Vacant(v) => {
                        v.insert(reg.to_string());
                    }
                }
            },
            d => {
                // unknown directive. skip ahead until we hit a ; so the parser can recover
                emit_error_at(directive.span(), format!("unknown directive '{}'", d));
                skip_until_semicolon(input);
            }
        }

        Ok(())
    }

    fn directive_const(stmts: &mut Vec<Stmt>, input: parse::ParseStream, size: Size) -> parse::Result<()> {
        // FIXME: this could be replaced by a Punctuated parser?
        // parse (expr (, expr)*)?
        if !(input.is_empty() || input.peek(Token![;])) {
            let expr: syn::Expr = input.parse()?;

            stmts.push(Stmt::ExprSigned(delimited(expr), size));

            while input.peek(Token![,]) {
                let _: Token![,] = input.parse()?;
                let expr: syn::Expr = input.parse()?;

                stmts.push(Stmt::ExprSigned(delimited(expr), size));
            }
        }

        Ok(())
    }
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
