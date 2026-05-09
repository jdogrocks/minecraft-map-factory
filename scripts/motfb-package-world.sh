#!/usr/bin/env bash
# Rebuild the MOTFB world zip from the live container's current world state.
#
# Usage: motfb-package-world.sh <zip-name> [--help]
#
# Pulls Times_Square__NYC from the live container and writes
# /home/jason/motfb-docs/<zip-name>.zip, overwriting any existing file.
# Run this after motfb-deploy.sh confirms the container matches HEAD.

set -euo pipefail

CONTAINER="minecraft-papermc"
WORLD="Times_Square__NYC"
WORLD_IN_CONTAINER="/data/$WORLD"
MOTFB_DOCS="/home/jason/motfb-docs"

usage() {
    cat <<'EOF'
Usage: motfb-package-world.sh <zip-name>

  <zip-name>   Output filename without .zip, e.g. motfb-phase-d-rev2

Pulls the live container world and writes /home/jason/motfb-docs/<zip-name>.zip.
Run after scripts/motfb-deploy.sh has confirmed the container matches HEAD and
all sibling sub-issues have merged to the branch.

Exit codes:
  0  zip written successfully
  1  validation or runtime error
EOF
    exit 0
}

[[ "${1:-}" == "--help" || "${1:-}" == "-h" ]] && usage

if [[ $# -lt 1 ]]; then
    echo "Error: missing <zip-name>" >&2
    echo "Run '$0 --help' for usage." >&2
    exit 1
fi

ZIP_NAME="$1"

if [[ "$ZIP_NAME" == *".."* || "$ZIP_NAME" == *"/"* ]]; then
    echo "Error: zip name contains invalid characters: $ZIP_NAME" >&2
    exit 1
fi

OUTPUT_ZIP="$MOTFB_DOCS/${ZIP_NAME}.zip"

if ! docker inspect --format='{{.State.Running}}' "$CONTAINER" 2>/dev/null | grep -q "true"; then
    echo "Error: container '$CONTAINER' is not running" >&2
    exit 1
fi

if [[ ! -d "$MOTFB_DOCS" ]]; then
    echo "Error: motfb-docs directory not found: $MOTFB_DOCS" >&2
    exit 1
fi

TMPDIR_WORLD=$(mktemp -d)
trap 'rm -rf "$TMPDIR_WORLD"' EXIT

echo "==> Pulling world from $CONTAINER:$WORLD_IN_CONTAINER..."
docker cp "$CONTAINER:$WORLD_IN_CONTAINER" "$TMPDIR_WORLD/$WORLD"

echo "==> Building zip: $OUTPUT_ZIP..."
rm -f "$OUTPUT_ZIP"
(cd "$TMPDIR_WORLD" && zip -r "$OUTPUT_ZIP" "$WORLD" -x "*.lock" -x "session.lock")

echo ""
echo "=== Package complete ==="
echo "  World:  $WORLD"
echo "  Output: $OUTPUT_ZIP"
echo "  Size:   $(du -sh "$OUTPUT_ZIP" | cut -f1)"
