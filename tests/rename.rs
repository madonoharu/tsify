#![allow(dead_code)]

use indoc::indoc;
use pretty_assertions::assert_eq;
use tsify::Tsify;

#[test]
fn test_rename() {
    /// Comment for RenamedStruct
    #[derive(Tsify)]
    struct RenamedStruct {
        /// Comment for X
        #[serde(rename = "X")]
        x: i32,
        /// Comment for Y
        #[serde(rename = "Y")]
        y: i32,
    }

    assert_eq!(
        RenamedStruct::DECL,
        indoc! {"
            /**
             * Comment for RenamedStruct
             */
            export interface RenamedStruct {
                /**
                 * Comment for X
                 */
                X: number;
                /**
                 * Comment for Y
                 */
                Y: number;
            }"
        }
    );

    /// Comment for RenamedEnum
    #[derive(Tsify)]
    enum RenamedEnum {
        /// Comment for X
        #[serde(rename = "X")]
        A(bool),
        /// Comment for Y
        #[serde(rename = "Y")]
        B(i64),
        /// Comment for Z
        #[serde(rename = "Z")]
        C(String),
        /// Comment for D
        #[serde(skip)]
        D(i32),
    }

    let expected = indoc! {r#"
        /**
         * Comment for RenamedEnum
         */
        export type RenamedEnum = { X: boolean } | { Y: number } | { Z: string };"#

    };

    assert_eq!(RenamedEnum::DECL, expected);
}

#[test]
fn test_rename_all() {
    /// Comment for Enum
    #[allow(clippy::enum_variant_names)]
    #[derive(Tsify)]
    #[serde(rename_all = "snake_case")]
    #[tsify(namespace)]
    enum Enum {
        /// Comment for snake_case
        SnakeCase {
            foo: bool,
            foo_bar: bool,
        },
        /// Comment for camel_case
        #[serde(rename_all = "camelCase")]
        CamelCase {
            foo: bool,
            foo_bar: bool,
        },
        /// Comment for kebab_case
        #[serde(rename_all = "kebab-case")]
        KebabCase {
            foo: bool,
            foo_bar: bool,
        },
        /// Comment for screaming_snake_case
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        ScreamingSnakeCase {
            foo: bool,
            foo_bar: bool,
        },
    }

    /// Comment for PascalCase
    #[derive(Tsify)]
    #[serde(rename_all = "PascalCase")]
    struct PascalCase {
        /// Comment for Foo
        foo: bool,
        /// Comment for FooBar
        foo_bar: bool,
    }

    /// Comment for ScreamingKebab
    #[derive(Tsify)]
    #[serde(rename_all = "SCREAMING-KEBAB-CASE")]
    struct ScreamingKebab {
        /// Comment for FOO
        foo: bool,
        /// Comment for FOO-BAR
        foo_bar: bool,
    }

    let expected = indoc! {r#"
        /**
         * Comment for Enum
         */
        declare namespace Enum {
            /**
             * Comment for snake_case
             */
            export type snake_case = { snake_case: { foo: boolean; foo_bar: boolean } };
            /**
             * Comment for camel_case
             */
            export type camel_case = { camel_case: { foo: boolean; fooBar: boolean } };
            /**
             * Comment for kebab_case
             */
            export type kebab_case = { kebab_case: { foo: boolean; "foo-bar": boolean } };
            /**
             * Comment for screaming_snake_case
             */
            export type screaming_snake_case = { screaming_snake_case: { FOO: boolean; FOO_BAR: boolean } };
        }

        /**
         * Comment for Enum
         */
        export type Enum = { snake_case: { foo: boolean; foo_bar: boolean } } | { camel_case: { foo: boolean; fooBar: boolean } } | { kebab_case: { foo: boolean; "foo-bar": boolean } } | { screaming_snake_case: { FOO: boolean; FOO_BAR: boolean } };"#
    };

    assert_eq!(Enum::DECL, expected);

    assert_eq!(
        PascalCase::DECL,
        indoc! {"
            /**
             * Comment for PascalCase
             */
            export interface PascalCase {
                /**
                 * Comment for Foo
                 */
                Foo: boolean;
                /**
                 * Comment for FooBar
                 */
                FooBar: boolean;
            }"
        }
    );

    assert_eq!(
        ScreamingKebab::DECL,
        indoc! {r#"
            /**
             * Comment for ScreamingKebab
             */
            export interface ScreamingKebab {
                /**
                 * Comment for FOO
                 */
                FOO: boolean;
                /**
                 * Comment for FOO-BAR
                 */
                "FOO-BAR": boolean;
            }"#
        }
    );
}
