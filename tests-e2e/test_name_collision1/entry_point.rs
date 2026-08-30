#![allow(dead_code)]

// #115 — a type of your own whose name matches one tsify special-cases is
// rendered as the built-in wherever it is referenced. The declaration is
// right; the reference is a different type, and it type-checks, so nothing
// reports it.
//
// The reference records the current output rather than the intended one, the
// way `test_generic_args1` records #76.
//
// Measured while writing this: whether a name collides depends on the arm.
// `String`, `str`, `char`, `Path`, `PathBuf`, `bool`, `ByteBuf`, `Duration`,
// `SystemTime` and the `Fn` family match on the identifier alone, so a type of
// yours by one of those names is replaced whatever shape it has. The rest —
// `Option`, `Result`, `Vec`, `HashMap`, `Range` and so on — are guarded by the
// number of type arguments, so they collide only when yours takes the same
// number. #115 lists them together; the reference below separates them.

use serde::{Deserialize, Serialize};
use tsify::Tsify;

// ── Collides on the name alone ───────────────────────────────────────────────

#[derive(Tsify, Serialize, Deserialize)]
pub struct Duration {
    pub secs: u64,
}

// Broken: renders `std::time::Duration`'s shape, not the interface above.
#[derive(Tsify, Serialize, Deserialize)]
pub struct HoldsDuration {
    pub d: crate::Duration,
}

// The types the shadowed names would otherwise be. Kept so that a fix which
// stops special-casing altogether shows up here rather than only above.
#[derive(Tsify, Serialize, Deserialize)]
pub struct HoldsStdTypes {
    pub std_duration: std::time::Duration,
    pub std_range: std::ops::Range<u32>,
}

// ── Names the prelude also uses ──────────────────────────────────────────────

// Shadowing these at the crate root would reach the code the derive generates,
// so they live in a module of their own. The arm matches the terminal
// identifier, so the path makes no difference. The first three are guarded by
// argument count; `Fn` is not.
pub mod shadowed {
    use super::*;

    // One argument, and the arm wants one: broken, renders as a std range.
    #[derive(Tsify, Serialize, Deserialize)]
    pub struct Range<T> {
        pub label: T,
    }

    // Two arguments, and the arm wants two: broken, renders as a union.
    #[derive(Tsify, Serialize, Deserialize)]
    pub struct Result<T, E> {
        pub good: T,
        pub bad: E,
    }

    // Same name, wrong arity for the arm: this one survives.
    #[derive(Tsify, Serialize, Deserialize)]
    pub struct Option<T, U> {
        pub a: T,
        pub b: U,
    }

    // `Fn` is not a Rust keyword, and its arm carries no arity guard, so a
    // type of yours by that name is replaced however many arguments it has.
    #[derive(Tsify, Serialize, Deserialize)]
    pub struct Fn {
        pub called: bool,
    }
}

#[derive(Tsify, Serialize, Deserialize)]
pub struct HoldsGuarded {
    pub range: shadowed::Range<String>,
    pub result: shadowed::Result<u32, String>,
    pub option: shadowed::Option<u32, String>,
    pub func: shadowed::Fn,
}

// ── Control ──────────────────────────────────────────────────────────────────

#[derive(Tsify, Serialize, Deserialize)]
pub struct Interval {
    pub secs: u64,
}

#[derive(Tsify, Serialize, Deserialize)]
pub struct HoldsInterval {
    pub i: crate::Interval,
}
