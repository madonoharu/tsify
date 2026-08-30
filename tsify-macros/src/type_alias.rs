use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    attrs::TypeGenerationConfig,
    comments::extract_doc_comments,
    decl::{resolve_defaults, TsTypeAliasDecl, TsTypeParam},
    error_tracker::ErrorTracker,
    typescript::{TsType, TypeContext},
};

/// Expand a `#[declare]` macro on a Rust `type = ...` expression.
pub fn expand(item: syn::ItemType) -> syn::Result<TokenStream> {
    let errors = ErrorTracker::new();

    let ctx = TypeContext {
        config: &TypeGenerationConfig::default(),
        generics: &item.generics,
    };

    let type_ann = TsType::from_syn_type(ctx, item.ty.as_ref());

    let mut type_params = item
        .generics
        .type_params()
        .map(|ty| TsTypeParam {
            name: ty.ident.to_string(),
            default: ty
                .default
                .as_ref()
                .map(|default| TsType::from_syn_type(ctx, default)),
        })
        .collect::<Vec<_>>();

    resolve_defaults(&mut type_params);

    let decl = TsTypeAliasDecl {
        id: item.ident.to_string(),
        export: true,
        type_params,
        type_ann,
        comments: extract_doc_comments(&item.attrs),
    };

    let decl_str = decl.to_string();

    let typescript_custom_section = quote! {
        const _: () = {
            use wasm_bindgen::prelude::*;
            #[wasm_bindgen(typescript_custom_section)]
            const TS_APPEND_CONTENT: &'static str = #decl_str;
        };
    };

    errors.check()?;

    let tokens = quote! {
      #item
      #typescript_custom_section
    };

    Ok(tokens)
}

#[cfg(test)]
#[path = "type_alias.test.rs"]
mod test;
