use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Point {
    x: i32,
    y: i32,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub fn call_js(point: &Point);
}

#[wasm_bindgen]
pub fn accept_ref_point(point: &Point) {}
