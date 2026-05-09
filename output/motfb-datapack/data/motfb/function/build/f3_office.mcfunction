# =============================================================================
# F3 — Office Floor Ceiling Fix (MIN-160)
# Jason's walkthrough found that the office roof cap (claimed in commit as
# "Roof y=113-115: flat quartz cap") was not at the expected coordinates or
# was absent. This function fills the cap and verifies internal ceiling.
#
# Office footprint derived from reset.mcfunction backup clone:
#   x=-12..12, z=-220..-200
#   Floor blocks at y=97 (walkable y=98)
#   Interior space to y=111
#   Roof cap: y=112-115 (smooth quartz flat top, matching mall exterior)
#
# Using fill ... replace air so we don't overwrite existing solid geometry.
# =============================================================================

# --- Roof cap: solid smooth quartz over the office footprint ---
fill -12 113 -220 12 113 -200 minecraft:smooth_quartz
fill -12 114 -220 12 114 -200 minecraft:smooth_quartz
fill -12 115 -220 12 115 -200 minecraft:smooth_quartz

# --- Interior ceiling at y=111 (top of office space): white concrete ---
fill -12 111 -220 12 111 -200 minecraft:white_concrete

# --- Sea lanterns recessed into office ceiling per §1.11 (Phase 1 state) ---
# Sea lanterns behind smooth stone slab grid at y=111
# These are the Phase 1 "surgically bright" lights — every 2 blocks
fill -12 111 -220 12 111 -200 minecraft:smooth_stone_slab[type=top] replace minecraft:white_concrete
fill -10 111 -218 10 111 -202 minecraft:sea_lantern replace minecraft:smooth_stone_slab
# Restore the outer slab ring (border stays slab, inner area alternates)
fill -12 111 -220 12 111 -220 minecraft:smooth_stone_slab[type=top] replace minecraft:sea_lantern
fill -12 111 -200 12 111 -200 minecraft:smooth_stone_slab[type=top] replace minecraft:sea_lantern
fill -12 111 -220 -12 111 -200 minecraft:smooth_stone_slab[type=top] replace minecraft:sea_lantern
fill 12 111 -220 12 111 -200 minecraft:smooth_stone_slab[type=top] replace minecraft:sea_lantern

# --- Office floor (y=97): light gray carpet per §1.11 ---
fill -11 97 -219 11 97 -201 minecraft:light_gray_carpet

# --- Office walls: smooth quartz with light gray concrete and dark oak trim ---
# Already built in rough pass; just ensure the exterior-facing roof cap is solid
# north wall cap above office
fill -12 112 -220 12 112 -220 minecraft:smooth_quartz
# south wall cap
fill -12 112 -200 12 112 -200 minecraft:smooth_quartz
# east wall cap
fill 12 112 -220 12 112 -200 minecraft:smooth_quartz
# west wall cap
fill -12 112 -220 -12 112 -200 minecraft:smooth_quartz

# --- Side walls of office: dark oak trim at ceiling level per §1.11 ---
fill -11 110 -219 11 110 -219 minecraft:dark_oak_planks replace minecraft:air
fill -11 110 -201 11 110 -201 minecraft:dark_oak_planks replace minecraft:air
fill -11 110 -219 -11 110 -201 minecraft:dark_oak_planks replace minecraft:air
fill 11 110 -219 11 110 -201 minecraft:dark_oak_planks replace minecraft:air

# --- Signing Lectern raised platform (§6.5) ---
# 1-block smooth quartz pad, lectern facing south (toward door)
setblock 0 98 -213 minecraft:smooth_quartz
setblock 0 99 -213 minecraft:lectern[facing=south]
setblock 0 99 -214 minecraft:end_rod[facing=north]

# --- Tearing Pad (§6.6): redstone block + pressure plate ---
setblock 0 98 -207 minecraft:redstone_block
setblock 0 99 -207 minecraft:oak_pressure_plate
# Magenta stained glass ring in floor around pad
setblock 1 98 -207 minecraft:magenta_stained_glass
setblock -1 98 -207 minecraft:magenta_stained_glass
setblock 0 98 -208 minecraft:magenta_stained_glass
setblock 0 98 -206 minecraft:magenta_stained_glass

# --- Gold stained glass ring around Signing Lectern ---
setblock 1 98 -213 minecraft:gold_block
setblock -1 98 -213 minecraft:gold_block
setblock 0 98 -214 minecraft:gold_block
setblock 0 98 -212 minecraft:gold_block

# --- Polished diorite behind desk area (§1.11) ---
fill -3 97 -212 3 97 -207 minecraft:polished_diorite
fill -11 97 -219 -4 97 -201 minecraft:polished_diorite

# --- Desk: dark oak planks + chiseled bookshelf (§1.11) ---
fill -2 98 -211 2 100 -209 minecraft:dark_oak_planks replace minecraft:air
setblock -2 100 -210 minecraft:chiseled_bookshelf

# --- Redstone lamp on desk (always-powered, Phase 2 sole light source §3.3) ---
setblock 0 101 -210 minecraft:redstone_lamp
setblock 0 101 -211 minecraft:redstone_block
