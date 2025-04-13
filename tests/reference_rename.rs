#![allow(dead_code)]

use indoc::indoc;
use pretty_assertions::assert_eq;
use tsify_next::Tsify;

#[test]
fn test_reference_rename() {
    #[derive(Tsify)]
    #[serde(rename = "foo")]
    pub struct Foo {
        x: i32,
    }

    #[derive(Tsify)]
    pub struct Bar {
        foo: Foo,
    }

    assert_eq!(
        Bar::DECL,
        indoc! {"
            export interface Bar {
                foo: Foo;
            }"
        }
    );
    assert_eq!(
        Foo::DECL,
        indoc! {"
            export interface Foo {
                x: number;
            }"
        }
    );
}
