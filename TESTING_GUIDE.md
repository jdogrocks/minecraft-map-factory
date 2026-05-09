# MOTFB Phase D — Testing Guide

**World**: `motfb-phase-d` on `minecraft-papermc` container (192.168.7.211)
**RCON**: `docker exec minecraft-papermc rcon-cli "<cmd>"`
**Difficulty**: Hard (set by `motfb:init`)

---

## Quick-start on a fresh load

After placing the datapack and doing `/reload`:

```
/function motfb:init
```

This sets difficulty, gamerules, scoreboards, spawns all boss entities, Lost Kid, and Bryan. World spawn is now **outside the south entrance** at `0 65 -90` facing north.

---

## Coordinate Reference

| Location | Coords | Notes |
|---|---|---|
| Exterior entrance spawn | `0 65 -90` | Facing north into mall |
| South entry / lobby | `0 65 -110` | PA welcome triggers here |
| Food court (Mall Pretzel) | `0 65 -140` | Auto-give on first entry |
| Fountain plaza | `0 65 -162` | Atrium fountain |
| Arcade (Lost Kid NPC) | `-30 65 -238` | In GameZone, west side |
| Signing Lectern | `0 99 -213` | Floor 3 office — ACCEPT path |
| Tearing Pad (redstone) | `0 99 -207` | Floor 3 office — ANNUL path |
| Bryan (Mall Manager) | `0 102 -212` | Floor 3 office |

### Store bosses (visible on world load)

| Store | Boss | Coords |
|---|---|---|
| Cluck-O-Mart (west far) | Colonel Kraw (Ghast) | `-28 72 -253` |
| SEARZ (west far end) | Mama SEARZ (Wither) | `0 68 -270` |
| GameZone (west) | The Pixel Lich (Husk) | `-28 65 -238` |
| Cinnabog (west) | The Candy Witch (Witch) | `-28 65 -223` |
| Build-A-Boss (west) | The Stitch Lord (Vindicator) | `-28 65 -208` |
| Hot-Topical (west) | The Vampire Queen (Wither Skeleton) | `-28 65 -193` |
| Spencer's (east far) | Imp Swarm (Vex ×5) | `22 67 -253` |
| Bath & Body (east) | The Exiled Saint (Evoker) | `28 65 -238` |
| Pretzel-Pretzel Pretzel (east) | Janice, the Knot God (Iron Golem) | `28 65 -223` |
| Spunky's Footwear (east) | The Speed Demon (Vindicator) | `28 65 -208` |

---

## Testing Steps — Pre-Game Setup

1. **Confirm datapack loaded**: `/datapack list` → should show `file/motfb (enabled)`.
2. **Confirm bosses visible**: Walk to `0 65 -220` and look both west and east — should see boss name tags floating in each storefront.
3. **Confirm spawn**: `/kill` yourself → respawn should land at `0 65 -90`.

---

## Testing Walkthrough — Common Path (All Three Endings)

### Step 1 — Enter mall / PA Welcome

Walk north from spawn into the south lobby (`z ≈ -110`). Within ~2 seconds:

- PA bell sound plays.
- Chat: `[PA] Welcome, welcome! Welcome to Liminal Lakes Mall...`

If not triggering: verify you're within `x=-20..20, y=60..80, z=-125..-101`. Check logs for parse errors.

### Step 2 — Food Court / Get Mall Pretzel

Walk into the food court zone (`z ≈ -140`). On first entry:

- You receive 1 **Mall Pretzel** (yellow bread).
- This item recruits the Lost Kid.

### Step 3 — Recruit the Lost Kid

Walk to GameZone on the west side (`x=-7` corridor opening at `z=-238`). Look for the glowing sign on the corridor wall. Enter the arcade. Approach **The Lost Kid** (`-30 65 -238`):

- Get within 3 blocks.
- The recruit check fires automatically when you have the Mall Pretzel in inventory.
- Chat: `The Lost Kid: "Sick. You're a real one. Lead the way, lowkey."`
- Lost Kid begins following you.

**Skip/cheat** (if needed): `/tag @s add lk_following`

### Step 4 — Defeat all 9 store bosses

Walk into any store corridor opening (west side `x=-7`, east side `x=7`). The boss fight triggers:

- Bedrock seals the entrance.
- Boss intro title appears.
- Boss is now hostile and has AI.

Fight and kill the boss. On death:
- Coupon awarded: `+1 Coupon` title.
- Bedrock seal removed.
- PA announcement plays.

Repeat for all 9 storefronts. Coupon count shows on HUD sidebar.

**Skip/cheat** (testing only):
```
/scoreboard players set #party mall.coupons 9
/function motfb:contract/unlock_office
```

### Step 5 — Unlock Floor 3 / Meet Bryan

After the 9th coupon:
- PA: `Mall Office, third floor. The handle is warm...`
- Escalator gate (bedrock at `z=-228..230`) opens.
- Follow the glowing lamps up to floor 3 (`y=98`).
- Approach Bryan at `0 102 -212` (within 4 blocks):
  - **The Original Contract** item given to inventory.
  - Contract lore explains all three endings.

