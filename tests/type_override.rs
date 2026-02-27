#![allow(dead_code)]

use indoc::indoc;
use pretty_assertions::assert_eq;
use tsify::Tsify;

struct Unsupported;

#[test]
fn test_struct_with_type_override() {
    /// Comment for Struct
    #[derive(Tsify)]
    struct Struct {
        /// Comment for a
        a: i32,
        /// Comment for b
        #[tsify(type = "0 | 1 | 2")]
        b: i32,
        /// Comment for c
        #[tsify(type = "string | null")]
        c: Unsupported,
    }

    /// Comment for Newtype
    #[derive(Tsify)]
    struct Newtype(#[tsify(type = "string | null")] Unsupported);

    assert_eq!(
        Struct::DECL,
        indoc! {r#"
            /**
             * Comment for Struct
             */
            export interface Struct {
                /**
                 * Comment for a
                 */
                a: number;
                /**
                 * Comment for b
                 */
                b: 0 | 1 | 2;
                /**
                 * Comment for c
                 */
                c: string | null;
            }"#
        }
    );

    assert_eq!(
        Newtype::DECL,
        indoc! {"
        /**
         * Comment for Newtype
         */
        export type Newtype = string | null;"
        }
    );
}

#[test]
fn test_enum_with_type_override() {
    /// Comment for Enum
    #[derive(Tsify)]
    enum Enum {
        /// Comment for Struct
        Struct {
            /// Comment for x
            #[tsify(type = "`tpl_lit_${string}`")]
            x: String,
            /// Comment for y
            #[tsify(type = "0 | 1 | 2")]
            y: i32,
        },
        /// Comment for Tuple
        Tuple(
            #[tsify(type = "`tpl_lit_${string}`")] String,
            #[tsify(type = "0 | 1 | 2")] i32,
        ),
        /// Comment for Newtype
        Newtype(#[tsify(type = "number")] Unsupported),
    }

    let expected = indoc! {r#"
        /**
         * Comment for Enum
         */
        export type Enum = {
            Struct: {
                /**
                 * Comment for x
                 */
                x: `tpl_lit_${string}`;
                /**
                 * Comment for y
                 */
                y: 0 | 1 | 2;
            };
        } | {
            Tuple: [`tpl_lit_${string}`, 0 | 1 | 2];
        } | {
            Newtype: number;
        };"#
    };

    assert_eq!(Enum::DECL, expected);
}

#[test]
fn test_generic_struct_with_type_override() {
    /// Comment for Foo
    #[derive(Tsify)]
    pub struct Foo<T> {
        /// Comment for bar
        #[tsify(type = "[T, ...T[]]")]
        bar: Vec<T>,
    }

    let expected = indoc! {r#"
        /**
         * Comment for Foo
         */
        export interface Foo<T> {
            /**
             * Comment for bar
             */
            bar: [T, ...T[]];
        }"#
    };

    assert_eq!(Foo::<()>::DECL, expected);
}

#[test]
fn test_generic_struct_with_container_param_override() {
    trait Trait {
        type Assoc;
    }

    #[derive(Tsify)]
    #[tsify(type_params = "T")]
    pub struct Foo<T: Trait> {
        #[tsify(type = "T.Assoc")]
        bar: T::Assoc,
    }

    #[derive(Tsify)]
    #[tsify(type = "{ Assoc: string }")]
    pub struct Bar;

    impl Trait for Bar {
        type Assoc = String;
    }

    let expected = indoc! {r#"
        export interface Foo<T> {
            bar: T.Assoc;
        }"#
    };

    assert_eq!(Foo::<Bar>::DECL, expected);
}

#[test]
fn test_generic_struct_with_field_param_override() {
    trait Trait {
        type Assoc;
    }

    #[derive(Tsify)]
    pub struct Foo<T: Trait> {
        #[tsify(type = "T.Assoc", type_params = "T")]
        bar: T::Assoc,
    }

    #[derive(Tsify)]
    #[tsify(type = "{ Assoc: string }")]
    pub struct Bar;

    impl Trait for Bar {
        type Assoc = String;
    }

    let expected = indoc! {r#"
        export interface Foo<T> {
            bar: T.Assoc;
        }"#
    };

    assert_eq!(Foo::<Bar>::DECL, expected);
}
