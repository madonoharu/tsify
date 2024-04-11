# tests-e2e

These tests test the actual `.d.ts` file output by `wasm-pack`. When `wasm-pack build` is run
on a project in one of the sub-folders, a `pkg/` directory will be created. Running `./reference_output/compare_output.sh`
will compare the reference output (stored in a directory match the test name) to that generated in the `pkg/` directory
output by `wasm-pack`.
