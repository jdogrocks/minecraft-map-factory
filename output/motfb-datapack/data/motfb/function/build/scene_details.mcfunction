# =============================================================================
# SCENE DESIGN: Store Entry Details + Corridor Polish (MIN-163)
# §1.0 — "End rods on dark oak brackets at each store entry transition"
# §1.0 — Structural columns: smooth quartz pillar + iron bars bracket
# §1.0 — Wall: smooth stone lower baseboard + light gray concrete upper stripe
#
# Store entry z-boundaries (west side x=-7, east side x=7):
#   Hot-Topical W:      z=-186 and z=-200
#   Build-A-Boss W:     z=-201 and z=-215
#   Cinnabog W:         z=-216 and z=-230
#   GameStomp W:        z=-231 and z=-245
#   Cluck-O-Mart W:     z=-246 and z=-260
#   Spencer's E:        z=-246 and z=-260
#   Bath & Body E:      z=-231 and z=-245
#   Pretzel E:          z=-216 and z=-230
#   Spunky's E:         z=-201 and z=-215
# SEARZ entry (full width): z=-261
# =============================================================================

# =============================================================================
# END RODS AT STORE ENTRIES — dark oak bracket + end rod facing outward
# Placed at y=76 (top of entry arch, visible from corridor)
# West side (x=-7): end rod facing west  East side (x=7): end rod facing east
# =============================================================================

# West side store entries
setblock -6 76 -186 minecraft:dark_oak_planks
setblock -7 76 -186 minecraft:end_rod[facing=west]
setblock -6 76 -201 minecraft:dark_oak_planks
setblock -7 76 -201 minecraft:end_rod[facing=west]
setblock -6 76 -216 minecraft:dark_oak_planks
setblock -7 76 -216 minecraft:end_rod[facing=west]
setblock -6 76 -231 minecraft:dark_oak_planks
setblock -7 76 -231 minecraft:end_rod[facing=west]
setblock -6 76 -246 minecraft:dark_oak_planks
setblock -7 76 -246 minecraft:end_rod[facing=west]
setblock -6 76 -260 minecraft:dark_oak_planks
setblock -7 76 -260 minecraft:end_rod[facing=west]
setblock -6 76 -200 minecraft:dark_oak_planks
setblock -7 76 -200 minecraft:end_rod[facing=west]
setblock -6 76 -215 minecraft:dark_oak_planks
setblock -7 76 -215 minecraft:end_rod[facing=west]
setblock -6 76 -230 minecraft:dark_oak_planks
setblock -7 76 -230 minecraft:end_rod[facing=west]
setblock -6 76 -245 minecraft:dark_oak_planks
setblock -7 76 -245 minecraft:end_rod[facing=west]

# East side store entries
setblock 6 76 -186 minecraft:dark_oak_planks
setblock 7 76 -186 minecraft:end_rod[facing=east]
setblock 6 76 -201 minecraft:dark_oak_planks
setblock 7 76 -201 minecraft:end_rod[facing=east]
setblock 6 76 -216 minecraft:dark_oak_planks
setblock 7 76 -216 minecraft:end_rod[facing=east]
setblock 6 76 -231 minecraft:dark_oak_planks
setblock 7 76 -231 minecraft:end_rod[facing=east]
setblock 6 76 -246 minecraft:dark_oak_planks
setblock 7 76 -246 minecraft:end_rod[facing=east]
setblock 6 76 -260 minecraft:dark_oak_planks
setblock 7 76 -260 minecraft:end_rod[facing=east]
setblock 6 76 -200 minecraft:dark_oak_planks
setblock 7 76 -200 minecraft:end_rod[facing=east]
setblock 6 76 -215 minecraft:dark_oak_planks
setblock 7 76 -215 minecraft:end_rod[facing=east]
setblock 6 76 -230 minecraft:dark_oak_planks
setblock 7 76 -230 minecraft:end_rod[facing=east]
setblock 6 76 -245 minecraft:dark_oak_planks
setblock 7 76 -245 minecraft:end_rod[facing=east]

# SEARZ grand entry (full width, z=-261)
setblock -6 76 -261 minecraft:dark_oak_planks
setblock -7 76 -261 minecraft:end_rod[facing=west]
setblock 6 76 -261 minecraft:dark_oak_planks
setblock 7 76 -261 minecraft:end_rod[facing=east]

# =============================================================================
# STRUCTURAL COLUMNS — smooth quartz pillar at corridor widths
# Placed at x=±5, y=65..71 (half-height decorative columns against the walls)
# at the midpoint z of each store bay (between entry z boundaries)
# =============================================================================

# Hot-Topical midpoint z=-193 — but Hot-Topical has purple terracotta walls; skip
# (hot-topical already has a distinct look; columns would clash)

# Build-A-Boss corridor midpoint z=-208
setblock -5 65 -208 minecraft:quartz_pillar[axis=y]
setblock -5 66 -208 minecraft:quartz_pillar[axis=y]
setblock -5 67 -208 minecraft:quartz_pillar[axis=y]
setblock -5 68 -208 minecraft:quartz_pillar[axis=y]
setblock -5 69 -208 minecraft:iron_bars
setblock 5 65 -208 minecraft:quartz_pillar[axis=y]
setblock 5 66 -208 minecraft:quartz_pillar[axis=y]
setblock 5 67 -208 minecraft:quartz_pillar[axis=y]
setblock 5 68 -208 minecraft:quartz_pillar[axis=y]
setblock 5 69 -208 minecraft:iron_bars

