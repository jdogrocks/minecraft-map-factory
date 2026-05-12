# =============================================================================
# F1 — Mall Aesthetic Décor (MIN-160)
# Adds food court section, atrium fountain, escalator suggestion, kiosk islands,
# period décor (potted plants, neon sign frames), Hot-Topical corridor distinction,
# and Kraw/Cluck-O-Mart pre-fight entrance archway (F9 pre-spawn state).
#
# Coordinate reference:
#   Mall exterior: x=-50..50, z=-280..-100, y=59..115
#   South entry/lobby: z=-101..-125
#   Food court zone: z=-126..-149
#   Fountain plaza: z=-150..-175
#   Transition to stores: z=-176..-185
#   Stores (west + east): z=-186..-260
#   SEARZ: z=-261..-280
# =============================================================================

# =============================================================================
# ATRIUM FOUNTAIN (§3.3 Setpiece 1) — central fountain plaza at z=-160..z=-163
# 3×3 water source pool, sea lanterns beneath, polished andesite rim
# =============================================================================

# Sea lanterns on pool floor (light rising through water)
fill -1 63 -163 1 63 -161 minecraft:sea_lantern
# Water source blocks (3×3, 1 deep)
fill -1 64 -163 1 64 -161 minecraft:water
# Polished andesite rim (raised 1 block above floor as pool wall)
fill -3 64 -165 3 64 -165 minecraft:polished_andesite
fill -3 64 -159 3 64 -159 minecraft:polished_andesite
fill -3 64 -165 -3 64 -159 minecraft:polished_andesite
fill 3 64 -165 3 64 -159 minecraft:polished_andesite
fill -3 65 -165 3 65 -165 minecraft:polished_andesite
fill -3 65 -159 3 65 -159 minecraft:polished_andesite
fill -3 65 -165 -3 65 -159 minecraft:polished_andesite
fill 3 65 -165 3 65 -159 minecraft:polished_andesite
# Corner cap: smooth quartz pillars at fountain corners
setblock -3 66 -165 minecraft:quartz_pillar[axis=y]
setblock 3 66 -165 minecraft:quartz_pillar[axis=y]
setblock -3 66 -159 minecraft:quartz_pillar[axis=y]
setblock 3 66 -159 minecraft:quartz_pillar[axis=y]
# Plaza floor around fountain: polished andesite "grout" expanding from rim
fill -5 64 -167 5 64 -157 minecraft:polished_andesite
fill -3 64 -165 3 64 -159 minecraft:smooth_quartz
fill -1 64 -163 1 64 -161 minecraft:water

# =============================================================================
# FOOD COURT ZONE (z=-126..-149): tiled floor + seating area
# Saturated 90s neon tile pattern per owner decision §0.3
# =============================================================================

# Base tile: orange terracotta (food court zone x=-24..24)
fill -24 64 -149 24 64 -126 minecraft:orange_terracotta

# White concrete grout grid (every 4 blocks in z)
fill -24 64 -148 24 64 -148 minecraft:white_concrete
fill -24 64 -144 24 64 -144 minecraft:white_concrete
fill -24 64 -140 24 64 -140 minecraft:white_concrete
fill -24 64 -136 24 64 -136 minecraft:white_concrete
fill -24 64 -132 24 64 -132 minecraft:white_concrete
fill -24 64 -128 24 64 -128 minecraft:white_concrete
# White concrete grout grid (every 4 blocks in x)
fill -24 64 -149 -24 64 -126 minecraft:white_concrete
fill -20 64 -149 -20 64 -126 minecraft:white_concrete
fill -16 64 -149 -16 64 -126 minecraft:white_concrete
fill -12 64 -149 -12 64 -126 minecraft:white_concrete
fill -8 64 -149 -8 64 -126 minecraft:white_concrete
fill -4 64 -149 -4 64 -126 minecraft:white_concrete
fill 0 64 -149 0 64 -126 minecraft:white_concrete
fill 4 64 -149 4 64 -126 minecraft:white_concrete
fill 8 64 -149 8 64 -126 minecraft:white_concrete
fill 12 64 -149 12 64 -126 minecraft:white_concrete
fill 16 64 -149 16 64 -126 minecraft:white_concrete
fill 20 64 -149 20 64 -126 minecraft:white_concrete
fill 24 64 -149 24 64 -126 minecraft:white_concrete

