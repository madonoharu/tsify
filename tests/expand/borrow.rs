use std::borrow::Cow;
use tsify_next::Tsify;

#[derive(Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
struct Borrow<'a> {
    raw: &'a str,
    cow: Cow<'a, str>,
}
