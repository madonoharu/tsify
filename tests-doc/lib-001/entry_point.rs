//! doctest at src/lib.rs:26
//! This file is auto-generated, please don't edit this file

            #![allow(unused)]
#[allow(unused_extern_crates)]
extern crate r#tsify;

            fn main() {

            use wasm_bindgen::prelude::*;
use tsify::{declare, Tsify, Ts};
use serde::{Deserialize, Serialize};

#[declare]
pub type Foo = (i32, String);

#[derive(Tsify, Serialize)]
pub struct Bar(pub Vec<Foo>);

#[wasm_bindgen]
pub fn returns_bar() -> Result<Ts<Bar>, JsError> {
    Ok(Bar(vec![(42, "forty two".to_string())]).into_ts()?)
}
            
}
