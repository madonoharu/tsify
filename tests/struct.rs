#![allow(dead_code)]

use std::collections::HashMap;

use indoc::indoc;
use pretty_assertions::assert_eq;
use tsify_next::Tsify;

#[test]
fn test_unit() {
    /// Comment for Unit
    #[derive(Tsify)]
    struct Unit;

    if cfg!(feature = "js") {
        assert_eq!(
            Unit::DECL,
            indoc! {"
            /**
             * Comment for Unit
             */
            export type Unit = undefined;"
            }
        );
    } else {
        assert_eq!(
            Unit::DECL,
            indoc! {"
            /**
             * Comment for Unit
             */
            export type Unit = null;"
            }
        );
    };
}

#[test]
fn test_named_fields() {
    /// Comment for Struct
    #[derive(Tsify)]
    struct A {
        /// Comment for a
        a: (usize, u64),
        /// Comment for b
        b: HashMap<String, i128>,
    }

    let expected = if cfg!(feature = "js") {
        indoc! {"
            /**
             * Comment for Struct
             */
            export interface A {
                /**
                 * Comment for a
                 */
                a: [number, number];
                /**
                 * Comment for b
                 */
                b: Map<string, bigint>;
            }"
        }
    } else {
        indoc! {"
            /**
             * Comment for Struct
             */
            export interface A {
                /**
                 * Comment for a
                 */
                a: [number, number];
                /**
                 * Comment for b
                 */
                b: Record<string, number>;
            }"
        }
    };

    assert_eq!(A::DECL, expected);
}

#[test]
fn test_newtype_struct() {
    /// Comment for Newtype
    #[derive(Tsify)]
    struct Newtype(i32);

    assert_eq!(
        Newtype::DECL,
        indoc! {"
        /**
         * Comment for Newtype
         */
        export type Newtype = number;"
        }
    );
}

#[test]
fn test_tuple_struct() {
    /// Comment for Tuple
    #[derive(Tsify)]
    struct Tuple(i32, String);
    /// Comment for EmptyTuple
    #[derive(Tsify)]
    struct EmptyTuple();

    assert_eq!(
        Tuple::DECL,
        indoc! {"
        /**
         * Comment for Tuple
         */
        export type Tuple = [number, string];"
        }
    );
    assert_eq!(
        EmptyTuple::DECL,
        indoc! {"
        /**
         * Comment for EmptyTuple
         */
        export type EmptyTuple = [];"
        }
    );
}

#[test]
fn test_nested_struct() {
    /// Comment for A
    #[derive(Tsify)]
    struct A {
        /// Comment for x
        x: f64,
    }

    /// Comment for B
    #[derive(Tsify)]
    struct B {
        /// Comment for a
        a: A,
    }

    assert_eq!(
        B::DECL,
        indoc! {"
            /**
             * Comment for B
             */
            export interface B {
                /**
                 * Comment for a
                 */
                a: A;
            }"
        }
    );
}

#[test]
fn test_struct_with_borrowed_fields() {
    use std::borrow::Cow;

    /// Comment for Borrow
    #[derive(Tsify)]
    struct Borrow<'a> {
        /// Comment for raw
        raw: &'a str,
        /// Comment for cow
        cow: Cow<'a, str>,
    }

    assert_eq!(
        Borrow::DECL,
        indoc! {"
            /**
             * Comment for Borrow
             */
            export interface Borrow {
                /**
                 * Comment for raw
                 */
                raw: string;
                /**
                 * Comment for cow
                 */
                cow: string;
            }"
        }
    );
}

#[test]
fn test_tagged_struct() {
    /// Comment for TaggedStruct
    #[derive(Tsify)]
    #[serde(tag = "type")]
    struct TaggedStruct {
        /// Comment for x
        x: i32,
        /// Comment for y
        y: i32,
    }

    assert_eq!(
        TaggedStruct::DECL,
        indoc! {r#"
            /**
             * Comment for TaggedStruct
             */
            export interface TaggedStruct {
                type: "TaggedStruct";
                /**
                 * Comment for x
                 */
                x: number;
                /**
                 * Comment for y
                 */
                y: number;
            }"#
        }
    );
}
