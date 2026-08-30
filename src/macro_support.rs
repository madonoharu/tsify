//! The name a type goes by inside a `#[wasm_bindgen]` signature.
//!
//! Not a public API. Everything here exists so that generated code can name a
//! type across the proc-macro boundary; the shape of it is free to change.
//!
//! wasm-bindgen does not read the `typescript_custom_section` when it writes a
//! signature. It reads a *descriptor* the module informs at build time, which
//! for a named type is `NAMED_EXTERNREF`, a character count, then one `u32` per
//! character. A `#[wasm_bindgen] extern` type carries one fixed name for all of
//! its uses, so on its own it can only ever describe `Response`, never
//! `Response<UserInfo>`.
//!
//! [`TsName`] rebuilds the name once per monomorphization instead: the derive
//! unrolls the characters of the type's own declared name and defers to each
//! type argument's own impl.
//!
//! # Two rules everything here obeys
//!
//! **Every character reaches the descriptor as a compile-time constant.**
//! wasm-bindgen interprets descriptors with an interpreter that mirrors the
//! size of linear memory but leaves it zero-filled, so a character read from
//! memory arrives as `\0`. That rules out walking a `&str`, and it rules out
//! `for c in ['a', 'b']`, which reads a materialized array: the loop runs zero
//! times and the descriptor is silently short.
//!
//! **No loops.** The interpreter only learned `loop`, `if`/`else` and branches
//! in wasm-bindgen 0.2.126, and this crate supports 0.2.104. A `while` in a
//! descriptor fails there with `unknown instruction Loop`, in dev builds; a
//! release build hides it, because rustc folds the loop away before the
//! interpreter sees it. Conditions on a `const` parameter are fine — those fold
//! at every profile — so [`ArrayName`] is unrolled behind them rather than
//! counted.
//!
//! A length that disagrees with the characters is the failure worth guarding
//! against: the count is what the decoder trusts, so a wrong one does not
//! truncate a name, it desynchronizes everything after it in the descriptor.

use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
use std::ops::{Range, RangeInclusive};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use wasm_bindgen::describe::{inform, NAMED_EXTERNREF, VECTOR};

/// `#[tsify(missing_as_null)]`.
pub const MISSING_AS_NULL: u8 = 1 << 0;
/// `#[tsify(hashmap_as_object)]`.
pub const HASHMAP_AS_OBJECT: u8 = 1 << 1;
/// `#[tsify(large_number_types_as_bigints)]`.
pub const LARGE_NUMBER_TYPES_AS_BIGINTS: u8 = 1 << 2;

/// The bits above are the whole protocol between the derive and this module.
/// Every other bit is reserved and ignored, so generated code from a newer
/// macro crate names types the same way an older one does rather than failing
/// to compile.
pub const KNOWN_CONFIG_BITS: u8 =
    MISSING_AS_NULL | HASHMAP_AS_OBJECT | LARGE_NUMBER_TYPES_AS_BIGINTS;

/// Whether a missing value is written as `undefined` rather than `null`.
const fn nullish_is_undefined(config: u8) -> bool {
    cfg!(feature = "js") && config & MISSING_AS_NULL == 0
}

/// Whether a map is written as a `Map` rather than a plain object.
const fn map_is_map(config: u8) -> bool {
    cfg!(feature = "js") && config & HASHMAP_AS_OBJECT == 0
}

/// Whether the 64-bit integers are written as `bigint`.
const fn wide_int_is_bigint(config: u8) -> bool {
    cfg!(feature = "js") && config & LARGE_NUMBER_TYPES_AS_BIGINTS != 0
}

/// The TypeScript name a type goes by, under the serialization config of the
/// value being written.
///
/// `CONFIG` is the config of the type the value is serialized *through*, not of
/// the type being named. One `serde_wasm_bindgen::Serializer` is built from the
/// root type's attributes and writes the whole value, nested types included, so
/// a name that followed the nested type's own attributes would describe a shape
/// that is never produced (see
/// <https://github.com/madonoharu/tsify/issues/125>). Every impl here threads
/// `CONFIG` down unchanged; only the root seeds it.
pub trait TsName<const CONFIG: u8 = 0> {
    /// The number of `char`s [`describe_name`](TsName::describe_name) informs.
    const NAME_LEN: u32;

    /// Informs the name one `char` at a time, in order.
    fn describe_name();

