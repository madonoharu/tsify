//! Generates expanded code for tests in `tests/expand/` directory.
//! To update the expected output, run with `MACROTEST=overwrite cargo test`
//! or delete the `*.expanded.rs` files.
//!
//! `--features json,wasm-bindgen` enables the *test project's* own features,
//! not just tsify's, so the optional `serde` and `wasm-bindgen` dependencies
//! are resolvable from the code `#[derive(Tsify)]` emits.

#[test]
fn expandtest() {
    macrotest::expand_args("tests/expand/*.rs", ["--features", "json,wasm-bindgen"]);
}
