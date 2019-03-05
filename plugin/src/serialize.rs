use syn;
use syn::parse;
use syn::spanned::Spanned;
use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{quote, quote_spanned, ToTokens};

use byteorder::{ByteOrder, LittleEndian};

/// Enum representing the result size of a value/expression/register/etc in bytes.
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

/// Converts a sequence of abstract Statements to actual tokens
pub fn serialize(name: &TokenTree, stmts: Vec<Stmt>) -> TokenStream {
    // first, try to fold constants into a byte stream
    let mut folded_stmts = Vec::new();
    let mut const_buffer = Vec::new();
    for stmt in stmts {
        match stmt {
            Stmt::Const(value, size) => {
                match size {
                    Size::BYTE => const_buffer.push(value as u8),
                    Size::WORD => {
                        let mut buffer = [0u8; 2];
                        LittleEndian::write_u16(&mut buffer, value as u16);
                        const_buffer.extend(&buffer);
                    },
                    Size::DWORD => {
                        let mut buffer = [0u8; 4];
                        LittleEndian::write_u32(&mut buffer, value as u32);
                        const_buffer.extend(&buffer);
                    },
                    Size::QWORD => {
                        let mut buffer = [0u8; 8];
                        LittleEndian::write_u64(&mut buffer, value as u64);
                        const_buffer.extend(&buffer);
                    },
                    _ => unimplemented!()
                }
            },
            Stmt::Extend(data) => {
                const_buffer.extend(data);
            },
            s => {
                // empty the const buffer
                if !const_buffer.is_empty() {
                    folded_stmts.push(Stmt::Extend(const_buffer));
                    const_buffer = Vec::new();
                }
                folded_stmts.push(s);
            }
        }
        while const_buffer.len() > 32 {
            let new_buffer = const_buffer.split_off(32);
            folded_stmts.push(Stmt::Extend(const_buffer));
            const_buffer = new_buffer;
        }
    }
    if !const_buffer.is_empty() {
        folded_stmts.push(Stmt::Extend(const_buffer));
    }

    // and now do the final output pass in one go
    let mut output = TokenStream::new();

    for stmt in folded_stmts {
        let (method, args) = match stmt {
            Stmt::Const(_, _) => unreachable!(),
            Stmt::ExprUnsigned(expr, Size::BYTE)  => ("push",     vec![expr]),
            Stmt::ExprUnsigned(expr, Size::WORD)  => ("push_u16", vec![expr]),
            Stmt::ExprUnsigned(expr, Size::DWORD) => ("push_u32", vec![expr]),
            Stmt::ExprUnsigned(expr, Size::QWORD) => ("push_u64", vec![expr]),
            Stmt::ExprUnsigned(_, _) => unimplemented!(),
            Stmt::ExprSigned(  expr, Size::BYTE)  => ("push_i8",  vec![expr]),
            Stmt::ExprSigned(  expr, Size::WORD)  => ("push_i16", vec![expr]),
            Stmt::ExprSigned(  expr, Size::DWORD) => ("push_i32", vec![expr]),
            Stmt::ExprSigned(  expr, Size::QWORD) => ("push_i64", vec![expr]),
            Stmt::ExprSigned(_, _) => unimplemented!(),
            Stmt::Extend(data)     => ("extend", vec![proc_macro2::Literal::byte_string(&data).into()]),
            Stmt::ExprExtend(expr) => ("extend", vec![expr]),
            Stmt::Align(expr)      => ("align", vec![expr]),
            Stmt::GlobalLabel(n) => ("global_label", vec![expr_string_from_ident(&n)]),
            Stmt::LocalLabel(n)  => ("local_label", vec![expr_string_from_ident(&n)]),
            Stmt::DynamicLabel(expr) => ("dynamic_label", vec![expr]),
            Stmt::GlobalJumpTarget(n,     reloc) => ("global_reloc"  , vec![expr_string_from_ident(&n), reloc]),
            Stmt::ForwardJumpTarget(n,    reloc) => ("forward_reloc" , vec![expr_string_from_ident(&n), reloc]),
            Stmt::BackwardJumpTarget(n,   reloc) => ("backward_reloc", vec![expr_string_from_ident(&n), reloc]),
            Stmt::DynamicJumpTarget(expr, reloc) => ("dynamic_reloc" , vec![expr, reloc]),
            Stmt::BareJumpTarget(expr, reloc)    => ("bare_reloc"    , vec![expr, reloc]),
            Stmt::Stmt(s) => {
                output.extend(quote! {
                    #s ;
                });
                continue;
            }
        };

        // and construct the appropriate method call
        let method = syn::Ident::new(method, Span::call_site());
        output.extend(quote! {
            #name . #method ( #( #args ),* ) ;
        })
    }

    // if we have nothing to emit, expand to nothing. Else, wrap it into a block.
    if output.is_empty() {
        output
    } else {
        quote!{
            {
                #output
            }
        }
    }
}

