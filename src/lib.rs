pub use tsify_macros::{tsify, Tsify};

pub trait Tsify {
    #[cfg(feature = "wasm-bindgen-impl")]
    type JsType;

    const DECL: &'static str;
}
