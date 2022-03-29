#![allow(dead_code)]

use tsify::Tsify;

struct Foo {
    a: i32,
    b: String,
}

#[test]
fn test_externally_tagged_enum() {
    #[derive(Tsify)]
    enum External {
        Struct { x: String, y: i32 },
        EmptyStruct {},
        Tuple(i32, String),
        EmptyTuple(),
        Newtype(Foo),
        Unit,
    }

    let expected = concat!(
        r#"export type External = "#,
        r#"{ Struct: { x: string; y: number } }"#,
        r#" | { EmptyStruct: {} }"#,
        r#" | { Tuple: [number, string] }"#,
        r#" | { EmptyTuple: [] }"#,
        r#" | { Newtype: Foo }"#,
        r#" | "Unit";"#
    );

    assert_eq!(expected, External::DECL);
}

#[test]
fn test_internally_tagged_enum() {
    #[derive(Tsify)]
    #[serde(tag = "t")]
    enum Internal {
        Struct { x: String, y: i32 },
        EmptyStruct {},
        Newtype(Foo),
        Unit,
    }

    let expected = concat!(
        r#"export type Internal = "#,
        r#"{ t: "Struct"; x: string; y: number }"#,
        r#" | { t: "EmptyStruct" }"#,
        r#" | ({ t: "Newtype" } & Foo)"#,
        r#" | { t: "Unit" };"#
    );

    assert_eq!(expected, Internal::DECL);
}

#[test]
fn test_adjacently_tagged_enum() {
    #[derive(Tsify)]
    #[serde(tag = "t", content = "c")]
    enum Adjacent {
        Struct { x: String, y: i32 },
        EmptyStruct {},
        Tuple(i32, String),
        EmptyTuple(),
        Newtype(Foo),
        Unit,
    }

    let expected = concat!(
        r#"export type Adjacent = "#,
        r#"{ t: "Struct"; c: { x: string; y: number } }"#,
        r#" | { t: "EmptyStruct"; c: {} }"#,
        r#" | { t: "Tuple"; c: [number, string] }"#,
        r#" | { t: "EmptyTuple"; c: [] }"#,
        r#" | { t: "Newtype"; c: Foo }"#,
        r#" | { t: "Unit"; c: null };"#
    );

    assert_eq!(expected, Adjacent::DECL);
}

#[test]
fn test_untagged_enum() {
    #[derive(Tsify)]
    #[serde(untagged)]
    enum Untagged {
        Struct { x: String, y: i32 },
        EmptyStruct {},
        Tuple(i32, String),
        EmptyTuple(),
        Newtype(Foo),
        Unit,
    }

    assert_eq!(
        r#"export type Untagged = { x: string; y: number } | {} | [number, string] | [] | Foo | null;"#,
        Untagged::DECL
    );
}
