//! The names generic types describe themselves by.
//!
//! Only the length is checked here: informing the characters themselves needs
//! wasm-bindgen's descriptor import, which only exists in a wasm build. What
//! the characters come out as is covered by `tests-e2e/test7`, which reads the
//! `.d.ts` wasm-bindgen writes.
//!
//! A length that disagrees with the characters is the failure worth catching:
//! wasm-bindgen reads the name as a count followed by that many characters, so
//! a wrong count does not truncate a name, it desynchronizes the rest of the
//! descriptor.
#![allow(dead_code)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tsify::{TsName, Tsify};

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
fn assert_name_len<T: TsName>(expected: &str) {
    assert_eq!(
        <T as TsName>::NAME_LEN,
        expected.chars().count() as u32,
        "name length for `{expected}`"
    );
}

#[test]
fn test_plain_name() {
    assert_name_len::<UserInfo>("UserInfo");
}

#[test]
fn test_generic_name() {
    assert_name_len::<Response<UserInfo>>("Response<UserInfo>");
}

#[test]
fn test_nested_generic_name() {
    assert_name_len::<Response<Response<UserInfo>>>("Response<Response<UserInfo>>");
}

#[test]
fn test_several_type_arguments() {
    assert_name_len::<Pair<u32, String>>("Pair<number, string>");
    assert_name_len::<Response<Pair<u32, String>>>("Response<Pair<number, string>>");
}

#[test]
fn test_builtin_type_arguments() {
    assert_name_len::<Response<Vec<String>>>("Response<string[]>");
    assert_name_len::<Response<bool>>("Response<boolean>");
    assert_name_len::<Response<Box<UserInfo>>>("Response<UserInfo>");

    if cfg!(feature = "js") {
        assert_name_len::<Response<Option<UserInfo>>>("Response<(UserInfo | undefined)>");
        assert_name_len::<Response<HashMap<String, u32>>>("Response<Map<string, number>>");
    } else {
        assert_name_len::<Response<Option<UserInfo>>>("Response<(UserInfo | null)>");
        assert_name_len::<Response<HashMap<String, u32>>>("Response<Record<string, number>>");
    }
}

#[test]
fn test_name_follows_the_declaration() {
    // `rename` names the declaration, so it names the type.
    assert_name_len::<Named<UserInfo>>("Renamed<UserInfo>");

    // A parameter no field mentions is declared nowhere, so applying it to the
    // declared name would be a type error in the `.d.ts`.
    assert_name_len::<Tagged<UserInfo>>("Tagged");

    // Neither lifetimes nor const parameters reach TypeScript at all.
    assert_name_len::<Sliced<'_, 3>>("Sliced");
}
