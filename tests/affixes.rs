#![allow(dead_code)]

use indoc::indoc;
use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tsify::Tsify;

#[test]
fn test_prefix() {
    type MyType = u32;

    #[derive(Tsify)]
    #[tsify(type_prefix = "Special")]
    struct PrefixedStruct {
        // Make sure that prefix isn't applied to builtin types
        x: u32,
        y: MyType,
    }

    assert_eq!(
        PrefixedStruct::DECL,
        indoc! {"
            export interface SpecialPrefixedStruct {
                x: number;
                y: SpecialMyType;
            }"
        }
    );

    #[derive(Tsify)]
    #[tsify(type_prefix = "Special")]
    enum PrefixedEnum {
        VariantA(MyType),
        VariantB(u32),
    }

    assert_eq!(
        PrefixedEnum::DECL,
        indoc! {"
            export type SpecialPrefixedEnum = { VariantA: SpecialMyType } | { VariantB: number };"
        }
    );
}

#[test]
fn test_suffix() {
    type MyType = u32;

    #[derive(Tsify)]
    #[tsify(type_suffix = "Special")]
    struct SuffixedStruct {
        // Make sure that prefix isn't applied to builtin types
        x: u32,
        y: MyType,
    }

    assert_eq!(
        SuffixedStruct::DECL,
        indoc! {"
            export interface SuffixedStructSpecial {
                x: number;
                y: MyTypeSpecial;
            }"
        }
    );

    #[derive(Tsify)]
    #[tsify(type_suffix = "Special")]
    enum SuffixedEnum {
        VariantA(MyType),
        VariantB(u32),
    }

    assert_eq!(
        SuffixedEnum::DECL,
        indoc! {"
            export type SuffixedEnumSpecial = { VariantA: MyTypeSpecial } | { VariantB: number };"
        }
    );
}

#[test]
fn test_prefix_suffix() {
    type MyType = u32;

    #[derive(Tsify)]
    #[tsify(type_prefix = "Pre", type_suffix = "Suf")]
    struct DoubleAffixedStruct {
        // Make sure that prefix isn't applied to builtin types
        x: u32,
        y: MyType,
    }

    assert_eq!(
        DoubleAffixedStruct::DECL,
        indoc! {"
            export interface PreDoubleAffixedStructSuf {
                x: number;
                y: PreMyTypeSuf;
            }"
        }
    );

    #[derive(Tsify)]
    #[tsify(type_prefix = "Pre", type_suffix = "Suf")]
    enum DoubleAffixedEnum {
        VariantA(MyType),
        VariantB(u32),
    }

    assert_eq!(
        DoubleAffixedEnum::DECL,
        indoc! {"
            export type PreDoubleAffixedEnumSuf = { VariantA: PreMyTypeSuf } | { VariantB: number };"
        }
    );
}

#[test]
fn test_affix_leaves_type_parameters_alone() {
    #[derive(Tsify)]
    #[tsify(type_prefix = "Ts")]
    struct PrefixedGeneric<T> {
        x: T,
        y: u32,
    }

    assert_eq!(
        PrefixedGeneric::<u32>::DECL,
        indoc! {"
            export interface TsPrefixedGeneric<T> {
                x: T;
                y: number;
            }"
        }
    );

    #[derive(Tsify)]
    #[tsify(type_suffix = "Sfx")]
    struct SuffixedGeneric<T> {
        x: T,
    }

    assert_eq!(
        SuffixedGeneric::<u32>::DECL,
        indoc! {"
            export interface SuffixedGenericSfx<T> {
                x: T;
            }"
        }
    );

    #[derive(Tsify)]
    #[tsify(namespace, type_prefix = "Ts")]
    enum PrefixedGenericEnum<T> {
        A(T),
        B(u32),
    }

    assert_eq!(
        PrefixedGenericEnum::<u32>::DECL,
        indoc! {"
            declare namespace TsPrefixedGenericEnum {
                export type A<T> = { A: T };
                export type B = { B: number };
            }

            export type TsPrefixedGenericEnum<T> = TsPrefixedGenericEnum.A<T> | TsPrefixedGenericEnum.B;"
        }
    );
}