# Food court seating area: oak slabs as table surfaces (1 block above floor)
# Table 1 (Lore §2 Tell #5 — Table 8)
setblock -18 65 -140 minecraft:oak_slab[type=top]
setblock -16 65 -140 minecraft:oak_slab[type=top]
setblock -18 65 -138 minecraft:oak_slab[type=top]
setblock -16 65 -138 minecraft:oak_slab[type=top]
# Table 2
setblock -18 65 -133 minecraft:oak_slab[type=top]
setblock -16 65 -133 minecraft:oak_slab[type=top]
setblock -18 65 -131 minecraft:oak_slab[type=top]
setblock -16 65 -131 minecraft:oak_slab[type=top]
# Table 3 (east side)
setblock 16 65 -140 minecraft:oak_slab[type=top]
setblock 18 65 -140 minecraft:oak_slab[type=top]
setblock 16 65 -138 minecraft:oak_slab[type=top]
setblock 18 65 -138 minecraft:oak_slab[type=top]
# Table 4
setblock 16 65 -133 minecraft:oak_slab[type=top]
setblock 18 65 -133 minecraft:oak_slab[type=top]
setblock 16 65 -131 minecraft:oak_slab[type=top]
setblock 18 65 -131 minecraft:oak_slab[type=top]

# Food court lighting: sea lanterns over tables at y=79 already covered by F2,
# but add iron lanterns on chain posts in the food court center (warm amber)
setblock -10 72 -138 minecraft:iron_bars
setblock -10 71 -138 minecraft:iron_bars
setblock -10 70 -138 minecraft:lantern[hanging=true]
setblock 10 72 -138 minecraft:iron_bars
setblock 10 71 -138 minecraft:iron_bars
setblock 10 70 -138 minecraft:lantern[hanging=true]
setblock -10 72 -132 minecraft:iron_bars
setblock -10 71 -132 minecraft:iron_bars
setblock -10 70 -132 minecraft:lantern[hanging=true]
setblock 10 72 -132 minecraft:iron_bars
setblock 10 71 -132 minecraft:iron_bars
setblock 10 70 -132 minecraft:lantern[hanging=true]

# Cinnabog kiosk overhang in food court (front-of-house counter visible from corridor)
# This is the food court facing side of Cinnabog (west side z=-216..-230 boundary)
# Accent orange concrete fascia sign visible to food court walkers
fill -14 67 -185 -8 69 -185 minecraft:orange_concrete
fill -14 70 -185 -8 70 -185 minecraft:oak_slab[type=top]

# =============================================================================
# ESCALATOR SUGGESTION (§1.0 note) — between food court and north store corridor
# Rough-build: stair+slabs with andesite frame, running east side of corridor
# Goes from floor 1 (y=65) up to floor 2 (y=82) via stair blocks at x=7..10
# =============================================================================

