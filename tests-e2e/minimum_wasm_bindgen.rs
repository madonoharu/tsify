#![allow(dead_code)]

// This is a build-success gate, not another declaration snapshot. These are the
// four arguments whose names #129's descriptor-facing code must evaluate once
// it is connected. At the declared wasm-bindgen minimum, a dev build rejects
// the control flow used by maps and wide integers and the unaligned byte access
// used by tuples and options. Keeping all four in one tiny crate covers both
// failure modes without rebuilding the reference-output suite or comparing a
// pkg/*.d.ts file.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tsify::{Ts, Tsify};
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize)]
pub struct Envelope<T> {
    value: T,
}

#[wasm_bindgen]
pub fn tuple(_value: Ts<Envelope<(u32, u32)>>) {}

#[wasm_bindgen]
pub fn option(_value: Ts<Envelope<Option<u32>>>) {}

#[wasm_bindgen]
pub fn map(_value: Ts<Envelope<HashMap<String, u32>>>) {}

#[wasm_bindgen]
pub fn wide_integer(_value: Ts<Envelope<u64>>) {}
