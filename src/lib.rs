#![allow(clippy::wrong_self_convention)]

#[cfg(all(feature = "json", not(feature = "js")))]
pub use gloo_utils::format::JsValueSerdeExt;
pub use tsify_macros::*;
#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::{JsCast, JsValue};

pub struct SerializationConfig {
    pub missing_as_null: bool,
    pub hashmap_as_object: bool,
}

pub trait Tsify {
    #[cfg(feature = "wasm-bindgen")]
    type JsType: JsCast;

    const DECL: &'static str;
    const SERIALIZATION_CONFIG: SerializationConfig = SerializationConfig {
        missing_as_null: false,
        hashmap_as_object: false,
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
            .serialize_maps_as_objects(config.hashmap_as_object);
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
}
