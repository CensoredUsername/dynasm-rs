use compiler;
use parser::{Ident, Size};

use syntax::ext::build::AstBuilder;
use syntax::ext::base::ExtCtxt;
use syntax::ast;
use syntax::ptr::P;
use syntax::parse::token::intern;


pub fn serialize(ecx: &mut ExtCtxt, name: Ident, stmts: compiler::StmtBuffer) -> Vec<ast::Stmt> {
    let mut buffer = Vec::new();

    // construction for `op.push(expr)` is as follows
    // op = Path::from_ident(name)
    // push = Ident::with_empty_ctxt(intern("push"))
    // expr = expr_lit(span, LitKind::Byte)
    // expr_method_call(span, op, Vec![expr])

    for stmt in stmts {
        use compiler::Stmt::*;
        let (method, args) = match stmt {
            Const(byte)            => ("push",    vec![ecx.expr_lit(ecx.call_site(), ast::LitKind::Byte(byte))]), // this span should never appear in an error message
            ExprConst(expr)        => ("push",    vec![expr]),

            Var(expr, Size::BYTE)  => ("push_8",  vec![expr]),
            Var(expr, Size::WORD)  => ("push_16", vec![expr]),
            Var(expr, Size::DWORD) => ("push_32", vec![expr]),
            Var(expr, Size::QWORD) => ("push_64", vec![expr]),
            Var(_, _)           => panic!("immediate serializaiton of this size is not supported yet"),

            Extend(expr)           => ("extend", vec![expr]),

            Align(expr)            => ("align",   vec![expr]),

            GlobalLabel(ident)     => ("global_label", vec![ecx.expr_lit(
                ident.span,
                ast::LitKind::Str(ident.node.name.as_str(), ast::StrStyle::Cooked)
            )]),
            LocalLabel(ident)      => ("local_label", vec![ecx.expr_lit(
                ident.span,
                ast::LitKind::Str(ident.node.name.as_str(), ast::StrStyle::Cooked)
            )]),
            DynamicLabel(expr)     => ("dynamic_label", vec![expr]),

            GlobalJumpTarget(ident, size) => ("global_reloc", vec![ecx.expr_lit(
                ident.span,
                ast::LitKind::Str(ident.node.name.as_str(), ast::StrStyle::Cooked)
            ), ecx.expr_u8(ident.span, size.in_bytes())]),
            ForwardJumpTarget(ident, size) => ("forward_reloc", vec![ecx.expr_lit(
                ident.span,
                ast::LitKind::Str(ident.node.name.as_str(), ast::StrStyle::Cooked)
            ), ecx.expr_u8(ident.span, size.in_bytes())]),
            BackwardJumpTarget(ident, size) => ("backward_reloc", vec![ecx.expr_lit(
                ident.span,
                ast::LitKind::Str(ident.node.name.as_str(), ast::StrStyle::Cooked)
            ), ecx.expr_u8(ident.span, size.in_bytes())]),
            DynamicJumpTarget(expr, size) => {
                let span = expr.span;
                ("dynamic_reloc", vec![expr, ecx.expr_u8(span, size.in_bytes())])
            }
        };

        let op = ecx.expr_path(ast::Path::from_ident(name.span, name.node));
        let method = ast::Ident::with_empty_ctxt(intern(method));
        let expr = ecx.expr_method_call(ecx.call_site(), op, method, args);

        buffer.push(ecx.stmt_semi(expr));
    }

    buffer
}

pub fn or_mask_shift_expr(ecx: &ExtCtxt, orig: P<ast::Expr>, mut expr: P<ast::Expr>, mask: u64, shift: i8) -> P<ast::Expr> {
    let span = expr.span;
    // take expr and return !((expr & mask) << shift)

    expr = ecx.expr_binary(span, ast::BinOpKind::BitAnd, expr, ecx.expr_lit(
        span, ast::LitKind::Int(mask as u64, ast::LitIntType::Unsuffixed)
    ));

    if shift < 0 {
        expr = ecx.expr_binary(span, ast::BinOpKind::Shr, expr, ecx.expr_lit(
            span, ast::LitKind::Int((-shift) as u64, ast::LitIntType::Unsuffixed)
        ));
    } else if shift > 0 {
        expr = ecx.expr_binary(span, ast::BinOpKind::Shl, expr, ecx.expr_lit(
            span, ast::LitKind::Int(shift as u64, ast::LitIntType::Unsuffixed)
        ));
    }

    ecx.expr_binary(span, ast::BinOpKind::BitOr, orig, expr)
}