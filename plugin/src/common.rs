//! This module contains various infrastructure that is common across all assembler backends
use proc_macro2::{Span, TokenTree, TokenStream, Literal, Group, Delimiter};
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::parse;
use syn::Token;

use crate::parse_helpers::{ParseOpt, eat_pseudo_keyword};
use crate::serialize;

/// Enum representing the result size of a value/expression/register/etc in bytes.
/// just friendly names really
#[allow(non_camel_case_types)]
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone, Copy)]
pub enum Size {
    BYTE = 1,
    B_2 = 2,
    B_4 = 4,
    B_6 = 6,
    B_8 = 8,
    B_10 = 10,
    B_16 = 16,
    B_32 = 32,
    B_64 = 64,
}

impl Size {
    pub fn in_bytes(self) -> u8 {
        self as u8
    }

    pub fn as_literal(self) -> syn::Ident {
        syn::Ident::new(match self {
            Size::BYTE  => "i8",
            Size::B_2  => "i16",
            Size::B_4 => "i32",
            Size::B_6 => "i48",
            Size::B_8 => "i64",
            Size::B_10 => "i80",
            Size::B_16 => "i128",
            Size::B_32 => "i256",
            Size::B_64 => "i512",
        }, Span::mixed_site())
    }
}


/**
 * Jump types
 */
#[derive(Debug, Clone)]
pub struct Jump {
    pub kind: JumpKind,
    pub offset: Option<syn::Expr>
}

#[derive(Debug, Clone)]
pub enum JumpKind {
    // note: these symbol choices try to avoid stuff that is a valid starting symbol for parse_expr
    // in order to allow the full range of expressions to be used. the only currently existing ambiguity is
    // with the symbol <, as this symbol is also the starting symbol for the universal calling syntax <Type as Trait>.method(args)
    Global(syn::Ident),   // -> label (["+" "-"] offset)?
    Backward(syn::Ident), //  > label (["+" "-"] offset)?
    Forward(syn::Ident),  //  < label (["+" "-"] offset)?
    Dynamic(syn::Expr),   // =>expr | => (expr) (["+" "-"] offset)?
    Bare(syn::Expr)       // jump to this address
}

impl ParseOpt for Jump {
    fn parse(input: parse::ParseStream) -> parse::Result<Option<Jump>> {
        // extern label
        if eat_pseudo_keyword(input, "extern") {
            let expr: syn::Expr = input.parse()?;

            return Ok(Some(Jump { kind: JumpKind::Bare(expr), offset: None }));
        }

        // -> global_label
        let kind = if input.peek(Token![->]) {
            let _: Token![->] = input.parse()?;
            let name: syn::Ident = input.parse()?;

            JumpKind::Global(name)

        // > forward_label
        } else if input.peek(Token![>]) {
            let _: Token![>] = input.parse()?;
            let name: syn::Ident = input.parse()?;

            JumpKind::Forward(name)

        // < backwards_label
        } else if input.peek(Token![<]) {
            let _: Token![<] = input.parse()?;
            let name: syn::Ident = input.parse()?;

            JumpKind::Backward(name)

        // => dynamic_label
        } else if input.peek(Token![=>]) {
            let _: Token![=>] = input.parse()?;

            let expr: syn::Expr = if input.peek(syn::token::Paren) {
                let inner;
                let _ = syn::parenthesized!(inner in input);
                let inner = &inner;

                inner.parse()?
            } else {
                input.parse()?
            };

            JumpKind::Dynamic(expr)

        // nothing
        } else {
            return Ok(None);
        };

        // parse optional offset
        let offset = if input.peek(Token![-]) || input.peek(Token![+]) {
            if input.peek(Token![+]) {
                let _: Token![+] = input.parse()?;
            }

            let expr: syn::Expr = input.parse()?;
            Some(expr)

        } else {
            None
        };

        Ok(Some(Jump::new(kind, offset)))
    }
}

impl Jump {
    pub fn new(kind: JumpKind, offset: Option<syn::Expr>) -> Jump {
        Jump {
            kind,
            offset
        }
    }

