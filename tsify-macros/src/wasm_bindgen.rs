use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use quote::quote_spanned;
use syn::parse_quote;

use crate::{container::Container, decl::Decl};

fn mark_deprecated(span: Span, note: &str) -> TokenStream {
    quote_spanned!(span =>
        #[allow(non_upper_case_globals)]
        const _: () = {
            #[deprecated(note = #note)]
            const _x: () = ();
            _x
        };
    )
}

pub fn expand(cont: &Container, decl: Decl) -> TokenStream {
    let attrs = &cont.attrs;
    let ident = cont.ident();

    let decl_str = decl.to_string();
    let generics = cont.generics_without_defaults();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let ts_name = expand_ts_name(cont, &decl);

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
    let maybe_deprecated = attrs.into_wasm_abi_span
        .or(attrs.from_wasm_abi_span)
        .map(|span| {
            mark_deprecated(span, "into_wasm_abi/from_wasm_abi are deprecated as they cause memory leaks (https://github.com/madonoharu/tsify/issues/65). Consider using `tsify::Ts` instead.")
        });

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
                #[derive(Clone)]
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

            #ts_name
            #typescript_custom_section
            #wasm_describe
            #into_wasm_abi
            #from_wasm_abi
            #maybe_deprecated
        };
    }
}

/// The declaration's parameters, resolved back to the Rust type parameters they
/// name.
///
/// `None` when one of them names no Rust parameter — `#[tsify(type_params)]`
/// can say anything — which leaves nothing to compose that argument's name
/// from, so the type keeps naming itself by its bare id.
fn name_type_params(cont: &Container, decl: &Decl) -> Option<Vec<syn::Ident>> {
    decl.type_params()
        .iter()
        .map(|declared| {
            cont.generics()
                .type_params()
                .find(|param| param.ident == *declared)
                .map(|param| param.ident.clone())
        })
        .collect()
}

/// Spells out the type's TypeScript name for `TsName`, one `char` at a time,
/// deferring to each argument's own impl for the arguments.
///
/// Unrolled rather than read from `DECL` because wasm-bindgen interprets the
/// descriptor without the module's data segments loaded, so a character that
/// comes from memory reaches it as a NUL.
///
/// The impl is generic over the config it is asked for and passes it down
/// unchanged. One serializer, built from the root type's attributes, writes the
/// whole value; a name that switched to this type's own attributes partway down
/// would describe a shape that is never produced (#125).
fn expand_ts_name(cont: &Container, decl: &Decl) -> TokenStream {
    let ident = cont.ident();
    let name_params = name_type_params(cont, decl).unwrap_or_default();

    let mut generics = cont.generics_without_defaults();
    if !name_params.is_empty() {
        let where_clause = generics.make_where_clause();
        for param in &name_params {
            where_clause
                .predicates
                .push(parse_quote!(#param: tsify::__macro_support::TsName<__TSIFY_CONFIG>));
        }
    }

    let (_, ty_generics, where_clause) = generics.split_for_impl();
    let impl_params = generics.params.iter();

    let id_chars = decl.id().chars().collect::<Vec<_>>();
    let id_len = id_chars.len() as u32;
    let id = quote!(#(tsify::__macro_support::inform_char(#id_chars);)*);

    let (name_len, describe_name) = if name_params.is_empty() {
        (quote!(#id_len), id)
    } else {
        // The id, `<`, `>`, and `, ` between each pair of arguments.
        let punctuation = id_len + 2 + 2 * (name_params.len() as u32 - 1);

        let args = name_params.iter().enumerate().map(|(i, param)| {
            let separator = (i > 0).then(|| {
                quote! {
                    tsify::__macro_support::inform_char(',');
                    tsify::__macro_support::inform_char(' ');
                }
            });

            quote! {
                #separator
                <#param as tsify::__macro_support::TsName<__TSIFY_CONFIG>>::describe_name();
            }
        });

        (
            quote! {
                #punctuation
                #(+ <#name_params as tsify::__macro_support::TsName<__TSIFY_CONFIG>>::NAME_LEN)*
            },
            quote! {
                #id
                tsify::__macro_support::inform_char('<');
                #(#args)*
                tsify::__macro_support::inform_char('>');
            },
        )
    };

    quote! {
        #[automatically_derived]
        impl<#(#impl_params,)* const __TSIFY_CONFIG: u8>
            tsify::__macro_support::TsName<__TSIFY_CONFIG> for #ident #ty_generics
            #where_clause
        {
            const NAME_LEN: u32 = #name_len;

            #[inline]
            fn describe_name() {
                #describe_name
            }
        }
    }
}

fn expand_into_wasm_abi(cont: &Container) -> TokenStream {
    let ident = cont.ident();
    let serde_path = cont.serde_container.attrs.serde_path();
    let mut generics = cont.generics_without_defaults();

    // A predicate's self type is a type position, where `Generics` would render
    // its declaration form — bounds, defaults, `const N: usize` — and hit E0229.
    let predicate: syn::WherePredicate = {
        let (_, ty_generics, _) = generics.split_for_impl();
        parse_quote!(#ident #ty_generics: #serde_path::Serialize)
    };
    generics.make_where_clause().predicates.push(predicate);

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
