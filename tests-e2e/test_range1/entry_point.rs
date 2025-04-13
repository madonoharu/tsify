use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Range {
    foo: u32,
    bar: String,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct A {
    range: Range,
}

#[wasm_bindgen]
pub fn consume(_range: Range) {}

#[wasm_bindgen]
pub fn into_js() -> Range {
    Range {
        foo: 42,
        bar: "BAR".to_string(),
    }
}

#[wasm_bindgen]
pub fn consume_vector(_ranges: Vec<Range>) {}

#[wasm_bindgen]
pub fn vector_into_js() -> Vec<Range> {
    vec![
        Range {
            foo: 42,
            bar: "BAR".to_string(),
        },
        Range {
            foo: 42,
            bar: "BAR".to_string(),
        },
        Range {
            foo: 42,
            bar: "BAR".to_string(),
        },
    ]
}
