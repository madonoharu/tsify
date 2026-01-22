#![allow(dead_code)]

use indoc::indoc;
use pretty_assertions::assert_eq;
use tsify::Tsify;

#[test]
fn test_simple_doc_comment() {
    /// A simple doc comment.
    #[derive(Tsify)]
    struct SimpleComment {
        x: i32,
    }

    assert_eq!(
        SimpleComment::DECL,
        indoc! {"
            /**
             * A simple doc comment.
             */
            export interface SimpleComment {
                x: number;
            }"
        }
    );
}

#[test]
fn test_doc_comment_with_quotes() {
    /// A comment with "quoted text" inside.
    #[derive(Tsify)]
    struct QuotedComment {
        x: i32,
    }

    assert_eq!(
        QuotedComment::DECL,
        indoc! {"
            /**
             * A comment with \"quoted text\" inside.
             */
            export interface QuotedComment {
                x: number;
            }"
        }
    );
}

#[test]
fn test_multiline_doc_comment() {
    /// First line of documentation.
    /// Second line with more details.
    /// Third line concludes.
    #[derive(Tsify)]
    struct MultilineComment {
        x: i32,
    }

    assert_eq!(
        MultilineComment::DECL,
        indoc! {"
            /**
             * First line of documentation.
             * Second line with more details.
             * Third line concludes.
             */
            export interface MultilineComment {
                x: number;
            }"
        }
    );
}

#[test]
fn test_field_doc_comments() {
    /// Struct-level comment.
    #[derive(Tsify)]
    struct FieldComments {
        /// The x coordinate.
        x: i32,
        /// The name field.
        name: String,
    }

    assert_eq!(
        FieldComments::DECL,
        indoc! {"
            /**
             * Struct-level comment.
             */
            export interface FieldComments {
                /**
                 * The x coordinate.
                 */
                x: number;
                /**
                 * The name field.
                 */
                name: string;
            }"
        }
    );
}

macro_rules! my_types {
    ($($decl:item)+) => {$(
        #[derive(Tsify)]
        $decl
    )+};
}

#[test]
fn test_doc_comment_without_quotes_through_macro() {
    my_types! {
        /// A comment without quotes.
        struct MacroNoQuotes {
            x: i32,
        }
    }

    assert_eq!(
        MacroNoQuotes::DECL,
        indoc! {"
            /**
             * A comment without quotes.
             */
            export interface MacroNoQuotes {
                x: number;
            }"
        }
    );
}

#[test]
fn test_doc_comment_with_quotes_through_macro() {
    my_types! {
        /// A comment with "quotes" inside.
        struct MacroWithQuotes {
            x: i32,
        }
    }

    assert_eq!(
        MacroWithQuotes::DECL,
        indoc! {"
            /**
             * A comment with \"quotes\" inside.
             */
            export interface MacroWithQuotes {
                x: number;
            }"
        }
    );
}

#[test]
fn test_field_doc_comment_through_macro() {
    my_types! {
        /// Struct comment
        struct MacroFieldComments {
            /// Field with "quoted" text
            x: i32,
            /// Field without quotes
            y: String,
        }
    }

    assert_eq!(
        MacroFieldComments::DECL,
        indoc! {"
            /**
             * Struct comment
             */
            export interface MacroFieldComments {
                /**
                 * Field with \"quoted\" text
                 */
                x: number;
                /**
                 * Field without quotes
                 */
                y: string;
            }"
        }
    );
}

#[test]
fn test_multiple_doc_lines_through_macro() {
    my_types! {
        /// First line of comment.
        /// Second line with "quotes".
        /// Third line.
        struct MacroMultiLineComment {
            x: i32,
        }
    }

    assert_eq!(
        MacroMultiLineComment::DECL,
        indoc! {"
            /**
             * First line of comment.
             * Second line with \"quotes\".
             * Third line.
             */
            export interface MacroMultiLineComment {
                x: number;
            }"
        }
    );
}
