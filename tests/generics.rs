#![allow(dead_code)]

use indoc::indoc;
use tsify::Tsify;

#[test]
fn test_generic_struct() {
    #[derive(Tsify)]
    pub struct GenericStruct<'a, A, B, C, D> {
        a: A,
        b: B,
        #[serde(skip)]
        c: &'a C,
        d: D,
    }

    assert_eq!(
        GenericStruct::<(), (), (), ()>::DECL,
        indoc! {"
            export interface GenericStruct<A, B, D> {
                a: A;
                b: B;
                d: D;
            }"
        }
    );

    #[derive(Tsify)]
    pub struct GenericNewtype<T>(T);

    assert_eq!(
        GenericNewtype::<()>::DECL,
        "export type GenericNewtype<T> = T;"
    );

    #[derive(Tsify)]
    pub struct GenericTuple<'a, A, B, C, D>(A, #[serde(skip)] &'a B, C, D);

    assert_eq!(
        GenericTuple::<(), (), (), ()>::DECL,
        "export type GenericTuple<A, C, D> = [A, C, D];"
    );
}

#[test]
fn test_generic_enum() {
    #[derive(Tsify)]
    pub enum GenericEnum<T, U> {
        Unit,
        NewType(T),
        Seq(T, U),
        Map { x: T, y: U },
    }

    let expected = indoc! {r#"
        declare namespace GenericEnum {
            export type Unit = "Unit";
            export type NewType<T> = { NewType: T };
            export type Seq<T, U> = { Seq: [T, U] };
            export type Map<T, U> = { Map: { x: T; y: U } };
        }
        
        export type GenericEnum<T, U> = GenericEnum.Unit | GenericEnum.NewType<T> | GenericEnum.Seq<T, U> | GenericEnum.Map<T, U>;"#
    };

    assert_eq!(GenericEnum::<(), ()>::DECL, expected);
}
