#!/bin/bash

set -ex

cargo test --all
cargo test --all -F js
wasm-pack test --node
wasm-pack test --node -F js

# Test the end-to-end tests
./tests-e2e/build_all.sh
./tests-e2e/reference_output/compare_output.sh