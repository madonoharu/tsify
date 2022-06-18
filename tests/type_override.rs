#![allow(dead_code)]

use indoc::indoc;
use tsify::Tsify;

struct Unsupported;

#[test]
fn test_struct_with_type_override() {
    #[derive(Tsify)]
    struct Struct {
        a: i32,
        #[tsify(type = "0 | 1 | 2")]
        b: i32,
        #[tsify(type = "string | null")]
        c: Unsupported,
    }

    #[derive(Tsify)]
    struct Newtype(#[tsify(type = "string | null")] Unsupported);

    assert_eq!(
        Struct::DECL,
        indoc! {r#"
            export interface Struct {
                a: number;
                b: 0 | 1 | 2;
                c: string | null;
            }"#
        }
    );

    assert_eq!(Newtype::DECL, "export type Newtype = string | null;");
}

#[test]
fn test_enum_with_type_override() {
    #[derive(Tsify)]
    enum Enum {
        Struct {
            #[tsify(type = "`tpl_lit_${string}`")]
            x: String,
            #[tsify(type = "0 | 1 | 2")]
            y: i32,
        },
        Tuple(
            #[tsify(type = "`tpl_lit_${string}`")] String,
            #[tsify(type = "0 | 1 | 2")] i32,
        ),
        Newtype(#[tsify(type = "number")] Unsupported),
    }

    let expected = concat!(
        r#"declare namespace Enum {"#, "\n",
        r#"    export type EnumStruct = { Struct: { x: `tpl_lit_${string}`; y: 0 | 1 | 2 } };"#, "\n",
        r#"    export type EnumTuple = { Tuple: [`tpl_lit_${string}`, 0 | 1 | 2] };"#, "\n",
        r#"    export type EnumNewtype = { Newtype: number };"#, "\n",
        r#"}"#, "\n",
        r#""#, "\n",
        r#"export type Enum = Enum.EnumStruct | Enum.EnumTuple | Enum.EnumNewtype;"#
    );

    assert_eq!(Enum::DECL, expected);
}
