use tsify_next::Tsify;
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum GenericEnum<T, U> {
    Unit,
    NewType(T),
    Seq(T, U),
    Map { x: T, y: U },
}
const _: () = {
    extern crate serde as _serde;
    use tsify_next::Tsify;
    use wasm_bindgen::{
        convert::{
            FromWasmAbi, VectorFromWasmAbi, IntoWasmAbi, VectorIntoWasmAbi,
            OptionFromWasmAbi, OptionIntoWasmAbi, RefFromWasmAbi,
        },
        describe::WasmDescribe, describe::WasmDescribeVector, prelude::*,
    };
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(typescript_type = "GenericEnum")]
        pub type JsType;
    }
    #[automatically_derived]
    impl<T, U> Tsify for GenericEnum<T, U> {
        type JsType = JsType;
        const DECL: &'static str = "export type GenericEnum<T, U> = \"Unit\" | { NewType: T } | { Seq: [T, U] } | { Map: { x: T; y: U } };";
        const SERIALIZATION_CONFIG: tsify_next::SerializationConfig = tsify_next::SerializationConfig {
            missing_as_null: false,
            hashmap_as_object: false,
            large_number_types_as_bigints: false,
        };
    }
    #[wasm_bindgen(typescript_custom_section)]
    const TS_APPEND_CONTENT: &'static str = "export type GenericEnum<T, U> = \"Unit\" | { NewType: T } | { Seq: [T, U] } | { Map: { x: T; y: U } };";
    #[automatically_derived]
    impl<T, U> WasmDescribe for GenericEnum<T, U> {
        #[inline]
        fn describe() {
            <Self as Tsify>::JsType::describe()
        }
    }
    #[automatically_derived]
    impl<T, U> WasmDescribeVector for GenericEnum<T, U> {
        #[inline]
        fn describe_vector() {
            <Self as Tsify>::JsType::describe_vector()
        }
    }
    #[automatically_derived]
    impl<T, U> IntoWasmAbi for GenericEnum<T, U>
    where
        GenericEnum<T, U>: _serde::Serialize,
    {
        type Abi = <JsType as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            match self.into_js() {
                Ok(js) => js.into_abi(),
                Err(err) => {
                    let loc = core::panic::Location::caller();
                    let msg = ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "(Converting type failed) {0} ({1}:{2}:{3})", err, loc
                                .file(), loc.line(), loc.column(),
                            ),
                        );
                        res
                    });
                    {
                        #[cold]
                        #[track_caller]
                        #[inline(never)]
                        #[rustc_const_panic_str]
                        #[rustc_do_not_const_check]
                        const fn panic_cold_display<T: ::core::fmt::Display>(
                            arg: &T,
                        ) -> ! {
                            ::core::panicking::panic_display(arg)
                        }
                        panic_cold_display(&msg);
                    };
                }
            }
        }
    }
    #[automatically_derived]
    impl<T, U> OptionIntoWasmAbi for GenericEnum<T, U>
    where
        GenericEnum<T, U>: _serde::Serialize,
    {
        #[inline]
        fn none() -> Self::Abi {
            <JsType as OptionIntoWasmAbi>::none()
        }
    }
    #[automatically_derived]
    impl<T, U> From<GenericEnum<T, U>> for JsValue
    where
        GenericEnum<T, U>: _serde::Serialize,
    {
        #[inline]
        fn from(value: GenericEnum<T, U>) -> Self {
            match value.into_js() {
                Ok(js) => js.into(),
                Err(err) => {
                    let loc = core::panic::Location::caller();
                    let msg = ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "(Converting type failed) {0} ({1}:{2}:{3})", err, loc
                                .file(), loc.line(), loc.column(),
                            ),
                        );
                        res
                    });
                    {
                        #[cold]
                        #[track_caller]
                        #[inline(never)]
                        #[rustc_const_panic_str]
                        #[rustc_do_not_const_check]
                        const fn panic_cold_display<T: ::core::fmt::Display>(
                            arg: &T,
                        ) -> ! {
                            ::core::panicking::panic_display(arg)
                        }
                        panic_cold_display(&msg);
                    };
                }
            }
        }
    }
    #[automatically_derived]
    impl<T, U> VectorIntoWasmAbi for GenericEnum<T, U>
    where
        GenericEnum<T, U>: _serde::Serialize,
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
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "(Converting type failed) {0} ({1}:{2}:{3})", err, loc
                                    .file(), loc.line(), loc.column(),
                                ),
                            );
                            res
                        });
                        {
                            #[cold]
                            #[track_caller]
                            #[inline(never)]
                            #[rustc_const_panic_str]
                            #[rustc_do_not_const_check]
                            const fn panic_cold_display<T: ::core::fmt::Display>(
                                arg: &T,
                            ) -> ! {
                                ::core::panicking::panic_display(arg)
                            }
                            panic_cold_display(&msg);
                        };
                    }
                })
                .collect();
            JsValue::vector_into_abi(values)
        }
    }
    #[automatically_derived]
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
    #[automatically_derived]
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
    #[automatically_derived]
    impl<T> ::core::ops::Deref for SelfOwner<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    #[automatically_derived]
    impl<T, U> RefFromWasmAbi for GenericEnum<T, U>
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
    impl<T, U> VectorFromWasmAbi for GenericEnum<T, U>
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
};
