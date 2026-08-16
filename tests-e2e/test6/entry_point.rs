#![allow(dead_code, unused)]

// This test ensures that Ts<T> can be used in various wasm-bindgen function signatures.

use serde::{Deserialize, Serialize};
use tsify::Ts;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize)]
pub struct Point {
    x: i32,
    y: i32,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub fn call_js_owned(point: Ts<Point>);
    #[wasm_bindgen]
    pub fn call_js_ref(point: &Ts<Point>);
    #[wasm_bindgen]
    pub fn call_js_return(point: &Ts<Point>) -> Ts<Point>;
    #[wasm_bindgen]
    pub async fn call_js_async(point: &Ts<Point>);
    #[wasm_bindgen]
    pub async fn call_js_ref_async(point: &Ts<Point>);
}

#[wasm_bindgen]
pub fn accept_point_owned(point: Ts<Point>) {}

#[wasm_bindgen]
pub fn accept_point_ref(point: &Ts<Point>) {}

#[wasm_bindgen]
pub fn return_point(point: &Ts<Point>) -> Ts<Point> {
    point.clone()
}

#[wasm_bindgen]
pub async fn accept_point_ref_async(point: &Ts<Point>) {}

// Async fns return through `IntoJsResult`; the bare, `Result`, and `Option`
// shapes below each require `Ts<T>: Into<JsValue>` via a distinct impl path.
#[wasm_bindgen]
pub async fn return_point_async(point: &Ts<Point>) -> Ts<Point> {
    point.clone()
}

#[wasm_bindgen]
pub async fn return_point_result_async(point: Ts<Point>) -> Result<Ts<Point>, JsError> {
    Ok(point)
}

#[wasm_bindgen]
pub async fn return_point_option_async(point: &Ts<Point>) -> Option<Ts<Point>> {
    Some(point.clone())
}

#[wasm_bindgen]
pub fn accept_point_vec(point: Vec<Ts<Point>>) {}

#[wasm_bindgen]
pub fn return_point_vec() -> Vec<Ts<Point>> {
    panic!()
}
