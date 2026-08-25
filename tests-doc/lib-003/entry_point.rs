//! doctest at src/lib.rs:142
//! This file is auto-generated, please don't edit this file

            #![allow(unused)]
#[allow(unused_extern_crates)]
extern crate r#tsify;

            fn main() {

            use tsify::Tsify;

#[derive(Tsify)]
struct Optional {
    #[tsify(optional)]
    a: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    b: Option<String>,
    #[serde(default)]
    c: i32,
}
            
}
            