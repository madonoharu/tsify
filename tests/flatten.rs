#![allow(dead_code)]

use indoc::indoc;
use pretty_assertions::assert_eq;
use tsify::Tsify;

#[test]
fn test_flatten() {
    #[derive(Tsify)]
    struct A {
        a: i32,
        b: String,
    }

    #[derive(Tsify)]
    struct B {
        #[serde(flatten)]
        extra: A,
        c: i32,
    }

    assert_eq!(
        B::DECL,
        indoc! {"
            export type B = {
                c: number;
            } & A;"
        }
    );
}

#[test]
fn test_flatten_option() {
    #[derive(Tsify)]
    struct A {
        a: i32,
        b: String,
    }

    #[derive(Tsify)]
    struct B {
        #[serde(flatten)]
        extra: Option<A>,
        c: i32,
    }

    assert_eq!(
        B::DECL,
        indoc! {"
            export type B = { c: number } & (A | {});"
        }
    );
}

#[test]
fn test_flatten_enum() {
    #[derive(Tsify)]
    #[serde(tag = "type")]
    enum A {
        A,
        B { b_data: String },
    }

    #[derive(Tsify)]
    struct B {
        #[serde(flatten)]
        a: A,
        b: i32,
    }

    assert_eq!(
        B::DECL,
        indoc! {"
            export type B = {
                b: number;
            } & A;"
        }
    )
}