---

## Ending A — HONORED (Sign the Contract)

**Trigger**: Right-click the carrot-on-stick (Original Contract) while standing *on* the **Signing Lectern** at `0 99 -213`.

**Steps**:
1. With the contract in hand, walk to `0 99 -213` (north side of the office, gold-block ring).
2. **Stand on top of the lectern block** (y=99, one step up from the carpet).
3. Right-click.

**Expected**:
- Title: `ENDING A — HONORED`
- Subtitle: `"Welcome aboard, sport. The mall thanks you."`
- Beacon activation sound.
- Credits sequence after ~10 seconds.

**Verify in logs**: No `Unknown command` or `Expected ...` errors.

---

## Ending B — THE VOID CONTRACT (Attack Bryan)

**Trigger**: After receiving the contract, attack Bryan with any weapon.

**Steps**:
1. Get the contract (Step 5 above).
2. Bryan is now vulnerable (not invulnerable) once you have the contract.
3. Hit Bryan with your sword / axe / fist.

**Expected**:
- Tick.mcfunction detects `mall.bryan_hp < 99` with `has_contract` player → calls `contract/attack`.
- `contract/attack` sets `mall.ending = 2` → calls `ending/b_voided`.
- Title: `ENDING B — THE VOID CONTRACT`
- Subtitle: `"This is really disappointing, sport."`
- Bryan becomes active and hostile (phase 1 fight).
- Fight Bryan to 0 HP for the finalize sequence.

**Cheat to skip to fight**: 
```
/scoreboard players set #party mall.ending 0
/function motfb:ending/b_voided
```

---

## Ending C — THE CONTRACT IS TORN (Annul with Shears)

**Prerequisites**: 3 journals collected AND hold Shears in inventory.

### Collect the 3 Journals

Walk near each journal lectern (glowing pillar with sign):

| Journal | Location | Coords |
|---|---|---|
| Journal 1 | Food court (oak pillar) | `0 67 -134` |
| Journal 2 | Inside SEARZ store (after defeating Mama SEARZ) | `0 71 -272` |
| Journal 3 | Office antechamber (floor 3, west side) | `-5 99 -218` |

Collection is automatic on proximity (within ~5 blocks). `mall.journals` scoreboard tracks count.

**Check**: `/scoreboard players get #party mall.journals` should read `3`.

### Get Shears

Craft shears (2 iron ingots) or use:
```
/give @s minecraft:shears 1
```

### Trigger the Ending

1. Hold shears in hand.
2. Walk to the **Tearing Pad** at `0 98 -207` (magenta glass ring, south side of office).
3. **Stand on the redstone_block** (y=98, below the pressure plate).
4. Right-click (use) the shears.

> Note: The game uses `minecraft.used:shears` stat. You must USE the shears (right-click), not just hold them.

**Expected**:
- Check: requires 3+ journals (otherwise PA: `Now sport, that's not how we do things...`).
- Contract item breaks (particle effect).
- Bryan freezes.
- Title: `ENDING C — THE CONTRACT IS TORN`
- Multi-step ending sequence plays over ~20 seconds.
- Credits roll.

**Cheat to skip prerequisite**:
```
/scoreboard players set #party mall.journals 3
/tag @s add journal1_found
/tag @s add journal2_found
/tag @s add journal3_found
```

---

## Reset Between Runs

```
/function motfb:reset
```

This kills all bosses and NPCs, clears inventory items, removes all player tags, restores store arenas, re-spawns bosses/Lost Kid/Bryan, and teleports players back to the exterior entrance.

---

## Checklist for Jason's Thumbs-Up

- [ ] Spawn at exterior entrance (z=-90, facing north)
- [ ] Walk into lobby → PA welcome plays
- [ ] Enter food court → Mall Pretzel given automatically
- [ ] Find sign near GameZone → Lost Kid visible, walk up → recruited by pretzel
- [ ] At least one storefront boss fight triggers (arena seals, boss intro, boss is hostile)
- [ ] 9 boss kills → office unlocks → contract given near Bryan
- [ ] Ending A: stand on lectern → right-click contract → HONORED title
- [ ] Ending B: attack Bryan after contract → VOID CONTRACT title → Bryan fights
- [ ] Ending C: collect 3 journals + shears → stand on redstone pad → right-click → CONTRACT TORN title
- [ ] `/function motfb:reset` cleans everything and restores correctly

---

## Known Phase D Limitations

- Boss entity AI starts in display mode (NoAI) on world load; full fight AI activates when a player enters the storefront.
- Wither (Mama SEARZ) summoned with `Silent:1b` to suppress spawn explosion sound on init; the full spawn sound plays when the fight triggers via `spawn_searz`.
- Imp Swarm shows only 2 representative entities at display time; full 5-vex swarm spawns when the fight triggers.
- Sky render (animated panels vs `sky.json`) not verified — separate issue per MIN-155.
- Resource pack must be applied separately; see `docs/smoke-testing.md`.
