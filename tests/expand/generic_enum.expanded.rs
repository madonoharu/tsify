use tsify::Tsify;
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum GenericEnum<T, U> {
    Unit,
    NewType(T),
    Seq(T, U),
    Map { x: T, y: U },
}
#[automatically_derived]
const _: () = {
    extern crate serde as _serde;
    use wasm_bindgen::{
        convert::{FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi},
        describe::WasmDescribe, prelude::*,
    };
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(typescript_type = "GenericEnum")]
        pub type JsType;
    }
    impl<T, U> Tsify for GenericEnum<T, U> {
        type JsType = JsType;
        const DECL: &'static str = "declare namespace GenericEnum {\n    export type Unit = \"Unit\";\n    export type NewType<T> = { NewType: T };\n    export type Seq<T, U> = { Seq: [T, U] };\n    export type Map<T, U> = { Map: { x: T; y: U } };\n}\n\nexport type GenericEnum<T, U> = GenericEnum.Unit | GenericEnum.NewType<T> | GenericEnum.Seq<T, U> | GenericEnum.Map<T, U>;";
    }
    #[wasm_bindgen(typescript_custom_section)]
    const TS_APPEND_CONTENT: &'static str = "declare namespace GenericEnum {\n    export type Unit = \"Unit\";\n    export type NewType<T> = { NewType: T };\n    export type Seq<T, U> = { Seq: [T, U] };\n    export type Map<T, U> = { Map: { x: T; y: U } };\n}\n\nexport type GenericEnum<T, U> = GenericEnum.Unit | GenericEnum.NewType<T> | GenericEnum.Seq<T, U> | GenericEnum.Map<T, U>;";
    impl<T, U> WasmDescribe for GenericEnum<T, U> {
        #[inline]
        fn describe() {
            <Self as Tsify>::JsType::describe()
        }
    }
    impl<T, U> IntoWasmAbi for GenericEnum<T, U>
    where
        Self: _serde::Serialize,
    {
        type Abi = <<Self as Tsify>::JsType as IntoWasmAbi>::Abi;
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
            <<Self as Tsify>::JsType as OptionIntoWasmAbi>::none()
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
            <<Self as Tsify>::JsType as OptionFromWasmAbi>::is_none(abi)
        }
    }
};
