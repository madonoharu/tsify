#![cfg(feature = "js")]
#![allow(dead_code)]

use std::collections::HashMap;

use indoc::indoc;
use pretty_assertions::assert_eq;
use tsify::Tsify;

#[test]
fn test_transparent() {
    #[derive(Tsify)]
    #[tsify(missing_as_null)]
    struct Optional {
        a: Option<u32>,
    }

    assert_eq!(
        Optional::DECL,
        indoc! {"
            export interface Optional {
                a: number | null;
            }"
        }
    );

    #[derive(Tsify)]
    #[tsify(hashmap_as_object)]
    struct MapWrap {
        a: HashMap<u32, u32>,
    }

    assert_eq!(
        MapWrap::DECL,
        indoc! {"
            export interface MapWrap {
                a: Record<number, number>;
            }"
        }
    );
}
