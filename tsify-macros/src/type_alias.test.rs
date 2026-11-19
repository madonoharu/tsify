use super::expand;
use proc_macro2::TokenStream;

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

fn parse(item: &str) -> syn::ItemType {
    syn::parse_str(item).expect("item should parse")
}

/// Expand `#[declare(#args)] #item` and render everything the macro emits.
fn declare(args: &str, item: &str) -> String {
    let args: TokenStream = args.parse().expect("arguments should be valid tokens");

    expand(args, parse(item)).unwrap().to_string()
}

/// The message `#[declare(#args)] #item` is rejected with.
fn declare_error(args: &str, item: &str) -> String {
    let args: TokenStream = args.parse().expect("arguments should be valid tokens");

    expand(args, parse(item)).unwrap_err().to_string()
}

#[test]
fn test_declare_emits_exported_type_alias() {
    let tokens = declare("", "pub type TypeAlias<T, U> = Foo<T, i32, U>;");
    assert_contains!(
        tokens,
        "export type TypeAlias<T, U> = Foo<T, number, U>;",
        "TS_APPEND_CONTENT",
        "typescript_custom_section",
    );
}

#[test]
fn test_declare_without_generics() {
    let tokens = declare("", "pub type Simple = Vec<String>;");
    assert_contains!(tokens, "export type Simple = string[];");
}

#[test]
fn test_declare_keeps_doc_comments() {
    let tokens = declare("", "/// Alias docs\npub type Documented = i32;");
    assert_contains!(tokens, "Alias docs", "export type Documented = number;");
}

#[test]
fn test_declare_rename_renames_the_alias_and_keeps_the_rust_name() {
    let tokens = declare(r#"rename = "RenamedAlias""#, "pub type A = Vec<u32>;");
    assert_contains!(
        tokens,
        "export type RenamedAlias = number[];",
        "pub type A = Vec < u32 > ;",
    );
}

#[test]
fn test_declare_rename_rejects_a_name_that_is_not_an_identifier() {
    let message = declare_error(r#"rename = "not-valid""#, "pub type A = u32;");
    assert!(
        message.contains("`rename` must be a valid TypeScript identifier"),
        "{message}"
    );
}

#[test]
fn test_declare_type_prefix_renames_the_alias_and_reaches_its_references() {
    let tokens = declare(r#"type_prefix = "Pre""#, "pub type B = Vec<Point>;");
    assert_contains!(tokens, "export type PreB = PrePoint[];");
}

#[test]
fn test_declare_type_suffix_renames_the_alias_and_reaches_its_references() {
    let tokens = declare(r#"type_suffix = "Suf""#, "pub type C = Vec<Point>;");
    assert_contains!(tokens, "export type CSuf = PointSuf[];");
}

#[test]
fn test_declare_rename_conflicts_with_an_affix() {
    let message = declare_error(
        r#"rename = "Renamed", type_prefix = "Pre""#,
        "pub type A = u32;",
    );
    assert!(
        message.contains("`rename` cannot be combined with `type_prefix` or `type_suffix`"),
        "{message}"
    );
}

#[test]
fn test_declare_type_overrides_the_alias_body() {
    let tokens = declare(r#"type = "string""#, "pub type D = u32;");
    assert_contains!(tokens, "export type D = string;");

    let tokens = declare(r#"type = "T | null""#, "pub type E<T> = Option<T>;");
    assert_contains!(tokens, "export type E<T> = T | null;");
}

#[test]
fn test_declare_forwards_its_arguments_without_re_emitting_them() {
    // `#[tsify(...)]` is a helper attribute of the `Tsify` derive, so one left on
    // a type alias is a compile error rather than inert noise.
    let tokens = declare(r#"rename = "RenamedAlias""#, "pub type A = u32;");
    assert!(
        !tokens.contains("# [tsify"),
        "the forwarded attribute should not reach the emitted item: {tokens}"
    );
}

#[test]
fn test_declare_rejects_attributes_that_describe_a_container() {
    let cases = [
        (
            "into_wasm_abi",
            "`into_wasm_abi` cannot be applied to a type alias",
        ),
        (
            "from_wasm_abi",
            "`from_wasm_abi` cannot be applied to a type alias",
        ),
        ("namespace", "`namespace` cannot be applied to a type alias"),
        (
            r#"type_params = "T""#,
            "`type_params` cannot be applied to a type alias",
        ),
    ];

    for (args, expected) in cases {
        let message = declare_error(args, "pub type A = u32;");
        assert!(message.contains(expected), "for {args}: {message}");
    }
}

#[test]
fn test_declare_rejects_an_unsupported_attribute() {
    let message = declare_error(r#"readonly = "yes""#, "pub type A = u32;");
    assert!(message.contains("unsupported tsify attribute"), "{message}");
}

#[test]
fn test_declare_rejects_a_duplicate_attribute() {
    let message = declare_error(
        r#"rename = "First", rename = "Second""#,
        "pub type A = u32;",
    );
    assert!(message.contains("duplicate attribute"), "{message}");
}

// The serializer options mean something only under the `js` feature, so a `json`
// build has to keep reporting that they are unavailable.
#[cfg(not(feature = "js"))]
#[test]
fn test_declare_requires_js_for_serializer_options() {
    let message = declare_error("missing_as_null", "pub type A = u32;");
    assert!(message.contains("requires the `js` feature"), "{message}");
}
