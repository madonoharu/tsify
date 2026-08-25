//! doctest at src/lib.rs:25
//! This file is auto-generated, please don't edit this file

            #![allow(unused)]
#[allow(unused_extern_crates)]
extern crate r#tsify;

            fn main() {

            use serde::{Deserialize, Serialize};
use tsify::Tsify;
use tsify::Ts;
use tsify::declare;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsError;

#[declare]
pub type Coordinate = i32;

#[derive(Tsify, Serialize, Deserialize)]
pub struct Point {
    x: Coordinate,
    y: Coordinate,
}

#[wasm_bindgen]
pub fn into_js() -> Result<Ts<Point>, JsError> {
    let point = Point { x: 0, y: 0 };
    Ok(point.into_ts()?)
}

#[wasm_bindgen]
pub fn from_js(point: Ts<Point>) -> Result<(), JsError> {
    let point: Point = point.to_rust()?;
    Ok(())
}
            
}
