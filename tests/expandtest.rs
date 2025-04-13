//! Generates expanded code for tests in `tests/expand/` directory.
//! To update the expected output, run with `MACROTEST=overwrite cargo test`
//! or delete the `.expanded.rs` files.

#[test]
fn expandtest() {
    macrotest::expand_args("tests/expand/*.rs", ["--features", "tsify-next/json"]);
}
