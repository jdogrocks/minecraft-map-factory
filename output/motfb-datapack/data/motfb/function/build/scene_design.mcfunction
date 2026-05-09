# =============================================================================
# MOTFB Scene Design Pass — Phase D-rev-1b (MIN-163)
# Scene Designer supplemental build on top of MIN-160 Game Dev foundation.
#
# What this adds:
#   scene_dome     — Fountain plaza glass dome atrium (§3.3 Setpiece 1)
#   scene_power    — Spencer's redstone lamps powered + color-shift glass
#   scene_details  — Store entry end rods, corridor columns, wall baseboards
#
# Run AFTER visual_rev1 (MIN-160 foundation must be applied first).
# Usage: function motfb:build/scene_design
# =============================================================================

tellraw @a {"text":"[MOTFB] Applying Scene Designer pass (MIN-163)...","color":"light_purple","italic":true}

function motfb:build/scene_dome
function motfb:build/scene_power
function motfb:build/scene_details

tellraw @a {"text":"[MOTFB] Scene design pass complete. Run /save-all before zipping.","color":"green","bold":true}
