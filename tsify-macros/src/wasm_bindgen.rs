use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::parse_quote;

use crate::decl::TsValueEnumDecl;
use crate::typescript::ToStringWithIndent;
use crate::{container::Container, decl::Decl};

pub fn expand(cont: &Container, decl: Decl) -> TokenStream {
    let attrs = &cont.attrs;
    let ident = cont.ident();

    let decl_str = decl.to_string();
    let generics = cont.generics_without_defaults();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let typescript_custom_section = quote! {
        #[wasm_bindgen(typescript_custom_section)]
        const TS_APPEND_CONTENT: &'static str = #decl_str;
    };

    let wasm_abi = attrs.into_wasm_abi || attrs.from_wasm_abi;

    let wasm_describe = wasm_abi.then(|| {
        quote! {
            #[automatically_derived]
            impl #impl_generics WasmDescribe for #ident #ty_generics #where_clause {
                #[inline]
                fn describe() {
                    <Self as Tsify>::JsType::describe()
                }
            }

            #[automatically_derived]
            impl #impl_generics WasmDescribeVector for #ident #ty_generics #where_clause {
                #[inline]
                fn describe_vector() {
                    <Self as Tsify>::JsType::describe_vector()
                }
            }
        }
    });

    let use_serde = wasm_abi.then(|| match cont.serde_container.attrs.custom_serde_path() {
        Some(path) => quote! {
            use #path as _serde;
        },
        None => quote! {
            extern crate serde as _serde;
        },
    });

    let into_wasm_abi = attrs.into_wasm_abi.then(|| expand_into_wasm_abi(cont));
    let from_wasm_abi = attrs.from_wasm_abi.then(|| expand_from_wasm_abi(cont));

    let inline_enum = match &decl {
        Decl::TsValueEnum(value) => Some(expand_inline_enum(value)),
        Decl::TsEnum(value) => value
            .discriminants
            .as_ref()
            .map(|value| expand_inline_enum(&value)),
        _ => None,
    };

    let typescript_type = decl.id();

    let missing_as_null = attrs.ty_config.missing_as_null;
    let hashmap_as_object = attrs.ty_config.hashmap_as_object;
    let large_number_types_as_bigints = attrs.ty_config.large_number_types_as_bigints;

    quote! {
        const _: () = {
            #use_serde
            use tsify::Tsify;
            use wasm_bindgen::{
                convert::{FromWasmAbi, VectorFromWasmAbi, IntoWasmAbi, VectorIntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi, RefFromWasmAbi},
                describe::WasmDescribe, describe::WasmDescribeVector,
                prelude::*,
            };


            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(typescript_type = #typescript_type)]
                pub type JsType;
            }

            #[automatically_derived]
            impl #impl_generics Tsify for #ident #ty_generics #where_clause {
                type JsType = JsType;
                const DECL: &'static str = #decl_str;
                const SERIALIZATION_CONFIG: tsify::SerializationConfig = tsify::SerializationConfig {
                    missing_as_null: #missing_as_null,
                    hashmap_as_object: #hashmap_as_object,
                    large_number_types_as_bigints: #large_number_types_as_bigints,
                };
            }

            #typescript_custom_section
            #wasm_describe
            #into_wasm_abi
            #from_wasm_abi
            #inline_enum
        };
    }
}

