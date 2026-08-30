#![allow(dead_code)]

// What `#[tsify(type_params = "...")]` writes today, including the two ways it
// is wrong. The reference records the current output rather than the intended
// one, the way `test_generic_args1` records #76: the lines below marked as
// broken should change when #112 and #113 are fixed, and the controls beside
// them should not.
//
// The emitted `.d.ts` does not type-check. That is the bug, not a defect in
// this crate, which is why there is no `tsc` step here yet. Measured against
// TypeScript 5.9.3: TS1005 and TS1109 on the two unions carrying a default, and
// TS2304 on `RenamedParam` when it is checked on its own — the parse errors
// suppress the semantic pass when everything is in one file.

use serde::{Deserialize, Serialize};
use tsify::Tsify;

// #113 — the attribute renames the declared parameter list wholesale, and the
// field keeps rendering the Rust parameter. `U` is declared and never used;
// `T` is used and never declared (TS2304).
#[derive(Tsify, Serialize, Deserialize)]
#[tsify(type_params = "U")]
pub struct RenamedParam<T> {
    pub data: T,
}

// The control for #113: names that happen to match the Rust parameters are the
// case that works, which is what makes the bug easy to miss.
#[derive(Tsify, Serialize, Deserialize)]
#[tsify(type_params = "T")]
pub struct MatchingParam<T> {
    pub data: T,
}

// #112 — the union a namespaced enum writes puts the whole parameter spec in a
// type-argument position, where a default is not allowed (TS1005). Measured:
// this happens whenever the spec carries an `=`, with or without a comma, so
// the two below fail the same way and the one after them does not. #112 says a
// single parameter with no comma works; that is not what the reference shows.
#[derive(Tsify, Serialize, Deserialize)]
#[tsify(namespace, type_params = "T = Record<string, number>")]
pub enum CompoundDefault<T> {
    V(T),
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(namespace, type_params = "T = string")]
pub enum SimpleDefault<T> {
    V(T),
}

// The same argument from outside a namespace. There is no union here, so the
// default lands in a declaration where it is legal, and this one type-checks.
// The comma split #112 describes happens inside the parser either way; nothing
// in this reference distinguishes a split spec from a rejoined one, so what a
// fix changes here is not yet visible.
#[derive(Tsify, Serialize, Deserialize)]
#[tsify(type_params = "T = Record<string, number>")]
pub struct CompoundDefaultStruct<T> {
    pub data: T,
}

// The control for the two above: a namespaced enum whose `type_params` carries
// no `=`. Its union is `NoDefault.V<T>`, which is valid, and it should stay
// that way.
#[derive(Tsify, Serialize, Deserialize)]
#[tsify(namespace, type_params = "T")]
pub enum NoDefault<T> {
    V(T),
}
