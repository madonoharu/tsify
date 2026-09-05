use tsify::Tsify;
#[tsify(rename = "RenamedWasmType")]
pub struct RustType {
    value: String,
}
const _: () = {
    use tsify::Tsify;
    use wasm_bindgen::{
        convert::{
            FromWasmAbi, VectorFromWasmAbi, IntoWasmAbi, VectorIntoWasmAbi,
            OptionFromWasmAbi, OptionIntoWasmAbi, RefFromWasmAbi,
        },
        describe::WasmDescribe, describe::WasmDescribeVector, prelude::*,
    };
    #[repr(transparent)]
    pub struct JsType {
        obj: wasm_bindgen::JsValue,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for JsType {
        #[inline]
        fn clone(&self) -> JsType {
            JsType {
                obj: ::core::clone::Clone::clone(&self.obj),
            }
        }
    }
    #[automatically_derived]
    const _: () = {
        use wasm_bindgen::convert::TryFromJsValue;
        use wasm_bindgen::convert::{IntoWasmAbi, FromWasmAbi};
        use wasm_bindgen::convert::{OptionIntoWasmAbi, OptionFromWasmAbi};
        use wasm_bindgen::convert::{RefFromWasmAbi, LongRefFromWasmAbi};
        use wasm_bindgen::describe::WasmDescribe;
        use wasm_bindgen::{JsValue, JsCast};
        use wasm_bindgen::__rt::{core, marker::ErasableGeneric};
        #[automatically_derived]
        impl WasmDescribe for JsType {
            fn describe() {
                use wasm_bindgen::describe::*;
                inform(NAMED_EXTERNREF);
                inform(15u32);
                inform(82u32);
                inform(101u32);
                inform(110u32);
                inform(97u32);
                inform(109u32);
                inform(101u32);
                inform(100u32);
                inform(87u32);
                inform(97u32);
                inform(115u32);
                inform(109u32);
                inform(84u32);
                inform(121u32);
                inform(112u32);
                inform(101u32);
            }
        }
        #[automatically_derived]
        impl IntoWasmAbi for JsType {
            type Abi = <JsValue as IntoWasmAbi>::Abi;
            #[inline]
            fn into_abi(self) -> Self::Abi {
                self.obj.into_abi()
            }
        }
        #[automatically_derived]
        impl OptionIntoWasmAbi for JsType {
            #[inline]
            fn none() -> Self::Abi {
                0
            }
        }
        #[automatically_derived]
        impl<'__wbg_ref> OptionIntoWasmAbi for &'__wbg_ref JsType {
            #[inline]
            fn none() -> Self::Abi {
                0
            }
        }
        #[automatically_derived]
        impl FromWasmAbi for JsType {
            type Abi = <JsValue as FromWasmAbi>::Abi;
            #[inline]
            unsafe fn from_abi(js: Self::Abi) -> Self {
                JsType {
                    obj: JsValue::from_abi(js).into(),
                }
            }
        }
        #[automatically_derived]
        impl OptionFromWasmAbi for JsType {
            #[inline]
            fn is_none(abi: &Self::Abi) -> bool {
                *abi == 0
            }
        }
        #[automatically_derived]
        impl<'__wbg_ref> IntoWasmAbi for &'__wbg_ref JsType {
            type Abi = <&'__wbg_ref JsValue as IntoWasmAbi>::Abi;
            #[inline]
            fn into_abi(self) -> Self::Abi {
                (&self.obj).into_abi()
            }
        }
        #[automatically_derived]
        impl RefFromWasmAbi for JsType {
            type Abi = <JsValue as RefFromWasmAbi>::Abi;
            type Anchor = wasm_bindgen::__rt::core::mem::ManuallyDrop<JsType>;
            #[inline]
            unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
                let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
                wasm_bindgen::__rt::core::mem::ManuallyDrop::new(JsType {
                    obj: wasm_bindgen::__rt::core::mem::ManuallyDrop::into_inner(tmp)
                        .into(),
                })
            }
        }
        #[automatically_derived]
        impl LongRefFromWasmAbi for JsType {
            type Abi = <JsValue as LongRefFromWasmAbi>::Abi;
            type Anchor = JsType;
            #[inline]
            unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
                let tmp = <JsValue as LongRefFromWasmAbi>::long_ref_from_abi(js);
                JsType { obj: tmp.into() }
            }
        }
        #[automatically_derived]
        impl AsRef<JsValue> for JsType {
            #[inline]
            fn as_ref(&self) -> &JsValue {
                self.obj.as_ref()
            }
        }
        #[automatically_derived]
        impl AsRef<JsType> for JsType {
            #[inline]
            fn as_ref(&self) -> &JsType {
                self
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::IntoJsGeneric for JsType
        where
            JsType: wasm_bindgen::JsGeneric,
        {
            type JsCanon = JsType;
            #[inline]
            fn to_js(self) -> JsType {
                unsafe {
                    wasm_bindgen::__rt::core::mem::transmute_copy(
                        &wasm_bindgen::__rt::core::mem::ManuallyDrop::new(self),
                    )
                }
            }
        }
        #[automatically_derived]
        impl From<JsValue> for JsType {
            #[inline]
            fn from(obj: JsValue) -> Self {
                JsType { obj: obj.into() }
            }
        }
        #[automatically_derived]
        impl From<JsType> for JsValue {
            #[inline]
            fn from(obj: JsType) -> JsValue {
                obj.obj.into()
            }
        }
        #[automatically_derived]
        impl JsCast for JsType {
            fn instanceof(val: &JsValue) -> bool {
                unsafe fn __wbg_instanceof_JsType_07a4c56f90e9a47b(_: u32) -> u32 {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("cannot check instanceof on non-wasm targets"),
                        );
                    };
                }
                unsafe {
                    let idx = val.into_abi();
                    __wbg_instanceof_JsType_07a4c56f90e9a47b(idx) != 0
                }
            }
            #[inline]
            fn unchecked_from_js(val: JsValue) -> Self {
                JsType { obj: val.into() }
            }
            #[inline]
            fn unchecked_from_js_ref(val: &JsValue) -> &Self {
                unsafe { &*(val as *const JsValue as *const Self) }
            }
        }
        unsafe impl ErasableGeneric for JsType {
            type Repr = JsValue;
        }
    };
    #[automatically_derived]
    impl wasm_bindgen::sys::Promising for JsType {
        type Resolution = JsType;
    }
    #[automatically_derived]
    impl wasm_bindgen::__rt::core::ops::Deref for JsType {
        type Target = wasm_bindgen::JsValue;
        #[inline]
        fn deref(&self) -> &wasm_bindgen::JsValue {
            &self.obj
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::UpcastFrom<JsType> for wasm_bindgen::JsValue {}
    #[automatically_derived]
    impl wasm_bindgen::convert::UpcastFrom<JsType>
    for wasm_bindgen::sys::JsOption<wasm_bindgen::JsValue> {}
    #[automatically_derived]
    impl wasm_bindgen::convert::UpcastFrom<JsType>
    for wasm_bindgen::sys::JsNullable<wasm_bindgen::JsValue> {}
    #[automatically_derived]
    impl wasm_bindgen::convert::UpcastFrom<JsType> for JsType {}
    #[automatically_derived]
    impl wasm_bindgen::convert::UpcastFrom<JsType>
    for wasm_bindgen::sys::JsOption<JsType> {}
    #[automatically_derived]
    impl wasm_bindgen::convert::UpcastFrom<JsType>
    for wasm_bindgen::sys::JsNullable<JsType> {}
    #[automatically_derived]
    impl Tsify for RustType {
        type JsType = JsType;
        const DECL: &'static str = "export interface RenamedWasmType {\n    value: string;\n}";
        const SERIALIZATION_CONFIG: tsify::SerializationConfig = tsify::SerializationConfig {
            missing_as_null: false,
            hashmap_as_object: false,
            large_number_types_as_bigints: false,
        };
    }
};
