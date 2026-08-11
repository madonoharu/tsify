use super::expand;

// `#[tsify::declare]` emits its TypeScript declaration only through
// `#[wasm_bindgen(typescript_custom_section)]`, which expands to nothing on
// non-wasm targets — so the expansion snapshots in tests/expand/ cannot see
// the declaration text. These tests assert on it directly.

fn expand_to_string(item: syn::ItemType) -> String {
    expand(item).unwrap().to_string()
}

#[test]
fn test_declare_emits_exported_type_alias() {
    let tokens = expand_to_string(syn::parse_quote! {
        type TypeAlias<T, U> = Foo<T, i32, U>;
    });
    assert!(
        tokens.contains("export type TypeAlias<T, U> = Foo<T, number, U>;"),
        "expected TS alias declaration in expansion, got: {tokens}"
    );
    assert!(tokens.contains("TS_APPEND_CONTENT"));
    assert!(tokens.contains("typescript_custom_section"));
}

#[test]
fn test_declare_without_generics() {
    let tokens = expand_to_string(syn::parse_quote! {
        type Simple = Vec<String>;
    });
    assert!(
        tokens.contains("export type Simple = string[];"),
        "expected TS alias declaration in expansion, got: {tokens}"
    );
}

#[test]
fn test_declare_keeps_doc_comments() {
    let tokens = expand_to_string(syn::parse_quote! {
        /// Alias docs
        type Documented = i32;
    });
    assert!(
        tokens.contains("Alias docs"),
        "expected doc comment to be carried into the TS declaration, got: {tokens}"
    );
    assert!(tokens.contains("export type Documented = number;"));
}
