//! This module contains various infrastructure that is common across all assembler backends
use proc_macro2::{Span, TokenTree, TokenStream, Literal, Group, Delimiter};
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::parse;
use syn::Token;

use crate::parse_helpers::{ParseOpt, eat_pseudo_keyword};

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

/// AST type for parsed dynasm label reference syntax.
/// Represents a reference to somewhere, either a label or an address, with an optional expression offset.
#[derive(Debug, Clone)]
pub struct JumpTarget {
    pub kind: JumpTargetKind,
    pub offset: Option<syn::Expr>
}

/// The different types of jump targets
#[derive(Debug, Clone)]
pub enum JumpTargetKind {
    // note: these symbol choices try to avoid stuff that is a valid starting symbol for parse_expr
    // in order to allow the full range of expressions to be used. the only currently existing ambiguity is
    // with the symbol <, as this symbol is also the starting symbol for the universal calling syntax <Type as Trait>.method(args)

    /// A global label: `->label (+/- offset_expr)`
    Global(syn::Ident),
    /// A backwards local label: `<label (+/- offset_expr)`
    Backward(syn::Ident),
    /// A forwards local label: `<label (+/- offset_expr)`
    Forward(syn::Ident),
    /// A dynamic label: `=>label_expr (+/- offset_expr)`
    Dynamic(TokenTree),
    /// A reference to an absolute address: `extern address_expr (+/- offset_expr)`
    Absolute(TokenTree),
    /// A relative offset. This doesn't have dedicated syntax but used internally by x86 which needs them for rip-relative addressing
    Relative(TokenTree),
}

impl ParseOpt for JumpTarget {
    fn parse(input: parse::ParseStream) -> parse::Result<Option<JumpTarget>> {
        // extern label
        if eat_pseudo_keyword(input, "extern") {
            let expr: syn::Expr = input.parse()?;

            return Ok(Some(JumpTarget { kind: JumpTargetKind::Absolute(delimited(expr)), offset: None }));
        }

        // -> global_label
        let kind = if input.peek(Token![->]) {
            let _: Token![->] = input.parse()?;
            let name: syn::Ident = input.parse()?;

            JumpTargetKind::Global(name)

        // > forward_label
        } else if input.peek(Token![>]) {
            let _: Token![>] = input.parse()?;
            let name: syn::Ident = input.parse()?;

            JumpTargetKind::Forward(name)

        // < backwards_label
        } else if input.peek(Token![<]) {
            let _: Token![<] = input.parse()?;
            let name: syn::Ident = input.parse()?;

            JumpTargetKind::Backward(name)

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

            JumpTargetKind::Dynamic(delimited(expr))

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

        Ok(Some(JumpTarget::new(kind, offset)))
    }
}

impl JumpTarget {
    pub fn new(kind: JumpTargetKind, offset: Option<syn::Expr>) -> JumpTarget {
        JumpTarget {
            kind,
            offset
        }
    }

    pub fn target_is_absolute(&self) -> bool {
        match self.kind {
            JumpTargetKind::Absolute(_) => true,
            _ => false
        }
    }

    /// Takes a jump and encodes it as a relocation starting `start_offset` bytes ago, relative to `ref_offset`.
    /// Any data detailing the type of relocation emitted should be contained in `data`, which is emitted as a tuple of u8's.
    pub fn encode(self, field_offset: u8, ref_offset: u8, relative_encoding: bool, encoding: RelocationEncoding) -> Stmt {
        let kind = match (relative_encoding, self.target_is_absolute()) {
            (true, false) => RelocationKind::Relative,
            (true, true) => RelocationKind::RelToAbs,
            (false, false) => RelocationKind::AbsToRel,
            (false, true) => RelocationKind::Absolute,
        };

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
            kind,
            encoding
        };
        match self.kind {
            JumpTargetKind::Global(ident) => Stmt::GlobalJumpTarget(ident, relocation),
            JumpTargetKind::Backward(ident) => Stmt::BackwardJumpTarget(ident, relocation),
            JumpTargetKind::Forward(ident) => Stmt::ForwardJumpTarget(ident, relocation),
            JumpTargetKind::Dynamic(expr) => Stmt::DynamicJumpTarget(expr, relocation),
            JumpTargetKind::Absolute(expr)
            | JumpTargetKind::Relative(expr) => Stmt::ValueJumpTarget(expr, relocation),
        }
    }

