#![allow(clippy::wrong_self_convention)]

#[cfg(not(any(feature = "json", feature = "js")))]
compile_error!(
    "Either the \"json\" or \"js\" feature must be enabled for tsify to function properly"
);

mod ts;
pub use ts::Ts;

#[cfg(all(feature = "json", not(feature = "js")))]
pub use gloo_utils::format::JsValueSerdeExt;
#[cfg(feature = "js")]
pub use serde_wasm_bindgen;
pub use tsify_macros::*;
#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::{JsCast, JsValue};

#[cfg(all(feature = "json", not(feature = "js")))]
pub type Error = serde_json::Error;
#[cfg(feature = "js")]
pub type Error = serde_wasm_bindgen::Error;

pub struct SerializationConfig {
    pub missing_as_null: bool,
    pub hashmap_as_object: bool,
    pub large_number_types_as_bigints: bool,
}

/// `Tsify` is a trait that allows you to convert a type to and from JavaScript.
/// Can be implemented manually if you need to customize the serialization or deserialization.
pub trait Tsify {
    #[cfg(feature = "wasm-bindgen")]
    type JsType: JsCast;

    const DECL: &'static str;
    const SERIALIZATION_CONFIG: SerializationConfig = SerializationConfig {
        missing_as_null: false,
        hashmap_as_object: false,
        large_number_types_as_bigints: false,
    };

    #[cfg(all(feature = "json", not(feature = "js")))]
    #[inline]
    fn into_js(&self) -> serde_json::Result<Self::JsType>
    where
        Self: serde::Serialize,
    {
        JsValue::from_serde(self).map(JsCast::unchecked_from_js)
    }

    #[cfg(all(feature = "json", not(feature = "js")))]
    #[inline]
    fn from_js<T: Into<JsValue>>(js: T) -> serde_json::Result<Self>
    where
        Self: serde::de::DeserializeOwned,
    {
        js.into().into_serde()
    }

    #[cfg(feature = "js")]
    #[inline]
    fn into_js(&self) -> Result<Self::JsType, serde_wasm_bindgen::Error>
    where
        Self: serde::Serialize,
    {
        let config = <Self as Tsify>::SERIALIZATION_CONFIG;
        let serializer = serde_wasm_bindgen::Serializer::new()
            .serialize_missing_as_null(config.missing_as_null)
            .serialize_maps_as_objects(config.hashmap_as_object)
            .serialize_large_number_types_as_bigints(config.large_number_types_as_bigints);
        self.serialize(&serializer).map(JsCast::unchecked_from_js)
    }

    #[cfg(feature = "js")]
    #[inline]
    fn from_js<T: Into<JsValue>>(js: T) -> Result<Self, serde_wasm_bindgen::Error>
    where
        Self: serde::de::DeserializeOwned,
    {
        serde_wasm_bindgen::from_value(js.into())
    }

    /// Calls `Ts::from_rust` on self, returning a `Result<Ts<Self>, crate::Error>`.
    ///
    /// This can (and should) be used with the [`-> Result<_, JsError>`][wasm_bindgen::JsError]
    /// pattern from wasm-bindgen to automatically throw any Err value returned.
    fn into_ts(&self) -> Result<Ts<Self>, crate::Error>
    where
        Self: Sized,
        Self: serde::Serialize,
    {
        Ts::from_rust(self)
    }
}
