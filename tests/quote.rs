#![allow(dead_code)]

use indoc::indoc;
use pretty_assertions::assert_eq;
use tsify::Tsify;

#[test]
fn test_quote() {
    #[derive(Tsify)]
    struct QuotedStruct {
        #[serde(rename = "with spaces")]
        #[tsify(quote)]
        with_spaces: i32,

        #[serde(rename = "with-hyphen")]
        #[tsify(quote)]
        with_hyphen: i32,

        #[serde(rename = "@invalid!ident")]
        #[tsify(quote)]
        invalid_ident: i32,
    }

    assert_eq!(
        QuotedStruct::DECL,
        indoc! { r#"
            export interface QuotedStruct {
                "with spaces": number;
                "with-hyphen": number;
                "@invalid!ident": number;
            }"#
        }
    );
}
