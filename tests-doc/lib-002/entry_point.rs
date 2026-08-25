//! doctest at src/lib.rs:122
//! This file is auto-generated, please don't edit this file

            #![allow(unused)]
#[allow(unused_extern_crates)]
extern crate r#tsify;

            fn main() {

            use tsify::Tsify;

#[derive(Tsify)]
pub struct Foo {
    #[tsify(type = "0 | 1 | 2")]
    x: i32,
}
            
}
