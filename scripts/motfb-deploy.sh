#!/usr/bin/env bash
# Deploy a specific git ref's datapack to the live minecraft-papermc container
# and verify the deployed content matches source + difficulty is Hard.
#
# Usage: motfb-deploy.sh --ref <commit/branch/tag> [--allow-dirty]
#
# Source:      Extracted via git archive from the named ref
# Destination: /data/Times_Square__NYC/datapacks/motfb  (in container)

set -euo pipefail

CONTAINER="minecraft-papermc"
WORLD="Times_Square__NYC"
DATAPACK_DEST="/data/$WORLD/datapacks/motfb"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

GIT_REF=""
ALLOW_DIRTY=0

usage() {
    cat <<'EOF'
Usage: motfb-deploy.sh --ref <commit/branch/tag> [--allow-dirty]

Deploys a specific git ref's datapack to the live minecraft-papermc container.
Working tree state is irrelevant — always extracts from the named ref.

  --ref <commit|branch|tag>  Required. Git ref (commit SHA, branch name, or tag) to deploy.
  --allow-dirty              Optional. Allow running with a dirty working tree
                             (normally rejected as a safety guard).

Steps:
  1. Validates container is running
  2. Checks working tree is clean (unless --allow-dirty)
  3. Extracts datapack from git ref to temp staging directory
  4. Replaces the live datapack via docker cp
  5. Reloads datapacks via rcon
  6. Content-level diff: staging vs live (verifies exact match)
  7. Runs motfb:init and verifies difficulty is Hard
  8. Exits non-zero on any failure

Exit codes:
  0  deploy successful; deployed ref matches source and difficulty confirmed Hard
  1  validation, git, or runtime error
EOF
    exit 0
}

parse_args() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --ref)
                GIT_REF="$2"
                shift 2
                ;;
            --allow-dirty)
                ALLOW_DIRTY=1
                shift
                ;;
            --help|-h)
                usage
                ;;
            *)
                echo "ERROR: unknown argument: $1" >&2
                echo "Use --help for usage" >&2
                exit 1
                ;;
        esac
    done
}

parse_args "$@"

if [[ -z "$GIT_REF" ]]; then
    echo "ERROR: --ref <commit/branch/tag> is required" >&2
    echo "Use --help for usage" >&2
    exit 1
fi

cd "$REPO_ROOT"

if [[ $ALLOW_DIRTY -eq 0 ]]; then
    if ! git diff-index --quiet HEAD --; then
        echo "ERROR: working tree is dirty" >&2
        echo "Commit changes or use --allow-dirty to override" >&2
        exit 1
    fi
fi

if ! docker inspect --format='{{.State.Running}}' "$CONTAINER" 2>/dev/null | grep -q "true"; then
    echo "Error: container '$CONTAINER' is not running" >&2
    exit 1
fi

DEPLOYED_COMMIT=$(git rev-parse "$GIT_REF")
if [[ -z "$DEPLOYED_COMMIT" ]]; then
    echo "ERROR: git ref not found: $GIT_REF" >&2
    exit 1
fi

echo "==> Git ref: $GIT_REF"
echo "==> Deployed commit: $DEPLOYED_COMMIT"
echo ""

STAGING_DIR=$(mktemp -d)
trap "rm -rf '$STAGING_DIR'" EXIT

echo "==> Extracting datapack from $DEPLOYED_COMMIT to $STAGING_DIR..."
git archive "$DEPLOYED_COMMIT" output/motfb-datapack/ | tar -x -C "$STAGING_DIR"

STAGING_PACK="$STAGING_DIR/output/motfb-datapack"
if [[ ! -d "$STAGING_PACK" ]]; then
    echo "ERROR: datapack not found in $DEPLOYED_COMMIT:output/motfb-datapack" >&2
    exit 1
fi

echo "==> Removing existing datapack from container..."
docker exec "$CONTAINER" rm -rf "$DATAPACK_DEST"

echo "==> Copying extracted datapack to $CONTAINER:$DATAPACK_DEST..."
docker cp "$STAGING_PACK" "$CONTAINER:$DATAPACK_DEST"

echo "==> Setting ownership..."
docker exec "$CONTAINER" chown -R minecraft:minecraft "$DATAPACK_DEST"

echo "==> Verifying deployed content matches source..."
LIVE_TEMP=$(mktemp -d)
trap "rm -rf '$STAGING_DIR' '$LIVE_TEMP'" EXIT
docker cp "$CONTAINER:$DATAPACK_DEST" "$LIVE_TEMP/motfb"
if ! diff -rq "$STAGING_PACK" "$LIVE_TEMP/motfb" > /dev/null; then
    echo "ERROR: deployed content does not match staging source" >&2
    echo "Staging: $STAGING_PACK"
    echo "Live:    $LIVE_TEMP/motfb"
    diff -rq "$STAGING_PACK" "$LIVE_TEMP/motfb" || true
    exit 1
fi
echo "  ✓ Content matches"

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
echo "  Deployed commit: $DEPLOYED_COMMIT"
echo "  Container:       $CONTAINER"
echo "  World:           $WORLD"
echo "  Datapack:        $DATAPACK_DEST"
echo "  Content diff:    ✓"
echo "  Difficulty:      Hard ✓"
