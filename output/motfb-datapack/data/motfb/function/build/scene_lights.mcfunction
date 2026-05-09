# =============================================================================
# SCENE LIGHTS — Store colored light zones (MIN-163 Phase D-rev-1.5)
# §3.2 Colored Light Placement — per-store motivated lighting on top of F2 base.
# Run AFTER f2_lighting.
# =============================================================================

# --- GameStomp: lime arcade-wall glow (§1.4 §3.2) ---
# Sea lanterns embedded in back wall face (x=-48), lime stained glass in front (x=-47)
# Arcade area: z=-232..-244 depth, x=-35..-49
fill -48 65 -244 -48 70 -232 minecraft:sea_lantern replace minecraft:air
fill -47 65 -244 -47 70 -232 minecraft:lime_stained_glass replace minecraft:air
# Glow item frames simulating arcade screen displays (entity — summon, not setblock)
# Facing:5 = east; frames sit on back wall (x=-47 glass) and face into the aisle
summon minecraft:glow_item_frame -46 67 -244 {Facing:5b}
summon minecraft:glow_item_frame -46 67 -240 {Facing:5b}
summon minecraft:glow_item_frame -46 67 -236 {Facing:5b}
summon minecraft:glow_item_frame -46 67 -232 {Facing:5b}

# --- Hot-Topical: arena floor soul lanterns (§1.5) ---
# "soul lanterns at floor level every 6 blocks" — Vampire Queen arena
# No ceiling change; ceiling soul lanterns already placed by f2_lighting
setblock -14 65 -189 minecraft:soul_lantern
setblock -26 65 -189 minecraft:soul_lantern
setblock -38 65 -189 minecraft:soul_lantern
setblock -14 65 -196 minecraft:soul_lantern
setblock -26 65 -196 minecraft:soul_lantern
setblock -38 65 -196 minecraft:soul_lantern

# --- Cinnabog: back-area soul lanterns on ceiling chains (§1.3) ---
# "Soul fire lanterns (teal) tucked in the back display case"
setblock -40 68 -222 minecraft:soul_lantern
setblock -44 68 -225 minecraft:soul_lantern
setblock -36 68 -228 minecraft:soul_lantern

# --- Bath & Bodywork Sanctum: lavender backlit wall panels (§1.7 §3.2) ---
# Sea lanterns behind north interior wall (z=-244), purple glass pane in front (z=-243)
# Uses replace-air so floor geometry at y=64-66 is not disturbed
fill 8 67 -244 48 70 -244 minecraft:sea_lantern replace minecraft:air
fill 8 67 -243 48 70 -243 minecraft:purple_stained_glass_pane replace minecraft:air

# --- Spunky's Sneakers: product-glow display ledge (§1.9 §3.2) ---
# Smooth quartz ledge at y=68 along east display wall (x=47), glowstone warm fill under
# Glowstone used as rough-build stand-in for powered redstone lamps (polish phase wires them)
fill 47 68 -214 47 68 -202 minecraft:smooth_quartz_slab[type=top] replace minecraft:air
fill 47 67 -214 47 67 -202 minecraft:glowstone replace minecraft:air

tellraw @a {"text":"[MOTFB] Scene lights: colored zones applied.","color":"gold"}
