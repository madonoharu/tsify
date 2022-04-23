use tsify::Tsify;
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct GenericStruct<T> {
    x: T,
}
#[automatically_derived]
const _: () = {
    extern crate serde as _serde;
    use wasm_bindgen::{
        convert::{FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi},
        describe::WasmDescribe,
        prelude::*,
    };
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(typescript_type = "GenericStruct")]
        pub type JsType;
    }
    impl<T> Tsify for GenericStruct<T> {
        type JsType = JsType;
        const DECL: &'static str = "export interface GenericStruct<T> {\n    x: T;\n}";
    }
    #[wasm_bindgen(typescript_custom_section)]
    const TS_APPEND_CONTENT: &'static str = "export interface GenericStruct<T> {\n    x: T;\n}";
    impl<T> WasmDescribe for GenericStruct<T> {
        #[inline]
        fn describe() {
            <Self as Tsify>::JsType::describe()
        }
    }
    impl<T> IntoWasmAbi for GenericStruct<T>
    where
        Self: _serde::Serialize,
    {
        type Abi = <<Self as Tsify>::JsType as IntoWasmAbi>::Abi;
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
            <<Self as Tsify>::JsType as OptionIntoWasmAbi>::none()
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
            <<Self as Tsify>::JsType as OptionFromWasmAbi>::is_none(abi)
        }
    }
};
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct GenericNewtype<T>(T);
#[automatically_derived]
const _: () = {
    extern crate serde as _serde;
    use wasm_bindgen::{
        convert::{FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi},
        describe::WasmDescribe,
        prelude::*,
    };
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(typescript_type = "GenericNewtype")]
        pub type JsType;
    }
    impl<T> Tsify for GenericNewtype<T> {
        type JsType = JsType;
        const DECL: &'static str = "export type GenericNewtype<T> = T;";
    }
    #[wasm_bindgen(typescript_custom_section)]
    const TS_APPEND_CONTENT: &'static str = "export type GenericNewtype<T> = T;";
    impl<T> WasmDescribe for GenericNewtype<T> {
        #[inline]
        fn describe() {
            <Self as Tsify>::JsType::describe()
        }
    }
    impl<T> IntoWasmAbi for GenericNewtype<T>
    where
        Self: _serde::Serialize,
    {
        type Abi = <<Self as Tsify>::JsType as IntoWasmAbi>::Abi;
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
            <<Self as Tsify>::JsType as OptionIntoWasmAbi>::none()
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
            <<Self as Tsify>::JsType as OptionFromWasmAbi>::is_none(abi)
        }
    }
};
