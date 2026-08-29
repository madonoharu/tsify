# tests-e2e

These tests test the actual `.d.ts` file output by `wasm-pack`. When `wasm-pack build` is run
on a project in one of the sub-folders, a `pkg/` directory will be created. Running `./reference_output/compare_output.sh`
will compare the reference output (stored in a directory match the test name) to that generated in the `pkg/` directory
output by `wasm-pack`.

When the change of output is one you meant to make, `./reference_output/update_output.sh` writes what
was built over the references, the way `MACROTEST=overwrite` blesses the expansion snapshots. Build
first — it blesses whatever is in `pkg/`, including a stale one — and then read the `git diff`.

`test_readme_quickstart1` is the README's quickstart rather than a copy of it: its `build.rs` extracts
the Rust block the README prints and the crate includes that. Its reference is the `.d.ts` the README
prints underneath, so a change of output shows up as a diff in the documentation, and
`update_output.sh` writes that block too ([#114](https://github.com/madonoharu/tsify/issues/114)).
