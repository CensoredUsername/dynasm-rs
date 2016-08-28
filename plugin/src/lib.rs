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
use syntax::fold::Folder;
use syntax::codemap::Span;
use syntax::ast;
use syntax::util::small_vector::SmallVector;
use syntax::parse::token::intern;
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

    Box::new(DynAsm {stmts: SmallVector::many(stmts)})
}

struct DynAsm {
    stmts: SmallVector<ast::Stmt>
}

impl MacResult for DynAsm {
    fn make_stmts(self: Box<Self>) -> Option<SmallVector<ast::Stmt>> {
        Some(self.stmts)
    }

    fn make_items(self: Box<Self>) -> Option<SmallVector<P<ast::Item>>> {
        if self.stmts.len() == 0 {
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
    let id = ecx.mod_path()[0];

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
