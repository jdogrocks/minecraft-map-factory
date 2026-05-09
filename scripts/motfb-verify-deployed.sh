#!/usr/bin/env bash
# Verify the live container's deployed datapack matches a specific git ref.
#
# Usage: motfb-verify-deployed.sh [--ref <commit/branch/tag>] [--help]
#
# Diffs the live container's /data/Times_Square__NYC/datapacks/motfb against
# the named ref (defaults to HEAD). Exits non-zero and names any divergent
# files on mismatch. Working tree state is irrelevant.

set -euo pipefail

CONTAINER="minecraft-papermc"
WORLD="Times_Square__NYC"
DATAPACK_IN_CONTAINER="/data/$WORLD/datapacks/motfb"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

GIT_REF="HEAD"

usage() {
    cat <<'EOF'
Usage: motfb-verify-deployed.sh [--ref <commit/branch/tag>]

Checks that the live container's deployed datapack matches a specific git ref.
Defaults to HEAD if no ref is specified. Working tree state is irrelevant.

  --ref <commit|branch|tag>  Git ref to verify against (defaults to HEAD)

Exit codes:
  0  live container matches the specified ref exactly
  1  mismatch or infrastructure error; divergent files printed to stderr
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

cd "$REPO_ROOT"

VERIFY_COMMIT=$(git rev-parse "$GIT_REF")
if [[ -z "$VERIFY_COMMIT" ]]; then
    echo "ERROR: git ref not found: $GIT_REF" >&2
    exit 1
fi

if ! docker inspect --format='{{.State.Running}}' "$CONTAINER" 2>/dev/null | grep -q "true"; then
    echo "Error: container '$CONTAINER' is not running" >&2
    exit 1
fi

TMPDIR_VERIFY=$(mktemp -d)
trap 'rm -rf "$TMPDIR_VERIFY"' EXIT

echo "==> Extracting $GIT_REF ($VERIFY_COMMIT) from git..."
git archive "$VERIFY_COMMIT" output/motfb-datapack/ | tar -x -C "$TMPDIR_VERIFY"
STAGING_PACK="$TMPDIR_VERIFY/output/motfb-datapack"

if [[ ! -d "$STAGING_PACK" ]]; then
    echo "ERROR: datapack not found in $VERIFY_COMMIT:output/motfb-datapack" >&2
    exit 1
fi

echo "==> Pulling deployed datapack from $CONTAINER:$DATAPACK_IN_CONTAINER..."
docker cp "$CONTAINER:$DATAPACK_IN_CONTAINER" "$TMPDIR_VERIFY/motfb-live"

echo "==> Diffing live container against $GIT_REF..."
DIFF_EXIT=0
DIFF_DETAIL=$(diff -r "$STAGING_PACK" "$TMPDIR_VERIFY/motfb-live" 2>&1) || DIFF_EXIT=$?

if [[ $DIFF_EXIT -ne 0 ]]; then
    echo ""
    echo "ERROR: Live container datapack does NOT match $GIT_REF." >&2
    echo "Run scripts/motfb-deploy.sh --ref $GIT_REF to sync." >&2
    echo ""
    echo "Divergent files:" >&2
    diff -rq "$STAGING_PACK" "$TMPDIR_VERIFY/motfb-live" 2>&1 \
        | sed "s|$STAGING_PACK|$GIT_REF|g; s|$TMPDIR_VERIFY/motfb-live|LIVE|g" >&2
    exit 1
fi

echo ""
echo "=== Verification passed ==="
echo "  Live container matches $GIT_REF ($VERIFY_COMMIT)"
echo "  Ref:   $GIT_REF"
echo "  Live:  $CONTAINER:$DATAPACK_IN_CONTAINER"
