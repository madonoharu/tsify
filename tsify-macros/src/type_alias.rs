use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;

use crate::{
    attrs::{AttrTarget, TsifyContainerAttrs},
    comments::extract_doc_comments,
    decl::TsTypeAliasDecl,
    error_tracker::ErrorTracker,
    typescript::{TsType, TypeContext},
};

/// Expand a `#[declare]` macro on a Rust `type ... = ...` item.
///
/// The macro's arguments are read through the same parser `#[tsify(...)]` uses, so
/// an alias takes the container attributes that mean something for it and rejects
/// the ones that do not rather than dropping them.
pub fn expand(args: TokenStream, item: syn::ItemType) -> syn::Result<TokenStream> {
    let attrs = {
        let mut written = item.attrs.clone();
        written.push(parse_quote!(#[tsify(#args)]));
        TsifyContainerAttrs::from_attrs(&written, AttrTarget::TypeAlias)?
    };

    let errors = ErrorTracker::new();

    let type_params: Vec<String> = item
        .generics
        .type_params()
        .map(|ty| ty.ident.to_string())
        .collect();

    let type_ann = match &attrs.type_override {
        Some(type_override) => TsType::Override {
            type_override: type_override.clone(),
            type_params: type_params.clone(),
        },
        None => TsType::from_syn_type(
            TypeContext {
                config: &attrs.ty_config,
                generics: &item.generics,
            },
            item.ty.as_ref(),
        ),
    };

    // The declaration carries the affix, exactly as a container's does, so a
    // reference rendered by a type with the same affix names the same type.
    let decl = TsTypeAliasDecl {
        id: attrs
            .rename
            .unwrap_or_else(|| attrs.ty_config.format_name(item.ident.to_string())),
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