    /// Informs the whole `NAMED_EXTERNREF` descriptor for this name.
    #[inline]
    fn describe_named_externref() {
        inform(NAMED_EXTERNREF);
        inform(Self::NAME_LEN);
        Self::describe_name();
    }

    /// Informs the descriptor for a vector of this name.
    #[inline]
    fn describe_named_externref_vector() {
        inform(VECTOR);
        Self::describe_named_externref();
    }
}

/// Informs one `char` of a name.
///
/// Only ever pass a literal, or defer to another impl: a character that comes
/// from memory reaches wasm-bindgen as `\0`.
#[inline]
pub fn inform_char(c: char) {
    inform(c as u32);
}

/// Implements [`TsName`] for types whose name is fixed, spelled one `char` at a
/// time so that each one is an immediate.
macro_rules! ts_name {
    ($($ty:ty),+ $(,)? => $($ch:literal),+ $(,)?) => {
        ts_name!(@each [$($ch),+] $($ty),+);
    };

    (@each $name:tt $($ty:ty),+) => {
        $(ts_name!(@one $ty, $name);)+
    };

    (@one $ty:ty, [$($ch:literal),+]) => {
        impl<const C: u8> TsName<C> for $ty {
            const NAME_LEN: u32 = [$($ch),+].len() as u32;

            #[inline]
            fn describe_name() {
                $(inform_char($ch);)+
            }
        }
    };
}

/// The same, for a name the config chooses between.
macro_rules! ts_name_by_config {
    ($($ty:ty),+ $(,)? => if $cond:ident { $($yes:literal),+ } else { $($no:literal),+ }) => {
        ts_name_by_config!(@each $cond [$($yes),+] [$($no),+] $($ty),+);
    };

    (@each $cond:ident $yes:tt $no:tt $($ty:ty),+) => {
        $(ts_name_by_config!(@one $cond, $ty, $yes, $no);)+
    };

    (@one $cond:ident, $ty:ty, [$($yes:literal),+], [$($no:literal),+]) => {
        impl<const C: u8> TsName<C> for $ty {
            const NAME_LEN: u32 = if $cond(C) {
                [$($yes),+].len() as u32
            } else {
                [$($no),+].len() as u32
            };

            #[inline]
            fn describe_name() {
                if $cond(C) {
                    $(inform_char($yes);)+
                } else {
                    $(inform_char($no);)+
                }
            }
        }
    };
}

ts_name!(u8, u16, u32, i8, i16, i32, f32, f64 => 'n', 'u', 'm', 'b', 'e', 'r');
ts_name!(bool => 'b', 'o', 'o', 'l', 'e', 'a', 'n');
ts_name!(String, str, char, Path, PathBuf => 's', 't', 'r', 'i', 'n', 'g');

// `#[tsify(large_number_types_as_bigints)]` is the only reason these are not
// plain numbers, and it is read from the root of the value being written.
ts_name_by_config!(
    u64, i64, usize, isize
    => if wide_int_is_bigint { 'b', 'i', 'g', 'i', 'n', 't' } else { 'n', 'u', 'm', 'b', 'e', 'r' }
);

// 128-bit integers never fit a JS number; through JSON serde has already
// narrowed them, which is a feature question rather than a config one.
#[cfg(feature = "js")]
ts_name!(u128, i128 => 'b', 'i', 'g', 'i', 'n', 't');
#[cfg(not(feature = "js"))]
ts_name!(u128, i128 => 'n', 'u', 'm', 'b', 'e', 'r');

/// Wrappers serde sees through, so TypeScript does too.
macro_rules! ts_name_transparent {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl<const C: u8, T: TsName<C> + ?Sized> TsName<C> for $ty {
                const NAME_LEN: u32 = <T as TsName<C>>::NAME_LEN;

                #[inline]
                fn describe_name() {
                    <T as TsName<C>>::describe_name();
                }
            }
        )+
    };
}

ts_name_transparent!(&T, &mut T, Box<T>, Rc<T>, Arc<T>);

macro_rules! ts_name_transparent_sized {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl<const C: u8, T: TsName<C>> TsName<C> for $ty {
                const NAME_LEN: u32 = <T as TsName<C>>::NAME_LEN;

                #[inline]
                fn describe_name() {
                    <T as TsName<C>>::describe_name();
                }
            }
        )+
    };
}

ts_name_transparent_sized!(Cell<T>, RefCell<T>);

