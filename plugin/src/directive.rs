use std::collections::hash_map::Entry;

use syntax::ext::base::ExtCtxt;
use syntax::parse::parser::Parser;
use syntax::parse::token;
use syntax::parse::PResult;
use syntax::codemap::Span;

use super::State;
use serialize::{Stmt, Size};
use arch::Arch;

impl<'a> State<'a> {
    pub fn evaluate_directive<'b>(&mut self, ecx: &mut ExtCtxt, parser: &mut Parser<'b>) -> PResult<'b, ()> {
        let start = parser.span;

        let directive = parser.parse_ident()?;

        match &*directive.name.as_str() {
            // TODO: oword, qword, float, double, long double

            ".arch" => {
                // ; .arch ident
                let arch = parser.parse_ident()?;
                if let Some(a) = Arch::from_str(&*arch.name.as_str()) {
                    self.crate_data.current_arch = a;
                } else {
                    ecx.span_err(parser.prev_span, &format!("Unknown architecture '{}'", &*arch.name.as_str()));
                }
            }
            // ; .byte (expr ("," expr)*)?
            "byte"  => self.directive_const(parser, Size::BYTE)?,
            "word"  => self.directive_const(parser, Size::WORD)?,
            "dword" => self.directive_const(parser, Size::DWORD)?,
            "qword" => self.directive_const(parser, Size::QWORD)?,
            "bytes" => {
                // ; .bytes expr
                let iterator = parser.parse_expr()?;
                self.stmts.push(Stmt::Extend(iterator));
            },
            "align" => {
                // ; .align expr
                // this might need to be architecture dependent
                let value = parser.parse_expr()?;
                self.stmts.push(Stmt::Align(value));
            },
            "alias" => {
                // ; .alias ident, ident
                // consider changing this to ; .alias ident = ident next breaking change
                let alias = parser.parse_ident()?.name;
                parser.expect(&token::Comma)?;
                let reg = parser.parse_ident()?.name;

                match self.crate_data.aliases.entry(alias) {
                    Entry::Occupied(_) => {
                        ecx.span_err(Span {hi: parser.prev_span.hi, ..start},
                                     &format!("Duplicate alias definition, alias '{}' was already defined", alias.as_str()));
                    },
                    Entry::Vacant(v) => {
                        v.insert(reg);
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

    fn directive_const<'b>(&mut self, parser: &mut Parser<'b>, size: Size) -> PResult<'b, ()> {
        if !parser.check(&token::Semi) && !parser.check(&token::Eof) {
            self.stmts.push(Stmt::Var(parser.parse_expr()?, size));

            while parser.eat(&token::Comma) {
                self.stmts.push(Stmt::Var(parser.parse_expr()?, size));
            }
        }

        Ok(())
    }
}
