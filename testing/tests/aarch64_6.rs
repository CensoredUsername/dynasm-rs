#![feature(proc_macro_hygiene)]
#![allow(unused_imports)]

use dynasm::dynasm;
use dynasmrt::DynasmApi;

include!("gen_aarch64/aarch64_tests_6.rs.gen");

