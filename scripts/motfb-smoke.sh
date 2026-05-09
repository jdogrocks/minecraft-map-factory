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
Usage: motfb-smoke.sh [OPTIONS] <datapack-path> [function1 function2 ...]

  <datapack-path>   local path to a datapack directory (must exist)
  [function...]     datapack functions to invoke via rcon after enabling the pack

Options:
  --behavioral      run behavioral assertions (Phase 2): sign content, floor flatness, etc.
  --spawn-bosses    (with --behavioral) spawn boss entities and assert their positions
  --help            show this message

Behavioral assertions verify in-world state, not just pack syntax:
  1. Sign content read-back — verify front_text.messages[0] on all 18 signs
  2. Floor flatness sweep — spot-check entrance floor at y=64
  3. Spawn block assertion — verify block at 0 64 -150 is not air
  4. Lighting density check — spot-check sea_lantern placement
  5. Boss entity coords — (optional) assert each boss is within storefront bounds
  6. Sign-format sanity — detect legacy Text1-4 schema (MIN-159 root cause)

Guardrails enforced:
  - Temp pack is always prefixed smoke_test_ and auto-cleaned via trap (success or error)
  - Only datapacks/ is touched; world data files are never modified
  - Forbidden commands (/op /stop /save-off /ban /kick) are never sent to rcon

Exit codes:
  0  pack accepted; all requested functions and assertions passed
  1  validation, runtime, or assertion error
EOF
    exit 0
}

[[ "${1:-}" == "--help" || "${1:-}" == "-h" ]] && usage

# Parse options
BEHAVIORAL_ASSERTIONS="false"
SPAWN_BOSSES="false"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --behavioral)
            BEHAVIORAL_ASSERTIONS="true"
            shift
            ;;
        --spawn-bosses)
            SPAWN_BOSSES="true"
            shift
            ;;
        --help|-h)
            usage
            ;;
        -*)
            echo "Error: unknown option '$1'" >&2
            exit 1
            ;;
        *)
            break
            ;;
    esac
done

