use std::borrow::Cow;
use tsify::Tsify;
#[tsify(into_wasm_abi, from_wasm_abi)]
struct Borrow<'a> {
    raw: &'a str,
    cow: Cow<'a, str>,
}
#[automatically_derived]
impl<'a> Tsify for Borrow<'a> {
    const DECL: &'static str = "export type Borrow = { raw: string; cow: string };";
}
#[automatically_derived]
const _: () = {
    use wasm_bindgen::{
        convert::{FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi},
        describe::WasmDescribe,
        prelude::*,
    };
    #[wasm_bindgen(typescript_custom_section)]
    const TS_APPEND_CONTENT: &'static str = "export type Borrow = { raw: string; cow: string };";
    impl<'a> WasmDescribe for Borrow<'a> {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_EXTERNREF);
            inform(6u32);
            inform(66u32);
            inform(111u32);
            inform(114u32);
            inform(114u32);
            inform(111u32);
            inform(119u32);
        }
    }
    extern crate serde as _serde;
    impl<'a> IntoWasmAbi for Borrow<'a>
    where
        Self: _serde::Serialize,
    {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            JsValue::from_serde(&self).unwrap_throw().into_abi()
        }
    }
    impl<'a> OptionIntoWasmAbi for Borrow<'a>
    where
        Self: _serde::Serialize,
    {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> FromWasmAbi for Borrow<'a>
    where
        Self: _serde::de::DeserializeOwned,
    {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            JsValue::from_abi(js).into_serde().unwrap_throw()
        }
    }
    impl<'a> OptionFromWasmAbi for Borrow<'a>
    where
        Self: _serde::de::DeserializeOwned,
    {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
};
