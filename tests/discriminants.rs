#![allow(dead_code)]

use indoc::indoc;
use pretty_assertions::assert_eq;
use tsify::Tsify;

struct Foo {
    a: i32,
    b: String,
}

#[test]
fn test_externally_tagged_enum_with_discriminants() {
    /// Comment for External
    #[derive(Tsify)]
    #[tsify(discriminants = "Disc")]
    enum External {
        /// Comment for Struct
        Struct { x: String, y: i32 },
        /// Comment for EmptyStruct
        EmptyStruct {},
        /// Comment for Tuple
        Tuple(i32, String),
        /// Comment for EmptyTuple
        EmptyTuple(),
        /// Comment for Newtype
        Newtype(Foo),
        /// Comment for Unit
        Unit,
    }

    let expected = indoc! {r#"
        export enum Disc {
            /**
             * Comment for Struct
             */
            Struct = "Struct",
            /**
             * Comment for EmptyStruct
             */
            EmptyStruct = "EmptyStruct",
            /**
             * Comment for Tuple
             */
            Tuple = "Tuple",
            /**
             * Comment for EmptyTuple
             */
            EmptyTuple = "EmptyTuple",
            /**
             * Comment for Newtype
             */
            Newtype = "Newtype",
            /**
             * Comment for Unit
             */
            Unit = "Unit",
        }

        /**
         * Comment for External
         */
        export type External = {
            [Disc.Struct]: {
                x: string;
                y: number;
            };
        } | {
            [Disc.EmptyStruct]: {};
        } | {
            [Disc.Tuple]: [number, string];
        } | {
            [Disc.EmptyTuple]: [];
        } | {
            [Disc.Newtype]: Foo;
        } | Disc.Unit;"#
    };

    assert_eq!(External::DECL, expected);
}

#[test]
fn test_externally_tagged_enum_with_namespace_and_discriminants() {
    /// Comment for External
    #[derive(Tsify)]
    #[tsify(namespace, discriminants)]
    enum External {
        /// Comment for Struct
        Struct { x: String, y: i32 },
        /// Comment for EmptyStruct
        EmptyStruct {},
        /// Comment for Tuple
        Tuple(i32, String),
        /// Comment for EmptyTuple
        EmptyTuple(),
        /// Comment for Newtype
        Newtype(Foo),
        /// Comment for Unit
        Unit,
    }

    let expected = indoc! {r#"
        export enum ExternalType {
            /**
             * Comment for Struct
             */
            Struct = "Struct",
            /**
             * Comment for EmptyStruct
             */
            EmptyStruct = "EmptyStruct",
            /**
             * Comment for Tuple
             */
            Tuple = "Tuple",
            /**
             * Comment for EmptyTuple
             */
            EmptyTuple = "EmptyTuple",
            /**
             * Comment for Newtype
             */
            Newtype = "Newtype",
            /**
             * Comment for Unit
             */
            Unit = "Unit",
        }

        type __ExternalFoo = Foo;
        /**
         * Comment for External
         */
        declare namespace External {
            /**
             * Comment for Struct
             */
            export type Struct = {
                [ExternalType.Struct]: {
                    x: string;
                    y: number;
                };
            };
            /**
             * Comment for EmptyStruct
             */
            export type EmptyStruct = {
                [ExternalType.EmptyStruct]: {};
            };
            /**
             * Comment for Tuple
             */
            export type Tuple = {
                [ExternalType.Tuple]: [number, string];
            };
            /**
             * Comment for EmptyTuple
             */
            export type EmptyTuple = {
                [ExternalType.EmptyTuple]: [];
            };
            /**
             * Comment for Newtype
             */
            export type Newtype = {
                [ExternalType.Newtype]: __ExternalFoo;
            };
            /**
             * Comment for Unit
             */
            export type Unit = ExternalType.Unit;
        }

        /**
         * Comment for External
         */
        export type External = External.Struct | External.EmptyStruct | External.Tuple | External.EmptyTuple | External.Newtype | External.Unit;"#

    };

    assert_eq!(External::DECL, expected);
}

