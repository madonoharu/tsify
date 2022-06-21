#![allow(dead_code)]

use indoc::indoc;
use tsify::Tsify;

#[test]
fn test_rename() {
    #[derive(Tsify)]
    struct RenamedStruct {
        #[serde(rename = "X")]
        x: i32,
        #[serde(rename = "Y")]
        y: i32,
    }

    assert_eq!(
        RenamedStruct::DECL,
        indoc! {"
            export interface RenamedStruct {
                X: number;
                Y: number;
            }"
        }
    );

    #[derive(Tsify)]
    enum RenamedEnum {
        #[serde(rename = "X")]
        A(bool),
        #[serde(rename = "Y")]
        B(i64),
        #[serde(rename = "Z")]
        C(String),
        #[serde(skip)]
        D(i32),
    }

    let expected = concat!(
        r#"declare namespace RenamedEnum {"#, "\n",
        r#"    export type X = { X: boolean };"#, "\n",
        r#"    export type Y = { Y: number };"#, "\n",
        r#"    export type Z = { Z: string };"#, "\n",
        r#"}"#, "\n",
        r#""#, "\n",
        r#"export type RenamedEnum = RenamedEnum.X | RenamedEnum.Y | RenamedEnum.Z;"#
    );

    assert_eq!(RenamedEnum::DECL, expected);
}

#[test]
fn test_rename_all() {
    #[derive(Tsify)]
    #[serde(rename_all = "snake_case")]
    enum Enum {
        SnakeCase {
            foo: bool,
            foo_bar: bool,
        },
        #[serde(rename_all = "camelCase")]
        CamelCase {
            foo: bool,
            foo_bar: bool,
        },
        #[serde(rename_all = "kebab-case")]
        KebabCase {
            foo: bool,
            foo_bar: bool,
        },
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        ScreamingSnakeCase {
            foo: bool,
            foo_bar: bool,
        },
    }

    #[derive(Tsify)]
    #[serde(rename_all = "PascalCase")]
    struct PascalCase {
        foo: bool,
        foo_bar: bool,
    }

    #[derive(Tsify)]
    #[serde(rename_all = "SCREAMING-KEBAB-CASE")]
    struct ScreamingKebab {
        foo: bool,
        foo_bar: bool,
    }

    let expected = concat!(
        r#"declare namespace Enum {"#, "\n",
        r#"    export type snake_case = { snake_case: { foo: boolean; foo_bar: boolean } };"#, "\n",
        r#"    export type camel_case = { camel_case: { foo: boolean; fooBar: boolean } };"#, "\n",
        r#"    export type kebab_case = { kebab_case: { foo: boolean; "foo-bar": boolean } };"#, "\n",
        r#"    export type screaming_snake_case = { screaming_snake_case: { FOO: boolean; FOO_BAR: boolean } };"#, "\n",
        r#"}"#, "\n",
        r#""#, "\n",
        r#"export type Enum = Enum.snake_case | Enum.camel_case | Enum.kebab_case | Enum.screaming_snake_case;"#
    );

    assert_eq!(expected, Enum::DECL);

    assert_eq!(
        PascalCase::DECL,
        indoc! {"
            export interface PascalCase {
                Foo: boolean;
                FooBar: boolean;
            }"
        }
    );

    assert_eq!(
        ScreamingKebab::DECL,
        indoc! {r#"
            export interface ScreamingKebab {
                FOO: boolean;
                "FOO-BAR": boolean;
            }"#
        }
    );
}
