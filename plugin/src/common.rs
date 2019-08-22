//! This module contains various infrastructure that is common across all assembler backends
use proc_macro2::{Span, TokenTree};
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::parse;
use syn::Token;

use crate::parse_helpers::{ParseOpt, eat_pseudo_keyword};
use crate::serialize;

/// Enum representing the result size of a value/expression/register/etc in bytes.
/// Uses the NASM syntax for sizes (a word is 16 bits)
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone, Copy)]
pub enum Size {
    BYTE  = 1,
    WORD  = 2,
    DWORD = 4,
    FWORD = 6,
    QWORD = 8,
    PWORD = 10,
    OWORD = 16,
    HWORD = 32,
}

impl Size {
    pub fn in_bytes(self) -> u8 {
        self as u8
    }

    pub fn as_literal(self) -> syn::Ident {
        syn::Ident::new(match self {
            Size::BYTE  => "i8",
            Size::WORD  => "i16",
            Size::DWORD => "i32",
            Size::FWORD => "i48",
            Size::QWORD => "i64",
            Size::PWORD => "i80",
            Size::OWORD => "i128",
            Size::HWORD => "i256"
        }, Span::call_site())
    }
}


/**
 * Jump types
 */

#[derive(Debug, Clone)]
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

impl JumpType {
    pub fn encode(self, data: &[u8]) -> Stmt {
        let span = self.span();

        let data = serialize::expr_tuple_of_u8s(span, data);
        match self {
            JumpType::Global(ident) => Stmt::GlobalJumpTarget(ident, data),
            JumpType::Backward(ident) => Stmt::BackwardJumpTarget(ident, data),
            JumpType::Forward(ident) => Stmt::ForwardJumpTarget(ident, data),
            JumpType::Dynamic(expr) => Stmt::DynamicJumpTarget(delimited(expr), data),
            JumpType::Bare(expr) => Stmt::BareJumpTarget(delimited(expr), data),
        }
    }

    pub fn span(&self) -> Span {
        match self {
            JumpType::Global(ident) => ident.span(),
            JumpType::Backward(ident) => ident.span(),
            JumpType::Forward(ident) => ident.span(),
            JumpType::Dynamic(expr) => expr.span(),
            JumpType::Bare(expr) => expr.span(),
        }
    }
}


/// An abstract representation of a dynasm runtime statement to be emitted
#[derive(Debug, Clone)]
pub enum Stmt {
    // simply push data into the instruction stream. unsigned
    Const(u64, Size),
    // push data that is stored inside of an expression. unsigned
    ExprUnsigned(TokenTree, Size),
    // push signed data into the instruction stream. signed
    ExprSigned(TokenTree, Size),

    // extend the instruction stream with unsigned bytes
    Extend(Vec<u8>),
    // extend the instruction stream with unsigned bytes
    ExprExtend(TokenTree),
    // align the instruction stream to some alignment
    Align(TokenTree),

    // label declarations
    GlobalLabel(syn::Ident),
    LocalLabel(syn::Ident),
    DynamicLabel(TokenTree),

    // and their respective relocations (as expressions as they differ per assembler)
    GlobalJumpTarget(  syn::Ident,    TokenTree),
    ForwardJumpTarget( syn::Ident,    TokenTree),
    BackwardJumpTarget(syn::Ident,    TokenTree),
    DynamicJumpTarget(TokenTree, TokenTree),
    BareJumpTarget(   TokenTree, TokenTree),

    // a random statement that has to be inserted between assembly hunks
    Stmt(TokenTree)
}

// convenience methods
impl Stmt {
    #![allow(dead_code)]

    pub fn u8(value: u8) -> Stmt {
        Stmt::Const(u64::from(value), Size::BYTE)
    }

    pub fn u16(value: u16) -> Stmt {
        Stmt::Const(u64::from(value), Size::WORD)
    }

    pub fn u32(value: u32) -> Stmt {
        Stmt::Const(u64::from(value), Size::DWORD)
    }

    pub fn u64(value: u64) -> Stmt {
        Stmt::Const(value, Size::QWORD)
    }
}


// Makes a None-delimited TokenTree item out of anything that can be converted to tokens.
// This is a useful shortcut to escape issues around not-properly delimited tokenstreams
// because it is guaranteed to be parsed back properly to its source ast at type-level.
pub fn delimited<T: ToTokens>(expr: T) -> TokenTree {
    let span = expr.span();
    let mut group = proc_macro2::Group::new(
        proc_macro2::Delimiter::None, expr.into_token_stream()
    );
    group.set_span(span);
    proc_macro2::TokenTree::Group(group)
}



// FIXME: temporary till Diagnostic gets stabilized
/// Emit a diagnostic at a certain span.
pub fn emit_error_at(span: Span, msg: String) {
    let span: proc_macro::Span = span.unstable();
    span.error(msg).emit();
}


/// Create a bitmask with `scale` bits set
pub fn bitmask(scale: u8) -> u32 {
    1u32.checked_shl(scale as u32).unwrap_or(0).wrapping_sub(1)
}


/// Create a bitmask with `scale` bits set
pub fn bitmask64(scale: u8) -> u64 {
    1u64.checked_shl(scale as u32).unwrap_or(0).wrapping_sub(1)
}

