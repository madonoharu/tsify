#[test]
fn expandtest() {
    macrotest::expand_args(
        "tests/expand/*.rs",
        ["--features", "tsify/wasm-bindgen-impl"],
    );
}
