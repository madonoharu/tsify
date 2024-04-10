#![allow(dead_code)]

use indoc::indoc;
use pretty_assertions::assert_eq;
use tsify_next::Tsify;

#[test]
fn test_skip() {
    /// Comment for Struct
    #[derive(Tsify)]
    struct Struct {
        /// Comment for a
        a: i32,
        /// Comment for b
        #[serde(skip)]
        b: i32,
        /// Comment for c
        #[serde(skip_serializing)]
        c: i32,
        /// Comment for d
        #[serde(skip_deserializing)]
        d: i32,
    }

    assert_eq!(
        Struct::DECL,
        indoc! {"
            /**
             * Comment for Struct
             */
            export interface Struct {
                /**
                 * Comment for a
                 */
                a: number;
            }"
        }
    );

    /// Comment for Tuple
    #[derive(Tsify)]
    struct Tuple(#[serde(skip)] String, i32);

    assert_eq!(
        Tuple::DECL,
        indoc! {"
        /**
         * Comment for Tuple
         */
        export type Tuple = [number];"
        }
    );

    /// Comment for Enum
    #[derive(Tsify)]
    #[tsify(namespace)]
    enum Enum {
        /// Comment for A
        #[serde(skip)]
        A,
        /// Comment for B
        #[serde(skip_serializing)]
        B,
        /// Comment for C
        #[serde(skip_deserializing)]
        C,
        /// Comment for D
        D,
    }

    let expected = indoc! {r#"
        /**
         * Comment for Enum
         */
        declare namespace Enum {
            /**
             * Comment for D
             */
            export type D = "D";
        }

        /**
         * Comment for Enum
         */
        export type Enum = "D";"#
    };

    assert_eq!(Enum::DECL, expected);
}
