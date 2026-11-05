//! The names types go by inside a `#[wasm_bindgen]` signature.
//!
//! Only the length is checked here: informing the characters needs
//! wasm-bindgen's descriptor import, which exists only in a wasm build. A
//! length that disagrees with the characters is the failure worth catching
//! anyway — the count is what the decoder trusts, so a wrong one does not
//! truncate a name, it desynchronizes everything after it in the descriptor.
#![allow(dead_code)]

use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};
use tsify::__macro_support::{
    TsName, HASHMAP_AS_OBJECT, LARGE_NUMBER_TYPES_AS_BIGINTS, MISSING_AS_NULL,
};
use tsify::Tsify;

#[derive(Tsify, Serialize, Deserialize)]
pub struct UserInfo {
    id: u32,
}

#[derive(Tsify, Serialize, Deserialize)]
pub struct Response<T> {
    data: T,
}

#[derive(Tsify, Serialize, Deserialize)]
pub struct Pair<A, B> {
    left: A,
    right: B,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(rename = "Renamed")]
pub struct Named<T> {
    data: T,
}

#[derive(Tsify, Serialize, Deserialize)]
pub struct Tagged<T> {
    #[serde(skip)]
    tag: std::marker::PhantomData<T>,
    id: u32,
}

#[derive(Tsify, Serialize)]
pub struct Sliced<'a, const N: usize> {
    data: &'a str,
}

#[track_caller]
fn assert_name<const C: u8, T: TsName<C>>(expected: &str) {
    assert_eq!(
        <T as TsName<C>>::NAME_LEN,
        expected.chars().count() as u32,
        "name length for `{expected}`"
    );
}

/// The default config: what a type reached from a root declaring no attributes
/// is named under.
const PLAIN: u8 = 0;

#[test]
fn test_derived_names() {
    assert_name::<PLAIN, UserInfo>("UserInfo");
    assert_name::<PLAIN, Response<UserInfo>>("Response<UserInfo>");
    assert_name::<PLAIN, Response<Response<UserInfo>>>("Response<Response<UserInfo>>");
    assert_name::<PLAIN, Pair<u32, String>>("Pair<number, string>");
    assert_name::<PLAIN, Response<Pair<u32, String>>>("Response<Pair<number, string>>");

    // `rename` names the declaration, so it names the type.
    assert_name::<PLAIN, Named<UserInfo>>("Renamed<UserInfo>");

    // A parameter no field mentions is declared nowhere, so applying it to the
    // declared name would be a type error in the `.d.ts`. Lifetimes and const
    // parameters never reach TypeScript at all.
    assert_name::<PLAIN, Tagged<UserInfo>>("Tagged");
    assert_name::<PLAIN, Sliced<'_, 3>>("Sliced");
}

#[test]
fn test_scalar_names() {
    assert_name::<PLAIN, u8>("number");
    assert_name::<PLAIN, f64>("number");
    assert_name::<PLAIN, bool>("boolean");
    assert_name::<PLAIN, String>("string");
    assert_name::<PLAIN, char>("string");
    assert_name::<PLAIN, ()>(if cfg!(feature = "js") {
        "undefined"
    } else {
        "null"
    });
}

#[test]
fn test_container_names() {
    assert_name::<PLAIN, Vec<String>>("string[]");
    assert_name::<PLAIN, Box<UserInfo>>("UserInfo");
    assert_name::<PLAIN, &[u32]>("number[]");
    assert_name::<PLAIN, Duration>("{ secs: number; nanos: number }");
    assert_name::<PLAIN, SystemTime>("{ secs_since_epoch: number; nanos_since_epoch: number }");
    assert_name::<PLAIN, std::ops::Range<u32>>("{ start: number; end: number }");
    assert_name::<PLAIN, std::ops::RangeInclusive<u32>>("{ start: number; end: number }");
}

#[test]
fn test_the_types_that_had_no_name() {
    // Everything the declaration path already renders has to keep compiling as
    // a type argument. These are the ones that did not, and that stopped
    // `tests-e2e/test_generic_args1` from building.
    assert_name::<PLAIN, Result<u32, String>>("({ Ok: number } | { Err: string })");
    assert_name::<PLAIN, (u32, String)>("[number, string]");
    assert_name::<PLAIN, (u32,)>("[number]");
    assert_name::<PLAIN, (u32, u32, u32, u32)>("[number, number, number, number]");
    assert_name::<PLAIN, Response<Result<u32, String>>>(
        "Response<({ Ok: number } | { Err: string })>",
    );
    assert_name::<PLAIN, Response<(u32, String)>>("Response<[number, string]>");
}

#[test]
fn test_fixed_size_arrays_split_where_the_declaration_splits() {
    // The declaration path renders sixteen or fewer as a tuple, and anything
    // longer as a sequence.
    assert_name::<PLAIN, [u32; 0]>("[]");
    assert_name::<PLAIN, [u32; 1]>("[number]");
    assert_name::<PLAIN, [u32; 2]>("[number, number]");
    assert_name::<PLAIN, [u32; 16]>(&format!("[{}]", ["number"; 16].join(", ")));
    assert_name::<PLAIN, [u32; 17]>("number[]");
    assert_name::<PLAIN, [u32; 20]>("number[]");
}

#[test]
fn test_the_config_reaches_the_name() {
    if cfg!(feature = "js") {
        assert_name::<PLAIN, HashMap<String, u32>>("Map<string, number>");
        assert_name::<HASHMAP_AS_OBJECT, HashMap<String, u32>>("Record<string, number>");

        assert_name::<PLAIN, Option<u32>>("(number | undefined)");
        assert_name::<MISSING_AS_NULL, Option<u32>>("(number | null)");

        assert_name::<PLAIN, u64>("number");
        assert_name::<LARGE_NUMBER_TYPES_AS_BIGINTS, u64>("bigint");
    } else {
        // Without `js` the value goes through JSON, which settles all three.
        assert_name::<PLAIN, HashMap<String, u32>>("Record<string, number>");
        assert_name::<HASHMAP_AS_OBJECT, HashMap<String, u32>>("Record<string, number>");

        assert_name::<PLAIN, Option<u32>>("(number | null)");
        assert_name::<MISSING_AS_NULL, Option<u32>>("(number | null)");

        assert_name::<PLAIN, u64>("number");
        assert_name::<LARGE_NUMBER_TYPES_AS_BIGINTS, u64>("number");
    }
}

#[test]
fn test_the_config_reaches_the_whole_way_down() {
    // One serializer, built from the root type's attributes, writes the whole
    // value — so a derived type passes the config it is asked for straight
    // through, and a map two levels down still follows the root.
    if cfg!(feature = "js") {
        assert_name::<PLAIN, Response<Response<HashMap<String, u32>>>>(
            "Response<Response<Map<string, number>>>",
        );
        assert_name::<HASHMAP_AS_OBJECT, Response<Response<HashMap<String, u32>>>>(
            "Response<Response<Record<string, number>>>",
        );
    }
}

#[test]
fn test_unknown_config_bits_are_ignored() {
    // A newer macro crate may set a bit this one does not know. Naming the type
    // anyway, rather than failing to compile, is what keeps the two versions
    // usable together.
    const UNKNOWN: u8 = 1 << 7;

    assert_name::<UNKNOWN, HashMap<String, u32>>(if cfg!(feature = "js") {
        "Map<string, number>"
    } else {
        "Record<string, number>"
    });
    assert_name::<{ HASHMAP_AS_OBJECT | (1 << 7) }, HashMap<String, u32>>("Record<string, number>");
}
