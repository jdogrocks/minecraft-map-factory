#!/usr/bin/env bash
# Negative test fixtures for motfb-smoke.sh behavioral assertions.
#
# This script sets up fixture state (legacy-format signs, raised floor blocks)
# and verifies that the smoke test assertions correctly detect and fail on them.
#
# Usage: scripts/motfb-smoke-selftest.sh <datapack-path>

set -euo pipefail

CONTAINER="minecraft-papermc"
WORLD="Times_Square__NYC"
DATAPACKS_PATH="/data/$WORLD/datapacks"

if [[ $# -lt 1 ]]; then
    echo "Usage: $0 <datapack-path>" >&2
    exit 1
fi

PACK_PATH="$1"

# Verify the container is running
if ! docker inspect --format='{{.State.Running}}' "$CONTAINER" 2>/dev/null | grep -q "true"; then
    echo "Error: container '$CONTAINER' is not running" >&2
    exit 1
fi

echo "==> Negative Test Fixture: Legacy-Format Sign"
echo ""
echo "Installing legacy-format sign at -1 71 -272..."
# Install a sign with pre-1.20 NBT format (Text1, Text2, etc.)
docker exec "$CONTAINER" rcon-cli "setblock -1 71 -272 minecraft:oak_wall_sign[facing=east]" >/dev/null
docker exec "$CONTAINER" rcon-cli "data merge block -1 71 -272 {Text1:'{\"text\":\"Legacy\"}'}" >/dev/null

echo "Running motfb-smoke.sh with --behavioral..."
if scripts/motfb-smoke.sh --behavioral "$PACK_PATH" motfb:init 2>&1 | tee /tmp/smoke_output.log; then
    echo ""
    echo "ERROR: smoke test should have FAILED on legacy-format sign, but passed" >&2
    exit 1
else
    echo ""
    if grep -q "legacy Text1-4 schema" /tmp/smoke_output.log; then
        echo "✓ PASS: smoke test correctly detected legacy-format sign and failed"
    else
        echo "ERROR: smoke test failed, but did not detect legacy-format sign" >&2
        exit 1
    fi
fi

echo ""
echo "==> Negative Test Fixture: Missing Floor Block"
echo ""
echo "Removing floor block at entrance floor (0 64 -93) so assertion 2 fails..."
docker exec "$CONTAINER" rcon-cli "setblock 0 64 -93 minecraft:air" >/dev/null

echo "Running motfb-smoke.sh with --behavioral..."
if scripts/motfb-smoke.sh --behavioral "$PACK_PATH" motfb:init 2>&1 | tee /tmp/smoke_output.log; then
    echo ""
    echo "ERROR: smoke test should have FAILED on missing floor block, but passed" >&2
    exit 1
else
    echo ""
    if grep -q "Floor flatness sweep" /tmp/smoke_output.log && grep -q "✗" /tmp/smoke_output.log; then
        echo "✓ PASS: smoke test correctly detected missing floor block and failed"
    else
        echo "ERROR: smoke test failed, but did not clearly report floor mismatch" >&2
        exit 1
    fi
fi

echo ""
echo "==> Cleanup: restoring fixture blocks"
docker exec "$CONTAINER" rcon-cli "setblock -1 71 -272 minecraft:air" >/dev/null
docker exec "$CONTAINER" rcon-cli "setblock 0 64 -93 minecraft:smooth_quartz" >/dev/null

echo ""
echo "=== Self-Test Summary ==="
echo "  Fixture 1 (legacy sign): PASS"
echo "  Fixture 2 (missing floor block): PASS"
echo "=========================="
