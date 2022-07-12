#![allow(dead_code)]

use indoc::indoc;
use tsify::Tsify;

#[test]
fn test_optional() {
    #[derive(Tsify)]
    struct Optional {
        #[tsify(optional)]
        a: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        b: Option<String>,
        #[serde(default)]
        c: i32,
        #[serde(default)]
        d: Option<String>,
    }

    #[derive(Tsify)]
    #[serde(default)]
    struct OptionalAll {
        a: i32,
        b: i32,
        c: Option<i32>,
    }

    assert_eq!(
        Optional::DECL,
        indoc! {"
            export interface Optional {
                a?: number;
                b?: string;
                c?: number;
                d?: string | null;
            }"
        }
    );

    assert_eq!(
        OptionalAll::DECL,
        indoc! {"
            export interface OptionalAll {
                a?: number;
                b?: number;
                c?: number | null;
            }"
        }
    );
}
