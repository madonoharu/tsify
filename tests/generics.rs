#![allow(dead_code)]

use indoc::indoc;
use pretty_assertions::assert_eq;
use tsify::Tsify;

#[test]
fn test_generic_struct() {
    /// Comment for GenericStruct
    #[derive(Tsify)]
    pub struct GenericStruct<'a, A, B, C, D> {
        /// Comment for a
        a: A,
        /// Comment for b
        b: B,
        /// Comment for c
        #[serde(skip)]
        c: &'a C,
        /// Comment for d
        d: D,
    }

    assert_eq!(
        GenericStruct::<(), (), (), ()>::DECL,
        indoc! {"
            /**
             * Comment for GenericStruct
             */
            export interface GenericStruct<A, B, D> {
                /**
                 * Comment for a
                 */
                a: A;
                /**
                 * Comment for b
                 */
                b: B;
                /**
                 * Comment for d
                 */
                d: D;
            }"
        }
    );

    /// Comment for GenericNewtype
    #[derive(Tsify)]
    pub struct GenericNewtype<T>(T);

    assert_eq!(
        GenericNewtype::<()>::DECL,
        indoc! {"
            /**
             * Comment for GenericNewtype
             */
            export type GenericNewtype<T> = T;"
        },
    );

    /// Comment for GenericTuple
    #[derive(Tsify)]
    pub struct GenericTuple<'a, A, B, C, D>(A, #[serde(skip)] &'a B, C, D);

    assert_eq!(
        GenericTuple::<(), (), (), ()>::DECL,
        indoc! {"
            /**
             * Comment for GenericTuple
             */
            export type GenericTuple<A, C, D> = [A, C, D];"
        ,}
    );
}

#[test]
fn test_generic_enum() {
    /// Comment for GenericEnum
    #[derive(Tsify)]
    pub enum GenericEnum<T, U> {
        Unit,
        NewType(T),
        Seq(T, U),
        Map { x: T, y: U },
    }

    let expected = indoc! {r#"
        /**
         * Comment for GenericEnum
         */
        export type GenericEnum<T, U> = "Unit" | { NewType: T } | { Seq: [T, U] } | { Map: { x: T; y: U } };"#
    };

    assert_eq!(GenericEnum::<(), ()>::DECL, expected);
}

#[test]
fn test_generic_enum_with_namespace() {
    /// Comment for GenericEnum
    #[derive(Tsify)]
    #[tsify(namespace)]
    pub enum GenericEnum<T, U> {
        /// Comment for Unit
        Unit,
        /// Comment for NewType
        NewType(T),
        /// Comment for Seq
        Seq(T, U),
        /// Comment for Map
        Map { x: T, y: U },
    }

    let expected = indoc! {r#"
        /**
         * Comment for GenericEnum
         */
        declare namespace GenericEnum {
            /**
             * Comment for Unit
             */
            export type Unit = "Unit";
            /**
             * Comment for NewType
             */
            export type NewType<T> = { NewType: T };
            /**
             * Comment for Seq
             */
            export type Seq<T, U> = { Seq: [T, U] };
            /**
             * Comment for Map
             */
            export type Map<T, U> = { Map: { x: T; y: U } };
        }

        /**
         * Comment for GenericEnum
         */
        export type GenericEnum<T, U> = "Unit" | { NewType: T } | { Seq: [T, U] } | { Map: { x: T; y: U } };"#
    };

    assert_eq!(GenericEnum::<(), ()>::DECL, expected);
}