#[test]
fn test_internally_tagged_enum_with_discriminants() {
    /// Comment for Internal
    #[derive(Tsify)]
    #[tsify(discriminants)]
    #[serde(tag = "t")]
    enum Internal {
        /// Comment for Struct
        Struct { x: String, y: i32 },
        /// Comment for EmptyStruct
        EmptyStruct {},
        /// Comment for Newtype
        Newtype(Foo),
        /// Comment for Unit
        Unit,
    }

    let expected = indoc! {r#"
        export enum InternalT {
            /**
             * Comment for Struct
             */
            Struct = "Struct",
            /**
             * Comment for EmptyStruct
             */
            EmptyStruct = "EmptyStruct",
            /**
             * Comment for Newtype
             */
            Newtype = "Newtype",
            /**
             * Comment for Unit
             */
            Unit = "Unit",
        }

        /**
         * Comment for Internal
         */
        export type Internal = {
            t: InternalT.Struct;
            x: string;
            y: number;
        } | {
            t: InternalT.EmptyStruct;
        } | ({
            t: InternalT.Newtype;
        } & Foo) | {
            t: InternalT.Unit;
        };"#
    };

    assert_eq!(Internal::DECL, expected);
}

#[test]
fn test_internally_tagged_enum_with_namespace_and_discriminants() {
    /// Comment for Internal
    #[derive(Tsify)]
    #[serde(tag = "t")]
    #[tsify(namespace, discriminants)]
    enum Internal {
        /// Comment for Struct
        Struct { x: String, y: i32 },
        /// Comment for EmptyStruct
        EmptyStruct {},
        /// Comment for Newtype
        Newtype(Foo),
        /// Comment for Unit
        Unit,
    }

    let expected = indoc! {r#"
        export enum InternalT {
            /**
             * Comment for Struct
             */
            Struct = "Struct",
            /**
             * Comment for EmptyStruct
             */
            EmptyStruct = "EmptyStruct",
            /**
             * Comment for Newtype
             */
            Newtype = "Newtype",
            /**
             * Comment for Unit
             */
            Unit = "Unit",
        }

        type __InternalFoo = Foo;
        /**
         * Comment for Internal
         */
        declare namespace Internal {
            /**
             * Comment for Struct
             */
            export type Struct = {
                t: InternalT.Struct;
                x: string;
                y: number;
            };
            /**
             * Comment for EmptyStruct
             */
            export type EmptyStruct = {
                t: InternalT.EmptyStruct;
            };
            /**
             * Comment for Newtype
             */
            export type Newtype = {
                t: InternalT.Newtype;
            } & __InternalFoo;
            /**
             * Comment for Unit
             */
            export type Unit = {
                t: InternalT.Unit;
            };
        }

        /**
         * Comment for Internal
         */
        export type Internal = Internal.Struct | Internal.EmptyStruct | Internal.Newtype | Internal.Unit;"#
    };

    assert_eq!(Internal::DECL, expected);
}

#[test]
fn test_untagged_enum() {
    /// Comment for Untagged
    #[derive(Tsify)]
    #[tsify(discriminants)]
    #[serde(untagged)]
    enum Untagged {
        /// Comment for Struct
        Struct { x: String, y: i32 },
        /// Comment for EmptyStruct
        EmptyStruct {},
        /// Comment for Tuple
        Tuple(i32, String),
        /// Comment for EmptyTuple
        EmptyTuple(),
        /// Comment for Newtype
        Newtype(Foo),
        /// Comment for Unit
        Unit,
    }

    let expected = if cfg!(feature = "js") {
        indoc! {r#"
            export enum UntaggedType {
                /**
                 * Comment for Struct
                 */
                Struct = "Struct",
                /**
                 * Comment for EmptyStruct
                 */
                EmptyStruct = "EmptyStruct",
                /**
                 * Comment for Tuple
                 */
                Tuple = "Tuple",
                /**
                 * Comment for EmptyTuple
                 */
                EmptyTuple = "EmptyTuple",
                /**
                 * Comment for Newtype
                 */
                Newtype = "Newtype",
                /**
                 * Comment for Unit
                 */
                Unit = "Unit",
            }

            /**
             * Comment for Untagged
             */
            export type Untagged = {
                x: string;
                y: number;
            } | {} | [number, string] | [] | Foo | undefined;"#
        }
    } else {
        indoc! {r#"
            export enum UntaggedType {
                /**
                 * Comment for Struct
                 */
                Struct = "Struct",
                /**
                 * Comment for EmptyStruct
                 */
                EmptyStruct = "EmptyStruct",
                /**
                 * Comment for Tuple
                 */
                Tuple = "Tuple",
                /**
                 * Comment for EmptyTuple
                 */
                EmptyTuple = "EmptyTuple",
                /**
                 * Comment for Newtype
                 */
                Newtype = "Newtype",
                /**
                 * Comment for Unit
                 */
                Unit = "Unit",
            }

            /**
             * Comment for Untagged
             */
            export type Untagged = {
                x: string;
                y: number;
            } | {} | [number, string] | [] | Foo | null;"#
        }
    };

    assert_eq!(Untagged::DECL, expected);
}

