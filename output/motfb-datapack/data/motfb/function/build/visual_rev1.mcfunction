# =============================================================================
# MOTFB Phase D-rev-1 — Visual Rework Master Function (MIN-160)
# Run once against the Times_Square__NYC world to apply all four defect fixes:
#   F1: 90s mall aesthetic (floors, décor, Hot-Topical, per-store theming)
#   F2: Interior lighting (sea-lantern stop-gap → period troffer grid)
#   F3: Office ceiling at y=113-115 (roof cap + interior ceiling)
#   F9: Kraw store entrance arch (visual pre-spawn state; DANGER lamps in spawn_kraw)
#
# Usage: function motfb:build/visual_rev1
# Expected runtime: ~2-3 server ticks (many fill commands; all valid in one call)
# =============================================================================

tellraw @a {"text":"[MOTFB] Applying Phase D-rev-1 visual rework...","color":"gold","italic":true}

# F2 first — lighting is the most critical (map was pitch black)
function motfb:build/f2_lighting

# F1 floors — per-store differentiation
function motfb:build/f1_floors

# F1 décor — fountain, food court, escalator, kiosks, neon, Hot-Topical, Kraw arch
function motfb:build/f1_decor

# F3 — office ceiling
function motfb:build/f3_office

# Ensure init settings are clean (difficulty was overridden during diagnosis)
difficulty normal
time set 13000
weather clear 999999

tellraw @a {"text":"[MOTFB] Phase D-rev-1 visual rework complete. /save-all before zipping.","color":"green","bold":true}
