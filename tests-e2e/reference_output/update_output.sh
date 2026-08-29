#!/bin/bash

# The inverse of compare_output.sh: writes what `wasm-pack build` last emitted
# over the references, and over the one block of the README that is a reference
# too (see ../test_readme_quickstart1). It blesses whatever is in `pkg/`, so
# build first.
#
#     ./tests-e2e/build_all.sh
#     ./tests-e2e/reference_output/update_output.sh [test_name]
#     git diff
#
# It walks the references rather than `pkg/`, so it updates what is already kept
# here and does not start tracking new files. Nothing is written until
# everything has been read, or a README it cannot parse would leave the
# references updated and the README not.

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
QUICKSTART="test_readme_quickstart1"
QUICKSTART_GENERATED="../${QUICKSTART}/pkg/${QUICKSTART}.d.ts"
# The line the README prints the quickstart's `.d.ts` under.
ANCHOR='Will generate the following `.d.ts` file:'

# The set of reference files to update, as `<reference>\t<generated>` lines.
# `find` is the same walk compare_output.sh makes, so the two stay in step.
pairs() {
    for FOLDERNAME in $(find . -maxdepth 1 -type d); do
        if [ "$FOLDERNAME" = "." ]; then
            continue
        fi
        FOLDERNAME="${FOLDERNAME#./}"
        if [ -n "$TARGET" ] && [ "$FOLDERNAME" != "$TARGET" ]; then
            continue
        fi
        for FILE in $(find "./${FOLDERNAME}/" -type f); do
            RELATIVE_PATH="${FILE#./${FOLDERNAME}/}"
            printf '%s\t%s\n' "$FILE" "../${FOLDERNAME}/pkg/${RELATIVE_PATH}"
        done
    done
}

# ── Phase 1: read and validate. No writes. ───────────────────────────────────

PAIRS=$(pairs)

echo "$PAIRS" | while IFS="$(printf '\t')" read -r FILE GENERATED; do
    [ -n "$FILE" ] || continue
    # A reference with nothing to update from means the build did not run, or
    # stopped emitting the file. Blessing the rest would hide that.
    if [ ! -f "$GENERATED" ]; then
        echo "Missing generated file: $GENERATED" >&2
        echo "   run ./tests-e2e/build_all.sh first" >&2
        exit 1
    fi
done

TMP=""
if [ -z "$TARGET" ] || [ "$TARGET" = "$QUICKSTART" ]; then
    # The README prints the quickstart's `.d.ts` in full, so the block and the
    # reference are the same text, and both come from the same build.
    # tests/matches_readme.rs is what fails when they stop being.
    TMP=$(mktemp)
    trap 'rm -f "$TMP"' EXIT

    awk -v generated="$QUICKSTART_GENERATED" -v anchor="$ANCHOR" '
        BEGIN {
            while ((getline line < generated) > 0) block = block line "\n"
            close(generated)
            state = 0
        }
        state == 0 { print; if ($0 == anchor) state = 1; next }
        # The block has to belong to the section the anchor is in. Walking past
        # a heading would rewrite something that belongs to another example —
        # see the note in ../test_readme_quickstart1/readme_block.rs.
        state == 1 {
            if ($0 ~ /^## /) {
                print "no `ts` block in the README section that follows " anchor > "/dev/stderr"
                bad = 1
                exit 1
            }
            print
            if ($0 == "```ts") { printf "%s", block; state = 2 }
            next
        }
        state == 2 { if ($0 == "```") { print; state = 3 } next }
        { print }
        END {
            if (!bad && state != 3) {
                print "could not find the `ts` block after " anchor > "/dev/stderr"
                exit 1
            }
        }
    ' "$README" > "$TMP"
fi

# ── Phase 2: write. Everything above has already succeeded. ──────────────────

echo "$PAIRS" | while IFS="$(printf '\t')" read -r FILE GENERATED; do
    [ -n "$FILE" ] || continue
    if cmp -s "$GENERATED" "$FILE"; then
        echo "Unchanged $FILE"
    else
        cp "$GENERATED" "$FILE"
        echo "Updated   $FILE"
    fi
done

if [ -n "$TMP" ]; then
    if cmp -s "$TMP" "$README"; then
        echo "Unchanged $README"
    else
        cp "$TMP" "$README"
        echo "Updated   $README"
    fi
fi
