#![feature(proc_macro_hygiene)]
#![allow(unused_imports)]

use dynasm::dynasm;
use dynasmrt::DynasmApi;

include!("gen_x64/ssse3.rs.gen");
