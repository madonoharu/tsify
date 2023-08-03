use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Parser;

use crate::{
    attrs::{ContainerType, TsifyContainerAttrs},
    ctxt::Ctxt,
    decl::TsTypeAliasDecl,
    typescript::TsType,
};

pub fn expend(args: TokenStream, item: syn::ItemType) -> syn::Result<TokenStream> {
    let mut attrs = TsifyContainerAttrs::default();
    let meta_parser = syn::meta::parser(|meta| attrs.from_nested_meta(meta, ContainerType::Alias));
    meta_parser.parse2(args)?;

    let ctxt = Ctxt::new();

    let type_ann = TsType::from_syn_type(&attrs.ty_config, item.ty.as_ref());

    let decl = TsTypeAliasDecl {
        id: attrs.ty_config.format_name(item.ident.to_string()),
        export: true,
        type_params: item
            .generics
            .type_params()
            .map(|ty| attrs.ty_config.format_name(ty.ident.to_string()))
            .collect(),
        type_ann,
    };

    let decl_str = decl.to_string();

    let typescript_custom_section = quote! {
        #[automatically_derived]
        const _: () = {
            use wasm_bindgen::prelude::*;
            #[wasm_bindgen(typescript_custom_section)]
            const TS_APPEND_CONTENT: &'static str = #decl_str;
        };
    };

    ctxt.check()?;

    let tokens = quote! {
      #item
      #typescript_custom_section
    };

    Ok(tokens)
}
