#![allow(dead_code)]

use indoc::indoc;
use pretty_assertions::assert_eq;
use tsify_next::Tsify;

#[test]
fn test_flatten() {
    /// Comment for A
    #[derive(Tsify)]
    struct A {
        /// Comment for a
        a: i32,
        /// Comment for b
        b: String,
    }

    /// Comment for B
    #[derive(Tsify)]
    struct B {
        /// Comment for extra
        #[serde(flatten)]
        extra: A,
        /// Comment for c
        c: i32,
    }

    assert_eq!(
        B::DECL,
        indoc! {"
            /**
             * Comment for B
             */
            export interface B extends A {
                /**
                 * Comment for c
                 */
                c: number;
            }"
        }
    );
}

#[test]
fn test_flatten_option() {
    /// Comment for A
    #[derive(Tsify)]
    struct A {
        /// Comment for a
        a: i32,
        /// Comment for b
        b: String,
    }

    /// Comment for B
    #[derive(Tsify)]
    struct B {
        /// Comment for extra
        #[serde(flatten)]
        extra: Option<A>,
        /// Comment for c
        c: i32,
    }

    assert_eq!(
        B::DECL,
        indoc! {"
            /**
             * Comment for B
             */
            export type B = { c: number } & (A | {});"
        }
    );
}
