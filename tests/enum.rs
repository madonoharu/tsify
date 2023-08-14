#![allow(dead_code)]

use indoc::indoc;
use pretty_assertions::assert_eq;
use tsify::Tsify;

struct Foo {
    a: i32,
    b: String,
}

#[test]
fn test_externally_tagged_enum() {
    /// Comment for External
    #[derive(Tsify)]
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
        /**
         * Comment for External
         */
        export type External = { Struct: { x: string; y: number } } | { EmptyStruct: {} } | { Tuple: [number, string] } | { EmptyTuple: [] } | { Newtype: Foo } | "Unit";"#
    };

    assert_eq!(External::DECL, expected);
}

#[test]
fn test_externally_tagged_enum_with_namespace() {
    /// Comment for External
    #[derive(Tsify)]
    #[tsify(namespace)]
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
        type __ExternalFoo = Foo;
        /**
         * Comment for External
         */
        declare namespace External {
            /**
             * Comment for Struct
             */
            export type Struct = { Struct: { x: string; y: number } };
            /**
             * Comment for EmptyStruct
             */
            export type EmptyStruct = { EmptyStruct: {} };
            /**
             * Comment for Tuple
             */
            export type Tuple = { Tuple: [number, string] };
            /**
             * Comment for EmptyTuple
             */
            export type EmptyTuple = { EmptyTuple: [] };
            /**
             * Comment for Newtype
             */
            export type Newtype = { Newtype: __ExternalFoo };
            /**
             * Comment for Unit
             */
            export type Unit = "Unit";
        }

        /**
         * Comment for External
         */
        export type External = { Struct: { x: string; y: number } } | { EmptyStruct: {} } | { Tuple: [number, string] } | { EmptyTuple: [] } | { Newtype: Foo } | "Unit";"#
    };

    assert_eq!(External::DECL, expected);
}

#[test]
fn test_internally_tagged_enum() {
    /// Comment for Internal
    #[derive(Tsify)]
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
        /**
         * Comment for Internal
         */
        export type Internal = { t: "Struct"; x: string; y: number } | { t: "EmptyStruct" } | ({ t: "Newtype" } & Foo) | { t: "Unit" };"#
    };

    assert_eq!(Internal::DECL, expected);
}

#[test]
fn test_internally_tagged_enum_with_namespace() {
    /// Comment for Internal
    #[derive(Tsify)]
    #[serde(tag = "t")]
    #[tsify(namespace)]
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
        type __InternalFoo = Foo;
        /**
         * Comment for Internal
         */
        declare namespace Internal {
            /**
             * Comment for Struct
             */
            export type Struct = { t: "Struct"; x: string; y: number };
            /**
             * Comment for EmptyStruct
             */
            export type EmptyStruct = { t: "EmptyStruct" };
            /**
             * Comment for Newtype
             */
            export type Newtype = { t: "Newtype" } & __InternalFoo;
            /**
             * Comment for Unit
             */
            export type Unit = { t: "Unit" };
        }

        /**
         * Comment for Internal
         */
        export type Internal = { t: "Struct"; x: string; y: number } | { t: "EmptyStruct" } | ({ t: "Newtype" } & Foo) | { t: "Unit" };"#
    };

    assert_eq!(Internal::DECL, expected);
}

#[test]
fn test_adjacently_tagged_enum() {
    /// Comment for Adjacent
    #[derive(Tsify)]
    #[serde(tag = "t", content = "c")]
    enum Adjacent {
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
        /**
         * Comment for Adjacent
         */
        export type Adjacent = { t: "Struct"; c: { x: string; y: number } } | { t: "EmptyStruct"; c: {} } | { t: "Tuple"; c: [number, string] } | { t: "EmptyTuple"; c: [] } | { t: "Newtype"; c: Foo } | { t: "Unit" };"#
    };

    assert_eq!(Adjacent::DECL, expected);
}

#[test]
fn test_adjacently_tagged_enum_with_namespace() {
    /// Comment for Adjacent
    #[derive(Tsify)]
    #[serde(tag = "t", content = "c")]
    #[tsify(namespace)]
    enum Adjacent {
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
        type __AdjacentFoo = Foo;
        /**
         * Comment for Adjacent
         */
        declare namespace Adjacent {
            /**
             * Comment for Struct
             */
            export type Struct = { t: "Struct"; c: { x: string; y: number } };
            /**
             * Comment for EmptyStruct
             */
            export type EmptyStruct = { t: "EmptyStruct"; c: {} };
            /**
             * Comment for Tuple
             */
            export type Tuple = { t: "Tuple"; c: [number, string] };
            /**
             * Comment for EmptyTuple
             */
            export type EmptyTuple = { t: "EmptyTuple"; c: [] };
            /**
             * Comment for Newtype
             */
            export type Newtype = { t: "Newtype"; c: __AdjacentFoo };
            /**
             * Comment for Unit
             */
            export type Unit = { t: "Unit" };
        }

        /**
         * Comment for Adjacent
         */
        export type Adjacent = { t: "Struct"; c: { x: string; y: number } } | { t: "EmptyStruct"; c: {} } | { t: "Tuple"; c: [number, string] } | { t: "EmptyTuple"; c: [] } | { t: "Newtype"; c: Foo } | { t: "Unit" };"#
    };

    assert_eq!(Adjacent::DECL, expected);
}

