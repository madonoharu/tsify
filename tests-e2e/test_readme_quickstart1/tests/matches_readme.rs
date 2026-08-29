//! Holds the reference `.d.ts` to the text the README prints.
//!
//! The harness compares that reference against what `wasm-pack build` emits;
//! this is the other link, that the reference is still the block the README
//! prints. `include_str!`, so a file that moves is a build error.

const README: &str = include_str!("../../../README.md");
const REFERENCE: &str =
    include_str!("../../reference_output/test_readme_quickstart1/test_readme_quickstart1.d.ts");

include!("../readme_block.rs");

#[test]
fn an_anchor_is_a_whole_line() {
    let text = "\n### Example\n\n```rust\nwrong\n```\n\n## Example\n\n```rust\nright\n```\n";
    assert_eq!(fenced_block_after(text, "## Example", "rust"), "right\n");
}

#[test]
fn a_fence_of_another_language_is_not_the_block() {
    let text = "\n## Example\n\n```toml\nwrong\n```\n\n```rust\nright\n```\n";
    assert_eq!(fenced_block_after(text, "## Example", "rust"), "right\n");
}

#[test]
#[should_panic(expected = "no `ts` block in the README section")]
fn a_block_in_a_later_section_is_not_the_block() {
    let text = "\n## Example\n\n```typescript\nrenamed\n```\n\n## Other\n\n```ts\nelsewhere\n```\n";
    fenced_block_after(text, "## Example", "ts");
}

#[test]
#[should_panic(expected = "is empty")]
fn an_empty_block_is_rejected() {
    let text = "\n## Example\n\n```ts\n```\n";
    fenced_block_after(text, "## Example", "ts");
}

#[test]
fn reference_output_is_what_the_readme_prints() {
    assert_eq!(
        fenced_block_after(README, "Will generate the following `.d.ts` file:", "ts"),
        REFERENCE,
        "the `.d.ts` the README prints and the reference this crate is compared \
         against have diverged; `./tests-e2e/reference_output/update_output.sh` \
         writes the emitted output over both"
    );
}
