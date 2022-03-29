#![allow(dead_code)]

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
        "export type GenericStruct<A, B, D> = { a: A; b: B; d: D };",
        GenericStruct::<(), (), (), ()>::DECL
    );

    #[derive(Tsify)]
    pub struct GenericNewtype<T>(T);

    assert_eq!(
        "export type GenericNewtype<T> = T;",
        GenericNewtype::<()>::DECL
    );

    #[derive(Tsify)]
    pub struct GenericTuple<'a, A, B, C, D>(A, #[serde(skip)] &'a B, C, D);

    assert_eq!(
        "export type GenericTuple<A, C, D> = [A, C, D];",
        GenericTuple::<(), (), (), ()>::DECL
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

    assert_eq!(
        r#"export type GenericEnum<T, U> = "Unit" | { NewType: T } | { Seq: [T, U] } | { Map: { x: T; y: U } };"#,
        GenericEnum::<(), ()>::DECL
    );
}
