#![allow(dead_code)]

use indoc::{formatdoc, indoc};
use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};
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
        export type GenericEnum<T, U> = GenericEnum.Unit | GenericEnum.NewType<T> | GenericEnum.Seq<T, U> | GenericEnum.Map<T, U>;"#
    };

    assert_eq!(GenericEnum::<(), ()>::DECL, expected);
}

#[test]
fn test_generics_with_default_params() {
    #![allow(deprecated)]

    // What `()` serializes to, and so what a `C = ()` default declares.
    let unit = if cfg!(feature = "js") {
        "undefined"
    } else {
        "null"
    };

    #[derive(Serialize, Tsify)]
    #[tsify(into_wasm_abi)]
    struct SerNamedTuple<A = i32, B = String, C = ()>(A, B, C);

    let expected =
        format!("export type SerNamedTuple<A = number, B = string, C = {unit}> = [A, B, C];");

    assert_eq!(SerNamedTuple::<(), (), ()>::DECL, expected);

    #[derive(Deserialize, Tsify)]
    #[tsify(from_wasm_abi)]
    struct DeNamedTuple<A = i32, B = String, C = ()>(A, B, C);

    let expected =
        format!("export type DeNamedTuple<A = number, B = string, C = {unit}> = [A, B, C];");

    assert_eq!(DeNamedTuple::<(), (), ()>::DECL, expected);

    #[derive(Serialize, Tsify)]
    #[tsify(into_wasm_abi)]
    struct SerNamedMap<A, B = (), C = i32> {
        a: A,
        b: B,
        c: C,
    }

    let expected = formatdoc! {r#"
        export interface SerNamedMap<A, B = {unit}, C = number> {{
            a: A;
            b: B;
            c: C;
        }}"#
    };

    assert_eq!(SerNamedMap::<(), (), ()>::DECL, expected);

    #[derive(Deserialize, Tsify)]
    #[tsify(from_wasm_abi)]
    struct DeNamedMap<A, B = (), C = i32> {
        a: A,
        b: B,
        c: C,
    }

    let expected = formatdoc! {r#"
        export interface DeNamedMap<A, B = {unit}, C = number> {{
            a: A;
            b: B;
            c: C;
        }}"#
    };

    assert_eq!(DeNamedMap::<(), (), ()>::DECL, expected);

    #[derive(Serialize, Tsify)]
    #[tsify(into_wasm_abi)]
    enum SerEnum<A, B = (), C = i32> {
        Unit,
        NewType(A),
        Seq(i8, B),
        Map { a: i8, b: B, c: C },
    }

    let expected = format!(
        "export type SerEnum<A, B = {unit}, C = number> = \"Unit\" | {{ NewType: A }} | {{ Seq: [number, B] }} | {{ Map: {{ a: number; b: B; c: C }} }};"
    );

    assert_eq!(SerEnum::<(), (), ()>::DECL, expected);

    #[derive(Deserialize, Tsify)]
    #[tsify(from_wasm_abi)]
    enum DeEnum<A, B = (), C = i32> {
        Unit,
        NewType(A),
        Seq(i8, B),
        Map { a: i8, b: B, c: C },
    }

    let expected = format!(
        "export type DeEnum<A, B = {unit}, C = number> = \"Unit\" | {{ NewType: A }} | {{ Seq: [number, B] }} | {{ Map: {{ a: number; b: B; c: C }} }};"
    );

    assert_eq!(DeEnum::<(), (), ()>::DECL, expected);
}

#[test]
fn test_default_param_is_declared_where_it_belongs() {
    #[derive(Tsify)]
    struct Nested<T = Vec<Option<u32>>> {
        x: T,
    }

    let expected = if cfg!(feature = "js") {
        indoc! {"
            export interface Nested<T = (number | undefined)[]> {
                x: T;
            }"}
    } else {
        indoc! {"
            export interface Nested<T = (number | null)[]> {
                x: T;
            }"}
    };

    assert_eq!(Nested::<()>::DECL, expected);

    // A default may name another parameter, as long as that one is declared.
    #[derive(Tsify)]
    struct NamesParam<A, B = A> {
        a: A,
        b: B,
    }

    assert_eq!(
        NamesParam::<(), ()>::DECL,
        indoc! {"
            export interface NamesParam<A, B = A> {
                a: A;
                b: B;
            }"}
    );

    // A parameter no field mentions is declared nowhere, so a default naming it
    // would point at a type that does not exist. The default goes instead.
    #[derive(Tsify)]
    struct NamesUndeclaredParam<A, B = A> {
        #[serde(skip)]
        a: std::marker::PhantomData<A>,
        b: B,
    }

    assert_eq!(
        NamesUndeclaredParam::<(), ()>::DECL,
        indoc! {"
            export interface NamesUndeclaredParam<B> {
                b: B;
            }"}
    );

    // Dropping that default would leave `<B = number, C>`, which TypeScript
    // rejects: a default may only appear on a trailing run of parameters. The
    // earlier default goes with it.
    #[derive(Tsify)]
    struct GapInTheMiddle<A, B = i32, C = A> {
        #[serde(skip)]
        a: std::marker::PhantomData<A>,
        b: B,
        c: C,
    }

    assert_eq!(
        GapInTheMiddle::<(), (), ()>::DECL,
        indoc! {"
            export interface GapInTheMiddle<B, C> {
                b: B;
                c: C;
            }"}
    );
}

#[test]
fn test_default_param_in_a_namespace() {
    // The declaration inside the namespace takes the default; the reference to
    // it from the union may not, and neither may the alias it is written as.
    #[derive(Tsify)]
    #[tsify(namespace)]
    enum Spaced<T = u32> {
        A(T),
    }

    assert_eq!(
        Spaced::<()>::DECL,
        indoc! {"
            declare namespace Spaced {
                export type A<T = number> = { A: T };
            }

            export type Spaced<T = number> = Spaced.A<T>;"}
    );
}

#[test]
fn test_container_type_params_override_keeps_its_default() {
    trait Trait {
        type Assoc;
    }

    // `type_params` is written as TypeScript, so whatever it says is what gets
    // declared -- including a default the Rust type does not have.
    #[derive(Tsify)]
    #[tsify(type_params = "T = string")]
    struct Foo<T: Trait> {
        #[tsify(type = "T")]
        bar: T::Assoc,
    }

    #[derive(Tsify)]
    #[tsify(type = "{ Assoc: string }")]
    struct Bar;

    impl Trait for Bar {
        type Assoc = String;
    }

    assert_eq!(
        Foo::<Bar>::DECL,
        indoc! {"
            export interface Foo<T = string> {
                bar: T;
            }"}
    );
}
