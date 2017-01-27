
#![feature(plugin)]
#![plugin(dynasm)]

#[macro_use]
extern crate dynasmrt;

use dynasmrt::DynasmApi;

include!("gen/avx512vbmi.rs.gen");