// below here are all kinds of utility functions to quickly generate TokenTree constructs
// this collection is arbitrary and purely based on what special things are needed for assembler
// codegen implementations

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

// expression of value 0. sometimes needed.
pub fn expr_zero() -> TokenTree {
    proc_macro2::Literal::u8_unsuffixed(0).into()
}

// given an ident, makes it into a "string"
pub fn expr_string_from_ident(i: &syn::Ident) -> TokenTree {
    let name = i.to_string();
    proc_macro2::Literal::string(&name).into()
}

// given an expression, turns it into a negate expression
pub fn inverted(expr: &TokenTree) -> TokenTree {
    delimited(quote! {
        ! #expr
    })
}

// 
pub fn expr_dynscale(scale: &TokenTree, rest: &TokenTree) -> (TokenTree, TokenTree) {
    let tempval = expr_encode_x64_sib_scale(&scale);
    (delimited(quote! {
        let temp = #tempval
    }), delimited(quote! {
         #rest | ((temp & 3) << 6)
    }))
}

// makes (a, b)
pub fn expr_tuple_of_u8s(span: Span, data: &[u8]) -> TokenTree {
    delimited(quote_spanned! {span=>
        (#(#data),*)
    })
}

// makes sum(exprs)
pub fn expr_add_many<T: Iterator<Item=TokenTree>>(span: Span, mut exprs: T) -> Option<TokenTree> {
    let first_expr = exprs.next()?;

    let tokens = quote_spanned!{ span=>
        #first_expr #( + #exprs )*
    };

    Some(delimited(tokens))
}

// makes (size_of<ty>() * value)
pub fn expr_size_of_scale(ty: &syn::Path, value: &TokenTree, size: Size) -> TokenTree {
    let span = value.span();
    let size = size.as_literal();

    delimited(quote_spanned! { span=>
        (::std::mem::size_of::<#ty>() as #size) * #value
    })
}

/// returns orig | ((expr & mask) << shift)
pub fn expr_mask_shift_or(orig: &TokenTree, expr: &TokenTree, mask: u64, shift: i8) -> TokenTree {
    let span = expr.span();

    let mask: TokenTree = proc_macro2::Literal::u64_unsuffixed(mask).into();

    delimited(if shift >= 0 {
        let shift: TokenTree = proc_macro2::Literal::i8_unsuffixed(shift).into();
        quote_spanned! { span=>
            #orig | ((#expr & #mask) << #shift)
        }
    } else {
        let shift: TokenTree = proc_macro2::Literal::i8_unsuffixed(-shift).into();
        quote_spanned! { span=>
            #orig | ((#expr & #mask) >> #shift)
        }
    })
}

/// returns (offset_of!(path, attr) as size)
pub fn expr_offset_of(path: &syn::Path, attr: &syn::Ident, size: Size) -> TokenTree {
    // generate a P<Expr> that resolves into the offset of an attribute to a type.
    // this is somewhat ridiculously complex because we can't expand macros here

    let span = path.span();
    let size = size.as_literal();

    delimited(quote_spanned! { span=>
        unsafe {
            let #path {#attr: _, ..};
            let temp: #path = ::std::mem::uninitialized();
            let rv = &temp.#attr as *const _ as usize - &temp as *const _ as usize;
            ::std::mem::forget(temp);
            rv as #size
        }
    })
}

// returns std::mem::size_of<path>()
pub fn expr_size_of(path: &syn::Path) -> TokenTree {
    // generate a P<Expr> that returns the size of type at path
    let span = path.span();

    delimited(quote_spanned! { span=>
        ::std::mem::size_of::<#path>()
    })
}

// makes the following
// match size {
//    8 => 3,
//    4 => 2,
//    2 => 1,
//    1 => 0,
//    _ => panic!r("Type size not representable as scale")
//}
pub fn expr_encode_x64_sib_scale(size: &TokenTree) -> TokenTree {
    let span = size.span();

    delimited(quote_spanned! { span=>
        match #size {
            8 => 3,
            4 => 2,
            2 => 1,
            1 => 0,
            _ => panic!("Type size not representable as scale")
        }
    })
}

// Reparses a tokentree into an expression
pub fn reparse(tt: &TokenTree) -> parse::Result<syn::Expr> {
    syn::parse2(tt.into_token_stream())
}
