//! doctest at src/ts.rs:31
//! This file is auto-generated, please don't edit this file

            #![allow(unused)]
#[allow(unused_extern_crates)]
extern crate r#tsify;

            fn main() {

            use tsify::Tsify;
use tsify::Ts;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsError;

#[derive(Tsify, serde::Deserialize, serde::Serialize)]
pub struct Vec2 {
   x: f64,
   y: f64,
}

#[wasm_bindgen]
pub fn rotate(v: Ts<Vec2>, theta_rad: f64) -> Result<Ts<Vec2>, JsError> {
    // Deserialize to rust type, throw deserialization error if fails
    let Vec2 { x, y } = v.to_rust()?;
    // Do some maths
    let cos = theta_rad.cos();
    let sin = theta_rad.sin();
    let result = Vec2 {
        x: x * cos - y * sin,
        y: x * sin + y * cos,
    };
    // Serialize back to JsValue, throw serialization error if fails
    Ok(result.into_ts()?)
}
            
}
            