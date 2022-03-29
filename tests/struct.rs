#![allow(dead_code)]

use tsify::Tsify;

#[test]
fn test_unit() {
    #[derive(Tsify)]
    struct Unit;

    assert_eq!("export type Unit = null;", Unit::DECL);
}

#[test]
fn test_named_fields() {
    #[derive(Tsify)]
    struct A {
        a: (u8, u8),
        b: String,
    }

    assert_eq!(
        r#"export type A = { a: [number, number]; b: string };"#,
        A::DECL
    );
}

#[test]
fn test_newtype_struct() {
    #[derive(Tsify)]
    struct Newtype(i32);

    assert_eq!("export type Newtype = number;", Newtype::DECL);
}

#[test]
fn test_tuple_struct() {
    #[derive(Tsify)]
    struct Tuple(i32, String);
    #[derive(Tsify)]
    struct EmptyTuple();

    assert_eq!("export type Tuple = [number, string];", Tuple::DECL);
    assert_eq!("export type EmptyTuple = [];", EmptyTuple::DECL);
}

#[test]
fn test_nested_struct() {
    #[derive(Tsify)]
    struct A {
        x: f64,
    }

    #[derive(Tsify)]
    struct B {
        a: A,
    }

    assert_eq!("export type B = { a: A };", B::DECL);
}

#[test]
fn test_struct_with_borrowed_fields() {
    use std::borrow::Cow;

    #[derive(Tsify)]
    struct Borrow<'a> {
        raw: &'a str,
        cow: Cow<'a, str>,
    }

    assert_eq!(
        "export type Borrow = { raw: string; cow: string };",
        Borrow::DECL
    );
}

#[test]
fn test_tagged_struct() {
    #[derive(Tsify)]
    #[serde(tag = "type")]
    struct TaggedStruct {
        x: i32,
        y: i32,
    }

    assert_eq!(
        TaggedStruct::DECL,
        "export type TaggedStruct = { type: \"TaggedStruct\"; x: number; y: number };"
    );
}
