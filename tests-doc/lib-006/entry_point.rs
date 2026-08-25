//! doctest at src/lib.rs:234
//! This file is auto-generated, please don't edit this file

            #![allow(unused)]
#[allow(unused_extern_crates)]
extern crate r#tsify;

            fn main() {

            use tsify::{declare, Tsify};

#[derive(Tsify)]
struct Foo<T>(T);

#[declare]
type Bar = Foo<i32>;
            
}
