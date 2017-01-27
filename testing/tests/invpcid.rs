
#![feature(plugin)]
#![plugin(dynasm)]

#[macro_use]
extern crate dynasmrt;

use dynasmrt::DynasmApi;

include!("gen/invpcid.rs.gen");

