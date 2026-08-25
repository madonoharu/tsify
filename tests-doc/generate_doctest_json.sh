#!/bin/bash

# Generate doctests as json in target/doctests.json
echo "Generating doctest JSON"
cargo +nightly rustdoc -p tsify --lib  -- -Zunstable-options --output-format doctest > target/doctests.json
