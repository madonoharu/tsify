//! Generic types keep their arguments in the signatures wasm-bindgen writes.
#![allow(deprecated)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tsify::{Ts, Tsify};
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize)]
pub struct UserInfo {
    id: u32,
    name: String,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Response<T> {
    data: T,
    ok: bool,
}

#[derive(Tsify, Serialize, Deserialize)]
pub struct Pair<A, B> {
    left: A,
    right: B,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi)]
pub enum Outcome<T> {
    Done(T),
    Failed(String),
}

/// A parameter no field mentions is not part of the declaration, so it is not
/// part of the name either.
#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi)]
pub struct Tagged<T> {
    #[serde(skip)]
    tag: std::marker::PhantomData<T>,
    id: u32,
}

#[wasm_bindgen]
pub fn get_user() -> Response<UserInfo> {
    Response {
        data: UserInfo {
            id: 0,
            name: String::new(),
        },
        ok: true,
    }
}

#[wasm_bindgen]
pub fn put_user(response: Response<UserInfo>) -> bool {
    response.ok
}

#[wasm_bindgen]
pub fn nested() -> Response<Response<UserInfo>> {
    Response {
        data: get_user(),
        ok: true,
    }
}

#[wasm_bindgen]
pub fn builtin_argument() -> Response<Vec<String>> {
    Response {
        data: Vec::new(),
        ok: true,
    }
}

#[wasm_bindgen]
pub fn optional_argument() -> Response<Option<UserInfo>> {
    Response {
        data: None,
        ok: true,
    }
}

#[wasm_bindgen]
pub fn map_argument() -> Response<HashMap<String, u32>> {
    Response {
        data: HashMap::new(),
        ok: true,
    }
}

#[wasm_bindgen]
pub fn several_arguments() -> Response<Pair<u32, String>> {
    Response {
        data: Pair {
            left: 0,
            right: String::new(),
        },
        ok: true,
    }
}

#[wasm_bindgen]
pub fn generic_enum() -> Outcome<UserInfo> {
    Outcome::Failed(String::new())
}

#[wasm_bindgen]
pub fn undeclared_parameter() -> Tagged<UserInfo> {
    Tagged {
        tag: std::marker::PhantomData,
        id: 0,
    }
}

/// The same thing through `Ts`, which is the way to cross the ABI without the
/// leak `into_wasm_abi`/`from_wasm_abi` have.
#[wasm_bindgen]
pub fn round_trip(response: Ts<Response<UserInfo>>) -> Result<Ts<Response<UserInfo>>, JsError> {
    let response: Response<UserInfo> = response.to_rust()?;
    Ok(response.into_ts()?)
}
