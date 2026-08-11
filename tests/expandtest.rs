//! Generates expanded code for tests in `tests/expand/` directory.
//! To update the expected output, run with `MACROTEST=overwrite cargo test`
//! or delete the `*.expanded.rs` files.

#[test]
fn expandtest() {
    // Enable the test project's own `json` and `wasm-bindgen` features (not
    // just tsify's) so the `serde` and `wasm-bindgen` optional dependencies
    // are resolvable from the code emitted by `#[derive(Tsify)]`.
    macrotest::expand_args("tests/expand/*.rs", ["--features", "json,wasm-bindgen"]);
}