impl<const C: u8, T: TsName<C> + ToOwned + ?Sized> TsName<C> for Cow<'_, T> {
    const NAME_LEN: u32 = <T as TsName<C>>::NAME_LEN;

    #[inline]
    fn describe_name() {
        <T as TsName<C>>::describe_name();
    }
}

/// Sequences, as `T[]`.
///
/// The element is parenthesized where it has to be, which is why the union
/// forms below carry their own parentheses.
macro_rules! ts_name_array {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl<const C: u8, T: TsName<C>> TsName<C> for $ty {
                const NAME_LEN: u32 = <T as TsName<C>>::NAME_LEN + 2;

                #[inline]
                fn describe_name() {
                    <T as TsName<C>>::describe_name();
                    inform_char('[');
                    inform_char(']');
                }
            }
        )+
    };
}

ts_name_array!(
    Vec<T>,
    VecDeque<T>,
    LinkedList<T>,
    HashSet<T>,
    BTreeSet<T>,
    [T]
);

/// Maps, as `Map<K, V>` or `Record<K, V>` depending on the config the value is
/// written under.
macro_rules! ts_name_map {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl<const C: u8, K: TsName<C>, V: TsName<C>> TsName<C> for $ty {
                // `Map<` or `Record<`, then K, `, `, V, `>`.
                const NAME_LEN: u32 = (if map_is_map(C) { 3 } else { 6 })
                    + 4
                    + <K as TsName<C>>::NAME_LEN
                    + <V as TsName<C>>::NAME_LEN;

                #[inline]
                fn describe_name() {
                    if map_is_map(C) {
                        inform_char('M');
                        inform_char('a');
                        inform_char('p');
                    } else {
                        inform_char('R');
                        inform_char('e');
                        inform_char('c');
                        inform_char('o');
                        inform_char('r');
                        inform_char('d');
                    }
                    inform_char('<');
                    <K as TsName<C>>::describe_name();
                    inform_char(',');
                    inform_char(' ');
                    <V as TsName<C>>::describe_name();
                    inform_char('>');
                }
            }
        )+
    };
}

ts_name_map!(HashMap<K, V>, BTreeMap<K, V>);

/// Informs `undefined` or `null`, whichever the config writes.
#[inline]
fn describe_nullish(config: u8) {
    if nullish_is_undefined(config) {
        inform_char('u');
        inform_char('n');
        inform_char('d');
        inform_char('e');
        inform_char('f');
        inform_char('i');
        inform_char('n');
        inform_char('e');
        inform_char('d');
    } else {
        inform_char('n');
        inform_char('u');
        inform_char('l');
        inform_char('l');
    }
}

const fn nullish_len(config: u8) -> u32 {
    if nullish_is_undefined(config) {
        9
    } else {
        4
    }
}

/// The unit type serializes to nothing, and is named for whatever that nothing
/// is written as.
impl<const C: u8> TsName<C> for () {
    const NAME_LEN: u32 = nullish_len(C);

    #[inline]
    fn describe_name() {
        describe_nullish(C);
    }
}

/// `(T | undefined)`.
///
/// The parentheses are always written. A union is only legal unparenthesized in
/// some of the positions a name can land in -- `Vec<Option<T>>` has to come out
/// as `(T | undefined)[]` -- and a name has no context to decide from.
impl<const C: u8, T: TsName<C>> TsName<C> for Option<T> {
    // `(` + T + ` | ` + nullish + `)`
    const NAME_LEN: u32 = <T as TsName<C>>::NAME_LEN + nullish_len(C) + 5;

    #[inline]
    fn describe_name() {
        inform_char('(');
        <T as TsName<C>>::describe_name();
        inform_char(' ');
        inform_char('|');
        inform_char(' ');
        describe_nullish(C);
        inform_char(')');
    }
}

/// `({ Ok: T } | { Err: E })`, which is what serde writes and what the
/// declaration already renders.
impl<const C: u8, T: TsName<C>, E: TsName<C>> TsName<C> for Result<T, E> {
    // `({ Ok: ` T ` } | { Err: ` E ` })`
    const NAME_LEN: u32 = 22 + <T as TsName<C>>::NAME_LEN + <E as TsName<C>>::NAME_LEN;