    /// Takes a jump and encodes it as a relocation starting `start_offset` bytes ago, relative to `ref_offset`.
    /// Any data detailing the type of relocation emitted should be contained in `data`, which is emitted as a tuple of u8's.
    pub fn encode(self, field_offset: u8, ref_offset: u8, data: &[u8]) -> Stmt {
        let span = self.span();

        let target_offset = if let Some(offset) = self.offset {
            delimited(offset)
        } else {
            TokenTree::Literal(Literal::isize_suffixed(0))
        };

        // Create a relocation descriptor, containing all information about the actual jump except for the target itself.
        let relocation = Relocation {
            target_offset,
            field_offset,
            ref_offset,
            kind: serialize::expr_tuple_of_u8s(span, data)
        };
        match self.kind {
            JumpKind::Global(ident) => Stmt::GlobalJumpTarget(ident, relocation),
            JumpKind::Backward(ident) => Stmt::BackwardJumpTarget(ident, relocation),
            JumpKind::Forward(ident) => Stmt::ForwardJumpTarget(ident, relocation),
            JumpKind::Dynamic(expr) => Stmt::DynamicJumpTarget(delimited(expr), relocation),
            JumpKind::Bare(expr) => Stmt::BareJumpTarget(delimited(expr), relocation),
        }
    }

    pub fn span(&self) -> Span {
        match &self.kind {
            JumpKind::Global(ident) => ident.span(),
            JumpKind::Backward(ident) => ident.span(),
            JumpKind::Forward(ident) => ident.span(),
            JumpKind::Dynamic(expr) => expr.span(),
            JumpKind::Bare(expr) => expr.span(),
        }
    }
}


/// A relocation entry description
#[derive(Debug, Clone)]
pub struct Relocation {
    pub target_offset: TokenTree,
    pub field_offset: u8,
    pub ref_offset: u8,
    pub kind: TokenTree,
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
    Align(TokenTree, TokenTree),

    // label declarations
    GlobalLabel(syn::Ident),
    LocalLabel(syn::Ident),
    DynamicLabel(TokenTree),

    // and their respective relocations (as expressions as they differ per assembler).
    GlobalJumpTarget(syn::Ident, Relocation),
    ForwardJumpTarget(syn::Ident, Relocation),
    BackwardJumpTarget(syn::Ident, Relocation),
    DynamicJumpTarget(TokenTree, Relocation),
    BareJumpTarget(TokenTree, Relocation),

    // a random statement that has to be inserted between assembly hunks
    Stmt(TokenStream)
}

// convenience methods
impl Stmt {
    #![allow(dead_code)]

    pub fn u8(value: u8) -> Stmt {
        Stmt::Const(u64::from(value), Size::BYTE)
    }

    pub fn u16(value: u16) -> Stmt {
        Stmt::Const(u64::from(value), Size::B_2)
    }

    pub fn u32(value: u32) -> Stmt {
        Stmt::Const(u64::from(value), Size::B_4)
    }

    pub fn u64(value: u64) -> Stmt {
        Stmt::Const(value, Size::B_8)
    }
}


/// Takes an arbitrary tokenstream as input, and ensures it can be interpolated safely.
/// returns a tokentree representing either a single token, or a delimited group.
///
/// If the given tokenstream contains multiple tokens, it will be parenthesized.
///
/// this will panic if given an empty tokenstream.
/// this would use delimiter::None if not for https://github.com/rust-lang/rust/issues/67062
pub fn delimited<T: ToTokens>(expr: T) -> TokenTree {
    let stream = expr.into_token_stream();

    // the stream api is very limited, but cloning a stream is luckily cheap.
    // so to check how many tokens are contained we can do this.
    let mut iter = stream.clone().into_iter();
    let first = iter.next().unwrap();
    if iter.next().is_none() {
        return first;
    }

    let span = stream.span();
    let mut group = Group::new(
        proc_macro2::Delimiter::Parenthesis, stream
    );
    group.set_span(span);
    TokenTree::Group(group)
}

/// Checks if the given tokenstream is a parenthesized expression to work around rustc giving
/// Unnecessary parenthesis warnings in macro-generated code, if this tokentree were to be used
/// as the argument to a single argument function
///
/// i.e. `function(#arg)` expanding to `function((expr))`, which should instead be expanded to
/// `function(expr)`
///
/// To check if this is valid, we should a: test that this tokentree node is a parenthesis delimited
/// node and b: there are no commas in its internal tokentree, because then it'd be a tuple, and
/// this transform would be invalid
pub fn is_parenthesized(expr: &TokenTree) -> bool {
    match expr {
        TokenTree::Group(group) => {
            if group.delimiter() != Delimiter::Parenthesis {
                return false
            }

            for item in group.stream() {
                if let TokenTree::Punct(punct) = item {
                    if punct.as_char() == ',' {
                        return false
                    }
                }
            }

            true
        },
        _ => false
    }
}

/// Create a bitmask with `scale` bits set
pub fn bitmask(scale: u8) -> u32 {
    1u32.checked_shl(u32::from(scale)).unwrap_or(0).wrapping_sub(1)
}


/// Create a bitmask with `scale` bits set
pub fn bitmask64(scale: u8) -> u64 {
    1u64.checked_shl(u32::from(scale)).unwrap_or(0).wrapping_sub(1)
}
