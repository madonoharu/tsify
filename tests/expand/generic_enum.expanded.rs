use tsify_next::Tsify;
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
    use tsify_next::Tsify;
    use wasm_bindgen::{
        convert::{
            FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi,
            RefFromWasmAbi,
        },
        describe::WasmDescribe, prelude::*,
    };
    #[automatically_derived]
    ///
    #[repr(transparent)]
    pub struct JsType {
        obj: wasm_bindgen::JsValue,
    }
    #[automatically_derived]
    const _: () = {
        use wasm_bindgen::convert::TryFromJsValue;
        use wasm_bindgen::convert::{IntoWasmAbi, FromWasmAbi};
        use wasm_bindgen::convert::{OptionIntoWasmAbi, OptionFromWasmAbi};
        use wasm_bindgen::convert::{RefFromWasmAbi, LongRefFromWasmAbi};
        use wasm_bindgen::describe::WasmDescribe;
        use wasm_bindgen::{JsValue, JsCast, JsObject};
        use wasm_bindgen::__rt::core;
        impl WasmDescribe for JsType {
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
        impl IntoWasmAbi for JsType {
            type Abi = <JsValue as IntoWasmAbi>::Abi;
            #[inline]
            fn into_abi(self) -> Self::Abi {
                self.obj.into_abi()
            }
        }
        impl OptionIntoWasmAbi for JsType {
            #[inline]
            fn none() -> Self::Abi {
                0
            }
        }
        impl<'a> OptionIntoWasmAbi for &'a JsType {
            #[inline]
            fn none() -> Self::Abi {
                0
            }
        }
        impl FromWasmAbi for JsType {
            type Abi = <JsValue as FromWasmAbi>::Abi;
            #[inline]
            unsafe fn from_abi(js: Self::Abi) -> Self {
                JsType {
                    obj: JsValue::from_abi(js).into(),
                }
            }
        }
        impl OptionFromWasmAbi for JsType {
            #[inline]
            fn is_none(abi: &Self::Abi) -> bool {
                *abi == 0
            }
        }
        impl<'a> IntoWasmAbi for &'a JsType {
            type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
            #[inline]
            fn into_abi(self) -> Self::Abi {
                (&self.obj).into_abi()
            }
        }
        impl RefFromWasmAbi for JsType {
            type Abi = <JsValue as RefFromWasmAbi>::Abi;
            type Anchor = core::mem::ManuallyDrop<JsType>;
            #[inline]
            unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
                let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
                core::mem::ManuallyDrop::new(JsType {
                    obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
                })
            }
        }
        impl LongRefFromWasmAbi for JsType {
            type Abi = <JsValue as LongRefFromWasmAbi>::Abi;
            type Anchor = JsType;
            #[inline]
            unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
                let tmp = <JsValue as LongRefFromWasmAbi>::long_ref_from_abi(js);
                JsType { obj: tmp.into() }
            }
        }
        impl From<JsValue> for JsType {
            #[inline]
            fn from(obj: JsValue) -> JsType {
                JsType { obj: obj.into() }
            }
        }
        impl AsRef<JsValue> for JsType {
            #[inline]
            fn as_ref(&self) -> &JsValue {
                self.obj.as_ref()
            }
        }
        impl AsRef<JsType> for JsType {
            #[inline]
            fn as_ref(&self) -> &JsType {
                self
            }
        }
        impl From<JsType> for JsValue {
            #[inline]
            fn from(obj: JsType) -> JsValue {
                obj.obj.into()
            }
        }
        impl JsCast for JsType {
            fn instanceof(val: &JsValue) -> bool {
                #[cfg(
                    not(
                        all(
                            target_arch = "wasm32",
                            not(any(target_os = "emscripten", target_os = "wasi"))
                        )
                    )
                )]
                unsafe fn __wbg_instanceof_JsType_1641ac20ec916ae7(_: u32) -> u32 {
                    {
                        ::std::rt::begin_panic(
                            "cannot check instanceof on non-wasm targets",
                        );
                    };
                }
                unsafe {
                    let idx = val.into_abi();
                    __wbg_instanceof_JsType_1641ac20ec916ae7(idx) != 0
                }
            }
            #[inline]
            fn unchecked_from_js(val: JsValue) -> Self {
                JsType { obj: val.into() }
            }
            #[inline]
            fn unchecked_from_js_ref(val: &JsValue) -> &Self {
                unsafe { &*(val as *const JsValue as *const JsType) }
            }
        }
        impl JsObject for JsType {}
    };
    #[automatically_derived]
    impl core::ops::Deref for JsType {
        type Target = wasm_bindgen::JsValue;
        #[inline]
        fn deref(&self) -> &wasm_bindgen::JsValue {
            &self.obj
        }
    }
    impl<T, U> Tsify for GenericEnum<T, U> {
        type JsType = JsType;
        const DECL: &'static str = "export type GenericEnum<T, U> = \"Unit\" | { NewType: T } | { Seq: [T, U] } | { Map: { x: T; y: U } };";
        const SERIALIZATION_CONFIG: tsify_next::SerializationConfig = tsify_next::SerializationConfig {
            missing_as_null: false,
            hashmap_as_object: false,
            large_number_types_as_bigints: false,
        };
    }
    impl<T, U> WasmDescribe for GenericEnum<T, U> {
        #[inline]
        fn describe() {
            <Self as Tsify>::JsType::describe()
        }
    }
    impl<T, U> IntoWasmAbi for GenericEnum<T, U>
    where
        GenericEnum<T, U>: _serde::Serialize,
    {
        type Abi = <JsType as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.into_js().unwrap_throw().into_abi()
        }
    }
    impl<T, U> OptionIntoWasmAbi for GenericEnum<T, U>
    where
        GenericEnum<T, U>: _serde::Serialize,
    {
        #[inline]
        fn none() -> Self::Abi {
            <JsType as OptionIntoWasmAbi>::none()
        }
    }
    impl<T, U> From<GenericEnum<T, U>> for JsValue
    where
        GenericEnum<T, U>: _serde::Serialize,
    {
        #[inline]
        fn from(value: GenericEnum<T, U>) -> Self {
            value.into_js().unwrap_throw().into()
        }
    }
    impl<T, U> FromWasmAbi for GenericEnum<T, U>
    where
        Self: _serde::de::DeserializeOwned,
    {
        type Abi = <JsType as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            let result = Self::from_js(&JsType::from_abi(js));
            if let Err(err) = result {
                wasm_bindgen::throw_str(err.to_string().as_ref());
            }
            result.unwrap_throw()
        }
    }
    impl<T, U> OptionFromWasmAbi for GenericEnum<T, U>
    where
        Self: _serde::de::DeserializeOwned,
    {
        #[inline]
        fn is_none(js: &Self::Abi) -> bool {
            <JsType as OptionFromWasmAbi>::is_none(js)
        }
    }
    pub struct SelfOwner<T>(T);
    impl<T> ::core::ops::Deref for SelfOwner<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<T, U> RefFromWasmAbi for GenericEnum<T, U>
    where
        Self: _serde::de::DeserializeOwned,
    {
        type Abi = <JsType as RefFromWasmAbi>::Abi;
        type Anchor = SelfOwner<Self>;
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            SelfOwner(Self::from_abi(js))
        }
    }
};
