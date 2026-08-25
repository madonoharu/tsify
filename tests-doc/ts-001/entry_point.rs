//! doctest at src/ts.rs:84
//! This file is auto-generated, please don't edit this file

            #![allow(unused)]
#[allow(unused_extern_crates)]
extern crate r#tsify;

            fn main() {

            use tsify::Tsify;
use tsify::Ts;
use wasm_bindgen::prelude::*;

#[derive(tsify::Tsify, serde::Deserialize)]
struct Point {
    x: f64,
    y: f64,
}

fn some_api(_closure: impl Fn(JsValue)) { /* ... */ }

some_api(|js_value: JsValue| {
    let ts_value: Ts<Point> = Ts::new_unchecked(js_value);
    let point = ts_value.to_rust().unwrap();
});
            
}
