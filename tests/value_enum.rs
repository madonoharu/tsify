#![allow(dead_code)]

use indoc::indoc;
use pretty_assertions::assert_eq;
use tsify::Tsify;

#[test]
fn value_enum() {
    /// Comment for External
    #[derive(Tsify)]
    #[tsify(value_enum)]
    enum External {
        /// Comment for Struct
        Alpha,
        /// Comment for EmptyStruct
        Beta,
        /// Comment for Tuple
        Gamma,
    }

    let expected = indoc! {r#"
        export enum External {
            /**
             * Comment for Struct
             */
            Alpha = "Alpha",
            /**
             * Comment for EmptyStruct
             */
            Beta = "Beta",
            /**
             * Comment for Tuple
             */
            Gamma = "Gamma",
        }"#
    };

    assert_eq!(External::DECL, expected);
}

#[test]
fn value_enum_renamed() {
    /// Comment for External
    #[derive(Tsify)]
    #[serde(rename_all = "camelCase")]
    #[tsify(value_enum)]
    enum External {
        /// Comment for Struct
        Alpha,
        /// Comment for EmptyStruct
        Beta,
        /// Comment for Tuple
        GammaDelta,
    }

    let expected = indoc! {r#"
        export enum External {
            /**
             * Comment for Struct
             */
            alpha = "alpha",
            /**
             * Comment for EmptyStruct
             */
            beta = "beta",
            /**
             * Comment for Tuple
             */
            gammaDelta = "gammaDelta",
        }"#
    };

    assert_eq!(External::DECL, expected);
}

#[test]
fn value_enum_renamed_variants() {
    /// Comment for External
    #[derive(Tsify)]
    #[tsify(value_enum, rename_variants)]
    #[serde(rename_all = "kebab-case")]
    enum External {
        /// Comment for Struct
        Alpha,
        /// Comment for EmptyStruct
        Beta,
        /// Comment for Tuple
        GammaDelta,
    }

    let expected = indoc! {r#"
        export enum External {
            /**
             * Comment for Struct
             */
            Alpha = "alpha",
            /**
             * Comment for EmptyStruct
             */
            Beta = "beta",
            /**
             * Comment for Tuple
             */
            GammaDelta = "gamma-delta",
        }"#
    };

    assert_eq!(External::DECL, expected);
}
