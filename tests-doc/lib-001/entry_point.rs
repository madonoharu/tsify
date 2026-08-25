//! doctest at src/lib.rs:33
//! This file is auto-generated, please don't edit this file

            #![allow(unused)]
#[allow(unused_extern_crates)]
extern crate r#tsify;

            fn main() {

            use wasm_bindgen::prelude::*;
use tsify::{declare, Tsify, Ts};
use serde::{Deserialize, Serialize};

#[derive(Tsify, Serialize, Deserialize)]
struct Foo<T>(T);

#[declare]
type Bar = Foo<Vec<(i32, usize)>>;

#[wasm_bindgen]
pub fn returns_bar() -> Result<Ts<Bar>, JsError> {
    Ok(Foo(vec![(-13,42)]).into_ts()?)
}
            
}
