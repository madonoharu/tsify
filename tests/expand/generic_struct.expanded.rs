use tsify::Tsify;
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct GenericStruct<T> {
    x: T,
}
const _: () = {
    extern crate serde as _serde;
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
    impl<T> Tsify for GenericStruct<T> {
        type JsType = JsType;
        const DECL: &'static str = "export interface GenericStruct<T> {\n    x: T;\n}";
        const SERIALIZATION_CONFIG: tsify::SerializationConfig = tsify::SerializationConfig {
            missing_as_null: false,
            hashmap_as_object: false,
            large_number_types_as_bigints: false,
        };
    }
    #[automatically_derived]
    impl<T> WasmDescribe for GenericStruct<T> {
        #[inline]
        fn describe() {
            <Self as Tsify>::JsType::describe()
        }
    }
    #[automatically_derived]
    impl<T> WasmDescribeVector for GenericStruct<T> {
        #[inline]
        fn describe_vector() {
            <Self as Tsify>::JsType::describe_vector()
        }
    }
    #[automatically_derived]
    impl<T> IntoWasmAbi for &GenericStruct<T>
    where
        GenericStruct<T>: _serde::Serialize,
    {
        type Abi = <JsType as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            match self.into_js() {
                Ok(js) => js.into_abi(),
                Err(err) => {
                    let loc = core::panic::Location::caller();
                    let msg = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "(Converting type failed) {0} ({1}:{2}:{3})", err, loc
                                .file(), loc.line(), loc.column(),
                            ),
                        )
                    });
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
            }
        }
    }
    #[automatically_derived]
    impl<T> IntoWasmAbi for GenericStruct<T>
    where
        GenericStruct<T>: _serde::Serialize,
    {
        type Abi = <JsType as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self).into_abi()
        }
    }
    #[automatically_derived]
    impl<T> OptionIntoWasmAbi for GenericStruct<T>
    where
        GenericStruct<T>: _serde::Serialize,
    {
        #[inline]
        fn none() -> Self::Abi {
            <JsType as OptionIntoWasmAbi>::none()
        }
    }
    #[automatically_derived]
    impl<T> From<GenericStruct<T>> for JsValue
    where
        GenericStruct<T>: _serde::Serialize,
    {
        #[inline]
        fn from(value: GenericStruct<T>) -> Self {
            match value.into_js() {
                Ok(js) => js.into(),
                Err(err) => {
                    let loc = core::panic::Location::caller();
                    let msg = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "(Converting type failed) {0} ({1}:{2}:{3})", err, loc
                                .file(), loc.line(), loc.column(),
                            ),
                        )
                    });
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
            }
        }
    }
    #[automatically_derived]
    impl<T> VectorIntoWasmAbi for GenericStruct<T>
    where
        GenericStruct<T>: _serde::Serialize,
    {
        type Abi = <JsType as VectorIntoWasmAbi>::Abi;
        #[inline]
        fn vector_into_abi(vector: Box<[Self]>) -> Self::Abi {
            let values = vector
                .iter()
                .map(|value| match value.into_js() {
                    Ok(js) => js.into(),
                    Err(err) => {
                        let loc = core::panic::Location::caller();
                        let msg = ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!(
                                    "(Converting type failed) {0} ({1}:{2}:{3})", err, loc
                                    .file(), loc.line(), loc.column(),
                                ),
                            )
                        });
                        {
                            ::core::panicking::panic_display(&msg);
                        };
                    }
                })
                .collect();
            JsValue::vector_into_abi(values)
        }
    }
    #[automatically_derived]
    impl<T> FromWasmAbi for GenericStruct<T>
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
    #[automatically_derived]
    impl<T> OptionFromWasmAbi for GenericStruct<T>
    where
        Self: _serde::de::DeserializeOwned,
    {
        #[inline]
        fn is_none(js: &Self::Abi) -> bool {
            <JsType as OptionFromWasmAbi>::is_none(js)
        }
    }
    pub struct SelfOwner<T>(T);
    #[automatically_derived]
    impl<T> ::core::ops::Deref for SelfOwner<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    #[automatically_derived]
    impl<T> RefFromWasmAbi for GenericStruct<T>
    where
        Self: _serde::de::DeserializeOwned,
    {
        type Abi = <JsType as RefFromWasmAbi>::Abi;
        type Anchor = SelfOwner<Self>;
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let result = Self::from_js(&*JsType::ref_from_abi(js));
            if let Err(err) = result {
                wasm_bindgen::throw_str(err.to_string().as_ref());
            }
            SelfOwner(result.unwrap_throw())
        }
    }
    #[automatically_derived]
    impl<T> VectorFromWasmAbi for GenericStruct<T>
    where
        Self: _serde::de::DeserializeOwned,
    {
        type Abi = <JsType as VectorFromWasmAbi>::Abi;
        #[inline]
        unsafe fn vector_from_abi(js: Self::Abi) -> Box<[Self]> {
            JsValue::vector_from_abi(js)
                .into_iter()
                .map(|value| {
                    let result = Self::from_js(value);
                    if let Err(err) = result {
                        wasm_bindgen::throw_str(err.to_string().as_ref());
                    }
                    result.unwrap_throw()
                })
                .collect()
        }
    }
    #[allow(non_upper_case_globals)]
    const _: () = {
        #[deprecated(
            note = "into_wasm_abi/from_wasm_abi are deprecated as they cause memory leaks (https://github.com/madonoharu/tsify/issues/65). Consider using `tsify::Ts` instead."
        )]
        const _x: () = ();
        _x
    };
};
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct GenericNewtype<T>(T);
const _: () = {
    extern crate serde as _serde;
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
    impl<T> Tsify for GenericNewtype<T> {
        type JsType = JsType;
        const DECL: &'static str = "export type GenericNewtype<T> = T;";
        const SERIALIZATION_CONFIG: tsify::SerializationConfig = tsify::SerializationConfig {
            missing_as_null: false,
            hashmap_as_object: false,
            large_number_types_as_bigints: false,
        };
    }
    #[automatically_derived]
    impl<T> WasmDescribe for GenericNewtype<T> {
        #[inline]
        fn describe() {
            <Self as Tsify>::JsType::describe()
        }
    }
    #[automatically_derived]
    impl<T> WasmDescribeVector for GenericNewtype<T> {
        #[inline]
        fn describe_vector() {
            <Self as Tsify>::JsType::describe_vector()
        }
    }
    #[automatically_derived]
    impl<T> IntoWasmAbi for &GenericNewtype<T>
    where
        GenericNewtype<T>: _serde::Serialize,
    {
        type Abi = <JsType as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            match self.into_js() {
                Ok(js) => js.into_abi(),
                Err(err) => {
                    let loc = core::panic::Location::caller();
                    let msg = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "(Converting type failed) {0} ({1}:{2}:{3})", err, loc
                                .file(), loc.line(), loc.column(),
                            ),
                        )
                    });
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
            }
        }
    }
    #[automatically_derived]
    impl<T> IntoWasmAbi for GenericNewtype<T>
    where
        GenericNewtype<T>: _serde::Serialize,
    {
        type Abi = <JsType as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self).into_abi()
        }
    }
    #[automatically_derived]
    impl<T> OptionIntoWasmAbi for GenericNewtype<T>
    where
        GenericNewtype<T>: _serde::Serialize,
    {
        #[inline]
        fn none() -> Self::Abi {
            <JsType as OptionIntoWasmAbi>::none()
        }
    }
    #[automatically_derived]
    impl<T> From<GenericNewtype<T>> for JsValue
    where
        GenericNewtype<T>: _serde::Serialize,
    {
        #[inline]
        fn from(value: GenericNewtype<T>) -> Self {
            match value.into_js() {
                Ok(js) => js.into(),
                Err(err) => {
                    let loc = core::panic::Location::caller();
                    let msg = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "(Converting type failed) {0} ({1}:{2}:{3})", err, loc
                                .file(), loc.line(), loc.column(),
                            ),
                        )
                    });
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
            }
        }
    }
    #[automatically_derived]
    impl<T> VectorIntoWasmAbi for GenericNewtype<T>
    where
        GenericNewtype<T>: _serde::Serialize,
    {
        type Abi = <JsType as VectorIntoWasmAbi>::Abi;
        #[inline]
        fn vector_into_abi(vector: Box<[Self]>) -> Self::Abi {
            let values = vector
                .iter()
                .map(|value| match value.into_js() {
                    Ok(js) => js.into(),
                    Err(err) => {
                        let loc = core::panic::Location::caller();
                        let msg = ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!(
                                    "(Converting type failed) {0} ({1}:{2}:{3})", err, loc
                                    .file(), loc.line(), loc.column(),
                                ),
                            )
                        });
                        {
                            ::core::panicking::panic_display(&msg);
                        };
                    }
                })
                .collect();
            JsValue::vector_into_abi(values)
        }
    }
    #[automatically_derived]
    impl<T> FromWasmAbi for GenericNewtype<T>
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
    #[automatically_derived]
    impl<T> OptionFromWasmAbi for GenericNewtype<T>
    where
        Self: _serde::de::DeserializeOwned,
    {
        #[inline]
        fn is_none(js: &Self::Abi) -> bool {
            <JsType as OptionFromWasmAbi>::is_none(js)
        }
    }
    pub struct SelfOwner<T>(T);
    #[automatically_derived]
    impl<T> ::core::ops::Deref for SelfOwner<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    #[automatically_derived]
    impl<T> RefFromWasmAbi for GenericNewtype<T>
    where
        Self: _serde::de::DeserializeOwned,
    {
        type Abi = <JsType as RefFromWasmAbi>::Abi;
        type Anchor = SelfOwner<Self>;
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let result = Self::from_js(&*JsType::ref_from_abi(js));
            if let Err(err) = result {
                wasm_bindgen::throw_str(err.to_string().as_ref());
            }
            SelfOwner(result.unwrap_throw())
        }
    }
    #[automatically_derived]
    impl<T> VectorFromWasmAbi for GenericNewtype<T>
    where
        Self: _serde::de::DeserializeOwned,
    {
        type Abi = <JsType as VectorFromWasmAbi>::Abi;
        #[inline]
        unsafe fn vector_from_abi(js: Self::Abi) -> Box<[Self]> {
            JsValue::vector_from_abi(js)
                .into_iter()
                .map(|value| {
                    let result = Self::from_js(value);
                    if let Err(err) = result {
                        wasm_bindgen::throw_str(err.to_string().as_ref());
                    }
                    result.unwrap_throw()
                })
                .collect()
        }
    }
    #[allow(non_upper_case_globals)]
    const _: () = {
        #[deprecated(
            note = "into_wasm_abi/from_wasm_abi are deprecated as they cause memory leaks (https://github.com/madonoharu/tsify/issues/65). Consider using `tsify::Ts` instead."
        )]
        const _x: () = ();
        _x
    };
};
