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
        r#"declare namespace External {"#, "\n",
        r#"    export type ExternalStruct = { Struct: { x: string; y: number } };"#, "\n",
        r#"    export type ExternalEmptyStruct = { EmptyStruct: {} };"#, "\n",
        r#"    export type ExternalTuple = { Tuple: [number, string] };"#, "\n",
        r#"    export type ExternalEmptyTuple = { EmptyTuple: [] };"#, "\n",
        r#"    export type ExternalNewtype = { Newtype: Foo };"#, "\n",
        r#"    export type ExternalUnit = "Unit";"#, "\n",
        r#"}"#, "\n",
        r#""#, "\n",
        r#"export type External = External.ExternalStruct | External.ExternalEmptyStruct | External.ExternalTuple | External.ExternalEmptyTuple | External.ExternalNewtype | External.ExternalUnit;"#,
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
        r#"declare namespace Internal {"#, "\n",
        r#"    export type InternalStruct = { t: "Struct"; x: string; y: number };"#, "\n",
        r#"    export type InternalEmptyStruct = { t: "EmptyStruct" };"#, "\n",
        r#"    export type InternalNewtype = { t: "Newtype" } & Foo;"#, "\n",
        r#"    export type InternalUnit = { t: "Unit" };"#, "\n",
        r#"}"#, "\n",
        r#""#, "\n",
        r#"export type Internal = Internal.InternalStruct | Internal.InternalEmptyStruct | Internal.InternalNewtype | Internal.InternalUnit;"#,
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
        r#"declare namespace Adjacent {"#, "\n",
        r#"    export type AdjacentStruct = { t: "Struct"; c: { x: string; y: number } };"#, "\n",
        r#"    export type AdjacentEmptyStruct = { t: "EmptyStruct"; c: {} };"#, "\n",
        r#"    export type AdjacentTuple = { t: "Tuple"; c: [number, string] };"#, "\n",
        r#"    export type AdjacentEmptyTuple = { t: "EmptyTuple"; c: [] };"#, "\n",
        r#"    export type AdjacentNewtype = { t: "Newtype"; c: Foo };"#, "\n",
        r#"    export type AdjacentUnit = { t: "Unit"; c: null };"#, "\n",
        r#"}"#, "\n",
        r#""#, "\n",
        r#"export type Adjacent = Adjacent.AdjacentStruct | Adjacent.AdjacentEmptyStruct | Adjacent.AdjacentTuple | Adjacent.AdjacentEmptyTuple | Adjacent.AdjacentNewtype | Adjacent.AdjacentUnit;"#,
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

    let expected = concat!(
        r#"declare namespace Untagged {"#, "\n",
        r#"    export type UntaggedStruct = { x: string; y: number };"#, "\n",
        r#"    export type UntaggedEmptyStruct = {};"#, "\n",
        r#"    export type UntaggedTuple = [number, string];"#, "\n",
        r#"    export type UntaggedEmptyTuple = [];"#, "\n",
        r#"    export type UntaggedNewtype = Foo;"#, "\n",
        r#"    export type UntaggedUnit = null;"#, "\n",
        r#"}"#, "\n",
        r#""#, "\n",
        r#"export type Untagged = Untagged.UntaggedStruct | Untagged.UntaggedEmptyStruct | Untagged.UntaggedTuple | Untagged.UntaggedEmptyTuple | Untagged.UntaggedNewtype | Untagged.UntaggedUnit;"#,
    );

    assert_eq!(expected, Untagged::DECL);
}

#[test]
fn test_module_reimport_enum() {
    #[derive(Tsify)]
    #[tsify(enum_reimport_module)]
    enum Internal {
        Struct { x: String, y: i32 },
        EmptyStruct {},
        Tuple(i32, String),
        EmptyTuple(),
        Newtype(Foo),
        Unit,
    }

    let expected = concat!(
        r#"import type * as Internal_Module from "./tsify";"#, "\n",
        r#"declare namespace Internal {"#, "\n",
        r#"    export type Struct = { Struct: { x: string; y: number } };"#, "\n",
        r#"    export type EmptyStruct = { EmptyStruct: {} };"#, "\n",
        r#"    export type Tuple = { Tuple: [number, string] };"#, "\n",
        r#"    export type EmptyTuple = { EmptyTuple: [] };"#, "\n",
        r#"    export type Newtype = { Newtype: Internal_Module.Foo };"#, "\n",
        r#"    export type Unit = "Unit";"#, "\n",
        r#"}"#, "\n",
        r#""#, "\n",
        r#"export type Internal = Internal.Struct "#,
        r#"| Internal.EmptyStruct | Internal.Tuple "#,
        r#"| Internal.EmptyTuple | Internal.Newtype "#,
        r#"| Internal.Unit;"#,
    );

    assert_eq!(expected, Internal::DECL);
}