    pub fn span(&self) -> Span {
        match &self.kind {
            JumpTargetKind::Global(ident)
            | JumpTargetKind::Backward(ident)
            | JumpTargetKind::Forward(ident) => ident.span(),
            JumpTargetKind::Dynamic(expr)
            | JumpTargetKind::Absolute(expr)
            | JumpTargetKind::Relative(expr) => expr.span(),
        }
    }
}


/// Different relocation behaviours for an encoding-target pair
/// This specifies how the relocation has to be adapted when the assembling buffer is moved.
/// Note that there's no absolute kind as an absolute encoding to an absolute target
/// is just an immediate.
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum RelocationKind {
    /// A relative encoding to a relative target
    Relative = 0,
    /// An absolute encoding to a relative target
    AbsToRel = 1,
    /// A relative encoding to an absolute target
    RelToAbs = 2,
    /// An absolute encoding to an absolute target (idk why)
    Absolute = 3,
}


/// Specifies the way that a relocation is encoded.
#[derive(Debug, Clone, Copy)]
pub enum RelocationEncoding {
    /// Just as a value of a certain size, encoded little-endian
    Simple(Size),
    /// An encoding custom to a certain instruction set. value 0-59 are available here.
    Custom(u8)
}

impl RelocationEncoding {
    /// pack the type of relocation that this is, based on the `kind` and `encoding` fields, in a
    /// single byte. This should match with the decoding logic in `dynasmrt`.
    pub fn encode(&self, kind: RelocationKind) -> u8 {
        (match self {
            RelocationEncoding::Simple(size) => match size {
                Size::BYTE => 0,
                Size::B_2 => 1,
                Size::B_4 => 2,
                Size::B_8 => 3,
                _ => panic!("Unencodable size given for simple relocation")
            },
            RelocationEncoding::Custom(code) => {
                (code + 4) & 0x3F
            }
        }) | ((kind as u8) << 6)
    }
}


/// A relocation entry description
#[derive(Debug, Clone)]
pub struct Relocation {
    pub target_offset: TokenTree,
    pub field_offset: u8,
    pub ref_offset: u8,
    pub kind: RelocationKind,
    pub encoding: RelocationEncoding
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
    ValueJumpTarget(TokenTree, Relocation),

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

/// Checks if the given `Group` is a parenthesized expression to work around rustc giving
/// Unnecessary parenthesis warnings in macro-generated code, if this tokentree were to be used
/// as the argument to a single argument function
///
/// i.e. `function(#arg)` expanding to `function((expr))`, which should instead be expanded to
/// `function(expr)`
///
/// To check if this is valid, we should a: test that this tokentree node is a parenthesis delimited
/// node and b: there are no commas in its internal tokentree, because then it'd be a tuple, and
/// this transform would be invalid
pub fn is_parenthesized(group: &Group) -> bool {
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
}

/// Returns the given `TokenTree`, but if it's a parenthesized group, it will change this
/// to a None-delimited group, if `is_parenthesized` deems this to be a valid transform
///
/// this is intended to work around unneeded parenthesis around function arguments warnings
pub fn strip_parenthesis(expr: &mut TokenTree) {
    if let TokenTree::Group(group) = &*expr {
        if is_parenthesized(group) {
            let mut stripped = TokenTree::Group(Group::new(Delimiter::None, group.stream()));
            stripped.set_span(group.span());
            *expr = stripped;
        }
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

/// Wrap the provided expression in an `Into::into` call.
///
/// This is only done when the crate is built with the operand_conversions feature enabled.
#[cfg(feature = "runtime_computations")]
pub fn maybe_into(expr: impl quote::ToTokens) -> proc_macro2::TokenStream {
    quote::quote_spanned! { expr.span()=>
        Into::into(#expr)
    }
}

#[cfg(not(feature = "runtime_computations"))]
pub fn maybe_into(expr: impl quote::ToTokens) -> proc_macro2::TokenStream {
    quote::ToTokens::into_token_stream(delimited(expr))
}