if [[ $# -lt 1 ]]; then
    echo "Error: missing <datapack-path>" >&2
    echo "Run '$0 --help' for usage." >&2
    exit 1
fi

PACK_PATH="$1"; shift
FUNCTIONS=("$@")

# --- Pre-merge audit: reject command-block payloads containing debug leftovers ---
# Checks non-comment lines in .mcfunction files for say/test/debug commands.
echo "==> Pre-merge audit: scanning for debug command-block payloads..."
DEBUG_HITS=$(grep -rn --include="*.mcfunction" -E '(^|\brun\s+)(say|test|debug)\s' "$PACK_PATH" 2>/dev/null || true)
if [[ -n "$DEBUG_HITS" ]]; then
    echo "ERROR: debug command-block payloads detected — remove before merging:" >&2
    echo "$DEBUG_HITS" >&2
    exit 1
fi
echo "(no debug payloads found)"

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

# --- Phase 2: Behavioral Assertions (if requested) ---

# Assertion helper function
assert_block_matches() {
    local x=$1 y=$2 z=$3 expected=$4 desc=$5
    local result=$(docker exec "$CONTAINER" rcon-cli "execute if block $x $y $z minecraft:$expected" 2>&1) || true
    if [[ "$result" == "1" ]] || echo "$result" | grep -q "Test passed"; then
        echo "  ✓ $desc at $x $y $z"
        return 0
    else
        echo "  ✗ $desc at $x $y $z: expected=$expected observed=no-match"
        return 1
    fi
}

assert_block_data() {
    local x=$1 y=$2 z=$3 path=$4 expected=$5 desc=$6
    local result=$(docker exec "$CONTAINER" rcon-cli "data get block $x $y $z $path" 2>&1) || true
    # Extract the JSON value and check if it matches expected
    if echo "$result" | grep -q "$expected"; then
        echo "  ✓ $desc at $x $y $z"
        return 0
    else
        echo "  ✗ $desc at $x $y $z: expected=$expected observed=$result"
        return 1
    fi
}

assert_entity_exists() {
    local selector=$1 desc=$2
    local result=$(docker exec "$CONTAINER" rcon-cli "execute if entity $selector" 2>&1) || true
    if [[ "$result" == "1" ]]; then
        echo "  ✓ $desc"
        return 0
    else
        echo "  ✗ $desc: entity not found"
        return 1
    fi
}

assert_no_legacy_sign() {
    local x=$1 y=$2 z=$3
    local result=$(docker exec "$CONTAINER" rcon-cli "data get block $x $y $z Text1" 2>&1) || true
    if echo "$result" | grep -qiE "(error|no such|cannot find|no elements matching)" || [[ -z "$result" ]]; then
        # Modern format - Text1 does not exist
        return 0
    else
        # Legacy format detected
        echo "  ✗ Sign at $x $y $z uses legacy Text1-4 schema (root cause of MIN-159 sign bug)"
        return 1
    fi
}

if [[ "${BEHAVIORAL_ASSERTIONS:-false}" == "true" ]]; then
    echo ""
    echo "==> Phase 2: Behavioral Assertions"
    ASSERTION_ERRORS=0

    # Assertion 1: Sign content read-back
    echo ""
    echo "  Assertion 1: Sign content read-back (18 signs)"
    SIGN_COORDS=(
        "-1:71:-272" "-6:70:-270" "6:70:-270" "-6:70:-253" "6:70:-253"
        "-6:67:-238" "-6:70:-238" "6:70:-238" "-6:70:-223" "6:70:-223"
        "-6:99:-218" "-6:70:-208" "6:70:-208" "-6:70:-193"
        "-1:67:-134" "0:71:-125" "-6:71:-110" "6:71:-110"
    )
    LEGACY_SIGN_COUNT=0
    for coord in "${SIGN_COORDS[@]}"; do
        IFS=':' read -r x y z <<< "$coord"
        # Check for legacy format
        if ! assert_no_legacy_sign "$x" "$y" "$z"; then
            LEGACY_SIGN_COUNT=$((LEGACY_SIGN_COUNT + 1))
            ASSERTION_ERRORS=$((ASSERTION_ERRORS + 1))
        fi
        # Read front text (basic check - just verify command succeeds)
        text_result=$(docker exec "$CONTAINER" rcon-cli "data get block $x $y $z front_text.messages[0]" 2>&1) || true
        if [[ -z "$text_result" || "$text_result" =~ error ]]; then
            echo "  ✗ Sign at $x $y $z: cannot read front_text.messages[0]"
            ASSERTION_ERRORS=$((ASSERTION_ERRORS + 1))
        else
            echo "  ✓ Sign at $x $y $z: ${text_result:0:60}..."
        fi
    done

    # Assertion 2: Floor flatness sweep (sample 5 entrance floor coords)
    echo ""
    echo "  Assertion 2: Floor flatness sweep (entrance y=64)"
    FLOOR_COORDS=(
        "-8:64:-100" "8:64:-100" "-8:64:-85" "8:64:-85" "0:64:-93"
    )
    for coord in "${FLOOR_COORDS[@]}"; do
        IFS=':' read -r x y z <<< "$coord"
        if ! assert_block_matches "$x" "$y" "$z" "smooth_quartz" "Entrance floor"; then
            ASSERTION_ERRORS=$((ASSERTION_ERRORS + 1))
        fi
    done

    # Assertion 3: Spawn block assertion
    echo ""
    echo "  Assertion 3: Spawn block assertion (0 64 -150)"
    spawn_result=$(docker exec "$CONTAINER" rcon-cli "data get block 0 64 -150" 2>&1) || true
    if [[ -z "$spawn_result" || "$spawn_result" =~ error ]]; then
        echo "  ✗ Spawn block at 0 64 -150: cannot read block data"
        ASSERTION_ERRORS=$((ASSERTION_ERRORS + 1))
    else
        # Verify it's not air
        if echo "$spawn_result" | grep -q "minecraft:air"; then
            echo "  ✗ Spawn block at 0 64 -150: is air"
            ASSERTION_ERRORS=$((ASSERTION_ERRORS + 1))
        else
            echo "  ✓ Spawn block at 0 64 -150: ${spawn_result:0:60}..."
        fi
    fi

    # Assertion 4: Lighting density check (spot-check sea lanterns)
    echo ""
    echo "  Assertion 4: Lighting density check (spot-checks)"
    LIGHT_COORDS=(
        "0:79:-279" "0:79:-275" "0:79:-203" "0:79:-199"
    )
    LIGHT_COUNT=0
    for coord in "${LIGHT_COORDS[@]}"; do
        IFS=':' read -r x y z <<< "$coord"
        light_result=$(docker exec "$CONTAINER" rcon-cli "execute if block $x $y $z minecraft:sea_lantern" 2>&1) || true
        if [[ "$light_result" == "1" ]] || echo "$light_result" | grep -q "Test passed"; then
            echo "  ✓ Light source found at $x $y $z"
            LIGHT_COUNT=$((LIGHT_COUNT + 1))
        fi
    done
    echo "  ℹ Light sources found: $LIGHT_COUNT/4 spot-checks"
    if [[ $LIGHT_COUNT -lt 2 ]]; then
        echo "  ✗ Insufficient light sources (expected >= 2, found $LIGHT_COUNT)"
        ASSERTION_ERRORS=$((ASSERTION_ERRORS + 1))
    fi

    # Assertion 5: Boss entity coords (optional - requires --spawn-bosses)
    if [[ "${SPAWN_BOSSES:-false}" == "true" ]]; then
        echo ""
        echo "  Assertion 5: Boss entity coords (9 bosses)"
        BOSS_BOUNDS=(
            "Kraw:-50:60:-260:44:20:14"
            "Imp_Swarm:6:60:-260:44:20:14"
            "Pixel_Lich:-50:60:-245:44:20:14"
            "Exiled_Saint:6:60:-245:44:20:14"
            "Candy_Witch:-50:60:-230:44:20:14"
            "Knot_God:6:60:-230:44:20:14"
            "Stitch_Lord:-50:60:-215:44:20:14"
            "Speed_Demon:6:60:-215:44:20:14"
            "Vampire_Queen:-50:60:-200:44:20:14"
        )
        for bound in "${BOSS_BOUNDS[@]}"; do
            IFS=':' read -r name x y z dx dy dz <<< "$bound"
            boss_tag="motfb_${name,,}_boss"
            if ! assert_entity_exists "@e[tag=$boss_tag,x=$x,y=$y,z=$z,dx=$dx,dy=$dy,dz=$dz]" "$name boss in bounds"; then
                ASSERTION_ERRORS=$((ASSERTION_ERRORS + 1))
            fi
        done
    fi

    # Assertion 6: Sign-format sanity (already checked above in Assertion 1)
    # Summary printed after Assertion 1

    echo ""
    echo "==> Behavioral Assertion Summary"
    if [[ $LEGACY_SIGN_COUNT -gt 0 ]]; then
        echo "  Legacy signs found: $LEGACY_SIGN_COUNT"
    fi
    echo "  Assertion errors: $ASSERTION_ERRORS"
    ERRORS=$((ERRORS + ASSERTION_ERRORS))
fi

# --- Summary ---

echo ""
echo "=== Smoke Test Summary ==="
echo "  Pack:        $PACK_NAME"
echo "  Smoke name:  $SMOKE_NAME"
echo "  Functions:   ${#FUNCTIONS[@]} invoked"
if [[ "${BEHAVIORAL_ASSERTIONS:-false}" == "true" ]]; then
    echo "  Assertions:  behavioral phase completed, $ASSERTION_ERRORS error(s)"
fi
echo "  Total errors: $ERRORS"
[[ -n "$LOG_HITS" ]] && echo "  Log hits:    (see above)"
echo "=========================="

echo ""
echo "*** POST-SESSION REMINDER ***"
echo "    If you placed any command blocks in-world during this test, remove them now."
echo "    Repeating command blocks survive server reloads and flood chat for all players."
echo "    See docs/smoke-testing.md -> Post-session cleanup for details."

[[ $ERRORS -eq 0 ]]
