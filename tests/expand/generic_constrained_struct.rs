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

#[derive(Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct GenericAssoc<T: Iterator<Item = u32>> {
    x: T,
}

#[derive(Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct GenericLifetime<'a: 'b, 'b> {
    x: &'a str,
    y: &'b str,
}

#[derive(Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct GenericConst<const N: usize> {
    x: u32,
}
