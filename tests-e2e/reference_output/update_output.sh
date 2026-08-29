#!/bin/bash

# The inverse of compare_output.sh: writes what `wasm-pack build` last emitted
# over the references, and then over the one block of the README that is a
# reference too (see ../test_readme_quickstart1).
#
# It blesses whatever is in `pkg/` — like `MACROTEST=overwrite` for the
# expansion snapshots, it is for a change of output you meant to make. Build
# first, or it blesses a stale artifact:
#
#     ./tests-e2e/build_all.sh
#     ./tests-e2e/reference_output/update_output.sh
#     git diff
#
# It walks the references rather than `pkg/`, so it updates the files already
# kept here and does not start tracking new ones.
#
# One test can be named, so that a change to a single example does not ask for
# every crate to have been built:
#
#     ./tests-e2e/reference_output/update_output.sh test_readme_quickstart1

set -e

cd "$(dirname "$0")" || exit 1

TARGET="${1:-}"
if [ -n "$TARGET" ] && [ ! -d "./$TARGET" ]; then
    echo "No reference directory ./$TARGET"
    echo "   the directories here are:"
    find . -maxdepth 1 -type d ! -name . -exec basename {} \;
    exit 1
fi

README="../../README.md"
QUICKSTART_REFERENCE="test_readme_quickstart1/test_readme_quickstart1.d.ts"
# The line the README prints the quickstart's `.d.ts` under.
ANCHOR='Will generate the following `.d.ts` file:'

for FOLDERNAME in $(find . -maxdepth 1 -type d); do
    if [ "$FOLDERNAME" = "." ]; then
        continue
    fi

    FOLDERNAME="${FOLDERNAME#./}"

    if [ -n "$TARGET" ] && [ "$FOLDERNAME" != "$TARGET" ]; then
        continue
    fi

    FILES=$(find "./${FOLDERNAME}/" -type f)
    for FILE in $FILES; do
        RELATIVE_PATH="${FILE#./${FOLDERNAME}/}"
        GENERATED="../${FOLDERNAME}/pkg/${RELATIVE_PATH}"

        # A reference with nothing to update from means the build did not run,
        # or stopped emitting the file. Blessing the rest would hide that.
        if [ ! -f "$GENERATED" ]; then
            echo "Missing generated file: $GENERATED"
            echo "   run ./tests-e2e/build_all.sh first"
            exit 1
        fi

        if cmp -s "$GENERATED" "$FILE"; then
            echo "Unchanged $FILE"
        else
            cp "$GENERATED" "$FILE"
            echo "Updated   $FILE"
        fi
    done
done

# The README prints the quickstart's `.d.ts` in full, so the block and the
# reference are the same text. tests/matches_readme.rs is what fails when they
# stop being.
if [ -n "$TARGET" ] && [ "$TARGET" != "test_readme_quickstart1" ]; then
    exit 0
fi

TMP=$(mktemp)
trap 'rm -f "$TMP"' EXIT

awk -v ref="$QUICKSTART_REFERENCE" -v anchor="$ANCHOR" '
    BEGIN {
        while ((getline line < ref) > 0) block = block line "\n"
        close(ref)
        state = 0
    }
    state == 0 { print; if ($0 == anchor) state = 1; next }
    state == 1 { print; if ($0 == "```ts") { printf "%s", block; state = 2 } next }
    state == 2 { if ($0 == "```") { print; state = 3 } next }
    { print }
    END {
        if (state != 3) {
            print "could not find the `ts` block after " anchor > "/dev/stderr"
            exit 1
        }
    }
' "$README" > "$TMP"

if cmp -s "$TMP" "$README"; then
    echo "Unchanged $README"
else
    cp "$TMP" "$README"
    echo "Updated   $README"
fi
