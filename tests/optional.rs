#![allow(dead_code)]

use tsify::Tsify;

#[test]
fn test_optional() {
    #[derive(Tsify)]
    struct Optional {
        #[tsify(optional)]
        a: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        b: Option<String>,
        #[serde(default)]
        c: i32,
        d: String,
    }

    #[derive(Tsify)]
    #[serde(default)]
    struct OptionalAll {
        a: i32,
        b: i32,
    }

    assert_eq!(
        Optional::DECL,
        "export type Optional = { a?: number; b?: string; c?: number; d: string };"
    );

    assert_eq!(
        OptionalAll::DECL,
        "export type OptionalAll = { a?: number; b?: number };"
    )
}