# Andesite frame pillars
fill 7 65 -180 7 80 -180 minecraft:polished_andesite
fill 10 65 -180 10 80 -180 minecraft:polished_andesite
fill 7 65 -165 7 80 -165 minecraft:polished_andesite
fill 10 65 -165 10 80 -165 minecraft:polished_andesite
# Escalator stair treads (ascending north to south, each tread raises 1 y per z)
setblock 8 65 -179 minecraft:polished_andesite_stairs[facing=south,half=bottom]
setblock 8 66 -178 minecraft:polished_andesite_stairs[facing=south,half=bottom]
setblock 8 67 -177 minecraft:polished_andesite_stairs[facing=south,half=bottom]
setblock 8 68 -176 minecraft:polished_andesite_stairs[facing=south,half=bottom]
setblock 8 69 -175 minecraft:polished_andesite_stairs[facing=south,half=bottom]
setblock 8 70 -174 minecraft:polished_andesite_stairs[facing=south,half=bottom]
setblock 8 71 -173 minecraft:polished_andesite_stairs[facing=south,half=bottom]
setblock 8 72 -172 minecraft:polished_andesite_stairs[facing=south,half=bottom]
setblock 8 73 -171 minecraft:polished_andesite_stairs[facing=south,half=bottom]
setblock 8 74 -170 minecraft:polished_andesite_stairs[facing=south,half=bottom]
setblock 8 75 -169 minecraft:polished_andesite_stairs[facing=south,half=bottom]
setblock 8 76 -168 minecraft:polished_andesite_stairs[facing=south,half=bottom]
setblock 8 77 -167 minecraft:polished_andesite_stairs[facing=south,half=bottom]
setblock 8 78 -166 minecraft:polished_andesite_stairs[facing=south,half=bottom]
setblock 9 65 -179 minecraft:polished_andesite_stairs[facing=south,half=bottom]
setblock 9 66 -178 minecraft:polished_andesite_stairs[facing=south,half=bottom]
setblock 9 67 -177 minecraft:polished_andesite_stairs[facing=south,half=bottom]
setblock 9 68 -176 minecraft:polished_andesite_stairs[facing=south,half=bottom]
setblock 9 69 -175 minecraft:polished_andesite_stairs[facing=south,half=bottom]
setblock 9 70 -174 minecraft:polished_andesite_stairs[facing=south,half=bottom]
setblock 9 71 -173 minecraft:polished_andesite_stairs[facing=south,half=bottom]
setblock 9 72 -172 minecraft:polished_andesite_stairs[facing=south,half=bottom]
setblock 9 73 -171 minecraft:polished_andesite_stairs[facing=south,half=bottom]
setblock 9 74 -170 minecraft:polished_andesite_stairs[facing=south,half=bottom]
setblock 9 75 -169 minecraft:polished_andesite_stairs[facing=south,half=bottom]
setblock 9 76 -168 minecraft:polished_andesite_stairs[facing=south,half=bottom]
setblock 9 77 -167 minecraft:polished_andesite_stairs[facing=south,half=bottom]
setblock 9 78 -166 minecraft:polished_andesite_stairs[facing=south,half=bottom]
# Chain detail on handrail side
fill 7 66 -179 7 79 -166 minecraft:iron_bars
fill 10 66 -179 10 79 -166 minecraft:iron_bars
# Landing platform at top (floor 2 level y=82)
fill 7 82 -166 10 82 -165 minecraft:polished_andesite_slab[type=top]

# =============================================================================
# KIOSK ISLANDS (§1.0 F1 requirement) — concourse kiosk carts every 16 blocks
# Kiosks are 3×2 smooth quartz + item frame "display" surfaces
# =============================================================================

# Kiosk A (z=-185, center corridor)
fill -2 65 -186 2 65 -184 minecraft:smooth_quartz
fill -2 66 -186 2 66 -184 minecraft:smooth_quartz_slab[type=top]
setblock 0 67 -185 minecraft:lantern

# Kiosk B (z=-169)
fill -2 65 -170 2 65 -168 minecraft:smooth_quartz
fill -2 66 -170 2 66 -168 minecraft:smooth_quartz_slab[type=top]
setblock 0 67 -169 minecraft:lantern

# Kiosk C (z=-153)
fill -2 65 -154 2 65 -152 minecraft:smooth_quartz
fill -2 66 -154 2 66 -152 minecraft:smooth_quartz_slab[type=top]
setblock 0 67 -153 minecraft:lantern

# =============================================================================
# PERIOD DÉCOR — potted plants throughout corridor every 12 blocks
# =============================================================================

