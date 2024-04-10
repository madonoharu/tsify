#![allow(dead_code)]

use indoc::indoc;
use pretty_assertions::assert_eq;
use tsify::Tsify;

#[test]
fn test_optional() {
    /// Comment for Optional
    #[derive(Tsify)]
    struct Optional {
        /// Comment for a
        #[tsify(optional)]
        a: Option<i32>,
        /// Comment for b
        #[serde(skip_serializing_if = "Option::is_none")]
        b: Option<String>,
        /// Comment for c
        #[serde(default)]
        c: i32,
        /// Comment for d
        #[serde(default)]
        d: Option<String>,
    }

    /// Comment for OptionalAll
    #[derive(Tsify)]
    #[serde(default)]
    struct OptionalAll {
        /// Comment for a
        a: i32,
        /// Comment for b
        b: i32,
        /// Comment for c
        c: Option<i32>,
    }

    if cfg!(feature = "js") {
        assert_eq!(
            Optional::DECL,
            indoc! {"
            /**
             * Comment for Optional
             */
            export interface Optional {
                /**
                 * Comment for a
                 */
                a?: number;
                /**
                 * Comment for b
                 */
                b?: string;
                /**
                 * Comment for c
                 */
                c?: number;
                /**
                 * Comment for d
                 */
                d?: string | undefined;
            }"
            }
        );
        assert_eq!(
            OptionalAll::DECL,
            indoc! {"
                /**
                 * Comment for OptionalAll
                 */
                export interface OptionalAll {
                    /**
                     * Comment for a
                     */
                    a?: number;
                    /**
                     * Comment for b
                     */
                    b?: number;
                    /**
                     * Comment for c
                     */
                    c?: number | undefined;
                }"
            }
        );
    } else {
        assert_eq!(
            Optional::DECL,
            indoc! {"
                /**
                 * Comment for Optional
                 */
                export interface Optional {
                    /**
                     * Comment for a
                     */
                    a?: number;
                    /**
                     * Comment for b
                     */
                    b?: string;
                    /**
                     * Comment for c
                     */
                    c?: number;
                    /**
                     * Comment for d
                     */
                    d?: string | null;
                }"
            }
        );
        assert_eq!(
            OptionalAll::DECL,
            indoc! {"
                /**
                 * Comment for OptionalAll
                 */
                export interface OptionalAll {
                    /**
                     * Comment for a
                     */
                    a?: number;
                    /**
                     * Comment for b
                     */
                    b?: number;
                    /**
                     * Comment for c
                     */
                    c?: number | null;
                }"
            }
        );
    }
}
