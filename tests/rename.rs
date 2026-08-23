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
fn test_tsify_container_rename() {
    #[derive(Tsify)]
    #[tsify(rename = "StructDeclaration")]
    struct RustStruct {
        value: String,
    }

    #[derive(Tsify)]
    #[tsify(rename = "EnumDeclaration")]
    enum RustEnum {
        Variant(bool),
    }

    assert_eq!(
        RustStruct::DECL,
        indoc! {"
            export interface StructDeclaration {
                value: string;
            }"
        }
    );
    assert_eq!(
        RustEnum::DECL,
        "export type EnumDeclaration = { Variant: boolean };"
    );
}

#[test]
fn test_tsify_container_rename_overrides_serde_rename() {
    #[derive(Tsify)]
    #[serde(rename = "SerdeDeclaration")]
    #[tsify(rename = "TsifyDeclaration")]
    struct RustDeclaration {
        value: String,
    }

    assert_eq!(
        RustDeclaration::DECL,
        indoc! {"
            export interface TsifyDeclaration {
                value: string;
            }"
        }
    );
}

#[test]
fn test_tsify_container_rename_does_not_change_internal_tag_value() {
    #[derive(Tsify)]
    #[serde(rename = "WireName", tag = "kind")]
    #[tsify(rename = "DeclarationName")]
    struct RustName {
        value: String,
    }

    assert_eq!(
        RustName::DECL,
        indoc! {r#"
            export interface DeclarationName {
                kind: "WireName";
                value: string;
            }"#
        }
    );
}

#[test]
fn test_tsify_container_rename_currently_requires_reference_override() {
    // `rename` names declarations and nothing else. Both references sit side by side
    // so that #103, if it lands, has to change this expectation deliberately.
    #[derive(Tsify)]
    #[tsify(rename = "PublicConfig")]
    struct Config {
        value: String,
    }

    #[derive(Tsify)]
    struct Holder {
        config: Config,
        #[tsify(type = "PublicConfig")]
        fixed_config: Config,
    }

    assert_eq!(
        Config::DECL,
        indoc! {"
            export interface PublicConfig {
                value: string;
            }"
        }
    );
    assert_eq!(
        Holder::DECL,
        indoc! {"
            export interface Holder {
                config: Config;
                fixed_config: PublicConfig;
            }"
        }
    );
}

#[test]
fn test_tsify_container_rename_namespace_with_generics() {
    struct Wrapper<T>(T);

    #[derive(Tsify)]
    #[tsify(namespace, rename = "PublicResult")]
    enum RustResult<T> {
        Value(Wrapper<Vec<T>>),
        Empty,
    }

    assert_eq!(
        RustResult::<u32>::DECL,
        indoc! {r#"
            type __PublicResultWrapper<A> = Wrapper<A>;
            declare namespace PublicResult {
                export type Value<T> = { Value: __PublicResultWrapper<T[]> };
                export type Empty = "Empty";
            }

            export type PublicResult<T> = PublicResult.Value<T> | PublicResult.Empty;"#
        }
    );
}

#[test]
fn test_tsify_container_rename_declaration_forms() {
    struct Unsupported;

    #[derive(Tsify)]
    #[tsify(rename = "RenamedNewtype")]
    struct RustNewtype(u32);

    #[derive(Tsify)]
    #[tsify(rename = "RenamedUnit")]
    struct RustUnit;

    #[derive(Tsify)]
    #[serde(transparent)]
    #[tsify(rename = "RenamedTransparent")]
    struct RustTransparent(String);

    #[derive(Tsify)]
    #[tsify(rename = "RenamedOverride", type = "{ value: string }")]
    struct RustOverride(Unsupported);

    assert_eq!(
        RustNewtype::DECL,
        indoc! {"
            export type RenamedNewtype = number;"
        }
    );
    assert_eq!(
        RustUnit::DECL,
        if cfg!(feature = "js") {
            indoc! {"
                export type RenamedUnit = undefined;"
            }
        } else {
            indoc! {"
                export type RenamedUnit = null;"
            }
        }
    );
    assert_eq!(
        RustTransparent::DECL,
        indoc! {"
            export type RenamedTransparent = string;"
        }
    );
    assert_eq!(
        RustOverride::DECL,
        indoc! {"
            export type RenamedOverride = { value: string };"
        }
    );
}

