#![feature(plugin_registrar, rustc_private)]
#![feature(const_fn)]

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
use syntax::codemap::Span;
use syntax::ast;
use syntax::util::small_vector::SmallVector;
use syntax::parse::token::{intern, str_to_ident};
use syntax::tokenstream::TokenTree;
use syntax::ptr::P;

use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::collections::HashMap;

use owning_ref::{OwningRef, RwLockReadGuardRef};

pub mod parser;
pub mod compiler;
pub mod x64data;
pub mod serialize;
pub mod debug;

/// Welcome to the documentation of the dynasm plugin. This mostly exists to ease
/// development and to show a glimpse of what is under the hood of dynasm. Please
/// be aware that nothing in here should be counted on to be stable, the only
/// guarantees are in the syntax the `dynasm!` macro parses and in the code it
/// generates.

#[plugin_registrar]
pub fn registrar(reg: &mut Registry) {
    reg.register_syntax_extension(
        intern("dynasm"),
        SyntaxExtension::NormalTT(
            Box::new(main),
            None,
            false
        )
    );
}

fn main<'cx>(ecx: &'cx mut ExtCtxt, span: Span, token_tree: &[TokenTree])
-> Box<MacResult + 'cx> {
    // expand all macros in our token tree first. This enables the use of rust macros
    // within dynasm
    let token_tree = ecx.expander().fold_tts(token_tree);

    let mut parser = ecx.new_parser_from_tts(&token_tree);

    // construct an ast of assembly nodes
    let (name, ast) = match parser::parse(ecx, &mut parser) {
        Ok(ast) => ast,
        Err(mut e) => {e.emit(); return DummyResult::any(span)}
    };

    // println!("{:?}", ast);

    let stmts = if let Ok(stmts) = compiler::compile(ecx, ast) {
        stmts
    } else {
        return DummyResult::any(span)
    };

    let stmts = serialize::serialize(ecx, name, stmts);

    Box::new(DynAsm {
        ecx: ecx,
        stmts: stmts
    })
}

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
            Some(SmallVector::zero())
        } else {
            None
        }
    }
}

// Crate local data implementation.

pub struct DynasmData {
    aliases: HashMap<ast::Name, (parser::RegId, parser::Size)>
}

pub struct CrateLocalData {
    inner: OwningRef<
        RwLockReadGuard<'static, DynasmStorage>,
        RwLock<DynasmData>
    >
}

impl CrateLocalData {
    fn read(&self) ->RwLockReadGuard<DynasmData> {
        self.inner.read().unwrap()
    }

    fn write(&self) -> RwLockWriteGuard<DynasmData> {
        self.inner.write().unwrap()
    }
}

pub fn crate_local_data(ecx: &ExtCtxt) -> CrateLocalData {
    let id = str_to_ident(&ecx.ecfg.crate_name);

    {
        let data = RwLockReadGuardRef::new(DYNASM_STORAGE.read().unwrap());

        if data.get(&id).is_some() {
            return CrateLocalData {
                inner: data.map(|x| x.get(&id).unwrap())
            }
        }
    }

    {
        let mut lock = DYNASM_STORAGE.write().unwrap();
        lock.insert(id, RwLock::new(DynasmData {
            aliases: HashMap::new()
        }));
    }
    CrateLocalData {
        inner: RwLockReadGuardRef::new(DYNASM_STORAGE.read().unwrap()).map(|x| x.get(&id).unwrap())
    }
}

// the root of all crate-local data
type DynasmStorage = HashMap<ast::Ident, RwLock<DynasmData>>;

lazy_static! {
    pub static ref DYNASM_STORAGE: RwLock<DynasmStorage> = RwLock::new(HashMap::new());
}
