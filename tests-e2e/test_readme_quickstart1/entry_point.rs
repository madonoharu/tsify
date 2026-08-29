// The README's quickstart, built as itself rather than as a copy.
//
// `build.rs` extracts the Rust block printed under `## Example` into `OUT_DIR`
// and this file includes it, so the example has exactly one home and the crate
// cannot build from anything else. What is left to check is the other half:
// the `.d.ts` printed beneath that block, which nothing verified (#114).
//
// The harness diffs what this crate emits against `reference_output/`, and
// `tests/matches_readme.rs` holds that reference to the text the README
// prints. To bless an intended change of output:
//
//     ./tests-e2e/build_all.sh
//     ./tests-e2e/reference_output/update_output.sh

#![allow(unused_variables)]

include!(concat!(env!("OUT_DIR"), "/readme_quickstart.rs"));