setblock -5 65 -120 minecraft:flower_pot
setblock 5 65 -120 minecraft:flower_pot
setblock -5 65 -132 minecraft:flower_pot
setblock 5 65 -132 minecraft:flower_pot
setblock -5 65 -144 minecraft:flower_pot
setblock 5 65 -144 minecraft:flower_pot
setblock -5 65 -156 minecraft:flower_pot
setblock 5 65 -156 minecraft:flower_pot
setblock -5 65 -168 minecraft:flower_pot
setblock 5 65 -168 minecraft:flower_pot
setblock -5 65 -180 minecraft:flower_pot
setblock 5 65 -180 minecraft:flower_pot

# Potted plants on column tops (if any exist at the corridor widening points)
setblock -6 65 -230 minecraft:flower_pot
setblock 6 65 -230 minecraft:flower_pot
setblock -6 65 -215 minecraft:flower_pot
setblock 6 65 -215 minecraft:flower_pot

# =============================================================================
# NEON SIGN FRAMES — colored glass + light block (stained glass backlit by
# sea lanterns) mounted above store arch height (y=72-74)
# Each store gets a neon accent band in its palette color
# =============================================================================

# Hot-Topical neon: purple stained glass strip above corridor entrance y=72-73
fill -5 72 -186 5 73 -186 minecraft:purple_stained_glass
fill -5 74 -186 5 74 -186 minecraft:purple_stained_glass_pane
# Back-light sea lanterns at y=75 behind the glass
fill -5 75 -186 5 75 -186 minecraft:sea_lantern

# Cluck-O-Mart neon (west side corridor-facing): red/yellow at x=-7, z=-253
fill -7 72 -260 -7 74 -246 minecraft:red_stained_glass
fill -7 75 -260 -7 75 -246 minecraft:sea_lantern

# Spencer's neon (east side): orange/lime at x=7, z=-253
fill 7 72 -260 7 74 -246 minecraft:orange_stained_glass
fill 7 75 -260 7 75 -246 minecraft:sea_lantern

# GameStomp neon: cyan stained glass (display windows) at x=-7, z=-238
fill -7 68 -245 -7 71 -231 minecraft:cyan_stained_glass
fill -7 72 -245 -7 74 -231 minecraft:dark_oak_planks
setblock -7 75 -238 minecraft:sea_lantern

# Bath & Body neon: lavender stained glass at x=7, z=-238
fill 7 68 -245 7 71 -231 minecraft:purple_stained_glass_pane
fill 7 72 -245 7 74 -231 minecraft:white_concrete
setblock 7 75 -238 minecraft:sea_lantern

# =============================================================================
# HOT-TOPICAL CORRIDOR (§1.5) — distinct eggplant aesthetic zone
# The corridor segment z=-186..-200 (connecting to Hot-Topical) must read
# as a separate aesthetic zone. Purple terracotta walls + black ceiling.
# =============================================================================

# Corridor walls (y=65..78) in the Hot-Topical approach zone (x=-5..5)
fill -5 65 -186 -5 78 -200 minecraft:purple_terracotta
fill 5 65 -186 5 78 -200 minecraft:purple_terracotta
fill -5 65 -200 5 65 -200 minecraft:purple_terracotta
# Black concrete ceiling patch over corridor at y=79 (overrides F2 troffer at these z levels)
fill -5 79 -200 5 79 -187 minecraft:black_concrete
# Soul lanterns at y=78 in the Hot-Topical corridor (cold blue-white)
setblock 0 78 -189 minecraft:soul_lantern
setblock 0 78 -194 minecraft:soul_lantern
setblock 0 78 -199 minecraft:soul_lantern
setblock -4 78 -193 minecraft:soul_lantern
setblock 4 78 -193 minecraft:soul_lantern

