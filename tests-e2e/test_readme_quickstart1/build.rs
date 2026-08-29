//! Extracts the README's quickstart so that the crate builds the example
//! itself rather than a copy of it.
//!
//! A copy would need something to keep it honest; the example having one home
//! needs nothing. It also keeps the README out of reach of `cargo fmt --check`,
//! which would otherwise decide the order of the imports a reader sees.
//!
//! The block is taken verbatim, so if the quickstart ever starts hiding lines
//! from the doctest with `# `, this wants a rule for stripping them. It fails
//! the build rather than going quiet: an anchor that moves panics here.

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
