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
        r#"type __ExternalFoo = Foo;"#, "\n",
        r#"declare namespace External {"#, "\n",
        r#"    export type Struct = { Struct: { x: string; y: number } };"#, "\n",
        r#"    export type EmptyStruct = { EmptyStruct: {} };"#, "\n",
        r#"    export type Tuple = { Tuple: [number, string] };"#, "\n",
        r#"    export type EmptyTuple = { EmptyTuple: [] };"#, "\n",
        r#"    export type Newtype = { Newtype: __ExternalFoo };"#, "\n",
        r#"    export type Unit = "Unit";"#, "\n",
        r#"}"#, "\n",
        r#""#, "\n",
        r#"export type External = External.Struct | External.EmptyStruct | External.Tuple | External.EmptyTuple | External.Newtype | External.Unit;"#,
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
        r#"type __InternalFoo = Foo;"#, "\n",
        r#"declare namespace Internal {"#, "\n",
        r#"    export type Struct = { t: "Struct"; x: string; y: number };"#, "\n",
        r#"    export type EmptyStruct = { t: "EmptyStruct" };"#, "\n",
        r#"    export type Newtype = { t: "Newtype" } & __InternalFoo;"#, "\n",
        r#"    export type Unit = { t: "Unit" };"#, "\n",
        r#"}"#, "\n",
        r#""#, "\n",
        r#"export type Internal = Internal.Struct | Internal.EmptyStruct | Internal.Newtype | Internal.Unit;"#,
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
        r#"type __AdjacentFoo = Foo;"#, "\n",
        r#"declare namespace Adjacent {"#, "\n",
        r#"    export type Struct = { t: "Struct"; c: { x: string; y: number } };"#, "\n",
        r#"    export type EmptyStruct = { t: "EmptyStruct"; c: {} };"#, "\n",
        r#"    export type Tuple = { t: "Tuple"; c: [number, string] };"#, "\n",
        r#"    export type EmptyTuple = { t: "EmptyTuple"; c: [] };"#, "\n",
        r#"    export type Newtype = { t: "Newtype"; c: __AdjacentFoo };"#, "\n",
        r#"    export type Unit = { t: "Unit"; c: null };"#, "\n",
        r#"}"#, "\n",
        r#""#, "\n",
        r#"export type Adjacent = Adjacent.Struct | Adjacent.EmptyStruct | Adjacent.Tuple | Adjacent.EmptyTuple | Adjacent.Newtype | Adjacent.Unit;"#,
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
        r#"type __UntaggedFoo = Foo;"#, "\n",
        r#"declare namespace Untagged {"#, "\n",
        r#"    export type Struct = { x: string; y: number };"#, "\n",
        r#"    export type EmptyStruct = {};"#, "\n",
        r#"    export type Tuple = [number, string];"#, "\n",
        r#"    export type EmptyTuple = [];"#, "\n",
        r#"    export type Newtype = __UntaggedFoo;"#, "\n",
        r#"    export type Unit = null;"#, "\n",
        r#"}"#, "\n",
        r#""#, "\n",
        r#"export type Untagged = Untagged.Struct | Untagged.EmptyStruct | Untagged.Tuple | Untagged.EmptyTuple | Untagged.Newtype | Untagged.Unit;"#,
    );

    assert_eq!(expected, Untagged::DECL);
}

#[test]
fn test_module_reimport_enum() {
    #[derive(Tsify)]
    enum Internal {
        Struct { x: String, y: i32 },
        EmptyStruct {},
        Tuple(i32, String),
        EmptyTuple(),
        Newtype(Foo),
        Newtype2(Foo),
        Unit,
    }

    let expected = concat!(
    r#"type __InternalFoo = Foo;"#, "\n",
    r#"declare namespace Internal {"#, "\n",
    r#"    export type Struct = { Struct: { x: string; y: number } };"#, "\n",
    r#"    export type EmptyStruct = { EmptyStruct: {} };"#, "\n",
    r#"    export type Tuple = { Tuple: [number, string] };"#, "\n",
    r#"    export type EmptyTuple = { EmptyTuple: [] };"#, "\n",
    r#"    export type Newtype = { Newtype: __InternalFoo };"#, "\n",
    r#"    export type Newtype2 = { Newtype2: __InternalFoo };"#, "\n",
    r#"    export type Unit = "Unit";"#, "\n",
    r#"}"#, "\n",
    r#""#, "\n",
    r#"export type Internal = Internal.Struct "#,
    r#"| Internal.EmptyStruct | Internal.Tuple "#,
    r#"| Internal.EmptyTuple | Internal.Newtype "#,
    r#"| Internal.Newtype2 | Internal.Unit;"#,
    );

    assert_eq!(expected, Internal::DECL);
}
