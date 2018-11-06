#![feature(proc_macro_diagnostic)]

// token/ast manipulation
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
extern crate quote;

// utility
extern crate lazy_static;
extern crate bitflags;
extern crate owning_ref;
extern crate byteorder;

use syn::parse;
use syn::{Token, parse_macro_input};
use proc_macro2::{Span, TokenTree};

use lazy_static::lazy_static;
use owning_ref::{OwningRef, RwLockReadGuardRef};

use std::sync::{RwLock, RwLockReadGuard, Mutex};
use std::collections::HashMap;

/// Module with architecture-specific assembler implementations
mod arch;
/// Module contaning the implementation of directives
mod directive;
/// Module containing utility functions for creating TokenTrees from assembler / directive output
mod serialize;

/// The whole point
#[proc_macro]
pub fn dynasm(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // try parsing the tokenstream into a dynasm struct containing
    // an abstract representation of the statements to create
    let dynasm = parse_macro_input!(tokens as Dynasm);

    // serialize the resulting output into tokens
    let res = serialize::serialize(&dynasm.target, dynasm.stmts).into();
    res
}

/// output from parsing a full dynasm invocation. target represents the first dynasm argument, being the assembler
/// variable being used. stmts contains an abstract representation of the statements to be generated from this dynasm
/// invocation.
struct Dynasm {
    target: TokenTree,
    stmts: Vec<serialize::Stmt>
}

/// top-level parsing. Handles common prefix symbols and diverts to the selected architecture
/// when an assembly instruction is encountered. When parsing fails an Err() is returned, when
/// non-parsing errors happen err() will be called, but this function returns Ok().
impl parse::Parse for Dynasm {
    fn parse(input: parse::ParseStream) -> parse::Result<Self> {

        // parse the assembler target declaration
        let target: syn::Expr = input.parse()?;
        // and just convert it back to a tokentree since that's how we'll always be using it.
        let target = serialize::delimited(target);

        // get crate-local data (alias definitions, current architecture) based on ???
        // FIXME: find a way to get the crate name? or maybe the file name at the expansion site?
        let crate_data = crate_local_data();
        let mut crate_data = crate_data.lock().unwrap();

        // prepare the statement buffer
        let mut stmts = Vec::new();

        // if we're not at the end of the macro, we should be expecting a semicolon and a new directive/statement/label/op
        while !input.is_empty() {
            let _: Token![;] = input.parse()?;

            // ;; stmt
            if input.peek(Token![;]) {
                let _: Token![;] = input.parse()?;

                let stmt: syn::Expr = input.parse()?; // FIXME statement used to be optional

                stmts.push(serialize::Stmt::Stmt(serialize::delimited(stmt)));
                continue;
            }

            // ; . directive
            if input.peek(Token![.]) {
                let _: Token![.] = input.parse()?;

                crate_data.evaluate_directive(&mut stmts, input)?;
                continue;
            }

            // ; -> label :
            if input.peek(Token![->]) {
                let _: Token![->] = input.parse()?;

                let name: syn::Ident = input.parse()?;
                let _: Token![:] = input.parse()?;

                stmts.push(serialize::Stmt::GlobalLabel(name));
                continue;
            }

            // ; => expr
            if input.peek(Token![=>]) {
                let _: Token![=>] = input.parse()?;

                let expr: syn::Expr = input.parse()?;

                stmts.push(serialize::Stmt::DynamicLabel(serialize::delimited(expr)));
                continue;
            }

            // ; label :
            if input.peek(syn::Ident) && input.peek2(Token![:]) {

                let name: syn::Ident = input.parse()?;
                let _: Token![:] = input.parse()?;

                stmts.push(serialize::Stmt::LocalLabel(name));
                continue;
            }

            // anything else is an assembly instruction which should be in current_arch
            let mut state = State {
                stmts: &mut stmts,
                target: &target,
                crate_data: &*crate_data,
            };
            crate_data.current_arch.compile_instruction(&mut state, input)?;

        }

        Ok(Dynasm {
            target,
            stmts
        })
    }
}

