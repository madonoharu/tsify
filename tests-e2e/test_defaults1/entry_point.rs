//! A default type parameter is declared with its default.
//!
//! This file is built twice: here with the default features, and by
//! `test_defaults_js1`, which points its `[lib] path` at it and turns on `js`.
//! The pair of reference outputs is what that feature changes — a default is
//! rendered like any other type, so `Option` and `HashMap` inside one move with
//! it.
//!
//! A default is read where the parameters are, so a name in it resolves against
//! the parameter list before the declarations around it. The cases below are
//! the ones where those two disagree.
#![allow(deprecated, dead_code)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tsify::Ts;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize)]
pub struct Wrapper<T = i32> {
    value: T,
}

/// A default is rendered like any other type, so the feature moves it.
#[derive(Tsify, Serialize, Deserialize)]
pub struct Rendered<T = Option<u32>, U = HashMap<String, u32>> {
    t: T,
    u: U,
}

/// A type of our own whose TypeScript name the parameter lists below also use.
#[derive(Tsify, Serialize, Deserialize)]
pub struct T {
    z: u32,
}

/// TypeScript reads the `T` in `U`'s default as the parameter declared after
/// it, and rejects that outright as `TS2744`.
#[derive(Tsify, Serialize, Deserialize)]
pub struct Later<U = crate::T, T = String> {
    u: U,
    t: T,
}

/// Accepted by TypeScript, but `T` there means the parameter and not the
/// interface — the same default reading as a different type.
#[derive(Tsify, Serialize, Deserialize)]
pub struct Earlier<T, U = crate::T> {
    t: T,
    u: U,
}

#[derive(Tsify, Serialize, Deserialize)]
pub struct Error {
    m: String,
}

/// Read from inside the namespace, `Error` would reach the sibling variant. The
/// alias the namespace hoists its other references to is what it resolves to.
#[derive(Tsify, Serialize, Deserialize)]
#[tsify(namespace)]
pub enum Outcome<T = crate::Error> {
    Done(T),
    Error(String),
}

/// A reference names its argument, whatever the declaration defaults to.
#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Holder {
    wrapped: Wrapper<u32>,
    outcome: Outcome<u32>,
}

#[wasm_bindgen]
pub fn into_js() -> Holder {
    Holder {
        wrapped: Wrapper { value: 0 },
        outcome: Outcome::Done(0),
    }
}

// This records #76 the way the `test_generic_args1` references do, and what a
// default does to it: the signature loses its argument, and `Wrapper` then
// resolves through the default to `Wrapper<number>` rather than failing. The
// Rust is `Wrapper<u32>`, so the two agree here only by accident of `i32` and
// `u32` sharing a TypeScript name. This line should name its argument when #76
// is fixed.
#[wasm_bindgen]
pub fn wrapped(v: Ts<Wrapper<u32>>) -> Result<(), JsError> {
    let _: Wrapper<u32> = v.to_rust()?;
    Ok(())
}