#[test]
fn test_tsify_container_rename_serde_tag_strategies() {
    #[derive(Tsify)]
    #[serde(tag = "kind")]
    #[tsify(rename = "RenamedInternal")]
    enum RustInternal<T> {
        Value { value: T },
        Empty,
    }

    #[derive(Tsify)]
    #[serde(tag = "kind", content = "content")]
    #[tsify(rename = "RenamedAdjacent")]
    enum RustAdjacent<T> {
        Value(T),
        Empty,
    }

    #[derive(Tsify)]
    #[serde(untagged)]
    #[tsify(rename = "RenamedUntagged")]
    enum RustUntagged<T> {
        Value(T),
        Empty,
    }

    assert_eq!(
        RustInternal::<String>::DECL,
        indoc! {r#"
            export type RenamedInternal<T> = { kind: "Value"; value: T } | { kind: "Empty" };"#
        }
    );
    assert_eq!(
        RustAdjacent::<String>::DECL,
        indoc! {r#"
            export type RenamedAdjacent<T> = { kind: "Value"; content: T } | { kind: "Empty" };"#
        }
    );
    assert_eq!(
        RustUntagged::<String>::DECL,
        if cfg!(feature = "js") {
            indoc! {"
                export type RenamedUntagged<T> = T | undefined;"
            }
        } else {
            indoc! {"
                export type RenamedUntagged<T> = T | null;"
            }
        }
    );
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
        SnakeCase { foo: bool, foo_bar: bool },
        /// Comment for camel_case
        #[serde(rename_all = "camelCase")]
        CamelCase { foo: bool, foo_bar: bool },
        /// Comment for kebab_case
        #[serde(rename_all = "kebab-case")]
        KebabCase { foo: bool, foo_bar: bool },
        /// Comment for screaming_snake_case
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        ScreamingSnakeCase { foo: bool, foo_bar: bool },
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
        export type Enum = Enum.snake_case | Enum.camel_case | Enum.kebab_case | Enum.screaming_snake_case;"#
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

#[test]
fn test_quote_non_identifiers() {
    #[derive(Tsify)]
    struct NonIdentifierRenameStruct {
        #[serde(rename = "1")]
        x: i32,
        #[serde(rename = "1x")]
        y: i32,
        #[serde(rename = "-")]
        z: i32,
        #[serde(rename = " ")]
        w: i32,
        #[serde(rename = "#")]
        q: i32,
        #[serde(rename = "should_not_quote")]
        p: i32,
        #[serde(rename = "should$not$quote")]
        r: i32,
    }

    assert_eq!(
        NonIdentifierRenameStruct::DECL,
        indoc! {"
            export interface NonIdentifierRenameStruct {
                \"1\": number;
                \"1x\": number;
                \"-\": number;
                \" \": number;
                \"#\": number;
                should_not_quote: number;
                should$not$quote: number;
            }"
        }
    );

    #[derive(Tsify)]
    enum NonIdentifierRenameEnum {
        #[serde(rename = "hello-world")]
        A(bool),
        #[serde(rename = "hel#&*world")]
        B(i64),
        #[serde(rename = "hello world")]
        C(String),
        #[serde(rename = "")]
        D(i32),
        #[serde(rename = "should_not_quote")]
        E(String),
    }

    let expected = indoc! {r#"
        export type NonIdentifierRenameEnum = { "hello-world": boolean } | { "hel#&*world": number } | { "hello world": string } | { "": number } | { should_not_quote: string };"#
    };

    assert_eq!(NonIdentifierRenameEnum::DECL, expected);
}
