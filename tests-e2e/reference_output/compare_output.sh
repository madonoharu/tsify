#!/bin/bash

# Given files <folder>/<file> search for identically named files in `../<folder>/pkg/<file>`
# and compare them. If the files differ, print the differences and return an error exit code.

# Change the current directory to the directory where the script is located
cd "$(dirname "$0")" || exit

DIFF_FOUND=0

# Find all directories in the current directory
for FOLDERNAME in $(find . -maxdepth 1 -type d); do
    # Skip the current directory
    if [ "$FOLDERNAME" = "." ]; then
        continue
    fi

    # Remove the leading "./" from the folder name
    FOLDERNAME="${FOLDERNAME#./}"

    # Find all files in the current folder and its subdirectories
    FILES=$(find "./${FOLDERNAME}/" -type f)
    for FILE in $FILES; do
        # Remove the leading "./<foldername>/" from the file path
        RELATIVE_PATH="${FILE#./${FOLDERNAME}/}"
        # Construct the path to the corresponding file in ../<foldername>/pkg/
        OTHER_FILE="../${FOLDERNAME}/pkg/${RELATIVE_PATH}"
        # Compare the files
        if [ -f "$OTHER_FILE" ]; then
            echo "Comparing $FILE with $OTHER_FILE"
            
            # suppress the exit code if the files differ beause we want to print out all differences
            DIFF_OUTPUT=$(diff --brief "$FILE" "$OTHER_FILE" || true)
            if [ -n "$DIFF_OUTPUT" ]; then
                echo "   $DIFF_OUTPUT"
                diff "$FILE" "$OTHER_FILE"
                DIFF_FOUND=1
            else
                echo "   Files are identical"
            fi
        fi
    done
done

exit $DIFF_FOUND