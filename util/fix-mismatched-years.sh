#!/usr/bin/env bash
BASE_DIR="/datalake/staging/transcode/source"

find "$BASE_DIR" -type f -name '*.mkv' | while read -r FILE; do
    DIRNAME=$(dirname "$FILE")
    BASENAME=$(basename "$FILE")

    # Find the closest matching parent directory like "Title (2025)"
    MATCHING_PARENT=$(echo "$DIRNAME" | tr '/' '\n' | grep -E '.* \([0-9]{4}\)' | tail -n 1)

    # Skip if no such parent
    if [[ -z "$MATCHING_PARENT" ]]; then
        echo "Skipping (no matching folder with (YYYY)): $FILE"
        continue
    fi

    # Extract just the "Title" part from "Title (2025)"
    TITLE_ONLY="${MATCHING_PARENT% \(*}"

    # Skip if file already starts with "Title (YYYY)"
    if [[ "$BASENAME" == "$MATCHING_PARENT - "* ]]; then
        echo "Skipping (already renamed): $FILE"
        continue
    fi

    # Strip matching title from beginning of filename (if exists)
    REMAINDER="$BASENAME"
    if [[ "$REMAINDER" == "$TITLE_ONLY - "* ]]; then
        REMAINDER="${REMAINDER#"$TITLE_ONLY - "}"
    fi

    # Construct new filename
    NEW_BASENAME="${MATCHING_PARENT} - ${REMAINDER}"
    NEW_PATH="${DIRNAME}/${NEW_BASENAME}"

    echo "mv -n \"$FILE\" \"$NEW_PATH\""
    mv -n "$FILE" "$NEW_PATH"
done