#[test]
fn test_affix_leaves_nested_type_parameters_alone() {
    type MyAlias = u32;

    #[derive(Tsify)]
    #[tsify(type_prefix = "Ts")]
    struct Nested<T, K, V> {
        vector: Vec<T>,
        map: HashMap<K, V>,
        optional: Option<T>,
        tuple: (T, MyAlias),
        boxed: Box<T>,
        function: fn(T) -> Option<T>,
        deeply_nested: Option<Vec<Box<T>>>,
    }

    let expected = if cfg!(feature = "js") {
        indoc! {"
            export interface TsNested<T, K, V> {
                vector: T[];
                map: Map<K, V>;
                optional: T | undefined;
                tuple: [T, TsMyAlias];
                boxed: T;
                function: (arg0: T) => T | undefined;
                deeply_nested: T[] | undefined;
            }"
        }
    } else {
        indoc! {"
            export interface TsNested<T, K, V> {
                vector: T[];
                map: Record<K, V>;
                optional: T | null;
                tuple: [T, TsMyAlias];
                boxed: T;
                function: (arg0: T) => T | null;
                deeply_nested: T[] | null;
            }"
        }
    };

    assert_eq!(Nested::<u32, String, bool>::DECL, expected);

    #[derive(Tsify)]
    #[tsify(type_prefix = "Ts")]
    struct Mixed<T> {
        param: T,
        real: MyAlias,
    }

    assert_eq!(
        Mixed::<u32>::DECL,
        indoc! {"
            export interface TsMixed<T> {
                param: T;
                real: TsMyAlias;
            }"
        }
    );
}

#[test]
fn test_affix_nested_generic_namespace() {
    struct Wrapper<T>(T);

    #[derive(Tsify)]
    #[tsify(namespace, type_prefix = "Ts")]
    enum NestedNamespace<T> {
        Wrapped(Wrapper<Vec<T>>),
        Empty,
    }

    assert_eq!(
        NestedNamespace::<u32>::DECL,
        indoc! {r#"
            type __TsNestedNamespaceTsWrapper<A> = TsWrapper<A>;
            declare namespace TsNestedNamespace {
                export type Wrapped<T> = { Wrapped: __TsNestedNamespaceTsWrapper<T[]> };
                export type Empty = "Empty";
            }

            export type TsNestedNamespace<T> = TsNestedNamespace.Wrapped<T> | TsNestedNamespace.Empty;"#
        }
    );
}

#[test]
fn test_affix_leaves_the_wire_tag_alone() {
    #[derive(Tsify, Serialize, Deserialize)]
    #[serde(tag = "kind")]
    #[tsify(type_prefix = "A")]
    struct Config {
        x: i32,
    }

    assert_eq!(
        Config::DECL,
        indoc! {r#"
            export interface AConfig {
                kind: "Config";
                x: number;
            }"#
        }
    );

    // The tag literal is a value serde writes, so pin it against serde rather than
    // against itself: the two cannot drift apart without this noticing.
    let json = serde_json::to_string(&Config { x: 1 }).unwrap();
    assert!(json.contains(r#""kind":"Config""#), "serde wrote {json}");

    // `#[serde(rename)]` still reaches the tag, since that one does change the wire.
    #[derive(Tsify, Serialize, Deserialize)]
    #[serde(tag = "kind", rename = "WireName")]
    #[tsify(type_prefix = "A")]
    struct Renamed {
        x: i32,
    }

    assert_eq!(
        Renamed::DECL,
        indoc! {r#"
            export interface ARenamed {
                kind: "WireName";
                x: number;
            }"#
        }
    );
}

#[test]
fn test_crate_wide_affix_resolves_generic_references() {
    #[derive(Tsify)]
    #[tsify(type_prefix = "Ts")]
    struct Inner<T> {
        x: T,
    }

    #[derive(Tsify)]
    #[tsify(type_prefix = "Ts")]
    struct Outer<T, U> {
        one: Inner<T>,
        other: Inner<U>,
        many: Vec<Inner<T>>,
    }

    assert_eq!(
        Inner::<u32>::DECL,
        indoc! {"
            export interface TsInner<T> {
                x: T;
            }"
        }
    );

    assert_eq!(
        Outer::<u32, bool>::DECL,
        indoc! {"
            export interface TsOuter<T, U> {
                one: TsInner<T>;
                other: TsInner<U>;
                many: TsInner<T>[];
            }"
        }
    );
}
