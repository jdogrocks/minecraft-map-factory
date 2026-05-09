# Negative Test Fixtures

These fixtures support behavioral assertion testing in `motfb-smoke.sh`. Each fixture sets up broken world state and verifies that assertions correctly detect and fail.

## Fixture 1: Legacy-Format Sign (Assertion 6)

**Root cause:** MIN-159 sign rendering bug — signs using pre-1.20 NBT format (`Text1`/`Text2`/`Text3`/`Text4`) render raw JSON strings instead of formatted text.

**Setup:** Install a sign at `-1 71 -272` with `Text1` NBT key (legacy format).

**Expected:** `motfb-smoke.sh --behavioral` exits non-zero and reports:
```
✗ Sign at -1 71 -272 uses legacy Text1-4 schema (root cause of MIN-159 sign bug)
```

**Run:** `scripts/motfb-smoke-selftest.sh output/motfb-datapack`

## Fixture 2: Raised Floor Block (Assertion 2)

**Root cause:** MIN-159 entrance floor bug — every third block in the entrance floor is raised, forcing the player to jump.

**Setup:** Install a smooth_quartz block at `0 65 -93` (one block above expected y=64).

**Expected:** `motfb-smoke.sh --behavioral` exits non-zero and reports floor sweep failure:
```
✗ Entrance floor at 0 64 -93: expected=smooth_quartz observed=no-match
```

**Run:** `scripts/motfb-smoke-selftest.sh output/motfb-datapack`

## Adding New Fixtures

1. Identify the assertion that should catch the bug (see `scripts/motfb-smoke.sh` Phase 2).
2. Create fixture setup code in `scripts/motfb-smoke-selftest.sh`.
3. Verify assertion fails with clear error message.
4. Document here with root cause, setup, and expected output.
