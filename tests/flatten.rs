#![allow(dead_code)]

use indoc::indoc;
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
            export interface B extends A {
                c: number;
            }"
        }
    );
}