    #[inline]
    fn describe_name() {
        inform_char('(');
        inform_char('{');
        inform_char(' ');
        inform_char('O');
        inform_char('k');
        inform_char(':');
        inform_char(' ');
        <T as TsName<C>>::describe_name();
        inform_char(' ');
        inform_char('}');
        inform_char(' ');
        inform_char('|');
        inform_char(' ');
        inform_char('{');
        inform_char(' ');
        inform_char('E');
        inform_char('r');
        inform_char('r');
        inform_char(':');
        inform_char(' ');
        <E as TsName<C>>::describe_name();
        inform_char(' ');
        inform_char('}');
        inform_char(')');
    }
}

/// Tuples, as `[A, B]`.
macro_rules! ts_name_tuple {
    ($(($($param:ident),+))+) => {
        $(
            impl<const C: u8, $($param: TsName<C>),+> TsName<C> for ($($param,)+) {
                // `[` + each name + `, ` between each pair + `]`
                const NAME_LEN: u32 = 2
                    + tuple_separators(&[$(<$param as TsName<C>>::NAME_LEN),+])
                    $(+ <$param as TsName<C>>::NAME_LEN)+;

                #[inline]
                fn describe_name() {
                    inform_char('[');
                    let mut first = true;
                    $(
                        if !first {
                            inform_char(',');
                            inform_char(' ');
                        }
                        first = false;
                        <$param as TsName<C>>::describe_name();
                    )+
                    let _ = first;
                    inform_char(']');
                }
            }
        )+
    };
}

/// `, ` between each pair of elements.
const fn tuple_separators(names: &[u32]) -> u32 {
    2 * (names.len() as u32 - 1)
}

ts_name_tuple! {
    (A)
    (A, B)
    (A, B, C2)
    (A, B, C2, D)
    (A, B, C2, D, E)
    (A, B, C2, D, E, F)
    (A, B, C2, D, E, F, G)
    (A, B, C2, D, E, F, G, H)
    (A, B, C2, D, E, F, G, H, I)
    (A, B, C2, D, E, F, G, H, I, J)
    (A, B, C2, D, E, F, G, H, I, J, K)
    (A, B, C2, D, E, F, G, H, I, J, K, L)
    (A, B, C2, D, E, F, G, H, I, J, K, L, M)
    (A, B, C2, D, E, F, G, H, I, J, K, L, M, N)
    (A, B, C2, D, E, F, G, H, I, J, K, L, M, N, O)
    (A, B, C2, D, E, F, G, H, I, J, K, L, M, N, O, P)
}

/// Fixed-size arrays, which the declaration path splits at sixteen: shorter
/// ones are a tuple, longer ones a sequence.
///
/// Unrolled behind conditions on `N` rather than counted in a loop, because a
/// loop in a descriptor is only interpretable from wasm-bindgen 0.2.126 and
/// this crate supports 0.2.104. A condition on a `const` folds at every
/// profile; a loop does not.
impl<const C: u8, T: TsName<C>, const N: usize> TsName<C> for [T; N] {
    const NAME_LEN: u32 = if N == 0 {
        2
    } else if N <= 16 {
        2 + N as u32 * <T as TsName<C>>::NAME_LEN + 2 * (N as u32 - 1)
    } else {
        <T as TsName<C>>::NAME_LEN + 2
    };

    #[inline]
    fn describe_name() {
        if N > 16 {
            <T as TsName<C>>::describe_name();
            inform_char('[');
            inform_char(']');
        } else {
            inform_char('[');
            if N > 0 {
                <T as TsName<C>>::describe_name();
            }
            if N > 1 {
                inform_char(',');
                inform_char(' ');
                <T as TsName<C>>::describe_name();
            }
            if N > 2 {
                inform_char(',');
                inform_char(' ');
                <T as TsName<C>>::describe_name();
            }
            if N > 3 {
                inform_char(',');
                inform_char(' ');
                <T as TsName<C>>::describe_name();
            }
            if N > 4 {
                inform_char(',');
                inform_char(' ');
                <T as TsName<C>>::describe_name();
            }
            if N > 5 {
                inform_char(',');
                inform_char(' ');
                <T as TsName<C>>::describe_name();
            }
            if N > 6 {
                inform_char(',');
                inform_char(' ');
                <T as TsName<C>>::describe_name();
            }
            if N > 7 {
                inform_char(',');
                inform_char(' ');
                <T as TsName<C>>::describe_name();
            }
            if N > 8 {
                inform_char(',');
                inform_char(' ');
                <T as TsName<C>>::describe_name();
            }
            if N > 9 {
                inform_char(',');
                inform_char(' ');
                <T as TsName<C>>::describe_name();
            }
            if N > 10 {
                inform_char(',');
                inform_char(' ');
                <T as TsName<C>>::describe_name();
            }
            if N > 11 {
                inform_char(',');
                inform_char(' ');
                <T as TsName<C>>::describe_name();
            }
            if N > 12 {
                inform_char(',');
                inform_char(' ');
                <T as TsName<C>>::describe_name();
            }
            if N > 13 {
                inform_char(',');
                inform_char(' ');
                <T as TsName<C>>::describe_name();
            }
            if N > 14 {
                inform_char(',');
                inform_char(' ');
                <T as TsName<C>>::describe_name();
            }
            if N > 15 {
                inform_char(',');
                inform_char(' ');
                <T as TsName<C>>::describe_name();
            }
            inform_char(']');
        }
    }
}

