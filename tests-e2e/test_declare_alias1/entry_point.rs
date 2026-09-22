#![allow(dead_code)]

// #116 — `#[declare(...)]` takes arguments and forwards them for a struct or an
// enum. On a type alias it used to accept them and drop them: no error, no
// warning, no note that the attribute went nowhere.
//
// The reference records what the alias path emits now. The attributes that
// describe a container rather than a declaration (`into_wasm_abi`,
// `from_wasm_abi`, `namespace`, `type_params`) are refused at compile time,
// which `tests/compile_fail.rs` covers; a reference to a renamed alias still
// emits its Rust name, which is #103 rather than this, so nothing below refers
// to `A` by the name it was given.

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

// Fixed: the alias is declared under the name it was given.
#[declare(rename = "RenamedAlias")]
pub type A = Vec<u32>;

// Fixed: the affix renames the alias, and does not reach the built-in behind it.
#[declare(type_prefix = "Pre")]
pub type B = Vec<u32>;

// The control: an alias with no arguments.
#[declare]
pub type C = Vec<u32>;

// Fixed: an alias body can be overridden the way a field's type can.
#[declare(type = "string")]
pub type D = u32;
