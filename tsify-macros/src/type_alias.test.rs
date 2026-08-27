use super::expand;

// `#[tsify::declare]` emits its TypeScript declaration only through
// `#[wasm_bindgen(typescript_custom_section)]`, which expands to nothing on
// non-wasm targets — so the snapshots in tests/expand/ never contain the
// declaration text. These tests assert on it directly.

macro_rules! assert_contains {
    ($tokens:expr, $( $needle:expr ),+ $(,)?) => {
        $(assert!(
            $tokens.contains($needle),
            "expected {:?} in expansion, got: {}",
            $needle,
            $tokens
        );)+
    };
}

fn expand_to_string(item: syn::ItemType) -> String {
    expand(item).unwrap().to_string()
}

#[test]
fn test_declare_emits_exported_type_alias() {
    let tokens = expand_to_string(syn::parse_quote! {
        type TypeAlias<T, U> = Foo<T, i32, U>;
    });
    assert_contains!(
        tokens,
        "export type TypeAlias<T, U> = Foo<T, number, U>;",
        "TS_APPEND_CONTENT",
        "typescript_custom_section",
    );
}

#[test]
fn test_declare_without_generics() {
    let tokens = expand_to_string(syn::parse_quote! {
        type Simple = Vec<String>;
    });
    assert_contains!(tokens, "export type Simple = string[];");
}

#[test]
fn test_declare_keeps_doc_comments() {
    let tokens = expand_to_string(syn::parse_quote! {
        /// Alias docs
        type Documented = i32;
    });
    assert_contains!(tokens, "Alias docs", "export type Documented = number;");
}

#[test]
fn test_declare_keeps_default_type_params() {
    let tokens = expand_to_string(syn::parse_quote! {
        type Defaulted<T = bool, U = Vec<i32>> = Foo<T, U>;
    });
    assert_contains!(
        tokens,
        "export type Defaulted<T = boolean, U = number[]> = Foo<T, U>;",
    );
}
