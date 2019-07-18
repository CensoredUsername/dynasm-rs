//! This file contains parsing helpers used by multiple parsing backends
use syn::{parse, Token};

/**
 * Jump types
 */

pub trait ParseOpt: Sized {
    fn parse(input: parse::ParseStream) -> parse::Result<Option<Self>>;
}

pub trait ParseOptExt {
    /// Parses a syntax tree node of type `T`, advancing the position of our
    /// parse stream past it if it was found.
    fn parse_opt<T: ParseOpt>(&self) -> parse::Result<Option<T>>;
}

impl<'a> ParseOptExt for parse::ParseBuffer<'a> {
    fn parse_opt<T: ParseOpt>(&self) -> parse::Result<Option<T>> {
        T::parse(self)
    }
}

#[derive(Debug)]
pub enum JumpType {
    // note: these symbol choices try to avoid stuff that is a valid starting symbol for parse_expr
    // in order to allow the full range of expressions to be used. the only currently existing ambiguity is
    // with the symbol <, as this symbol is also the starting symbol for the universal calling syntax <Type as Trait>.method(args)
    Global(syn::Ident),   // -> label
    Backward(syn::Ident), //  > label
    Forward(syn::Ident),  //  < label
    Dynamic(syn::Expr),   // => expr
    Bare(syn::Expr)       // jump to this address
}

impl ParseOpt for JumpType {
    fn parse(input: parse::ParseStream) -> parse::Result<Option<JumpType>> {
        // -> global_label
        Ok(if input.peek(Token![->]) {
            let _: Token![->] = input.parse()?;
            let name: syn::Ident = input.parse()?;

            Some(JumpType::Global(name))

        // > forward_label
        } else if input.peek(Token![>]) {
            let _: Token![>] = input.parse()?;
            let name: syn::Ident = input.parse()?;

            Some(JumpType::Forward(name))

        // < backwards_label
        } else if input.peek(Token![<]) {
            let _: Token![<] = input.parse()?;
            let name: syn::Ident = input.parse()?;

            Some(JumpType::Backward(name))
            
        // => dynamic_label
        } else if input.peek(Token![=>]) {
            let _: Token![=>] = input.parse()?;
            let expr: syn::Expr = input.parse()?;

            Some(JumpType::Dynamic(expr))

        // extern label
        } else if eat_pseudo_keyword(input, "extern") {
            let expr: syn::Expr = input.parse()?;

            Some(JumpType::Bare(expr))

        } else {
            None
        })
    }
}

/// Tries to parse an ident that has a specific name as a keyword. Returns true if it worked.
pub fn eat_pseudo_keyword(input: parse::ParseStream, kw: &str) -> bool {
    input.step(|cursor| {
        if let Some((ident, rest)) = cursor.ident() {
            if ident == kw {
                return Ok(((), rest));
            }
        }
        Err(cursor.error("expected identifier"))
    }).is_ok()
}

/// parses an ident, but instead of syn's Parse impl it does also parse keywords as idents
pub fn parse_ident_or_rust_keyword(input: parse::ParseStream) -> parse::Result<syn::Ident> {
    input.step(|cursor| {
        if let Some((ident, rest)) = cursor.ident() {
            return Ok((ident, rest));
        }
        Err(cursor.error("expected identifier"))
    })
}

/// checks if an expression is simply an ident, and if so, returns a clone of it.
pub fn as_ident(expr: &syn::Expr) -> Option<&syn::Ident> {
    let path = match *expr {
        syn::Expr::Path(syn::ExprPath {ref path, qself: None, ..}) => path,
        _ => return None
    };

    if path.leading_colon.is_some() || path.segments.len() != 1 {
        return None;
    }

    let segment = &path.segments[0];
    if segment.arguments != syn::PathArguments::None {
        return None;
    }

    Some(&segment.ident)
}

/// checks if an expression is a simple literal, allowing us to perform compile-time analysis of an expression
pub fn as_lit(expr: &syn::Expr) -> Option<&syn::Lit> {
    // strip any wrapping Group nodes due to delimiting
    let mut inner = expr;
    while let syn::Expr::Group(syn::ExprGroup { expr, .. }) = inner {
        inner = expr;
    }

    match inner {
        syn::Expr::Lit(syn::ExprLit { ref lit, .. } ) => Some(lit),
        _ => None
    }
}

/// checks if an expression is a constant number literal
pub fn as_number(expr: &syn::Expr) -> Option<u64> {
    match as_lit(expr)?  {
        syn::Lit::Int(i) => Some(i.value()),
        _ => None
    }
}

/// checks if an expression is a constant float literal
pub fn as_float(expr: &syn::Expr) -> Option<f64> {
    match as_lit(expr)?  {
        syn::Lit::Float(i) => Some(i.value()),
        _ => None
    }
}
