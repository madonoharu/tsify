#![doc = include_str!("../README.md")]
#![allow(clippy::wrong_self_convention)]

#[cfg(not(any(feature = "json", feature = "js")))]
compile_error!(
    "Either the \"json\" or \"js\" feature must be enabled for tsify to function properly"
);

mod ts;
pub use ts::Ts;
mod error;
pub use error::Error;
/// Not a public API. Generated code needs a path it can name across the
/// proc-macro boundary; nothing here is meant to be referred to by hand, and
/// its shape is free to change.
#[cfg(feature = "wasm-bindgen")]
#[doc(hidden)]
#[path = "macro_support.rs"]
pub mod __macro_support;

#[cfg(all(feature = "json", not(feature = "js")))]
pub use gloo_utils::format::JsValueSerdeExt;
#[cfg(feature = "js")]
pub use serde_wasm_bindgen;
pub use tsify_macros::*;

/// The `declare` macro, used in `#[declare]` annotations.
///
/// It gives a type alias a name in TypeScript. The alias stays transparent to
/// Rust, so a type that has to cross the wasm ABI still needs a container of
/// its own.
///
/// ## Examples
///
/// ```
/// # use wasm_bindgen::prelude::*;
/// use tsify::{declare, Tsify, Ts};
/// use serde::{Deserialize, Serialize};
///
/// #[declare]
/// pub type Foo = (i32, String);
///
/// #[derive(Tsify, Serialize)]
/// pub struct Bar(pub Vec<Foo>);
///
/// #[wasm_bindgen]
/// pub fn returns_bar() -> Result<Ts<Bar>, JsError> {
///     Ok(Bar(vec![(42, "forty two".to_string())]).into_ts()?)
/// }
/// ```
///
/// which generates the following ts:
///
/// ```ts
/// /* tslint:disable */
/// /* eslint-disable */
/// export type Bar = Foo[];
///
/// export type Foo = [number, string];
///
///
/// export function returns_bar(): Bar;
/// ```
///
/// Naming the alias is not enough to carry it across on its own. This does not
/// compile — `Vec<Foo>` is not a `Tsify` container, and `Ts<T>` needs one:
///
/// ```compile_fail
/// use tsify::{declare, Ts};
/// use wasm_bindgen::prelude::*;
///
/// #[declare]
/// pub type Foo = (i32, String);
///
/// #[wasm_bindgen]
/// pub fn returns_foo() -> Ts<Vec<Foo>> {
///     unimplemented!()
/// }
/// ```
///
/// Instead, create a wrapper struct for every distinct return type, as shown
/// above. It has to be a distinct type rather than another alias, and it
/// cannot be generic: a generic type reaches the boundary without its
/// arguments ([#76](https://github.com/madonoharu/tsify/issues/76)). That
/// means one wrapper per instantiation.
pub use tsify_macros::declare;

#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::{JsCast, JsValue};

pub struct SerializationConfig {
    pub missing_as_null: bool,
    pub hashmap_as_object: bool,
    pub large_number_types_as_bigints: bool,
}

/// `Tsify` is a trait that allows you to convert a type to and from JavaScript.
/// Can be implemented manually if you need to customize the serialization or deserialization.
pub trait Tsify {
    /// Must be a type imported through `#[wasm_bindgen] extern "C" { .. }`.
    /// [`Ts<T>`] is `#[repr(transparent)]` over this and passes it across the
    /// ABI as a plain JS handle, which any other representation would break.
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
