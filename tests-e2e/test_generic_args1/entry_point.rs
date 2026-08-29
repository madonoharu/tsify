#![allow(deprecated, dead_code)]

// This file is built twice: here with the default features, and by
// `test_generic_args_js1`, which points its `[lib] path` at it and turns on
// `js`. The pair of reference outputs is what that feature changes.
//
// A declaration and the name a type goes by in a signature come from two
// different places — a `typescript_custom_section` and the descriptor the
// module informs — and nothing compares them. The reference output is where
// they can be read together.
//
// Both references record #76: a generic reaches a signature without its
// arguments, so `Envelope` stands where `Envelope<Payload>` is meant. Those
// lines should change when it is fixed. What should hold either way is that a
// signature agrees with the declaration of the type it names.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tsify::Ts;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize)]
pub struct Envelope<T> {
    pub data: T,
}

#[derive(Tsify, Serialize, Deserialize)]
pub struct Payload {
    pub name: String,
}

// A non-generic control: nothing to lose, so these two are correct today.
#[wasm_bindgen]
pub fn plain_arg(v: Ts<Payload>) -> Result<(), JsError> {
    let _: Payload = v.to_rust()?;
    Ok(())
}

#[wasm_bindgen]
pub fn plain_return(v: Ts<Payload>) -> Result<Ts<Payload>, JsError> {
    Ok(v)
}

#[wasm_bindgen]
pub fn generic_arg(v: Ts<Envelope<Payload>>) -> Result<(), JsError> {
    let _: Envelope<Payload> = v.to_rust()?;
    Ok(())
}

#[wasm_bindgen]
pub fn generic_return(v: Ts<Envelope<Payload>>) -> Result<Ts<Envelope<Payload>>, JsError> {
    Ok(v)
}

#[wasm_bindgen]
pub fn generic_nested(v: Ts<Envelope<Envelope<Payload>>>) -> Result<(), JsError> {
    let _: Envelope<Envelope<Payload>> = v.to_rust()?;
    Ok(())
}

#[wasm_bindgen]
pub fn generic_builtin(v: Ts<Envelope<u32>>) -> Result<(), JsError> {
    let _: Envelope<u32> = v.to_rust()?;
    Ok(())
}

// The deprecated attributes reach the descriptor by their own route.
#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct AbiEnvelope<T> {
    pub data: T,
}

#[wasm_bindgen]
pub fn abi_roundtrip(v: AbiEnvelope<u32>) -> AbiEnvelope<u32> {
    v
}

// Type arguments the declaration path already renders. They compile today, and
// are here to keep compiling: requiring every type argument to carry a name of
// its own would break the build rather than change the reference.
#[wasm_bindgen]
pub fn arg_result(v: Ts<Envelope<Result<u32, String>>>) -> Result<(), JsError> {
    let _: Envelope<Result<u32, String>> = v.to_rust()?;
    Ok(())
}

#[wasm_bindgen]
pub fn arg_tuple(v: Ts<Envelope<(u32, String)>>) -> Result<(), JsError> {
    let _: Envelope<(u32, String)> = v.to_rust()?;
    Ok(())
}

#[wasm_bindgen]
pub fn arg_vec(v: Ts<Envelope<Vec<u32>>>) -> Result<(), JsError> {
    let _: Envelope<Vec<u32>> = v.to_rust()?;
    Ok(())
}

// The three types the `js` attributes below rename, with no attribute on them.
#[wasm_bindgen]
pub fn arg_option(v: Ts<Envelope<Option<u32>>>) -> Result<(), JsError> {
    let _: Envelope<Option<u32>> = v.to_rust()?;
    Ok(())
}

#[wasm_bindgen]
pub fn arg_map(v: Ts<Envelope<HashMap<String, u32>>>) -> Result<(), JsError> {
    let _: Envelope<HashMap<String, u32>> = v.to_rust()?;
    Ok(())
}

#[wasm_bindgen]
pub fn arg_u64(v: Ts<Envelope<u64>>) -> Result<(), JsError> {
    let _: Envelope<u64> = v.to_rust()?;
    Ok(())
}

#[derive(Tsify, Serialize, Deserialize)]
pub struct Defaulted {
    pub opt: Option<u32>,
    pub map: HashMap<String, u32>,
    pub big: u64,
}

// The declaration path splits fixed-size arrays at length 16.
#[derive(Tsify, Serialize, Deserialize)]
pub struct Arrays {
    pub small: [u32; 2],
    pub big: [u32; 20],
}

#[wasm_bindgen]
pub fn arg_array_small(v: Ts<Envelope<[u32; 2]>>) -> Result<(), JsError> {
    let _: Envelope<[u32; 2]> = v.to_rust()?;
    Ok(())
}

#[wasm_bindgen]
pub fn arg_array_big(v: Ts<Envelope<[u32; 20]>>) -> Result<(), JsError> {
    let _: Envelope<[u32; 20]> = v.to_rust()?;
    Ok(())
}

// These three attributes are rejected without `js`, and are set per container
// rather than per crate. `Defaulted` and the `arg_option` / `arg_map` /
// `arg_u64` signatures above are the rows to read each of them against.
#[cfg(feature = "js")]
mod configured {
    use super::*;

    #[derive(Tsify, Serialize, Deserialize)]
    #[tsify(missing_as_null, hashmap_as_object, large_number_types_as_bigints)]
    pub struct Configured {
        pub opt: Option<u32>,
        pub map: HashMap<String, u32>,
        pub big: u64,
    }

    #[derive(Tsify, Serialize, Deserialize)]
    #[tsify(missing_as_null, hashmap_as_object, large_number_types_as_bigints)]
    pub struct ConfiguredEnvelope<T> {
        pub data: T,
    }

    #[wasm_bindgen]
    pub fn configured_option(v: Ts<ConfiguredEnvelope<Option<u32>>>) -> Result<(), JsError> {
        let _: ConfiguredEnvelope<Option<u32>> = v.to_rust()?;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn configured_map(v: Ts<ConfiguredEnvelope<HashMap<String, u32>>>) -> Result<(), JsError> {
        let _: ConfiguredEnvelope<HashMap<String, u32>> = v.to_rust()?;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn configured_u64(v: Ts<ConfiguredEnvelope<u64>>) -> Result<(), JsError> {
        let _: ConfiguredEnvelope<u64> = v.to_rust()?;
        Ok(())
    }
}