fn expand_into_wasm_abi(cont: &Container) -> TokenStream {
    let ident = cont.ident();
    let serde_path = cont.serde_container.attrs.serde_path();
    let mut generics = cont.generics_without_defaults();
    let borrowed_generics = generics.clone();

    generics
        .make_where_clause()
        .predicates
        .push(parse_quote!(#ident #borrowed_generics: #serde_path::Serialize));

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        #[automatically_derived]
        impl #impl_generics IntoWasmAbi for &#ident #ty_generics #where_clause {
            type Abi = <JsType as IntoWasmAbi>::Abi;

            #[inline]
            fn into_abi(self) -> Self::Abi {
                // wasm_bindgen doesn't forward the error message from the `into_js` result.
                // https://github.com/rustwasm/wasm-bindgen/issues/2732
                // Until that issue is fixed, we don't directly use `unwrap_throw()` and instead build our
                // own error message.
                // Convert to `self.into_js().unwrap_throw().into_abi()` when fixed.
                match self.into_js() {
                    Ok(js) => js.into_abi(),
                    Err(err) => {
                        let loc = core::panic::Location::caller();
                        let msg = format!("(Converting type failed) {} ({}:{}:{})", err, loc.file(), loc.line(), loc.column());
                        // In theory, `wasm_bindgen::throw_str(&msg)` should work, but the error emitted by `wasm_bindgen::throw_str` cannot be picked up by `#[should_panic(expect = ...)]` in tests, so we use a regular panic.
                        panic!("{}", msg);
                    }
                }
            }
        }

        #[automatically_derived]
        impl #impl_generics IntoWasmAbi for #ident #ty_generics #where_clause {
            type Abi = <JsType as IntoWasmAbi>::Abi;

            #[inline]
            fn into_abi(self) -> Self::Abi {
                (&self).into_abi()
            }
        }

        #[automatically_derived]
        impl #impl_generics OptionIntoWasmAbi for #ident #ty_generics #where_clause {
            #[inline]
            fn none() -> Self::Abi {
                <JsType as OptionIntoWasmAbi>::none()
            }
        }

        #[automatically_derived]
        impl #impl_generics From<#ident #ty_generics> for JsValue #where_clause {
            #[inline]
            fn from(value: #ident #ty_generics) -> Self {
                // wasm_bindgen doesn't forward the error message from the `into_js` result.
                // https://github.com/rustwasm/wasm-bindgen/issues/2732
                // Until that issue is fixed, we don't directly use `unwrap_throw()` and instead build our
                // own error message.
                // Convert to `value.into_js().unwrap_throw().into()` when fixed.
                match value.into_js() {
                    Ok(js) => js.into(),
                    Err(err) => {
                        let loc = core::panic::Location::caller();
                        let msg = format!("(Converting type failed) {} ({}:{}:{})", err, loc.file(), loc.line(), loc.column());
                        // In theory, `wasm_bindgen::throw_str(&msg)` should work, but the error emitted by `wasm_bindgen::throw_str` cannot be picked up by `#[should_panic(expect = ...)]` in tests, so we use a regular panic.
                        panic!("{}", msg);
                    }
                }
            }
        }

        #[automatically_derived]
        impl #impl_generics VectorIntoWasmAbi for #ident #ty_generics #where_clause {
            type Abi = <JsType as VectorIntoWasmAbi>::Abi;

            #[inline]
            fn vector_into_abi(vector: Box<[Self]>) -> Self::Abi {
                let values = vector
                    .iter()
                    .map(|value|
                        // wasm_bindgen doesn't forward the error message from the `into_js` result.
                        // https://github.com/rustwasm/wasm-bindgen/issues/2732
                        // Until that issue is fixed, we don't directly use `unwrap_throw()` and instead build our
                        // own error message.
                        match value.into_js() {
                        Ok(js) => js.into(),
                        Err(err) => {
                            let loc = core::panic::Location::caller();
                            let msg = format!("(Converting type failed) {} ({}:{}:{})", err, loc.file(), loc.line(), loc.column());
                            // In theory, `wasm_bindgen::throw_str(&msg)` should work, but the error emitted by `wasm_bindgen::throw_str` cannot be picked up by `#[should_panic(expect = ...)]` in tests, so we use a regular panic.
                            panic!("{}", msg);
                        }
                    })
                    .collect();

                JsValue::vector_into_abi(values)
            }
        }
    }
}

fn expand_from_wasm_abi(cont: &Container) -> TokenStream {
    let ident = cont.ident();
    let serde_path = cont.serde_container.attrs.serde_path();

    let mut generics = cont.generics_without_defaults();

    generics
        .make_where_clause()
        .predicates
        .push(parse_quote!(Self: #serde_path::de::DeserializeOwned));

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        #[automatically_derived]
        impl #impl_generics FromWasmAbi for #ident #ty_generics #where_clause {
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
        impl #impl_generics OptionFromWasmAbi for #ident #ty_generics #where_clause {
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
        impl #impl_generics RefFromWasmAbi for #ident #ty_generics #where_clause {
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
        impl #impl_generics VectorFromWasmAbi for #ident #ty_generics #where_clause {
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
    }
}

fn expand_inline_enum(enum_value: &TsValueEnumDecl) -> TokenStream {
    let ident_string = &enum_value.id;

    let members = enum_value
        .members
        .iter()
        .map(|member| {
            let prefix = ident_string.to_string_with_indent(4);
            format!("{}[\"{}\"]{};", prefix, member.id, member.value)
        })
        .collect::<Vec<_>>()
        .join("\n");

    let js = format!(
        "export var {0};\n(function ({0}) {{\n{1}\n}})({0} || ({0} = {{}}));",
        ident_string, members
    );
    let js_lit = syn::LitStr::new(js.as_str(), Span::call_site());
    let ident = syn::Ident::new(ident_string, Span::call_site());
    let fn_name = format_ident!("__inline_enum_tsify_helper_{}", ident);

    quote! {
        #[wasm_bindgen::prelude::wasm_bindgen(inline_js = #js_lit)]
        extern "C" {
            #[wasm_bindgen(reexport, js_name = #ident, skip_typescript)]
            fn #fn_name();
        }
    }
}
