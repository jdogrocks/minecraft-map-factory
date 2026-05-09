# =============================================================================
# MOTFB build pipeline — constructs all physical mall structure and decor.
# Called automatically from motfb:init. Safe to re-run (all fills are idempotent).
#
# Pipeline order:
#   1. visual_rev1   — lighting base, per-store floors, food-court decor, office
#   2. scene_design  — atrium glass dome, Spencer's powered lamps, store entries
#   3. scene_lights  — colored accent lighting per store zone
#   4. entrance      — exterior south approach and guaranteed spawn platform
# =============================================================================

tellraw @a {"text":"[MOTFB] Constructing mall structure (moment)...","color":"dark_gray","italic":true}

# Phase 1: lighting foundation + floors + major decor + office ceiling
function motfb:build/visual_rev1

# Phase 2: scene designer — dome atrium, powered lamps, column details
function motfb:build/scene_design

# Phase 3: colored store accent lighting on top of F2 base
function motfb:build/scene_lights

# Phase 4: exterior entrance approach and spawn platform floor
function motfb:build/entrance

tellraw @a {"text":"[MOTFB] Mall ready. Spawn is at the south entrance (z=-90).","color":"green","bold":true}
