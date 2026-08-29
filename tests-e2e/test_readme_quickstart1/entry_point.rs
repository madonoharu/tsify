// The README's quickstart, built as itself rather than as a copy: `build.rs`
// extracts the block and this includes it. What the harness then compares is
// the `.d.ts` printed beneath that block (#114).

#![allow(unused_variables)]

include!(concat!(env!("OUT_DIR"), "/readme_quickstart.rs"));