/// `{ secs: number; nanos: number }`, as the declaration renders it.
impl<const C: u8> TsName<C> for Duration {
    const NAME_LEN: u32 = 31;

    #[inline]
    fn describe_name() {
        inform_char('{');
        inform_char(' ');
        inform_char('s');
        inform_char('e');
        inform_char('c');
        inform_char('s');
        inform_char(':');
        inform_char(' ');
        inform_char('n');
        inform_char('u');
        inform_char('m');
        inform_char('b');
        inform_char('e');
        inform_char('r');
        inform_char(';');
        inform_char(' ');
        inform_char('n');
        inform_char('a');
        inform_char('n');
        inform_char('o');
        inform_char('s');
        inform_char(':');
        inform_char(' ');
        inform_char('n');
        inform_char('u');
        inform_char('m');
        inform_char('b');
        inform_char('e');
        inform_char('r');
        inform_char(' ');
        inform_char('}');
    }
}

/// `{ secs_since_epoch: number; nanos_since_epoch: number }`.
impl<const C: u8> TsName<C> for SystemTime {
    const NAME_LEN: u32 = 55;

    #[inline]
    fn describe_name() {
        inform_char('{');
        inform_char(' ');
        inform_char('s');
        inform_char('e');
        inform_char('c');
        inform_char('s');
        inform_char('_');
        inform_char('s');
        inform_char('i');
        inform_char('n');
        inform_char('c');
        inform_char('e');
        inform_char('_');
        inform_char('e');
        inform_char('p');
        inform_char('o');
        inform_char('c');
        inform_char('h');
        inform_char(':');
        inform_char(' ');
        inform_char('n');
        inform_char('u');
        inform_char('m');
        inform_char('b');
        inform_char('e');
        inform_char('r');
        inform_char(';');
        inform_char(' ');
        inform_char('n');
        inform_char('a');
        inform_char('n');
        inform_char('o');
        inform_char('s');
        inform_char('_');
        inform_char('s');
        inform_char('i');
        inform_char('n');
        inform_char('c');
        inform_char('e');
        inform_char('_');
        inform_char('e');
        inform_char('p');
        inform_char('o');
        inform_char('c');
        inform_char('h');
        inform_char(':');
        inform_char(' ');
        inform_char('n');
        inform_char('u');
        inform_char('m');
        inform_char('b');
        inform_char('e');
        inform_char('r');
        inform_char(' ');
        inform_char('}');
    }
}

/// `{ start: T; end: T }`, for both range forms.
macro_rules! ts_name_range {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl<const C: u8, T: TsName<C>> TsName<C> for $ty {
                // `{ start: ` T `; end: ` T ` }`
                const NAME_LEN: u32 = 18 + 2 * <T as TsName<C>>::NAME_LEN;

                #[inline]
                fn describe_name() {
                    inform_char('{');
                    inform_char(' ');
                    inform_char('s');
                    inform_char('t');
                    inform_char('a');
                    inform_char('r');
                    inform_char('t');
                    inform_char(':');
                    inform_char(' ');
                    <T as TsName<C>>::describe_name();
                    inform_char(';');
                    inform_char(' ');
                    inform_char('e');
                    inform_char('n');
                    inform_char('d');
                    inform_char(':');
                    inform_char(' ');
                    <T as TsName<C>>::describe_name();
                    inform_char(' ');
                    inform_char('}');
                }
            }
        )+
    };
}

ts_name_range!(Range<T>, RangeInclusive<T>);
