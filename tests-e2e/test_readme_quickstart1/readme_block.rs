// Shared by `build.rs`, which extracts the README's Rust block so that this
// crate builds the example itself, and by `tests/matches_readme.rs`, which
// extracts the TypeScript block printed beneath it. `include!`d rather than
// imported: a build script and an integration test have no crate in common,
// and the two must agree on what "the block" means.

/// The body of the first fenced `lang` block that follows the line `anchor` and
/// belongs to the same section as it.
///
/// Both bounds matter. `anchor` is a whole line, so `### Example` is not
/// `## Example`. The search stops at the next `## ` heading, so renaming a
/// fence to a language that renders the same — ```` ```ts ```` to
/// ```` ```typescript ```` — fails here instead of quietly matching a block
/// in some later section that everything downstream would then agree on.
fn fenced_block_after<'a>(text: &'a str, anchor: &str, lang: &str) -> &'a str {
    let anchor_line = format!("\n{anchor}\n");
    let after_anchor = text
        .split_once(anchor_line.as_str())
        .unwrap_or_else(|| panic!("the README no longer has a line {anchor:?}"))
        .1;
    let section = match after_anchor.find("\n## ") {
        Some(next_heading) => &after_anchor[..next_heading],
        None => after_anchor,
    };
    let open = format!("```{lang}\n");
    let body = section
        .split_once(open.as_str())
        .unwrap_or_else(|| {
            panic!("no `{lang}` block in the README section that follows {anchor:?}")
        })
        .1;
    if body.starts_with("```") {
        panic!("the `{lang}` block after {anchor:?} in the README is empty");
    }
    let end = body
        .find("\n```")
        .unwrap_or_else(|| panic!("unterminated `{lang}` block after {anchor:?} in the README"));
    &body[..end + 1]
}
