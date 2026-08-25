//! doctest at src/lib.rs:168
//! This file is auto-generated, please don't edit this file

            #![allow(unused)]
#[allow(unused_extern_crates)]
extern crate r#tsify;

            fn main() {

            use tsify::Tsify;

#[derive(Tsify)]
enum Color {
    Red,
    Blue,
    Green,
    Rgb(u8, u8, u8),
    Hsv {
        hue: f64,
        saturation: f64,
        value: f64,
    },
}
            
}
