use compiler;
use parser::{Ident, Size};

use syntax::ext::build::AstBuilder;
use syntax::ext::base::ExtCtxt;
use syntax::ast;
use syntax::parse::token::intern;


pub fn serialize(ecx: &mut ExtCtxt, name: Ident, stmts: compiler::StmtBuffer) -> Vec<ast::Stmt> {
    let mut buffer = Vec::new();
    let (stmts, labels) = stmts.into_vec();

    // construction for `let label = value usize is as follows`
    // expr = expr_lit(span, LitKind::Int(val, LitIntType::Signed(IntTy::Is)))
    // stmt_let(span, false, ident, expr);

    for (label, value) in labels {
        let expr = ecx.expr_lit(label.span, ast::LitKind::Int(value as u64, ast::LitIntType::Unsuffixed));
        buffer.push(ecx.stmt_let(label.span, false, label.node, expr));
    }

    // construction for `op.push(expr)` is as follows
    // op = Path::from_ident(name)
    // push = Ident::with_empty_ctxt(intern("push"))
    // expr = expr_lit(span, LitKind::Byte)
    // expr_method_call(span, op, Vec![expr])

    for stmt in stmts {
        use compiler::Stmt::*;
        let (method, expr) = match stmt {
            Const(byte)            => ("push",    ecx.expr_lit(ecx.call_site(), ast::LitKind::Byte(byte))),
            Var(expr, Size::BYTE)  => ("push_8",  expr),
            Var(expr, Size::WORD)  => ("push_16", expr),
            Var(expr, Size::DWORD) => ("push_32", expr),
            Var(expr, Size::QWORD) => ("push_64", expr)
        };

        let op = ecx.expr_path(ast::Path::from_ident(name.span, name.node));
        let method = ast::Ident::with_empty_ctxt(intern(method));
        let expr = ecx.expr_method_call(ecx.call_site(), op, method, vec![expr]);

        buffer.push(ecx.stmt_semi(expr));
    }

    buffer
}
