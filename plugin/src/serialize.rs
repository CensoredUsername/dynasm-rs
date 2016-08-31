use std::rc::Rc;

use compiler;
use parser::{Ident, Size};

use syntax::ext::build::AstBuilder;
use syntax::ext::base::ExtCtxt;
use syntax::ast;
use syntax::ptr::P;
use syntax::parse::token::intern;
use syntax::codemap::{Span, Spanned};


pub fn serialize(ecx: &mut ExtCtxt, name: Ident, stmts: compiler::StmtBuffer) -> Vec<ast::Stmt> {
    let mut buffer = Vec::new();

    // construction for `op.push(expr)` is as follows
    // op = Path::from_ident(name)
    // push = Ident::with_empty_ctxt(intern("push"))
    // expr = expr_lit(span, LitKind::Byte)
    // expr_method_call(span, op, Vec![expr])

    let mut stmts = stmts.into_iter().peekable();

    while let Some(stmt) = stmts.next() {
        use compiler::Stmt::*;

        let (method, args) = match stmt {
            Const(byte)            => {
                let mut bytes = vec![byte];
                while let Some(&Const(byte)) = stmts.peek() {
                    bytes.push(byte);
                    stmts.next();
                    if stmts.len() == 32 {
                        break;
                    }
                }

                if bytes.len() == 1 {
                    ("push",   vec![ecx.expr_lit(ecx.call_site(), ast::LitKind::Byte(bytes[0]))])
                } else {
                    ("extend", vec![ecx.expr_lit(ecx.call_site(), ast::LitKind::ByteStr(Rc::new(bytes)))])
                }
            },
            ExprConst(expr)        => ("push",    vec![expr]),

            Var(expr, Size::BYTE)  => ("push_i8",  vec![expr]),
            Var(expr, Size::WORD)  => ("push_i16", vec![expr]),
            Var(expr, Size::DWORD) => ("push_i32", vec![expr]),
            Var(expr, Size::QWORD) => ("push_i64", vec![expr]),
            Var(_, _)           => panic!("immediate serializaiton of this size is not supported yet"),

            Extend(expr)           => ("extend", vec![expr]),

            DynScale(scale, rest)  => {
                let temp = ast::Ident::with_empty_ctxt(intern("temp"));
                buffer.push(ecx.stmt_let(ecx.call_site(), false, temp, encoded_size(ecx, name, scale)));
                ("push", vec![or_mask_shift_expr(ecx, rest, ecx.expr_ident(ecx.call_site(), temp), 3, 6)])
            },

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

pub fn add_exprs<T: Iterator<Item=P<ast::Expr>>>(ecx: &ExtCtxt, span: Span, mut exprs: T) -> Option<P<ast::Expr>> {
    exprs.next().map(|mut accum| {
        for next in exprs {
            accum = ecx.expr_binary(span, ast::BinOpKind::Add, accum, next);
        }
        accum
    })
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

pub fn offset_of(ecx: &ExtCtxt, path: ast::Path, attr: ast::Ident) -> P<ast::Expr> {
    // generate a P<Expr> that resolves into the offset of an attribute to a type.
    // this is somewhat ridiculously complex because we can't expand macros here

    let span = path.span;

    let structpat = ecx.pat_struct(span, path.clone(), vec![
        Spanned {span: span, node: ast::FieldPat {
            ident: attr,
            pat: ecx.pat_wild(span),
            is_shorthand: false
        }},
    ]).map(|mut pat| {
        if let ast::PatKind::Struct(_, _, ref mut dotdot) = pat.node {
            *dotdot = true;
        }
        pat
    });

    // there's no default constructor function for let pattern;
    let validation_stmt = ast::Stmt {
        id: ast::DUMMY_NODE_ID,
        span: span,
        node: ast::StmtKind::Local(P(ast::Local {
            pat: structpat,
            ty: None,
            init: None,
            id: ast::DUMMY_NODE_ID,
            span: span,
            attrs: ast::ThinVec::new()
        }))
    };

    let temp     = ast::Ident::with_empty_ctxt(intern("temp"));
    let rv       = ast::Ident::with_empty_ctxt(intern("rv"));
    let usize_id = ast::Ident::with_empty_ctxt(intern("usize"));
    let i32_id   = ast::Ident::with_empty_ctxt(intern("i32"));
    let uninitialized = ["std", "mem", "uninitialized"].iter().cloned().map(intern).map(ast::Ident::with_empty_ctxt).collect();
    let forget        = ["std", "mem", "forget"       ].iter().cloned().map(intern).map(ast::Ident::with_empty_ctxt).collect();

    // unsafe {
    let block = ecx.block(span, vec![
        // let path { attr: _, ..};
        validation_stmt,
        // let temp: path = ::std::mem::uninitialized();
        ecx.stmt_let_typed(span, false, temp, ecx.ty_path(path),
            ecx.expr_call_global(span, uninitialized, Vec::new())
        ).unwrap(),
        // let rv = &temp.attr as *const _ as usize - &temp as *const _ as usize;
        ecx.stmt_let(span,
            false,
            rv,
            ecx.expr_binary(span, ast::BinOpKind::Sub,
                ecx.expr_cast(span,
                    ecx.expr_cast(span,
                        ecx.expr_addr_of(span,
                            ecx.expr_field_access(span,
                                ecx.expr_ident(span, temp),
                                attr
                            )
                        ), ecx.ty_ptr(span, ecx.ty_infer(span), ast::Mutability::Immutable)
                    ), ecx.ty_ident(span, usize_id)
                ),
                ecx.expr_cast(span,
                    ecx.expr_cast(span,
                        ecx.expr_addr_of(span, ecx.expr_ident(span, temp)),
                        ecx.ty_ptr(span, ecx.ty_infer(span), ast::Mutability::Immutable)
                    ), ecx.ty_ident(span, usize_id)
                )
            )
        ),
        // ::std::mem::forget(temp);
        ecx.stmt_semi(ecx.expr_call_global(span, forget, vec![ecx.expr_ident(span, temp)])),
        // rv as u32
        ecx.stmt_expr(ecx.expr_cast(span, ecx.expr_ident(span, rv), ecx.ty_ident(span, i32_id)))
    ]).map(|mut b| {
        b.rules = ast::BlockCheckMode::Unsafe(ast::UnsafeSource::CompilerGenerated);
        b
    });

    ecx.expr_block(block)
}

pub fn size_of(ecx: &ExtCtxt, path: ast::Path) -> P<ast::Expr> {
    // generate a P<Expr> that returns the size of type at path
    let span = path.span;

    let ty = ecx.ty_path(path);
    let idents = ["std", "mem", "size_of"].iter().cloned().map(intern).map(ast::Ident::with_empty_ctxt).collect();
    let size_of = ecx.path_all(span, true, idents, Vec::new(), vec![ty], Vec::new());
    ecx.expr_call(span, ecx.expr_path(size_of), Vec::new())
}

pub fn encoded_size(ecx: &ExtCtxt, name: Ident, size: P<ast::Expr>) -> P<ast::Expr> {
    let span = size.span;

    ecx.expr_match(span, size, vec![
        ecx.arm(span, vec![ecx.pat_lit(span, ecx.expr_usize(span, 8))], ecx.expr_u8(span, 3)),
        ecx.arm(span, vec![ecx.pat_lit(span, ecx.expr_usize(span, 4))], ecx.expr_u8(span, 2)),
        ecx.arm(span, vec![ecx.pat_lit(span, ecx.expr_usize(span, 2))], ecx.expr_u8(span, 1)),
        ecx.arm(span, vec![ecx.pat_lit(span, ecx.expr_usize(span, 1))], ecx.expr_u8(span, 0)),
        ecx.arm(span, vec![ecx.pat_wild(span)], ecx.expr_method_call(span,
            ecx.expr_ident(name.span, name.node),
            ast::Ident::with_empty_ctxt(intern("runtime_error")),
            vec![ecx.expr_str(span,
                intern("Type size not representable as scale").as_str()
            )]
        ))
    ])
}
