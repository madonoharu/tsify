use proc_macro2::TokenTree;
use quote::ToTokens;
use syn::LitStr;

/// Extract the documentation comments from a Vec of attributes
pub fn extract_doc_comments(attrs: &[syn::Attribute]) -> Vec<String> {
    attrs
        .iter()
        .filter_map(|a| {
            // if the path segments include an ident of "doc" we know
            // this is a doc comment
            if a.path().segments.iter().any(|s| s.ident == "doc") {
                Some(a.to_token_stream().into_iter().filter_map(|t| match t {
                    TokenTree::Group(group) => {
                        // this will return the inner tokens of the group
                        // which will be the doc comments
                        Some(
                            group
                                .stream()
                                .into_iter()
                                .filter_map(|t| match t {
                                    TokenTree::Literal(lit) => {
                                        // Parse as LitStr to get the actual string value,
                                        // regardless of raw string syntax (r"...", r#"..."#, etc.)
                                        syn::parse2::<LitStr>(lit.into_token_stream())
                                            .ok()
                                            .map(|s| s.value())
                                    }
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                                .join(""),
                        )
                    }
                    _ => None,
                }))
            } else {
                None
            }
        })
        //Fold up the [[String]] iter we created into Vec<String>
        .fold(vec![], |mut acc, a| {
            acc.extend(a);
            acc
        })
}

/// Output extracted doc comments as Typescript doc comments.
pub fn write_doc_comments(
    f: &mut std::fmt::Formatter<'_>,
    comments: &[String],
) -> Result<(), std::fmt::Error> {
    if comments.is_empty() {
        return Ok(());
    }

    let comment = comments
        .iter()
        .map(|line| format!(" *{}\n", line))
        .collect::<Vec<_>>()
        .join("");

    write!(f, "{}", format_args!("/**\n{} */\n", comment))
}
