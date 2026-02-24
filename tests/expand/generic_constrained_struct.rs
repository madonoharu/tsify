use tsify::Tsify;

pub trait Constraint {}

#[derive(Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct GenericStruct<T: Constraint> {
    x: T,
}

#[derive(Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct GenericNewtype<T: Constraint>(T);