#[test]
fn test_untagged_enum() {
    /// Comment for Untagged
    #[derive(Tsify)]
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
            /**
             * Comment for Untagged
             */
            export type Untagged = { x: string; y: number } | {} | [number, string] | [] | Foo | undefined;"#
        }
    } else {
        indoc! {r#"
            /**
             * Comment for Untagged
             */
            export type Untagged = { x: string; y: number } | {} | [number, string] | [] | Foo | null;"#
        }
    };

    assert_eq!(Untagged::DECL, expected);
}

#[test]
fn test_untagged_enum_with_namespace() {
    /// Comment for Untagged
    #[derive(Tsify)]
    #[serde(untagged)]
    #[tsify(namespace)]
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
            type __UntaggedFoo = Foo;
            /**
             * Comment for Untagged
             */
            declare namespace Untagged {
                /**
                 * Comment for Struct
                 */
                export type Struct = { x: string; y: number };
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
            export type Untagged = { x: string; y: number } | {} | [number, string] | [] | Foo | undefined;"#
        }
    } else {
        indoc! {r#"
            type __UntaggedFoo = Foo;
            /**
             * Comment for Untagged
             */
            declare namespace Untagged {
                /**
                 * Comment for Struct
                 */
                export type Struct = { x: string; y: number };
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
            export type Untagged = { x: string; y: number } | {} | [number, string] | [] | Foo | null;"#
        }
    };

    assert_eq!(Untagged::DECL, expected);
}

#[test]
fn test_module_reimport_enum() {
    /// Comment for Internal
    #[derive(Tsify)]
    #[tsify(namespace)]
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
        /// Comment for Newtype2
        Newtype2(Foo),
        /// Comment for Unit
        Unit,
    }

    let expected = indoc! {r#"
        type __InternalFoo = Foo;
        /**
         * Comment for Internal
         */
        declare namespace Internal {
            /**
             * Comment for Struct
             */
            export type Struct = { Struct: { x: string; y: number } };
            /**
             * Comment for EmptyStruct
             */
            export type EmptyStruct = { EmptyStruct: {} };
            /**
             * Comment for Tuple
             */
            export type Tuple = { Tuple: [number, string] };
            /**
             * Comment for EmptyTuple
             */
            export type EmptyTuple = { EmptyTuple: [] };
            /**
             * Comment for Newtype
             */
            export type Newtype = { Newtype: __InternalFoo };
            /**
             * Comment for Newtype2
             */
            export type Newtype2 = { Newtype2: __InternalFoo };
            /**
             * Comment for Unit
             */
            export type Unit = "Unit";
        }

        /**
         * Comment for Internal
         */
        export type Internal = { Struct: { x: string; y: number } } | { EmptyStruct: {} } | { Tuple: [number, string] } | { EmptyTuple: [] } | { Newtype: Foo } | { Newtype2: Foo } | "Unit";"#
    };

    assert_eq!(Internal::DECL, expected);
}

#[test]
fn test_module_template_enum() {
    /// Comment for Test
    struct Test<T> {
        /// Comment for inner
        inner: T,
    }

    /// Comment for Internal
    #[derive(Tsify)]
    #[tsify(namespace)]
    enum Internal<T> {
        /// Comment for Newtype
        Newtype(Test<T>),
        /// Comment for NewtypeF
        NewtypeF(Test<Foo>),
        /// Comment for NewtypeL
        NewtypeL(Test<Foo>),
        /// Comment for Unit
        Unit,
    }
    let expected = indoc! {r#"
        type __InternalFoo = Foo;
        type __InternalTest<A> = Test<A>;
        /**
         * Comment for Internal
         */
        declare namespace Internal {
            /**
             * Comment for Newtype
             */
            export type Newtype<T> = { Newtype: __InternalTest<T> };
            /**
             * Comment for NewtypeF
             */
            export type NewtypeF = { NewtypeF: __InternalTest<__InternalFoo> };
            /**
             * Comment for NewtypeL
             */
            export type NewtypeL = { NewtypeL: __InternalTest<__InternalFoo> };
            /**
             * Comment for Unit
             */
            export type Unit = "Unit";
        }

        /**
         * Comment for Internal
         */
        export type Internal<T> = { Newtype: Test<T> } | { NewtypeF: Test<Foo> } | { NewtypeL: Test<Foo> } | "Unit";"#
    };

    assert_eq!(expected, Internal::<Foo>::DECL);
}

struct Test<T> {
    inner: T,
}

#[test]
fn test_module_template_enum_inner() {
    /// Comment for Test
    struct Test<T> {
        /// Comment for inner
        inner: T,
    }

    /// Comment for Internal
    #[derive(Tsify)]
    #[tsify(namespace)]
    enum Internal {
        /// Comment for Newtype
        Newtype(Test<Foo>),
        /// Comment for Unit
        Unit,
    }

    let expected = indoc! {r#"
        type __InternalFoo = Foo;
        type __InternalTest<A> = Test<A>;
        /**
         * Comment for Internal
         */
        declare namespace Internal {
            /**
             * Comment for Newtype
             */
            export type Newtype = { Newtype: __InternalTest<__InternalFoo> };
            /**
             * Comment for Unit
             */
            export type Unit = "Unit";
        }

        /**
         * Comment for Internal
         */
        export type Internal = { Newtype: Test<Foo> } | "Unit";"#
    };

    assert_eq!(Internal::DECL, expected);
}