#[test]
fn test_untagged_enum_with_namespace() {
    /// Comment for Untagged
    #[derive(Tsify)]
    #[serde(untagged)]
    #[tsify(namespace, discriminants)]
    enum Untagged {
        /// Comment for Struct
        Struct { x: String, y: i32 },
        /// Comment for EmptyStruct
        EmptyStruct {},
        /// Comment for Tuple
        Tuple(i32, String),
        /// Comment for EmptyTuple
        EmptyTuple(),
        /// Comment for Newtype
        Newtype(Foo),
        /// Comment for Unit
        Unit,
    }

    let expected = if cfg!(feature = "js") {
        indoc! {r#"
            export enum UntaggedType {
                /**
                 * Comment for Struct
                 */
                Struct = "Struct",
                /**
                 * Comment for EmptyStruct
                 */
                EmptyStruct = "EmptyStruct",
                /**
                 * Comment for Tuple
                 */
                Tuple = "Tuple",
                /**
                 * Comment for EmptyTuple
                 */
                EmptyTuple = "EmptyTuple",
                /**
                 * Comment for Newtype
                 */
                Newtype = "Newtype",
                /**
                 * Comment for Unit
                 */
                Unit = "Unit",
            }

            type __UntaggedFoo = Foo;
            /**
             * Comment for Untagged
             */
            declare namespace Untagged {
                /**
                 * Comment for Struct
                 */
                export type Struct = {
                    x: string;
                    y: number;
                };
                /**
                 * Comment for EmptyStruct
                 */
                export type EmptyStruct = {};
                /**
                 * Comment for Tuple
                 */
                export type Tuple = [number, string];
                /**
                 * Comment for EmptyTuple
                 */
                export type EmptyTuple = [];
                /**
                 * Comment for Newtype
                 */
                export type Newtype = __UntaggedFoo;
                /**
                 * Comment for Unit
                 */
                export type Unit = undefined;
            }

            /**
             * Comment for Untagged
             */
            export type Untagged = Untagged.Struct | Untagged.EmptyStruct | Untagged.Tuple | Untagged.EmptyTuple | Untagged.Newtype | Untagged.Unit;"#
        }
    } else {
        indoc! {r#"
            export enum UntaggedType {
                /**
                 * Comment for Struct
                 */
                Struct = "Struct",
                /**
                 * Comment for EmptyStruct
                 */
                EmptyStruct = "EmptyStruct",
                /**
                 * Comment for Tuple
                 */
                Tuple = "Tuple",
                /**
                 * Comment for EmptyTuple
                 */
                EmptyTuple = "EmptyTuple",
                /**
                 * Comment for Newtype
                 */
                Newtype = "Newtype",
                /**
                 * Comment for Unit
                 */
                Unit = "Unit",
            }

            type __UntaggedFoo = Foo;
            /**
             * Comment for Untagged
             */
            declare namespace Untagged {
                /**
                 * Comment for Struct
                 */
                export type Struct = {
                    x: string;
                    y: number;
                };
                /**
                 * Comment for EmptyStruct
                 */
                export type EmptyStruct = {};
                /**
                 * Comment for Tuple
                 */
                export type Tuple = [number, string];
                /**
                 * Comment for EmptyTuple
                 */
                export type EmptyTuple = [];
                /**
                 * Comment for Newtype
                 */
                export type Newtype = __UntaggedFoo;
                /**
                 * Comment for Unit
                 */
                export type Unit = null;
            }

            /**
             * Comment for Untagged
             */
            export type Untagged = Untagged.Struct | Untagged.EmptyStruct | Untagged.Tuple | Untagged.EmptyTuple | Untagged.Newtype | Untagged.Unit;"#
        }
    };

    assert_eq!(Untagged::DECL, expected);
}

#[test]
fn test_enum_rename_all_fields_with_discriminants() {
    #[derive(Tsify)]
    #[serde(rename_all_fields = "camelCase")]
    #[tsify(discriminants)]
    enum Renamed {
        First { foo_bar: String, baz_quoox: i32 },
        Second { asdf_asdf: String, qwer_qwer: i32 },
    }

    let expected = indoc! {r#"
        export enum RenamedType {
            First = "First",
            Second = "Second",
        }

        export type Renamed = {
            [RenamedType.First]: {
                fooBar: string;
                bazQuoox: number;
            };
        } | {
            [RenamedType.Second]: {
                asdfAsdf: string;
                qwerQwer: number;
            };
        };"#
    };

    assert_eq!(Renamed::DECL, expected);
}

