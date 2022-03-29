use tsify::Tsify;
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum GenericEnum<T, U> {
    Unit,
    NewType(T),
    Seq(T, U),
    Map { x: T, y: U },
}
#[automatically_derived]
impl<T, U> Tsify for GenericEnum<T, U> {
    const DECL : & 'static str = "export type GenericEnum<T, U> = \"Unit\" | { NewType: T } | { Seq: [T, U] } | { Map: { x: T; y: U } };" ;
}
#[automatically_derived]
const _: () = {
    use wasm_bindgen::{
        convert::{FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi},
        describe::WasmDescribe,
        prelude::*,
    };
    #[wasm_bindgen(typescript_custom_section)]
    const TS_APPEND_CONTENT : & 'static str = "export type GenericEnum<T, U> = \"Unit\" | { NewType: T } | { Seq: [T, U] } | { Map: { x: T; y: U } };" ;
    impl<T, U> WasmDescribe for GenericEnum<T, U> {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_EXTERNREF);
            inform(11u32);
            inform(71u32);
            inform(101u32);
            inform(110u32);
            inform(101u32);
            inform(114u32);
            inform(105u32);
            inform(99u32);
            inform(69u32);
            inform(110u32);
            inform(117u32);
            inform(109u32);
        }
    }
    extern crate serde as _serde;
    impl<T, U> IntoWasmAbi for GenericEnum<T, U>
    where
        Self: _serde::Serialize,
    {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            JsValue::from_serde(&self).unwrap_throw().into_abi()
        }
    }
    impl<T, U> OptionIntoWasmAbi for GenericEnum<T, U>
    where
        Self: _serde::Serialize,
    {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<T, U> FromWasmAbi for GenericEnum<T, U>
    where
        Self: _serde::de::DeserializeOwned,
    {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            JsValue::from_abi(js).into_serde().unwrap_throw()
        }
    }
    impl<T, U> OptionFromWasmAbi for GenericEnum<T, U>
    where
        Self: _serde::de::DeserializeOwned,
    {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
};
