#![allow(dead_code)]

use indoc::indoc;
use pretty_assertions::assert_eq;
use tsify::Tsify;

#[test]
fn test_skip() {
    /// Comment for Struct
    #[derive(Tsify)]
    struct Struct {
        /// Comment for a
        a: i32,
        /// Comment for b
        #[serde(skip)]
        b: i32,
        /// Comment for c
        #[serde(skip_serializing)]
        c: i32,
        /// Comment for d
        #[serde(skip_deserializing)]
        d: i32,
    }

    assert_eq!(
        Struct::DECL,
        indoc! {"
            /**
             * Comment for Struct
             */
            export interface Struct {
                /**
                 * Comment for a
                 */
                a: number;
            }"
        }
    );

    /// Comment for Tuple
    #[derive(Tsify)]
    struct Tuple(#[serde(skip)] String, i32);

    assert_eq!(
        Tuple::DECL,
        indoc! {"
        /**
         * Comment for Tuple
         */
        export type Tuple = [number];"
        }
    );

    /// Comment for Enum
    #[derive(Tsify)]
    #[tsify(namespace)]
    enum Enum {
        /// Comment for A
        #[serde(skip)]
        A,
        /// Comment for B
        #[serde(skip_serializing)]
        B,
        /// Comment for C
        #[serde(skip_deserializing)]
        C,
        /// Comment for D
        D,
        /// Comment for Struct
        Struct {
            #[serde(skip)]
            field_a: bool,
            field_b: u8,
            field_c: String,
        },
        /// Comment for Tuple
        Tuple(#[serde(skip)] bool, u8, String),
        /// Comment for NewType
        NewType(#[serde(skip)] bool),
    }

    let expected = indoc! {r#"
        /**
         * Comment for Enum
         */
        declare namespace Enum {
            /**
             * Comment for D
             */
            export type D = "D";
            /**
             * Comment for Struct
             */
            export type Struct = {
                Struct: {
                    field_b: number;
                    field_c: string;
                };
            };
            /**
             * Comment for Tuple
             */
            export type Tuple = {
                Tuple: [number, string];
            };
            /**
             * Comment for NewType
             */
            export type NewType = "NewType";
        }

        /**
         * Comment for Enum
         */
        export type Enum = Enum.D | Enum.Struct | Enum.Tuple | Enum.NewType;"#
    };

    assert_eq!(Enum::DECL, expected);

    /// Comment for InternalTagEnum
    #[derive(Tsify)]
    #[serde(tag = "type")]
    #[tsify(namespace)]
    enum InternalTagEnum {
        /// Comment for Unit
        Unit,
        /// Comment for Struct
        Struct {
            #[serde(skip)]
            field_a: bool,
            field_b: u8,
        },
        /// Comment for NewType
        NewType(#[serde(skip)] bool),
    }

    let expected = indoc! {r#"
        /**
         * Comment for InternalTagEnum
         */
        declare namespace InternalTagEnum {
            /**
             * Comment for Unit
             */
            export type Unit = {
                type: "Unit";
            };
            /**
             * Comment for Struct
             */
            export type Struct = {
                type: "Struct";
                field_b: number;
            };
            /**
             * Comment for NewType
             */
            export type NewType = {
                type: "NewType";
            };
        }

        /**
         * Comment for InternalTagEnum
         */
        export type InternalTagEnum = InternalTagEnum.Unit | InternalTagEnum.Struct | InternalTagEnum.NewType;"#
    };

    assert_eq!(InternalTagEnum::DECL, expected);
}
