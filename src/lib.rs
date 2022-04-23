pub use tsify_macros::*;

pub trait Tsify {
    #[cfg(feature = "wasm-bindgen-impl")]
    type JsType;

    const DECL: &'static str;
}
