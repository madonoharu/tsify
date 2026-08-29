// Shared by `build.rs`, which extracts the README's Rust block so that this
// crate builds the example itself, and by `tests/matches_readme.rs`, which
// extracts the TypeScript block printed beneath it. `include!`d rather than
// imported: a build script and an integration test have no crate in common,
// and the two must agree on what "the block" means.

/// The body of the first fenced `lang` block that follows the line `anchor`.
///
/// `anchor` is matched as a whole line, so a deeper heading ending in the same
/// words is not mistaken for it, and neither is a mention of it in prose.
fn fenced_block_after<'a>(text: &'a str, anchor: &str, lang: &str) -> &'a str {
    let anchor_line = format!("\n{anchor}\n");
    let after_anchor = text
        .split_once(anchor_line.as_str())
        .unwrap_or_else(|| panic!("the README no longer has a line {anchor:?}"))
        .1;
    let open = format!("```{lang}\n");
    let body = after_anchor
        .split_once(open.as_str())
        .unwrap_or_else(|| panic!("no `{lang}` block after {anchor:?} in the README"))
        .1;
    body.split_once("```")
        .unwrap_or_else(|| panic!("unterminated `{lang}` block after {anchor:?} in the README"))
        .0
}
