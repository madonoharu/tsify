//! What a name inside a default resolves to.
//!
//! A default is written inside the parameter list, so that is where its names
//! are read: against the parameters first, and the declarations around them
//! only after. Every case here is one where those two disagree.
#![allow(dead_code)]

use std::collections::HashMap;

use indoc::indoc;
use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

/// A type of our own whose TypeScript name a parameter below also uses. It has
/// to be named through a path for Rust to read it as the type rather than the
/// parameter, which is the situation this file is about.
#[derive(Tsify, Serialize, Deserialize)]
pub struct T {
    z: u32,
}

#[derive(Tsify, Serialize, Deserialize)]
pub struct Error {
    m: String,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(rename = "Renamed")]
pub struct Original {
    v: u32,
}

#[test]
fn test_default_naming_a_type_a_parameter_shadows() {
    // TypeScript reads the `T` in `U`'s default as the parameter declared after
    // it, which it rejects outright: `TS2744`.
    #[derive(Tsify, Serialize, Deserialize)]
    struct Later<U = crate::T, T = String> {
        u: U,
        t: T,
    }

    assert_eq!(
        Later::<(), ()>::DECL,
        indoc! {"
            export interface Later<U, T = string> {
                u: U;
                t: T;
            }"}
    );

    // Declared the other way round it is accepted, and means the parameter
    // rather than the interface — the same default, quietly reading as a
    // different type. Both lose the default; nothing else can be said in a
    // parameter list, where a parameter shadows.
    #[derive(Tsify, Serialize, Deserialize)]
    struct Earlier<T, U = crate::T> {
        t: T,
        u: U,
    }

    assert_eq!(
        Earlier::<(), ()>::DECL,
        indoc! {"
            export interface Earlier<T, U> {
                t: T;
                u: U;
            }"}
    );
}

#[test]
fn test_default_naming_a_synthetic_name_a_parameter_shadows() {
    // A `HashMap` renders as a name tsify makes up, and which one depends on
    // the feature. Whichever it is, a parameter of that name shadows it.
    #[derive(Tsify, Serialize, Deserialize)]
    struct ShadowsMap<Map, T = HashMap<String, u32>> {
        a: Map,
        t: T,
    }

    #[derive(Tsify, Serialize, Deserialize)]
    struct ShadowsRecord<Record, T = HashMap<String, u32>> {
        a: Record,
        t: T,
    }

    let (shadowed, untouched) = if cfg!(feature = "js") {
        (ShadowsMap::<(), ()>::DECL, ShadowsRecord::<(), ()>::DECL)
    } else {
        (ShadowsRecord::<(), ()>::DECL, ShadowsMap::<(), ()>::DECL)
    };

    assert!(
        !shadowed.contains(" = "),
        "the default should be gone: {shadowed}"
    );
    assert!(
        untouched.contains(" = "),
        "the default should be kept: {untouched}"
    );
}

#[test]
fn test_default_in_a_namespace_is_read_inside_it() {
    // `Error` in the default means the interface. Inside the namespace it would
    // reach the sibling variant instead, so it is rewritten to the alias the
    // namespace already hoists its other references to.
    #[derive(Tsify, Serialize, Deserialize)]
    #[tsify(namespace)]
    enum Outcome<T = crate::Error> {
        Done(T),
        Error(String),
    }

    assert_eq!(
        Outcome::<()>::DECL,
        indoc! {"
            type __OutcomeError = Error;
            declare namespace Outcome {
                export type Done<T = __OutcomeError> = { Done: T };
                export type Error = { Error: string };
            }

            export type Outcome<T = Error> = Outcome.Done<T> | Outcome.Error;"}
    );
}

#[test]
fn test_default_naming_a_renamed_declaration() {
    // A reference emits the Rust ident and does not follow `rename`, so this
    // names a type that is declared as `Renamed`: `TS2304`. A field of the same
    // type emits the same thing, which is
    // https://github.com/madonoharu/tsify/issues/103 and not particular to
    // defaults. `#[tsify(type_params = "T = Renamed")]` is the way to say it
    // until that is fixed.
    #[derive(Tsify, Serialize, Deserialize)]
    struct UsesRenamed<T = Original> {
        t: T,
    }

    #[derive(Tsify, Serialize, Deserialize)]
    struct HasRenamedField {
        o: Original,
    }

    assert_eq!(
        UsesRenamed::<()>::DECL,
        indoc! {"
            export interface UsesRenamed<T = Original> {
                t: T;
            }"}
    );

    assert_eq!(
        HasRenamedField::DECL,
        indoc! {"
            export interface HasRenamedField {
                o: Original;
            }"}
    );

    // Said through the attribute, it lands on the declared name.
    #[derive(Tsify, Serialize, Deserialize)]
    #[tsify(type_params = "T = Renamed")]
    struct SaysRenamed<T = Original> {
        t: T,
    }

    assert_eq!(
        SaysRenamed::<()>::DECL,
        indoc! {"
            export interface SaysRenamed<T = Renamed> {
                t: T;
            }"}
    );
}
