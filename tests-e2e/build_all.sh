#!/bin/bash

# Define the root directory for the search
ROOT_DIR="tests-e2e"

# Find all Cargo.toml files in the root directory and its direct subdirectories
FILES=$(find "$ROOT_DIR" -maxdepth 2 -name Cargo.toml)

for FILE in $FILES; do
    # Get the directory of the file
    DIR=$(dirname "$FILE")
    # Push the directory onto the stack and change to it
    pushd "$DIR" > /dev/null || exit
    
    echo ""
    echo "Building in $DIR"
    echo ""

    # Run wasm-pack build
    wasm-pack build
    # Pop the directory from the stack and change back to the original directory
    popd > /dev/null || exit
done