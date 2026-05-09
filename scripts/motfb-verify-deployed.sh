#!/usr/bin/env bash
# Verify the live container's deployed datapack matches the current branch HEAD.
#
# Usage: motfb-verify-deployed.sh [--help]
#
# Diffs the live container's /data/Times_Square__NYC/datapacks/motfb against
# output/motfb-datapack in the current worktree. Exits non-zero and names any
# divergent files on mismatch.

set -euo pipefail

CONTAINER="minecraft-papermc"
WORLD="Times_Square__NYC"
DATAPACK_IN_CONTAINER="/data/$WORLD/datapacks/motfb"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
DATAPACK_SRC="$REPO_ROOT/output/motfb-datapack"

usage() {
    cat <<'EOF'
Usage: motfb-verify-deployed.sh [--help]

Checks that the live container's deployed datapack matches the current branch HEAD.

  HEAD source: <repo>/output/motfb-datapack
  Live source: /data/Times_Square__NYC/datapacks/motfb  (inside container)

Exit codes:
  0  live container matches HEAD exactly
  1  mismatch or infrastructure error; divergent files printed to stderr
EOF
    exit 0
}

[[ "${1:-}" == "--help" || "${1:-}" == "-h" ]] && usage

if [[ ! -d "$DATAPACK_SRC" ]]; then
    echo "Error: local datapack not found at $DATAPACK_SRC" >&2
    exit 1
fi

if ! docker inspect --format='{{.State.Running}}' "$CONTAINER" 2>/dev/null | grep -q "true"; then
    echo "Error: container '$CONTAINER' is not running" >&2
    exit 1
fi

TMPDIR_VERIFY=$(mktemp -d)
trap 'rm -rf "$TMPDIR_VERIFY"' EXIT

echo "==> Pulling deployed datapack from $CONTAINER:$DATAPACK_IN_CONTAINER..."
docker cp "$CONTAINER:$DATAPACK_IN_CONTAINER" "$TMPDIR_VERIFY/motfb"

echo "==> Diffing live container against local HEAD..."
DIFF_EXIT=0
DIFF_DETAIL=$(diff -r "$DATAPACK_SRC" "$TMPDIR_VERIFY/motfb" 2>&1) || DIFF_EXIT=$?

if [[ $DIFF_EXIT -ne 0 ]]; then
    echo ""
    echo "ERROR: Live container datapack does NOT match HEAD." >&2
    echo "Run scripts/motfb-deploy.sh to sync." >&2
    echo ""
    echo "Divergent files:" >&2
    diff -rq "$DATAPACK_SRC" "$TMPDIR_VERIFY/motfb" 2>&1 \
        | sed "s|$DATAPACK_SRC|HEAD|g; s|$TMPDIR_VERIFY/motfb|LIVE|g" >&2
    exit 1
fi

echo ""
echo "=== Verification passed ==="
echo "  Live container matches HEAD"
echo "  HEAD:  $DATAPACK_SRC"
echo "  Live:  $CONTAINER:$DATAPACK_IN_CONTAINER"
