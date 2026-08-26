use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

use wasm_bindgen::describe::{inform, NAMED_EXTERNREF, VECTOR};

/// The TypeScript name a type goes by inside a `#[wasm_bindgen]` signature.
///
/// wasm-bindgen does not read the `typescript_custom_section` when it writes a
/// signature into the `.d.ts`. It learns the type from a *descriptor*: a stream
/// of `u32`s the compiled module informs at build time, which for a named type
/// is `NAMED_EXTERNREF`, a character count, and then one `u32` per character.
/// A `#[wasm_bindgen] extern` type carries one fixed name for all of its uses,
/// so on its own it can only ever describe `Response` — never
/// `Response<UserInfo>`, which is what the declaration in the custom section
/// actually has to be applied to.
///
/// This trait rebuilds the name once per monomorphization instead: the derive
/// unrolls the characters of the type's own declared name, and every type
/// argument defers to its own `TsName` impl. So `Response<UserInfo>` describes
/// itself under exactly that name, and nesting (`Response<Vec<UserInfo>>`)
/// composes.
///
/// Every character has to reach the descriptor as a compile-time constant.
/// wasm-bindgen interprets the descriptor with a small interpreter that mirrors
/// linear memory but never loads the module's data segments, so walking the
/// bytes of a `&'static str` at describe time would inform a run of NULs. That
/// is why the name is a sequence of [`inform_char`] calls and a `const` length
/// rather than a `&str`.
///
/// # Implementing by hand
///
/// `#[derive(Tsify)]` writes this impl for you. If you implement [`Tsify`] by
/// hand, write it with [`ts_name!`]:
///
/// ```
/// struct Rgb(u8, u8, u8);
///
/// tsify::ts_name!(Rgb => 'R', 'g', 'b');
/// ```
///
/// [`Tsify`]: crate::Tsify
/// [`ts_name!`]: crate::ts_name
#[diagnostic::on_unimplemented(
    message = "`{Self}` has no TypeScript name",
    label = "used here as a type argument of a type that crosses the wasm ABI",
    note = "a type argument needs a name of its own for the generic type to be named after it",
    note = "derive `Tsify` for `{Self}`, or name it by hand with `tsify::ts_name!({Self} => 'N', 'a', 'm', 'e');`"
)]
pub trait TsName {
    /// The number of `char`s that [`describe_name`](TsName::describe_name)
    /// informs.
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

/// Informs one `char` of a type name, for use inside
/// [`TsName::describe_name`].
///
/// Only pass a literal, or defer to another impl's `describe_name`: a character
/// loaded from memory at describe time reaches wasm-bindgen as `\0`.
#[inline]
pub fn inform_char(c: char) {
    inform(c as u32);
}

/// Implements [`TsName`] for one or more types under a fixed name, spelled one
/// `char` at a time.
///
/// ```
/// struct Rgb(u8, u8, u8);
/// struct Rgba(u8, u8, u8, u8);
///
/// tsify::ts_name!(Rgb => 'R', 'g', 'b');
/// tsify::ts_name!(Rgba => 'R', 'g', 'b', 'a');
/// ```
///
/// The name is a `char` list rather than a string literal because every
/// character has to reach wasm-bindgen's descriptor as a compile-time constant.
/// See [`TsName`].
#[macro_export]
macro_rules! ts_name {
    ($($ty:ty),+ $(,)? => $($ch:literal),+ $(,)?) => {
        // The name is carried along as one token tree so that it can be
        // repeated per type: a repetition cannot mix two metavariables that
        // repeat a different number of times.
        $crate::ts_name!(@each [$($ch),+] $($ty),+);
    };

    (@each $name:tt $($ty:ty),+) => {
        $($crate::ts_name!(@one $ty, $name);)+
    };

    (@one $ty:ty, [$($ch:literal),+]) => {
        impl $crate::TsName for $ty {
            const NAME_LEN: u32 = [$($ch),+].len() as u32;

            #[inline]
            fn describe_name() {
                $($crate::inform_char($ch);)+
            }
        }
    };
}

ts_name!(
    u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64
    => 'n', 'u', 'm', 'b', 'e', 'r'
);

// `u128`/`i128` follow the same rule the declaration does: they are only a JS
// number when going through JSON, which has already narrowed them.
#[cfg(feature = "js")]
ts_name!(u128, i128 => 'b', 'i', 'g', 'i', 'n', 't');
#[cfg(not(feature = "js"))]
ts_name!(u128, i128 => 'n', 'u', 'm', 'b', 'e', 'r');

ts_name!(String, str, char, Path, PathBuf => 's', 't', 'r', 'i', 'n', 'g');
ts_name!(bool => 'b', 'o', 'o', 'l', 'e', 'a', 'n');

/// Wrappers serde sees through, so TypeScript does too.
macro_rules! ts_name_transparent {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl<T: TsName + ?Sized> TsName for $ty {
                const NAME_LEN: u32 = <T as TsName>::NAME_LEN;

                #[inline]
                fn describe_name() {
                    <T as TsName>::describe_name();
                }
            }
        )+
    };
}