/// This is only compiled when the dynasm_opmap feature is used. It exports the internal assembly listings
/// into a string that can then be included into the documentation for dynasm.
#[cfg(feature = "dynasm_opmap")]
#[proc_macro]
pub fn dynasm_opmap(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {

    // parse to ensure that no macro arguments were provided
    let _ = parse_macro_input!(tokens as DynasmOpmap);

    let mut s = String::new();
    s.push_str("% Instruction Reference\n\n");

    let mut mnemnonics: Vec<_> = arch::x64::x64data::mnemnonics().cloned().collect();
    mnemnonics.sort();
    for mnemnonic in mnemnonics {
        // get the data for this mnemnonic
        let data = arch::x64::x64data::get_mnemnonic_data(mnemnonic).unwrap();
        // format the data for the opmap docs
        let mut formats = data.into_iter()
            .map(|x| arch::x64::debug::format_opdata(mnemnonic, x))
            .flat_map(|x| x)
            .map(|x| x.replace(">>> ", ""))
            .collect::<Vec<_>>();
        formats.sort();

        // push mnemnonic name as title
        s.push_str("### ");
        s.push_str(mnemnonic);
        s.push_str("\n```\n");

        // push the formats
        s.push_str(&formats.join("\n"));
        s.push_str("\n```\n");
    }

    let token = quote::quote! {
        #s
    };
    token.into()
}

/// As dynasm_opmap takes no args it doesn't parse to anything
struct DynasmOpmap {

}

/// As dynasm_opmap takes no args it doesn't parse to anything.
/// This just exists so syn will give an error when no args are present.
impl parse::Parse for DynasmOpmap {
    fn parse(_input: parse::ParseStream) -> parse::Result<Self> {
        Ok(DynasmOpmap {})
    }
}

/// This struct contains all non-parsing state that dynasm! requires while parsing and compiling
struct State<'a> {
    pub stmts: &'a mut Vec<serialize::Stmt>,
    pub target: &'a TokenTree,
    pub crate_data: &'a DynasmData,
}

// Crate local data implementation.

type DynasmStorage = HashMap<String, Mutex<DynasmData>>;

struct DynasmData {
    pub current_arch: Box<arch::Arch>,
    pub aliases: HashMap<String, String>,
}

impl DynasmData {
    fn new() -> DynasmData {
        DynasmData {
            current_arch:
                arch::from_str(arch::CURRENT_ARCH).expect("Default architecture is invalid"),
            aliases: HashMap::new(),
        }
    }
}

type CrateLocalData = OwningRef<RwLockReadGuard<'static, DynasmStorage>, Mutex<DynasmData>>;

fn crate_local_data() -> CrateLocalData {
    let id = &"FIXME".to_string(); // FIXME: do we have any way of getting the crate name? or maybe the source file name

    {
        let data = RwLockReadGuardRef::new(DYNASM_STORAGE.read().unwrap());

        if data.get(id).is_some() {
            return data.map(|x| x.get(id).unwrap());
        }
    }

    {
        let mut lock = DYNASM_STORAGE.write().unwrap();
        lock.insert(id.clone(), Mutex::new(DynasmData::new()));
    }
    RwLockReadGuardRef::new(DYNASM_STORAGE.read().unwrap()).map(|x| x.get(id).unwrap())
}

// this is where the actual storage resides.

lazy_static! {
    static ref DYNASM_STORAGE: RwLock<DynasmStorage> = RwLock::new(HashMap::new());
}


// temporary till Diagnostic gets stabilized
fn err(span: Span, msg: String) { // FIXME
    let span: proc_macro::Span = span.unstable();
    let diag = proc_macro::Diagnostic::spanned(span, proc_macro::Level::Error, msg);
    diag.emit();
}
