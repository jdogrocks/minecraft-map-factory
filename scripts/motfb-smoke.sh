#!/usr/bin/env bash
# Smoke-test a datapack against the live minecraft-papermc Docker container.
#
# Usage: motfb-smoke.sh [--help] <datapack-path> [function1 function2 ...]

set -euo pipefail

CONTAINER="minecraft-papermc"
WORLD="Times_Square__NYC"
DATAPACKS_PATH="/data/$WORLD/datapacks"

usage() {
    cat <<'EOF'
Usage: motfb-smoke.sh <datapack-path> [function1 function2 ...]

  <datapack-path>   local path to a datapack directory (must exist)
  [function...]     datapack functions to invoke via rcon after enabling the pack

Guardrails enforced:
  - Temp pack is always prefixed smoke_test_ and auto-cleaned via trap (success or error)
  - Only datapacks/ is touched; world data files are never modified
  - Forbidden commands (/op /stop /save-off /ban /kick) are never sent to rcon

Exit codes:
  0  pack accepted; all requested functions ran without error
  1  validation or runtime error
EOF
    exit 0
}

[[ "${1:-}" == "--help" || "${1:-}" == "-h" ]] && usage

if [[ $# -lt 1 ]]; then
    echo "Error: missing <datapack-path>" >&2
    echo "Run '$0 --help' for usage." >&2
    exit 1
fi

PACK_PATH="$1"; shift
FUNCTIONS=("$@")

# --- Input validation ---

if [[ ! -e "$PACK_PATH" ]]; then
    echo "Error: datapack path does not exist: $PACK_PATH" >&2
    exit 1
fi

PACK_NAME="$(basename "${PACK_PATH%/}")"

# Guard against path traversal in the pack name
if [[ "$PACK_NAME" == *".."* || "$PACK_NAME" == *"/"* ]]; then
    echo "Error: pack name contains invalid characters: $PACK_NAME" >&2
    exit 1
fi

SMOKE_NAME="smoke_test_${PACK_NAME}"
REMOTE_PACK="$DATAPACKS_PATH/$SMOKE_NAME"

# Guardrail: the destination must stay inside DATAPACKS_PATH
RESOLVED_REMOTE="$(realpath -m "$REMOTE_PACK")"
RESOLVED_DATAPACKS="$(realpath -m "$DATAPACKS_PATH")"
if [[ "$RESOLVED_REMOTE" != "$RESOLVED_DATAPACKS"/* ]]; then
    echo "Error: resolved remote path escapes datapacks dir: $RESOLVED_REMOTE" >&2
    exit 1
fi

# Guardrail: reject forbidden command names in the function list
FORBIDDEN=(op stop save-off ban kick)
for fn in "${FUNCTIONS[@]+"${FUNCTIONS[@]}"}"; do
    fn_basename="${fn##*:}"
    for bad in "${FORBIDDEN[@]}"; do
        if [[ "$fn_basename" == "$bad" ]]; then
            echo "Error: forbidden function name '$fn'" >&2
            exit 1
        fi
    done
done

# Verify the container is running
if ! docker inspect --format='{{.State.Running}}' "$CONTAINER" 2>/dev/null | grep -q "true"; then
    echo "Error: container '$CONTAINER' is not running" >&2
    exit 1
fi

# --- Cleanup trap ---
# Runs on exit (success or error): removes the smoke pack and reloads datapacks.
cleanup() {
    local rc=$?
    echo ""
    echo "==> Cleanup: removing $SMOKE_NAME from container..."
    docker exec "$CONTAINER" rm -rf "$REMOTE_PACK" 2>/dev/null || true
    docker exec "$CONTAINER" rcon-cli reload >/dev/null 2>&1 || true
    exit $rc
}
trap cleanup EXIT

# Record log anchor before we do anything so log grep is scoped to this run
LOG_SINCE=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

# --- Deploy pack ---

echo "==> Copying pack -> $CONTAINER:$REMOTE_PACK"
docker cp "$PACK_PATH" "$CONTAINER:$REMOTE_PACK"

echo "==> Setting ownership..."
docker exec "$CONTAINER" chown -R minecraft:minecraft "$REMOTE_PACK"

# --- Reload and enable ---

echo "==> Reloading datapacks..."
docker exec "$CONTAINER" rcon-cli reload

echo "==> Enabling pack: file/$SMOKE_NAME"
ENABLE_OUT=$(docker exec "$CONTAINER" rcon-cli "datapack enable \"file/$SMOKE_NAME\"" 2>&1) || true
echo "$ENABLE_OUT"

ERRORS=0

if echo "$ENABLE_OUT" | grep -qiE "(unknown|no such|failed|error)"; then
    echo "ERROR: pack enable rejected or failed" >&2
    ERRORS=$((ERRORS + 1))
fi

# --- Capture pack_format and log warnings ---

echo ""
echo "==> Scanning container logs for pack_format / WARN / ERROR..."
LOG_HITS=$(docker logs --since "$LOG_SINCE" "$CONTAINER" 2>&1 | grep -iE "(pack_format|WARN|ERROR)" || true)
if [[ -n "$LOG_HITS" ]]; then
    echo "--- relevant log lines ---"
    echo "$LOG_HITS"
else
    echo "(no pack_format/WARN/ERROR lines in last 100 log lines)"
fi

# --- Run requested functions ---

if [[ ${#FUNCTIONS[@]} -gt 0 ]]; then
    echo ""
    echo "==> Running ${#FUNCTIONS[@]} function(s)..."
    for fn in "${FUNCTIONS[@]}"; do
        echo "--- function: $fn ---"
        FN_OUT=$(docker exec "$CONTAINER" rcon-cli "function $fn" 2>&1) || true
        echo "$FN_OUT"
        if echo "$FN_OUT" | grep -qiE "(error|unknown function|no function|failed)"; then
            echo "WARNING: function '$fn' reported an error" >&2
            ERRORS=$((ERRORS + 1))
        fi
    done
fi

# --- Summary ---

echo ""
echo "=== Smoke Test Summary ==="
echo "  Pack:        $PACK_NAME"
echo "  Smoke name:  $SMOKE_NAME"
echo "  Functions:   ${#FUNCTIONS[@]} invoked, $ERRORS error(s)"
[[ -n "$LOG_HITS" ]] && echo "  Log hits:    (see above)"
echo "=========================="

[[ $ERRORS -eq 0 ]]
