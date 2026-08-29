//! Extracts the README's quickstart, so the example has one home instead of a
//! copy to keep honest. Do not replace this with a checked-in copy: a copy is a
//! workspace source, so `cargo fmt --all -- --check` would reorder the imports
//! a reader sees in the README.
//!
//! The block is taken verbatim: if the quickstart ever hides lines from the
//! doctest with `# `, this wants a rule for stripping them.

use std::{env, fs, path::Path};

include!("readme_block.rs");

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    let readme = Path::new(&manifest_dir).join("../../README.md");
    println!("cargo:rerun-if-changed={}", readme.display());
    println!("cargo:rerun-if-changed=readme_block.rs");

    let text = fs::read_to_string(&readme).unwrap_or_else(|e| panic!("{}: {e}", readme.display()));
    let block = fenced_block_after(&text, "## Example", "rust");

    let out = Path::new(&env::var("OUT_DIR").expect("OUT_DIR")).join("readme_quickstart.rs");
    fs::write(&out, block).unwrap_or_else(|e| panic!("{}: {e}", out.display()));
}
