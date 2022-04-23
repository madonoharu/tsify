use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;

use crate::{container::Container, typescript::TsTypeAliasDecl};

pub fn expand(cont: &Container, decl: TsTypeAliasDecl) -> TokenStream {
    let attrs = &cont.attrs;
    let ident = cont.ident();

    let decl_str = decl.to_string();
    let (impl_generics, ty_generics, where_clause) = cont.generics().split_for_impl();

    let typescript_custom_section = quote! {
        #[wasm_bindgen(typescript_custom_section)]
        const TS_APPEND_CONTENT: &'static str = #decl_str;
    };

    let wasm_abi = attrs.into_wasm_abi || attrs.from_wasm_abi;

    let wasm_describe = wasm_abi.then(|| {
        quote! {
            impl #impl_generics WasmDescribe for #ident #ty_generics #where_clause {
                #[inline]
                fn describe() {
                    <Self as Tsify>::JsType::describe()
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

    let typescript_type = decl.id.to_string();

    quote! {
        #[automatically_derived]
        const _: () = {
            #use_serde
            use wasm_bindgen::{
                convert::{FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi},
                describe::WasmDescribe,
                prelude::*,
            };

            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(typescript_type = #typescript_type)]
                pub type JsType;
            }

            impl #impl_generics Tsify for #ident #ty_generics #where_clause {
                type JsType = JsType;
                const DECL: &'static str = #decl_str;
            }

            #typescript_custom_section
            #wasm_describe
            #into_wasm_abi
            #from_wasm_abi
        };
    }
}

fn expand_into_wasm_abi(cont: &Container) -> TokenStream {
    let ident = cont.ident();
    let serde_path = cont.serde_container.attrs.serde_path();

    let mut generics = cont.generics().clone();
    generics
        .make_where_clause()
        .predicates
        .push(parse_quote!(Self: #serde_path::Serialize));

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics IntoWasmAbi for #ident #ty_generics #where_clause {
            type Abi = <<Self as Tsify>::JsType as IntoWasmAbi>::Abi;

            #[inline]
            fn into_abi(self) -> Self::Abi {
                JsValue::from_serde(&self).unwrap_throw().into_abi()
            }
        }

        impl #impl_generics OptionIntoWasmAbi for #ident #ty_generics #where_clause {
            #[inline]
            fn none() -> Self::Abi {
                <<Self as Tsify>::JsType as OptionIntoWasmAbi>::none()
            }
        }
    }
}

fn expand_from_wasm_abi(cont: &Container) -> TokenStream {
    let ident = cont.ident();
    let serde_path = cont.serde_container.attrs.serde_path();

    let mut generics = cont.generics().clone();

    generics
        .make_where_clause()
        .predicates
        .push(parse_quote!(Self: #serde_path::de::DeserializeOwned));

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics FromWasmAbi for #ident #ty_generics #where_clause {
            type Abi = <JsValue as FromWasmAbi>::Abi;

            #[inline]
            unsafe fn from_abi(js: Self::Abi) -> Self {
                JsValue::from_abi(js).into_serde().unwrap_throw()
            }
        }

        impl #impl_generics OptionFromWasmAbi for #ident #ty_generics #where_clause {
            #[inline]
            fn is_none(abi: &Self::Abi) -> bool {
                <<Self as Tsify>::JsType as OptionFromWasmAbi>::is_none(abi)
            }
        }
    }
}
