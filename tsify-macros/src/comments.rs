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
            if a.path()
                .segments
                .iter()
                .any(|s| s.ident.to_string() == "doc")
            {
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

pub fn format_doc_comments(comments: &Vec<String>) -> String {
    let comment = comments
        .iter()
        .map(|line| format!(" *{}\n", line.trim_matches('"')))
        .collect::<Vec<_>>()
        .join("");

    format!("/**\n{} */\n", comment)
}

pub fn clean_comments(typ: &mut TsType) -> () {
    if let TsType::TypeLit(ref mut lit) = typ {
        lit.members.iter_mut().for_each(|elem| {
            elem.comments = vec![];
            // Recurse
            clean_comments(&mut elem.type_ann);
        });
    }
}
