#!/bin/bash

set -Eeuo pipefail

repo_root=$(cd "$(dirname "$0")/.." && pwd)
manifest_template="$repo_root/tests-e2e/minimum_wasm_bindgen.Cargo.toml"
source_file=${1:-"$repo_root/tests-e2e/minimum_wasm_bindgen.rs"}

if [ "$#" -gt 1 ] || [ ! -f "$source_file" ]; then
    echo "usage: $0 [entry-point.rs]" >&2
    exit 2
fi

# wasm-bindgen executes descriptors with its own interpreter. The interpreter
# in the declared minimum cannot execute control flow or unaligned byte access
# left in a dev Wasm module; release optimization can fold both away, and newer
# interpreters accept both. The compatibility gate is therefore this one small
# crate built with --dev *and* with the dependency and matching CLI pinned to
# the declared minimum. Neither half is a useful test on its own.
#
# Cargo reports a bare `0.2.104` requirement as `^0.2.104`. Accept exactly that
# simple shape so Cargo.toml remains the only place that owns the version. If
# the requirement becomes a range or moves, this extraction fails loudly
# instead of silently testing a different floor.
minimum_version=$(
    cargo metadata \
        --manifest-path "$repo_root/Cargo.toml" \
        --no-deps \
        --format-version 1 \
        | /usr/bin/jq -er --arg manifest "$repo_root/Cargo.toml" '
            [
                .packages[]
                | select(.manifest_path == $manifest)
                | .dependencies[]
                | select(.name == "wasm-bindgen" and .optional == true)
                | .req
                | capture("^\\^(?<version>[0-9]+\\.[0-9]+\\.[0-9]+)$").version
            ]
            | if length == 1 then .[0]
              else error("expected one simple wasm-bindgen minimum requirement")
              end
        '
)

# Keep Cargo.lock, target, and pkg outside tests-e2e. Besides leaving no stale
# workspace member behind, this lets the normal release build keep its own pkg
# output. wasm-pack selects the wasm-bindgen CLI matching the exact library
# version in this generated manifest.
probe_dir=$(mktemp -d "${TMPDIR:-/tmp}/tsify-wasm-bindgen-minimum.XXXXXX")
trap 'rm -rf "$probe_dir"' EXIT

# JSON strings are valid TOML basic strings for a filesystem path, and jq
# handles any quote or backslash in the checkout path before it is inserted.
tsify_path=$(/usr/bin/jq -Rn --arg path "$repo_root" '$path')
while IFS= read -r line || [ -n "$line" ]; do
    line=${line//@WASM_BINDGEN_MINIMUM@/$minimum_version}
    line=${line//@TSIFY_PATH@/$tsify_path}
    printf '%s\n' "$line"
done < "$manifest_template" > "$probe_dir/Cargo.toml"
cp "$source_file" "$probe_dir/entry_point.rs"

echo ""
echo "Building dev descriptor probe with wasm-bindgen $minimum_version"
echo "Source: $source_file"
echo ""

(
    cd "$probe_dir"
    wasm-pack build --target nodejs --dev
)
