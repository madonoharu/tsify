//! A default type parameter is declared with its default.
#![allow(deprecated)]

use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize)]
pub struct Wrapper<T = i32> {
    value: T,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(namespace)]
pub enum Outcome<T = String> {
    Done(T),
    Failed(u32),
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
        outcome: Outcome::Failed(0),
    }
}
