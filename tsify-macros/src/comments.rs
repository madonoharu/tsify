use proc_macro2::TokenTree;
use quote::ToTokens;

use crate::typescript::TsType;

/// Extract the documentation comments from a Vec of attributes
pub fn extract_doc_comments(attrs: &[syn::Attribute]) -> Vec<String> {
    attrs
        .iter()
        .filter_map(|a| {
            // if the path segments include an ident of "doc" we know this
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
                                        // this will always return the quoted string, we deal with
                                        // that in the cli when we read in the comments
                                        Some(lit.to_string())
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
        .map(|line| format!(" *{}\n", line.trim_matches('"')))
        .collect::<Vec<_>>()
        .join("");

    write!(f, "{}", format_args!("/**\n{} */\n", comment))
}

/// Remove all comments from a `TsType::TypeLit`
pub fn clean_comments(typ: &mut TsType) {
    if let TsType::TypeLit(ref mut lit) = typ {
        lit.members.iter_mut().for_each(|elem| {
            elem.comments = vec![];
            // Recurse
            clean_comments(&mut elem.type_ann);
        });
    }
}