# Cinnabog midpoint z=-223
setblock -5 65 -223 minecraft:quartz_pillar[axis=y]
setblock -5 66 -223 minecraft:quartz_pillar[axis=y]
setblock -5 67 -223 minecraft:quartz_pillar[axis=y]
setblock -5 68 -223 minecraft:quartz_pillar[axis=y]
setblock -5 69 -223 minecraft:iron_bars
setblock 5 65 -223 minecraft:quartz_pillar[axis=y]
setblock 5 66 -223 minecraft:quartz_pillar[axis=y]
setblock 5 67 -223 minecraft:quartz_pillar[axis=y]
setblock 5 68 -223 minecraft:quartz_pillar[axis=y]
setblock 5 69 -223 minecraft:iron_bars

# GameStomp / Spencer's midpoint z=-238
setblock -5 65 -238 minecraft:quartz_pillar[axis=y]
setblock -5 66 -238 minecraft:quartz_pillar[axis=y]
setblock -5 67 -238 minecraft:quartz_pillar[axis=y]
setblock -5 68 -238 minecraft:quartz_pillar[axis=y]
setblock -5 69 -238 minecraft:iron_bars
setblock 5 65 -238 minecraft:quartz_pillar[axis=y]
setblock 5 66 -238 minecraft:quartz_pillar[axis=y]
setblock 5 67 -238 minecraft:quartz_pillar[axis=y]
setblock 5 68 -238 minecraft:quartz_pillar[axis=y]
setblock 5 69 -238 minecraft:iron_bars

# Cluck-O-Mart / Bath & Body midpoint z=-253
setblock -5 65 -253 minecraft:quartz_pillar[axis=y]
setblock -5 66 -253 minecraft:quartz_pillar[axis=y]
setblock -5 67 -253 minecraft:quartz_pillar[axis=y]
setblock -5 68 -253 minecraft:quartz_pillar[axis=y]
setblock -5 69 -253 minecraft:iron_bars
setblock 5 65 -253 minecraft:quartz_pillar[axis=y]
setblock 5 66 -253 minecraft:quartz_pillar[axis=y]
setblock 5 67 -253 minecraft:quartz_pillar[axis=y]
setblock 5 68 -253 minecraft:quartz_pillar[axis=y]
setblock 5 69 -253 minecraft:iron_bars

# Food court / lobby column positions (south of store zone)
setblock -5 65 -175 minecraft:quartz_pillar[axis=y]
setblock -5 66 -175 minecraft:quartz_pillar[axis=y]
setblock -5 67 -175 minecraft:quartz_pillar[axis=y]
setblock -5 68 -175 minecraft:quartz_pillar[axis=y]
setblock -5 69 -175 minecraft:iron_bars
setblock 5 65 -175 minecraft:quartz_pillar[axis=y]
setblock 5 66 -175 minecraft:quartz_pillar[axis=y]
setblock 5 67 -175 minecraft:quartz_pillar[axis=y]
setblock 5 68 -175 minecraft:quartz_pillar[axis=y]
setblock 5 69 -175 minecraft:iron_bars

# =============================================================================
# CORRIDOR WALL BASEBOARD — smooth stone lower 2 blocks (y=65-66)
# Only in the central corridor spine (x=-6..6) not overwriting store walls
# Uses replace air so we only add to existing gaps, not overwrite store blocks
# §1.0: "smooth stone (lower 2 blocks, baseboard)"
# =============================================================================

# West corridor wall baseboard (x=-6 face, y=65-66 on the corridor side)
fill -6 65 -185 -6 66 -115 minecraft:smooth_stone replace minecraft:air
# East corridor wall baseboard
fill 6 65 -185 6 66 -115 minecraft:smooth_stone replace minecraft:air

# =============================================================================
# CORRIDOR CEILING UPPER STRIPE — light gray concrete near ceiling
# §1.0: "light gray concrete (upper stripe near ceiling)"
# Add at y=78 (1 block below the troffer ceiling at y=79) on the corridor walls
# =============================================================================

fill -6 78 -279 -6 78 -186 minecraft:light_gray_concrete replace minecraft:air
fill 6 78 -279 6 78 -186 minecraft:light_gray_concrete replace minecraft:air

# =============================================================================
# FOOD COURT ENTRY ARCH — visual transition from lobby to food court
# Marks the food court zone entry at z=-126 with an orange concrete arch
# =============================================================================

setblock -5 65 -126 minecraft:orange_concrete
setblock -5 66 -126 minecraft:orange_concrete
setblock -5 67 -126 minecraft:orange_concrete
setblock -5 68 -126 minecraft:orange_concrete
setblock 5 65 -126 minecraft:orange_concrete
setblock 5 66 -126 minecraft:orange_concrete
setblock 5 67 -126 minecraft:orange_concrete
setblock 5 68 -126 minecraft:orange_concrete
fill -4 70 -126 4 70 -126 minecraft:orange_concrete
fill -3 69 -126 3 69 -126 minecraft:yellow_concrete

tellraw @a {"text":"[MOTFB] Store entry end rods, corridor columns, and wall baseboards added.","color":"green"}
