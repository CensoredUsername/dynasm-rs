#![feature(plugin_registrar, rustc_private)]
#![feature(const_fn)]
#![feature(i128_type)]

extern crate syntax;
extern crate rustc_plugin;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitflags;
extern crate owning_ref;

use rustc_plugin::registry::Registry;
use syntax::ext::base::{SyntaxExtension, ExtCtxt, MacResult, DummyResult};
use syntax::ext::build::AstBuilder;
use syntax::fold::Folder;
use syntax::codemap::{Span, Spanned};
use syntax::ast;
use syntax::util::small_vector::SmallVector;
use syntax::parse::parser::Parser;
use syntax::parse::PResult;
use syntax::symbol::Symbol;
use syntax::parse::token;
use syntax::tokenstream::TokenTree;
use syntax::ptr::P;

use std::sync::{RwLock, RwLockReadGuard, Mutex};
use std::collections::HashMap;

use owning_ref::{OwningRef, RwLockReadGuardRef};

pub mod arch;
mod directive;
mod serialize;


/// Welcome to the documentation of the dynasm plugin. This mostly exists to ease
/// development and to show a glimpse of what is under the hood of dynasm. Please
/// be aware that nothing in here should be counted on to be stable, the only
/// guarantees are in the syntax the `dynasm!` macro parses and in the code it
/// generates.

#[plugin_registrar]
pub fn registrar(reg: &mut Registry) {
    reg.register_syntax_extension(
        Symbol::intern("dynasm"),
        SyntaxExtension::NormalTT(
            Box::new(dynasm),
            None,
            false
        )
    );
}

/// dynasm! macro expansion result type
struct DynAsm<'cx, 'a: 'cx> {
    ecx: &'cx ExtCtxt<'a>,
    stmts: Vec<ast::Stmt>
}

impl<'cx, 'a> MacResult for DynAsm<'cx, 'a> {
    fn make_expr(self: Box<Self>) -> Option<P<ast::Expr>> {
        Some(self.ecx.expr_block(self.ecx.block(self.ecx.call_site(), self.stmts)))
    }

    fn make_stmts(self: Box<Self>) -> Option<SmallVector<ast::Stmt>> {
        Some(SmallVector::many(self.stmts))
    }

    fn make_items(self: Box<Self>) -> Option<SmallVector<P<ast::Item>>> {
        if self.stmts.is_empty() {
            Some(SmallVector::new())
        } else {
            None
        }
    }
}

/// The guts of the dynasm! macro start here.
fn dynasm<'cx>(ecx: &'cx mut ExtCtxt, span: Span, token_tree: &[TokenTree])
-> Box<MacResult + 'cx> {
    // expand all macros in our token tree first. This enables the use of rust macros
    // within dynasm (whenever this actually gets implemented within rustc)
    let token_tree = ecx.expander().fold_tts(token_tree);

    let mut parser = ecx.new_parser_from_tts(&token_tree);

    // due to the structure of directives / assembly, we have to evaluate while parsing as
    // things like aliases, arch choices, affect the way in which parsing works.

    match compile(ecx, &mut parser) {
        Ok(stmts) => Box::new(DynAsm {
            ecx: ecx,
            stmts: stmts
        }),
        Err(mut e) => {
            e.emit();
            DummyResult::any(span)
        }
    }
}

/// This struct contains all non-parsing state that dynasm! requires while parsing and compiling
pub struct State<'a> {
    pub stmts: &'a mut Vec<serialize::Stmt>,
    pub target: &'a P<ast::Expr>,
    pub crate_data: &'a DynasmData
}

/// top-level parsing. Handles common prefix symbols and diverts to the selected architecture
/// when an assembly instruction is encountered. When parsing fails an Err() is returned, when
/// non-parsing errors happen a local error message is generated but the function returns Ok().
fn compile<'a>(ecx: &mut ExtCtxt, parser: &mut Parser<'a>) -> PResult<'a, Vec<ast::Stmt>> {
    let target = parser.parse_expr()?;

    let crate_data = crate_local_data(ecx);
    let mut crate_data = crate_data.lock().unwrap();

    let mut stmts = Vec::new();

    while !parser.check(&token::Eof) {
        parser.expect(&token::Semi)?;

        // ;; stmt
        if parser.eat(&token::Semi) {
            if let Some(stmt) = parser.parse_stmt()? {
                stmts.push(serialize::Stmt::Stmt(stmt));
            }
            continue;
        }

        // ; . directive
        if parser.eat(&token::Dot) {
            crate_data.evaluate_directive(&mut stmts, ecx, parser)?;
            continue;
        }

        // ; -> label :
        if parser.eat(&token::RArrow) {
            let span = parser.span;
            let name = parser.parse_ident()?;
            parser.expect(&token::Colon)?;
            stmts.push(serialize::Stmt::GlobalLabel(Spanned {span: span, node: name} ));
            continue;
        }

        // ; => expr
        if parser.eat(&token::FatArrow) {
            let expr = parser.parse_expr()?;
            stmts.push(serialize::Stmt::DynamicLabel(expr));
            continue;
        }

        // ; label :
        if parser.token.is_ident() && parser.look_ahead(1, |t| t == &token::Colon) {
            let span = parser.span;
            let name = parser.parse_ident()?;
            parser.expect(&token::Colon)?;
            stmts.push(serialize::Stmt::LocalLabel(Spanned {span: span, node: name} ));
            continue;
        }

        // anything else is an assembly instruction which should be in current_arch
        let mut state = State {
            stmts: &mut stmts,
            target: &target,
            crate_data: &*crate_data
        };
        crate_data.current_arch.compile_instruction(&mut state, ecx, parser)?;
    }

    Ok(serialize::serialize(ecx, target, stmts))
}

// Crate local data implementation.

type DynasmStorage = HashMap<String, Mutex<DynasmData>>;

pub struct DynasmData {
    pub current_arch: Box<arch::Arch>,
    pub aliases: HashMap<String, String>
}

impl DynasmData {
    fn new() -> DynasmData {
        DynasmData {
            current_arch: arch::from_str(arch::CURRENT_ARCH).expect("Default architecture is invalid"),
            aliases: HashMap::new()
        }
    }
}

pub type CrateLocalData = OwningRef<
    RwLockReadGuard<'static, DynasmStorage>,
    Mutex<DynasmData>
>;

pub fn crate_local_data(ecx: &ExtCtxt) -> CrateLocalData {
    let id = &ecx.ecfg.crate_name;

    {
        let data = RwLockReadGuardRef::new(DYNASM_STORAGE.read().unwrap());

        if data.get(id).is_some() {
            return data.map(|x| x.get(id).unwrap())
        }
    }

    {
        let mut lock = DYNASM_STORAGE.write().unwrap();
        lock.insert(id.clone(), Mutex::new(DynasmData::new()));
    }
    RwLockReadGuardRef::new(DYNASM_STORAGE.read().unwrap())
                       .map(|x| x.get(id).unwrap())
}

// this is where the actual storage resides.

lazy_static! {
    static ref DYNASM_STORAGE: RwLock<DynasmStorage> = RwLock::new(HashMap::new());
}