#[test]
fn test_enum_rename_all_with_discriminants() {
    /// Comment for Internal
    #[derive(Tsify)]
    #[serde(rename_all = "camelCase")]
    #[tsify(namespace, discriminants)]
    enum Internal {
        /// Comment for Struct
        Struct { x: String, y: i32 },
        /// Comment for EmptyStruct
        EmptyStruct {},
        /// Comment for Tuple
        Tuple(i32, String),
        /// Comment for EmptyTuple
        EmptyTuple(),
        /// Comment for Newtype
        Newtype(Foo),
        /// Comment for Unit
        Unit,
    }

    let expected = indoc! {r#"
        export enum InternalType {
            /**
             * Comment for Struct
             */
            struct = "struct",
            /**
             * Comment for EmptyStruct
             */
            emptyStruct = "emptyStruct",
            /**
             * Comment for Tuple
             */
            tuple = "tuple",
            /**
             * Comment for EmptyTuple
             */
            emptyTuple = "emptyTuple",
            /**
             * Comment for Newtype
             */
            newtype = "newtype",
            /**
             * Comment for Unit
             */
            unit = "unit",
        }

        type __InternalFoo = Foo;
        /**
         * Comment for Internal
         */
        declare namespace Internal {
            /**
             * Comment for Struct
             */
            export type struct = {
                [InternalType.struct]: {
                    x: string;
                    y: number;
                };
            };
            /**
             * Comment for EmptyStruct
             */
            export type emptyStruct = {
                [InternalType.emptyStruct]: {};
            };
            /**
             * Comment for Tuple
             */
            export type tuple = {
                [InternalType.tuple]: [number, string];
            };
            /**
             * Comment for EmptyTuple
             */
            export type emptyTuple = {
                [InternalType.emptyTuple]: [];
            };
            /**
             * Comment for Newtype
             */
            export type newtype = {
                [InternalType.newtype]: __InternalFoo;
            };
            /**
             * Comment for Unit
             */
            export type unit = InternalType.unit;
        }

        /**
         * Comment for Internal
         */
        export type Internal = Internal.struct | Internal.emptyStruct | Internal.tuple | Internal.emptyTuple | Internal.newtype | Internal.unit;"#
    };

    assert_eq!(Internal::DECL, expected);
}

#[test]
fn test_enum_rename_all_rename_variants_with_discriminants() {
    /// Comment for Internal
    #[derive(Tsify)]
    #[serde(rename_all = "camelCase")]
    #[tsify(namespace, discriminants, rename_variants)]
    enum Internal {
        /// Comment for Struct
        Struct { x: String, y: i32 },
        /// Comment for EmptyStruct
        EmptyStruct {},
        /// Comment for Tuple
        Tuple(i32, String),
        /// Comment for EmptyTuple
        EmptyTuple(),
        /// Comment for Newtype
        Newtype(Foo),
        /// Comment for Unit
        Unit,
    }

    let expected = indoc! {r#"
        export enum InternalType {
            /**
             * Comment for Struct
             */
            Struct = "struct",
            /**
             * Comment for EmptyStruct
             */
            EmptyStruct = "emptyStruct",
            /**
             * Comment for Tuple
             */
            Tuple = "tuple",
            /**
             * Comment for EmptyTuple
             */
            EmptyTuple = "emptyTuple",
            /**
             * Comment for Newtype
             */
            Newtype = "newtype",
            /**
             * Comment for Unit
             */
            Unit = "unit",
        }

        type __InternalFoo = Foo;
        /**
         * Comment for Internal
         */
        declare namespace Internal {
            /**
             * Comment for Struct
             */
            export type Struct = {
                [InternalType.Struct]: {
                    x: string;
                    y: number;
                };
            };
            /**
             * Comment for EmptyStruct
             */
            export type EmptyStruct = {
                [InternalType.EmptyStruct]: {};
            };
            /**
             * Comment for Tuple
             */
            export type Tuple = {
                [InternalType.Tuple]: [number, string];
            };
            /**
             * Comment for EmptyTuple
             */
            export type EmptyTuple = {
                [InternalType.EmptyTuple]: [];
            };
            /**
             * Comment for Newtype
             */
            export type Newtype = {
                [InternalType.Newtype]: __InternalFoo;
            };
            /**
             * Comment for Unit
             */
            export type Unit = InternalType.Unit;
        }

        /**
         * Comment for Internal
         */
        export type Internal = Internal.Struct | Internal.EmptyStruct | Internal.Tuple | Internal.EmptyTuple | Internal.Newtype | Internal.Unit;"#
    };

    assert_eq!(Internal::DECL, expected);
}
