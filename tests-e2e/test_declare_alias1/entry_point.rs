#![allow(dead_code)]

// #116 — `#[declare]` takes arguments and forwards them for a struct or an
// enum. On a type alias it accepts them and drops them: no error, no warning,
// no note that the attribute went nowhere.
//
// The reference records the current output rather than the intended one, the
// way `test_generic_args1` records #76. The alias rows should change when this
// is fixed; the struct and enum rows beside them should not.

use serde::{Deserialize, Serialize};
use tsify::declare;

// Forwarded, as they should be.
#[declare(rename = "RenamedStruct")]
#[derive(Serialize, Deserialize)]
pub struct S {
    pub a: u32,
}

#[declare(rename = "RenamedEnum")]
#[derive(Serialize, Deserialize)]
pub enum E {
    V(u32),
}

// Broken: the alias keeps its Rust name.
#[declare(rename = "RenamedAlias")]
pub type A = Vec<u32>;

// Broken the same way for the other container attributes an alias might want.
#[declare(type_prefix = "Pre")]
pub type B = Vec<u32>;

// The control: an alias with no arguments, which is all the path supports today.
#[declare]
pub type C = Vec<u32>;
