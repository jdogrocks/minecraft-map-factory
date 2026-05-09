#!/usr/bin/env bash
# Push the current branch's datapack to the live minecraft-papermc container
# and verify difficulty flips to Hard.
#
# Usage: motfb-deploy.sh [--help]
#
# Source:      output/motfb-datapack  (repo root)
# Destination: /data/Times_Square__NYC/datapacks/motfb  (in container)

set -euo pipefail

CONTAINER="minecraft-papermc"
WORLD="Times_Square__NYC"
DATAPACK_DEST="/data/$WORLD/datapacks/motfb"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
DATAPACK_SRC="$REPO_ROOT/output/motfb-datapack"

usage() {
    cat <<'EOF'
Usage: motfb-deploy.sh [--help]

Deploys the current branch HEAD datapack to the live minecraft-papermc container.

  Source:      <repo>/output/motfb-datapack
  Destination: /data/Times_Square__NYC/datapacks/motfb  (inside container)

Steps:
  1. Validates container is running and source datapack exists
  2. Replaces the live datapack via docker cp
  3. Reloads datapacks via rcon
  4. Runs motfb:init and verifies difficulty is Hard
  5. Exits non-zero on any failure

Exit codes:
  0  deploy successful; difficulty confirmed Hard
  1  validation or runtime error
EOF
    exit 0
}

[[ "${1:-}" == "--help" || "${1:-}" == "-h" ]] && usage

if [[ ! -d "$DATAPACK_SRC" ]]; then
    echo "Error: datapack not found at $DATAPACK_SRC" >&2
    exit 1
fi

if ! docker inspect --format='{{.State.Running}}' "$CONTAINER" 2>/dev/null | grep -q "true"; then
    echo "Error: container '$CONTAINER' is not running" >&2
    exit 1
fi

echo "==> Removing existing datapack from container..."
docker exec "$CONTAINER" rm -rf "$DATAPACK_DEST"

echo "==> Copying $DATAPACK_SRC -> $CONTAINER:$DATAPACK_DEST"
docker cp "$DATAPACK_SRC" "$CONTAINER:$DATAPACK_DEST"

echo "==> Setting ownership..."
docker exec "$CONTAINER" chown -R minecraft:minecraft "$DATAPACK_DEST"

echo "==> Reloading datapacks..."
docker exec "$CONTAINER" rcon-cli reload

echo "==> Running motfb:init..."
INIT_OUT=$(docker exec "$CONTAINER" rcon-cli "function motfb:init" 2>&1)
echo "$INIT_OUT"
if echo "$INIT_OUT" | grep -qiE "(error|unknown function|no function|failed)"; then
    echo "ERROR: motfb:init reported an error" >&2
    exit 1
fi

echo "==> Verifying difficulty is Hard..."
DIFF_OUT=$(docker exec "$CONTAINER" rcon-cli "difficulty" 2>&1)
echo "$DIFF_OUT"
if ! echo "$DIFF_OUT" | grep -qi "hard"; then
    echo "ERROR: difficulty is not Hard after motfb:init — got: $DIFF_OUT" >&2
    exit 1
fi

echo ""
echo "=== Deploy successful ==="
echo "  Container:  $CONTAINER"
echo "  World:      $WORLD"
echo "  Datapack:   $DATAPACK_DEST"
echo "  Difficulty: Hard ✓"