ts_name_transparent!(&T, &mut T, Box<T>, Rc<T>, Arc<T>);

macro_rules! ts_name_transparent_sized {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl<T: TsName> TsName for $ty {
                const NAME_LEN: u32 = <T as TsName>::NAME_LEN;

                #[inline]
                fn describe_name() {
                    <T as TsName>::describe_name();
                }
            }
        )+
    };
}

ts_name_transparent_sized!(Cell<T>, RefCell<T>);

impl<T: TsName + ToOwned + ?Sized> TsName for Cow<'_, T> {
    const NAME_LEN: u32 = <T as TsName>::NAME_LEN;

    #[inline]
    fn describe_name() {
        <T as TsName>::describe_name();
    }
}

/// Sequences, as `T[]`.
///
/// The element has to be parenthesized when it is a union, which is why
/// [`Option`] below carries its own parentheses.
macro_rules! ts_name_array {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl<T: TsName> TsName for $ty {
                const NAME_LEN: u32 = <T as TsName>::NAME_LEN + 2;

                #[inline]
                fn describe_name() {
                    <T as TsName>::describe_name();
                    inform_char('[');
                    inform_char(']');
                }
            }
        )+
    };
}

ts_name_array!(Vec<T>, VecDeque<T>, LinkedList<T>, HashSet<T>, BTreeSet<T>);

impl<T: TsName> TsName for [T] {
    const NAME_LEN: u32 = <T as TsName>::NAME_LEN + 2;

    #[inline]
    fn describe_name() {
        <T as TsName>::describe_name();
        inform_char('[');
        inform_char(']');
    }
}

impl<T: TsName, const N: usize> TsName for [T; N] {
    const NAME_LEN: u32 = <T as TsName>::NAME_LEN + 2;

    #[inline]
    fn describe_name() {
        <T as TsName>::describe_name();
        inform_char('[');
        inform_char(']');
    }
}

// `Map` while serde-wasm-bindgen is producing a real `Map`, `Record` once the
// value has been through JSON and come out an object.
#[cfg(feature = "js")]
const MAP_NAME_LEN: u32 = 3;
#[cfg(not(feature = "js"))]
const MAP_NAME_LEN: u32 = 6;

// Spelled out rather than walked out of a `const` array: an indexing expression
// materializes the array in memory, and memory reads at describe time come back
// zeroed. See [`TsName`].
#[cfg(feature = "js")]
fn describe_map_name() {
    inform_char('M');
    inform_char('a');
    inform_char('p');
}

#[cfg(not(feature = "js"))]
fn describe_map_name() {
    inform_char('R');
    inform_char('e');
    inform_char('c');
    inform_char('o');
    inform_char('r');
    inform_char('d');
}

macro_rules! ts_name_map {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl<K: TsName, V: TsName> TsName for $ty {
                // `Map<` + K + `, ` + V + `>`
                const NAME_LEN: u32 = MAP_NAME_LEN
                    + 4
                    + <K as TsName>::NAME_LEN
                    + <V as TsName>::NAME_LEN;

                #[inline]
                fn describe_name() {
                    describe_map_name();
                    inform_char('<');
                    <K as TsName>::describe_name();
                    inform_char(',');
                    inform_char(' ');
                    <V as TsName>::describe_name();
                    inform_char('>');
                }
            }
        )+
    };
}

ts_name_map!(HashMap<K, V>, BTreeMap<K, V>);

// The nullish half matches what the declaration uses: `undefined` while
// serde-wasm-bindgen leaves a missing value undefined, `null` once it has been
// through JSON.
#[cfg(feature = "js")]
const NULLISH_LEN: u32 = 9;
#[cfg(not(feature = "js"))]
const NULLISH_LEN: u32 = 4;

#[cfg(feature = "js")]
fn describe_nullish() {
    inform_char('u');
    inform_char('n');
    inform_char('d');
    inform_char('e');
    inform_char('f');
    inform_char('i');
    inform_char('n');
    inform_char('e');
    inform_char('d');
}

#[cfg(not(feature = "js"))]
fn describe_nullish() {
    inform_char('n');
    inform_char('u');
    inform_char('l');
    inform_char('l');
}

/// The unit type serializes to nothing, and is named for whatever that nothing
/// comes out as.
impl TsName for () {
    const NAME_LEN: u32 = NULLISH_LEN;

    #[inline]
    fn describe_name() {
        describe_nullish();
    }
}

/// `(T | undefined)`.
///
/// The parentheses are always written. A union is only legal unparenthesized in
/// some of the positions a name can land in — `Vec<Option<T>>` has to come out
/// as `(T | undefined)[]` — and a name has no context to decide from at
/// describe time.
impl<T: TsName> TsName for Option<T> {
    // `(` + T + ` | ` + nullish + `)`
    const NAME_LEN: u32 = <T as TsName>::NAME_LEN + NULLISH_LEN + 5;

    #[inline]
    fn describe_name() {
        inform_char('(');
        <T as TsName>::describe_name();
        inform_char(' ');
        inform_char('|');
        inform_char(' ');
        describe_nullish();
        inform_char(')');
    }
}
