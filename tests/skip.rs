#![allow(dead_code)]

use tsify::Tsify;

#[test]
fn test_skip() {
    #[derive(Tsify)]
    struct Struct {
        a: i32,
        #[serde(skip)]
        b: i32,
        #[serde(skip_serializing)]
        c: i32,
        #[serde(skip_deserializing)]
        d: i32,
    }

    assert_eq!("export type Struct = { a: number };", Struct::DECL);

    #[derive(Tsify)]
    struct Tuple(#[serde(skip)] String, i32);

    assert_eq!("export type Tuple = [number];", Tuple::DECL);

    #[derive(Tsify)]
    enum Enum {
        #[serde(skip)]
        A,
        #[serde(skip_serializing)]
        B,
        #[serde(skip_deserializing)]
        C,
        D,
    }

    assert_eq!(r#"export type Enum = "D";"#, Enum::DECL);
}
