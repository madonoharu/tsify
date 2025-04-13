#![allow(dead_code)]

use indoc::indoc;
use pretty_assertions::assert_eq;
use tsify::Tsify;

#[test]
fn test_transparent() {
    /// Comment for A
    #[derive(Tsify)]
    #[serde(transparent)]
    struct A(String, #[serde(skip)] f64);

    /// Comment for B
    #[derive(Tsify)]
    #[serde(transparent)]
    struct B {
        /// Comment for x
        #[serde(skip)]
        x: String,
        /// Comment for y
        y: f64,
    }

    assert_eq!(
        A::DECL,
        indoc! {"
        /**
         * Comment for A
         */
        export type A = string;"
        }
    );
    assert_eq!(
        B::DECL,
        indoc! {"
        /**
         * Comment for B
         */
        export type B = number;"
        }
    );
}
