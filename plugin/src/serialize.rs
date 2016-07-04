use compiler;
use parser;

use syntax::ext::build::AstBuilder;
use syntax::ext::base::ExtCtxt;
use syntax::ast;
use syntax::parse::token::intern;


pub fn serialize(ecx: &mut ExtCtxt, name: parser::Ident, stmts: Vec<compiler::Stmt>) -> Vec<ast::Stmt> {
    let mut buffer = Vec::new();

    // construction for `op.push(expr)` is as follows
    // op = Path::from_ident(name)
    // push = Ident::with_empty_ctxt(intern("push"))
    // expr = expr_lit(span, LitKind::Byte)
    // expr_method_call(span, op, Vec![expr])

    for stmt in stmts {
        use compiler::Stmt::*;
        let (method, expr) = match stmt {
            Const(byte) => ("push",    ecx.expr_lit(ecx.call_site(), ast::LitKind::Byte(byte))),
            Byte(expr)  => ("push_8",  expr),
            Word(expr)  => ("push_16", expr),
            DWord(expr) => ("push_32", expr),
            QWord(expr) => ("push_64", expr)
        };

        let op = ecx.expr_path(ast::Path::from_ident(name.span, name.node));
        let method = ast::Ident::with_empty_ctxt(intern(method));
        let expr = ecx.expr_method_call(ecx.call_site(), op, method, vec![expr]);

        buffer.push(ecx.stmt_semi(expr));
    }

    buffer
}
