use std::collections::hash_map::Entry;

use syntax::ext::base::ExtCtxt;
use syntax::parse::parser::Parser;
use syntax::parse::token;
use syntax::parse::PResult;

use super::DynasmData;
use serialize::{Stmt, Size, Ident};
use arch;

impl DynasmData {
    pub fn evaluate_directive<'b>(&mut self, stmts: &mut Vec<Stmt>, ecx: &mut ExtCtxt, parser: &mut Parser<'b>) -> PResult<'b, ()> {
        let start = parser.span;

        let directive = parser.parse_ident()?;

        match &*directive.name.as_str() {
            // TODO: oword, qword, float, double, long double

            "arch" => {
                // ; .arch ident
                let arch = parser.parse_ident()?;
                if let Some(a) = arch::from_str(&*arch.name.as_str()) {
                    self.current_arch = a;
                } else {
                    ecx.span_err(parser.prev_span, &format!("Unknown architecture '{}'", &*arch.name.as_str()));
                }
            },
            "feature" => {
                // ; .feature ident ("," ident) *
                let mut features = Vec::new();
                let ident = parser.parse_ident()?;
                features.push(Ident {span: parser.prev_span, node: ident});

                while parser.eat(&token::Token::Comma) {
                    let ident = parser.parse_ident()?;
                    features.push(Ident {span: parser.prev_span, node: ident});
                }
                self.current_arch.set_features(ecx, &features);
            },
            // ; .byte (expr ("," expr)*)?
            "byte"  => Self::directive_const(stmts, parser, Size::BYTE)?,
            "word"  => Self::directive_const(stmts, parser, Size::WORD)?,
            "dword" => Self::directive_const(stmts, parser, Size::DWORD)?,
            "qword" => Self::directive_const(stmts, parser, Size::QWORD)?,
            "bytes" => {
                // ; .bytes expr
                let iterator = parser.parse_expr()?;
                stmts.push(Stmt::ExprExtend(iterator));
            },
            "align" => {
                // ; .align expr
                // this might need to be architecture dependent
                let value = parser.parse_expr()?;
                stmts.push(Stmt::Align(value));
            },
            "alias" => {
                // ; .alias ident, ident
                // consider changing this to ; .alias ident = ident next breaking change
                let alias = parser.parse_ident()?.name;
                parser.expect(&token::Comma)?;
                let reg = parser.parse_ident()?.name;

                match self.aliases.entry(alias.as_str().to_string()) {
                    Entry::Occupied(_) => {
                        ecx.span_err(start.with_hi(parser.prev_span.hi()),
                                     &format!("Duplicate alias definition, alias '{}' was already defined", alias.as_str()));
                    },
                    Entry::Vacant(v) => {
                        v.insert(reg.as_str().to_string());
                    }
                }
            },
            d => {
                // unknown directive. skip ahead until we hit a ; so the parser can recover
                ecx.span_err(parser.span, &format!("unknown directive '{}'", d));
                while !(parser.check(&token::Semi) && parser.check(&token::Eof)) {
                    parser.bump();
                }
            }
        }

        Ok(())
    }

    fn directive_const<'b>(stmts: &mut Vec<Stmt>, parser: &mut Parser<'b>, size: Size) -> PResult<'b, ()> {
        if !parser.check(&token::Semi) && !parser.check(&token::Eof) {
            stmts.push(Stmt::ExprSigned(parser.parse_expr()?, size));

            while parser.eat(&token::Comma) {
                stmts.push(Stmt::ExprSigned(parser.parse_expr()?, size));
            }
        }

        Ok(())
    }
}
