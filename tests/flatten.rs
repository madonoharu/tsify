#![allow(dead_code)]

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

    assert_eq!(B::DECL, "export type B = { c: number } & A;");
}
