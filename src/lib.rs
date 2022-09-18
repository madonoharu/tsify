pub use tsify_macros::*;

#[cfg(feature = "json")]
#[doc(hidden)]
pub mod __rt {
    pub use gloo_utils::format::JsValueSerdeExt;
}

pub trait Tsify {
    #[cfg(feature = "wasm-bindgen")]
    type JsType;

    const DECL: &'static str;
}
