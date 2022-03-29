#![allow(dead_code)]

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
        "export type RenamedStruct = { X: number; Y: number };",
        RenamedStruct::DECL
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
        r#"export type RenamedEnum ="#,
        r#" { X: boolean }"#,
        r#" | { Y: number }"#,
        r#" | { Z: string };"#
    );

    assert_eq!(expected, RenamedEnum::DECL);
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
        "export type Enum = ",
        "{ snake_case: { foo: boolean; foo_bar: boolean } }",
        " | { camel_case: { foo: boolean; fooBar: boolean } }",
        " | { kebab_case: { foo: boolean; \"foo-bar\": boolean } }",
        " | { screaming_snake_case: { FOO: boolean; FOO_BAR: boolean } };"
    );

    assert_eq!(expected, Enum::DECL);
    assert_eq!(
        "export type PascalCase = { Foo: boolean; FooBar: boolean };",
        PascalCase::DECL
    );
    assert_eq!(
        r#"export type ScreamingKebab = { FOO: boolean; "FOO-BAR": boolean };"#,
        ScreamingKebab::DECL
    )
}