# Vampire Queen jewelry case (coffin-shaped centerpiece §1.5)
fill -30 65 -193 -28 65 -191 minecraft:dark_oak_slab[type=top]
fill -30 66 -193 -28 66 -191 minecraft:glass_pane
fill -30 64 -193 -28 64 -191 minecraft:purple_glazed_terracotta

# =============================================================================
# F9 PRE-FIGHT: CLUCK-O-MART ENTRANCE ARCHWAY
# Distinctive red/yellow arch frame at x=-7 (corridor side of Kraw door)
# so Jason can see WHERE the entrance is before the bedrock wall seals it.
# The visual state CHANGE (lamps) happens in spawn_kraw.mcfunction.
# =============================================================================

# Arch column pillars (red concrete) at z=-260 and z=-246 corners, y=62..69
fill -7 62 -260 -7 69 -260 minecraft:red_concrete
fill -7 62 -246 -7 69 -246 minecraft:red_concrete
# Arch lintel (yellow concrete) spanning z=-260..-246 at y=70
fill -7 70 -260 -7 70 -246 minecraft:yellow_concrete
# Neon "CLUCK-O-MART" sign band: red/yellow alternating at y=71-72
fill -7 71 -260 -7 72 -252 minecraft:red_concrete
fill -7 71 -251 -7 72 -246 minecraft:yellow_concrete
# Glowstone backlight behind sign (at y=73)
fill -7 73 -260 -7 73 -246 minecraft:glowstone
# Redstone lamp "DANGER" indicators (dormant until spawn_kraw fires)
setblock -7 62 -253 minecraft:redstone_lamp
setblock -7 69 -253 minecraft:redstone_lamp
# Door frame on sides (dark oak for the "opening" feel)
fill -7 62 -259 -7 69 -247 minecraft:dark_oak_planks
fill -8 62 -260 -8 69 -246 minecraft:dark_oak_planks

# Build-A-Boss entrance archway (west side, x=-7, z=-201..-215): pastel pink frame
fill -7 62 -215 -7 69 -215 minecraft:pink_concrete
fill -7 62 -201 -7 69 -201 minecraft:pink_concrete
fill -7 70 -215 -7 70 -201 minecraft:lime_concrete
fill -7 71 -214 -7 71 -202 minecraft:light_blue_concrete

# GameStomp entrance archway (x=-7, z=-231..-245): dark oak frame
fill -7 62 -245 -7 69 -245 minecraft:dark_oak_planks
fill -7 62 -231 -7 69 -231 minecraft:dark_oak_planks
fill -7 70 -245 -7 70 -231 minecraft:blackstone_slab[type=top]

# Cinnabog entrance archway (x=-7, z=-216..-230): warm sandstone frame
fill -7 62 -230 -7 69 -230 minecraft:smooth_sandstone
fill -7 62 -216 -7 69 -216 minecraft:smooth_sandstone
fill -7 70 -230 -7 70 -216 minecraft:orange_concrete

# East side store archways (x=7)
# Spencer's entrance
fill 7 62 -260 7 69 -260 minecraft:orange_concrete
fill 7 62 -246 7 69 -246 minecraft:orange_concrete
fill 7 70 -260 7 70 -246 minecraft:lime_concrete
# Bath & Body entrance
fill 7 62 -245 7 69 -245 minecraft:smooth_quartz
fill 7 62 -231 7 69 -231 minecraft:smooth_quartz
fill 7 70 -245 7 70 -231 minecraft:white_concrete
# Knot God / Pretzel entrance
fill 7 62 -230 7 69 -230 minecraft:smooth_sandstone
fill 7 62 -216 7 69 -216 minecraft:smooth_sandstone
fill 7 70 -230 7 70 -216 minecraft:yellow_concrete
# Spunky's Sneakers entrance
fill 7 62 -215 7 69 -215 minecraft:white_concrete
fill 7 62 -201 7 69 -201 minecraft:white_concrete
fill 7 70 -215 7 70 -201 minecraft:orange_concrete
