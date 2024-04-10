#![allow(dead_code)]

use indoc::indoc;
use pretty_assertions::assert_eq;
use tsify_next::Tsify;

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
