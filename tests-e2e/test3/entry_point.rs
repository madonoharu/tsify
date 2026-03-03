#![allow(deprecated)]

use serde::Serialize;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct Identified<Id, Value>
where
    Id: Sync,
    Value: 'static,
{
    pub id: Id,
    pub value: Value,
}
