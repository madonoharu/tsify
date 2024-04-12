use serde::Serialize;
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct Identified<Id, Value> {
    pub id: Id,
    pub value: Value,
}
