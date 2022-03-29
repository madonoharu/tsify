use tsify::Tsify;
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct GenericStruct<T> {
    x: T,
}
#[automatically_derived]
impl<T> Tsify for GenericStruct<T> {
    const DECL: &'static str = "export type GenericStruct<T> = { x: T };";
}
#[automatically_derived]
const _: () = {
    use wasm_bindgen::{
        convert::{FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi},
        describe::WasmDescribe,
        prelude::*,
    };
    #[wasm_bindgen(typescript_custom_section)]
    const TS_APPEND_CONTENT: &'static str = "export type GenericStruct<T> = { x: T };";
    impl<T> WasmDescribe for GenericStruct<T> {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_EXTERNREF);
            inform(13u32);
            inform(71u32);
            inform(101u32);
            inform(110u32);
            inform(101u32);
            inform(114u32);
            inform(105u32);
            inform(99u32);
            inform(83u32);
            inform(116u32);
            inform(114u32);
            inform(117u32);
            inform(99u32);
            inform(116u32);
        }
    }
    extern crate serde as _serde;
    impl<T> IntoWasmAbi for GenericStruct<T>
    where
        Self: _serde::Serialize,
    {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            JsValue::from_serde(&self).unwrap_throw().into_abi()
        }
    }
    impl<T> OptionIntoWasmAbi for GenericStruct<T>
    where
        Self: _serde::Serialize,
    {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<T> FromWasmAbi for GenericStruct<T>
    where
        Self: _serde::de::DeserializeOwned,
    {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            JsValue::from_abi(js).into_serde().unwrap_throw()
        }
    }
    impl<T> OptionFromWasmAbi for GenericStruct<T>
    where
        Self: _serde::de::DeserializeOwned,
    {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
};
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct GenericNewtype<T>(T);
#[automatically_derived]
impl<T> Tsify for GenericNewtype<T> {
    const DECL: &'static str = "export type GenericNewtype<T> = T;";
}
#[automatically_derived]
const _: () = {
    use wasm_bindgen::{
        convert::{FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi},
        describe::WasmDescribe,
        prelude::*,
    };
    #[wasm_bindgen(typescript_custom_section)]
    const TS_APPEND_CONTENT: &'static str = "export type GenericNewtype<T> = T;";
    impl<T> WasmDescribe for GenericNewtype<T> {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_EXTERNREF);
            inform(14u32);
            inform(71u32);
            inform(101u32);
            inform(110u32);
            inform(101u32);
            inform(114u32);
            inform(105u32);
            inform(99u32);
            inform(78u32);
            inform(101u32);
            inform(119u32);
            inform(116u32);
            inform(121u32);
            inform(112u32);
            inform(101u32);
        }
    }
    extern crate serde as _serde;
    impl<T> IntoWasmAbi for GenericNewtype<T>
    where
        Self: _serde::Serialize,
    {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            JsValue::from_serde(&self).unwrap_throw().into_abi()
        }
    }
    impl<T> OptionIntoWasmAbi for GenericNewtype<T>
    where
        Self: _serde::Serialize,
    {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<T> FromWasmAbi for GenericNewtype<T>
    where
        Self: _serde::de::DeserializeOwned,
    {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            JsValue::from_abi(js).into_serde().unwrap_throw()
        }
    }
    impl<T> OptionFromWasmAbi for GenericNewtype<T>
    where
        Self: _serde::de::DeserializeOwned,
    {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
};
