# =============================================================================
# F1 — SPRAWL EXPANSION (MIN-207)
# Transforms the mall from a 98×178-block hallway into a 90s super-regional
# footprint: 40-block promenade, 172-block total width, 320-block spine,
# plus two lateral wings creating a "+" silhouette from overhead.
#
# Reference: Houston Galleria / Mall of America layout typology.
# Additive pass — runs AFTER all existing build functions. Expands outward
# without destroying mechanics-critical existing interiors (SEARZ z=-261..-279,
# fountain z=-150..-175, food court z=-126..-149 all untouched).
#
# New footprint:
#   Promenade:      x=-20..20 (40 blocks wide, was 13)
#   E-W main shell: x=-86..86 (172 blocks, was 100)
#   N-S spine:      z=-100..-420 (320 blocks, was 180)
#   Lateral wings:  z=-195..-245 extending x=-150..-87 and x=87..150
#   SEARZ anchor:   existing z=-261..-279 + north extension z=-340..-420
#
# Fill-size note: all fills kept ≤32,768 blocks per command.
#   Wide+tall zones split by y-level or z-chunk as noted.
# =============================================================================

tellraw @a {"text":"[MOTFB] Applying sprawl expansion (MIN-207)...","color":"aqua","italic":true}

# =============================================================================
# STEP A — WIDEN THE PROMENADE (x=-20..-7 and x=7..20, z=-100..-280)
#
# Clears existing store-front fills in the new promenade footprint, lays
# smooth quartz floor, and caps ceiling with slab + sea-lantern troffers
# matching the existing pattern from f2_lighting.
#
# Each strip: 14 wide × 14 tall × 181 long = 35,476 → split at z=-190.
# =============================================================================

# --- Clear existing store fronts from the wider promenade zone ---
fill -20 65 -280 -7 78 -190 minecraft:air
fill -20 65 -190 -7 78 -100 minecraft:air
fill 7 65 -280 20 78 -190 minecraft:air
fill 7 65 -190 20 78 -100 minecraft:air

# --- Promenade floor: smooth quartz across new strips ---
fill -20 64 -280 -7 64 -100 minecraft:smooth_quartz
fill 7 64 -280 20 64 -100 minecraft:smooth_quartz

# --- Polished andesite grout lines (every 4z, matching f1_floors pattern) ---
fill -20 64 -279 -7 64 -279 minecraft:polished_andesite
fill 7 64 -279 20 64 -279 minecraft:polished_andesite
fill -20 64 -275 -7 64 -275 minecraft:polished_andesite
fill 7 64 -275 20 64 -275 minecraft:polished_andesite
fill -20 64 -271 -7 64 -271 minecraft:polished_andesite
fill 7 64 -271 20 64 -271 minecraft:polished_andesite
fill -20 64 -267 -7 64 -267 minecraft:polished_andesite
fill 7 64 -267 20 64 -267 minecraft:polished_andesite
fill -20 64 -263 -7 64 -263 minecraft:polished_andesite
fill 7 64 -263 20 64 -263 minecraft:polished_andesite
fill -20 64 -259 -7 64 -259 minecraft:polished_andesite
fill 7 64 -259 20 64 -259 minecraft:polished_andesite
fill -20 64 -255 -7 64 -255 minecraft:polished_andesite
fill 7 64 -255 20 64 -255 minecraft:polished_andesite
fill -20 64 -251 -7 64 -251 minecraft:polished_andesite
fill 7 64 -251 20 64 -251 minecraft:polished_andesite
fill -20 64 -247 -7 64 -247 minecraft:polished_andesite
fill 7 64 -247 20 64 -247 minecraft:polished_andesite
fill -20 64 -243 -7 64 -243 minecraft:polished_andesite
fill 7 64 -243 20 64 -243 minecraft:polished_andesite
fill -20 64 -239 -7 64 -239 minecraft:polished_andesite
fill 7 64 -239 20 64 -239 minecraft:polished_andesite
fill -20 64 -235 -7 64 -235 minecraft:polished_andesite
fill 7 64 -235 20 64 -235 minecraft:polished_andesite
fill -20 64 -231 -7 64 -231 minecraft:polished_andesite
fill 7 64 -231 20 64 -231 minecraft:polished_andesite
fill -20 64 -227 -7 64 -227 minecraft:polished_andesite
fill 7 64 -227 20 64 -227 minecraft:polished_andesite
fill -20 64 -223 -7 64 -223 minecraft:polished_andesite
fill 7 64 -223 20 64 -223 minecraft:polished_andesite
fill -20 64 -219 -7 64 -219 minecraft:polished_andesite
fill 7 64 -219 20 64 -219 minecraft:polished_andesite
fill -20 64 -215 -7 64 -215 minecraft:polished_andesite
fill 7 64 -215 20 64 -215 minecraft:polished_andesite
fill -20 64 -211 -7 64 -211 minecraft:polished_andesite
fill 7 64 -211 20 64 -211 minecraft:polished_andesite
fill -20 64 -207 -7 64 -207 minecraft:polished_andesite
fill 7 64 -207 20 64 -207 minecraft:polished_andesite
fill -20 64 -203 -7 64 -203 minecraft:polished_andesite
fill 7 64 -203 20 64 -203 minecraft:polished_andesite
fill -20 64 -199 -7 64 -199 minecraft:polished_andesite
fill 7 64 -199 20 64 -199 minecraft:polished_andesite
fill -20 64 -195 -7 64 -195 minecraft:polished_andesite
fill 7 64 -195 20 64 -195 minecraft:polished_andesite
fill -20 64 -191 -7 64 -191 minecraft:polished_andesite
fill 7 64 -191 20 64 -191 minecraft:polished_andesite
fill -20 64 -187 -7 64 -187 minecraft:polished_andesite
fill 7 64 -187 20 64 -187 minecraft:polished_andesite
fill -20 64 -183 -7 64 -183 minecraft:polished_andesite
fill 7 64 -183 20 64 -183 minecraft:polished_andesite
fill -20 64 -179 -7 64 -179 minecraft:polished_andesite
fill 7 64 -179 20 64 -179 minecraft:polished_andesite
fill -20 64 -175 -7 64 -175 minecraft:polished_andesite
fill 7 64 -175 20 64 -175 minecraft:polished_andesite
fill -20 64 -171 -7 64 -171 minecraft:polished_andesite
fill 7 64 -171 20 64 -171 minecraft:polished_andesite
fill -20 64 -167 -7 64 -167 minecraft:polished_andesite
fill 7 64 -167 20 64 -167 minecraft:polished_andesite
fill -20 64 -163 -7 64 -163 minecraft:polished_andesite
fill 7 64 -163 20 64 -163 minecraft:polished_andesite
fill -20 64 -159 -7 64 -159 minecraft:polished_andesite
fill 7 64 -159 20 64 -159 minecraft:polished_andesite
fill -20 64 -155 -7 64 -155 minecraft:polished_andesite
fill 7 64 -155 20 64 -155 minecraft:polished_andesite
fill -20 64 -151 -7 64 -151 minecraft:polished_andesite
fill 7 64 -151 20 64 -151 minecraft:polished_andesite
fill -20 64 -147 -7 64 -147 minecraft:polished_andesite
fill 7 64 -147 20 64 -147 minecraft:polished_andesite
fill -20 64 -143 -7 64 -143 minecraft:polished_andesite
fill 7 64 -143 20 64 -143 minecraft:polished_andesite
fill -20 64 -139 -7 64 -139 minecraft:polished_andesite
fill 7 64 -139 20 64 -139 minecraft:polished_andesite
fill -20 64 -135 -7 64 -135 minecraft:polished_andesite
fill 7 64 -135 20 64 -135 minecraft:polished_andesite
fill -20 64 -131 -7 64 -131 minecraft:polished_andesite
fill 7 64 -131 20 64 -131 minecraft:polished_andesite
fill -20 64 -127 -7 64 -127 minecraft:polished_andesite
fill 7 64 -127 20 64 -127 minecraft:polished_andesite
fill -20 64 -123 -7 64 -123 minecraft:polished_andesite
fill 7 64 -123 20 64 -123 minecraft:polished_andesite
fill -20 64 -119 -7 64 -119 minecraft:polished_andesite
fill 7 64 -119 20 64 -119 minecraft:polished_andesite
fill -20 64 -115 -7 64 -115 minecraft:polished_andesite
fill 7 64 -115 20 64 -115 minecraft:polished_andesite
fill -20 64 -111 -7 64 -111 minecraft:polished_andesite
fill 7 64 -111 20 64 -111 minecraft:polished_andesite
fill -20 64 -107 -7 64 -107 minecraft:polished_andesite
fill 7 64 -107 20 64 -107 minecraft:polished_andesite
fill -20 64 -103 -7 64 -103 minecraft:polished_andesite
fill 7 64 -103 20 64 -103 minecraft:polished_andesite

# --- Promenade ceiling: smooth stone slab drop-ceiling on the new strips ---
fill -20 79 -280 -7 79 -100 minecraft:smooth_stone_slab[type=top]
fill 7 79 -280 20 79 -100 minecraft:smooth_stone_slab[type=top]

# --- Sea lantern troffers in new promenade strips (every 4z, matching f2_lighting) ---
fill -20 79 -279 -7 79 -279 minecraft:sea_lantern
fill 7 79 -279 20 79 -279 minecraft:sea_lantern
fill -20 79 -275 -7 79 -275 minecraft:sea_lantern
fill 7 79 -275 20 79 -275 minecraft:sea_lantern
fill -20 79 -271 -7 79 -271 minecraft:sea_lantern
fill 7 79 -271 20 79 -271 minecraft:sea_lantern
fill -20 79 -267 -7 79 -267 minecraft:sea_lantern
fill 7 79 -267 20 79 -267 minecraft:sea_lantern
fill -20 79 -263 -7 79 -263 minecraft:sea_lantern
fill 7 79 -263 20 79 -263 minecraft:sea_lantern
fill -20 79 -259 -7 79 -259 minecraft:sea_lantern
fill 7 79 -259 20 79 -259 minecraft:sea_lantern
fill -20 79 -255 -7 79 -255 minecraft:sea_lantern
fill 7 79 -255 20 79 -255 minecraft:sea_lantern
fill -20 79 -251 -7 79 -251 minecraft:sea_lantern
fill 7 79 -251 20 79 -251 minecraft:sea_lantern
fill -20 79 -247 -7 79 -247 minecraft:sea_lantern
fill 7 79 -247 20 79 -247 minecraft:sea_lantern
fill -20 79 -243 -7 79 -243 minecraft:sea_lantern
fill 7 79 -243 20 79 -243 minecraft:sea_lantern
fill -20 79 -239 -7 79 -239 minecraft:sea_lantern
fill 7 79 -239 20 79 -239 minecraft:sea_lantern
fill -20 79 -235 -7 79 -235 minecraft:sea_lantern
fill 7 79 -235 20 79 -235 minecraft:sea_lantern
fill -20 79 -231 -7 79 -231 minecraft:sea_lantern
fill 7 79 -231 20 79 -231 minecraft:sea_lantern
fill -20 79 -227 -7 79 -227 minecraft:sea_lantern
fill 7 79 -227 20 79 -227 minecraft:sea_lantern
fill -20 79 -223 -7 79 -223 minecraft:sea_lantern
fill 7 79 -223 20 79 -223 minecraft:sea_lantern
fill -20 79 -219 -7 79 -219 minecraft:sea_lantern
fill 7 79 -219 20 79 -219 minecraft:sea_lantern
fill -20 79 -215 -7 79 -215 minecraft:sea_lantern
fill 7 79 -215 20 79 -215 minecraft:sea_lantern
fill -20 79 -211 -7 79 -211 minecraft:sea_lantern
fill 7 79 -211 20 79 -211 minecraft:sea_lantern
fill -20 79 -207 -7 79 -207 minecraft:sea_lantern
fill 7 79 -207 20 79 -207 minecraft:sea_lantern
fill -20 79 -203 -7 79 -203 minecraft:sea_lantern
fill 7 79 -203 20 79 -203 minecraft:sea_lantern
fill -20 79 -199 -7 79 -199 minecraft:sea_lantern
fill 7 79 -199 20 79 -199 minecraft:sea_lantern
fill -20 79 -195 -7 79 -195 minecraft:sea_lantern
fill 7 79 -195 20 79 -195 minecraft:sea_lantern
fill -20 79 -191 -7 79 -191 minecraft:sea_lantern
fill 7 79 -191 20 79 -191 minecraft:sea_lantern
fill -20 79 -187 -7 79 -187 minecraft:sea_lantern
fill 7 79 -187 20 79 -187 minecraft:sea_lantern
fill -20 79 -183 -7 79 -183 minecraft:sea_lantern
fill 7 79 -183 20 79 -183 minecraft:sea_lantern
fill -20 79 -179 -7 79 -179 minecraft:sea_lantern
fill 7 79 -179 20 79 -179 minecraft:sea_lantern
fill -20 79 -175 -7 79 -175 minecraft:sea_lantern
fill 7 79 -175 20 79 -175 minecraft:sea_lantern
fill -20 79 -171 -7 79 -171 minecraft:sea_lantern
fill 7 79 -171 20 79 -171 minecraft:sea_lantern
fill -20 79 -167 -7 79 -167 minecraft:sea_lantern
fill 7 79 -167 20 79 -167 minecraft:sea_lantern
fill -20 79 -163 -7 79 -163 minecraft:sea_lantern
fill 7 79 -163 20 79 -163 minecraft:sea_lantern
fill -20 79 -159 -7 79 -159 minecraft:sea_lantern
fill 7 79 -159 20 79 -159 minecraft:sea_lantern
fill -20 79 -155 -7 79 -155 minecraft:sea_lantern
fill 7 79 -155 20 79 -155 minecraft:sea_lantern
fill -20 79 -151 -7 79 -151 minecraft:sea_lantern
fill 7 79 -151 20 79 -151 minecraft:sea_lantern
fill -20 79 -147 -7 79 -147 minecraft:sea_lantern
fill 7 79 -147 20 79 -147 minecraft:sea_lantern
fill -20 79 -143 -7 79 -143 minecraft:sea_lantern
fill 7 79 -143 20 79 -143 minecraft:sea_lantern
fill -20 79 -139 -7 79 -139 minecraft:sea_lantern
fill 7 79 -139 20 79 -139 minecraft:sea_lantern
fill -20 79 -135 -7 79 -135 minecraft:sea_lantern
fill 7 79 -135 20 79 -135 minecraft:sea_lantern
fill -20 79 -131 -7 79 -131 minecraft:sea_lantern
fill 7 79 -131 20 79 -131 minecraft:sea_lantern
fill -20 79 -127 -7 79 -127 minecraft:sea_lantern
fill 7 79 -127 20 79 -127 minecraft:sea_lantern
fill -20 79 -123 -7 79 -123 minecraft:sea_lantern
fill 7 79 -123 20 79 -123 minecraft:sea_lantern
fill -20 79 -119 -7 79 -119 minecraft:sea_lantern
fill 7 79 -119 20 79 -119 minecraft:sea_lantern
fill -20 79 -115 -7 79 -115 minecraft:sea_lantern
fill 7 79 -115 20 79 -115 minecraft:sea_lantern
fill -20 79 -111 -7 79 -111 minecraft:sea_lantern
fill 7 79 -111 20 79 -111 minecraft:sea_lantern
fill -20 79 -107 -7 79 -107 minecraft:sea_lantern
fill 7 79 -107 20 79 -107 minecraft:sea_lantern
fill -20 79 -103 -7 79 -103 minecraft:sea_lantern
fill 7 79 -103 20 79 -103 minecraft:sea_lantern

# =============================================================================
# STEP B — WEST OUTER SHELL EXPANSION (x=-51..-86, z=-100..-280)
#
# Pushes the western mall wall from x=-50 to x=-86, creating a 36-block-deep
# new store zone per side. Clears NYC terrain, lays floor and ceiling.
# 36 wide × 14 tall × 181 long per z-chunk — split into 4 z-chunks of ~45.
# =============================================================================

# Sub-floor foundation (y=59..63) and floor (y=64): white concrete base
fill -86 59 -280 -51 64 -225 minecraft:white_concrete
fill -86 59 -225 -51 64 -170 minecraft:white_concrete
fill -86 59 -170 -51 64 -115 minecraft:white_concrete
fill -86 59 -115 -51 64 -100 minecraft:white_concrete

# Interior void: clear NYC terrain per y-level (36×1×45 = 1,620 — well within limit)
fill -85 65 -280 -51 78 -235 minecraft:air
fill -85 65 -235 -51 78 -190 minecraft:air
fill -85 65 -190 -51 78 -145 minecraft:air
fill -85 65 -145 -51 78 -100 minecraft:air

# Ceiling: white concrete cap at y=79 with sea lantern troffers every 4z
fill -86 79 -280 -51 79 -100 minecraft:white_concrete
# Troffer strips (every 4z, matching main-hall grid)
fill -86 79 -279 -51 79 -279 minecraft:sea_lantern
fill -86 79 -275 -51 79 -275 minecraft:sea_lantern
fill -86 79 -271 -51 79 -271 minecraft:sea_lantern
fill -86 79 -267 -51 79 -267 minecraft:sea_lantern
fill -86 79 -263 -51 79 -263 minecraft:sea_lantern
fill -86 79 -259 -51 79 -259 minecraft:sea_lantern
fill -86 79 -255 -51 79 -255 minecraft:sea_lantern
fill -86 79 -251 -51 79 -251 minecraft:sea_lantern
fill -86 79 -247 -51 79 -247 minecraft:sea_lantern
fill -86 79 -243 -51 79 -243 minecraft:sea_lantern
fill -86 79 -239 -51 79 -239 minecraft:sea_lantern
fill -86 79 -235 -51 79 -235 minecraft:sea_lantern
fill -86 79 -231 -51 79 -231 minecraft:sea_lantern
fill -86 79 -227 -51 79 -227 minecraft:sea_lantern
fill -86 79 -223 -51 79 -223 minecraft:sea_lantern
fill -86 79 -219 -51 79 -219 minecraft:sea_lantern
fill -86 79 -215 -51 79 -215 minecraft:sea_lantern
fill -86 79 -211 -51 79 -211 minecraft:sea_lantern
fill -86 79 -207 -51 79 -207 minecraft:sea_lantern
fill -86 79 -203 -51 79 -203 minecraft:sea_lantern
fill -86 79 -199 -51 79 -199 minecraft:sea_lantern
fill -86 79 -195 -51 79 -195 minecraft:sea_lantern
fill -86 79 -191 -51 79 -191 minecraft:sea_lantern
fill -86 79 -187 -51 79 -187 minecraft:sea_lantern
fill -86 79 -183 -51 79 -183 minecraft:sea_lantern
fill -86 79 -179 -51 79 -179 minecraft:sea_lantern
fill -86 79 -175 -51 79 -175 minecraft:sea_lantern
fill -86 79 -171 -51 79 -171 minecraft:sea_lantern
fill -86 79 -167 -51 79 -167 minecraft:sea_lantern
fill -86 79 -163 -51 79 -163 minecraft:sea_lantern
fill -86 79 -159 -51 79 -159 minecraft:sea_lantern
fill -86 79 -155 -51 79 -155 minecraft:sea_lantern
fill -86 79 -151 -51 79 -151 minecraft:sea_lantern
fill -86 79 -147 -51 79 -147 minecraft:sea_lantern
fill -86 79 -143 -51 79 -143 minecraft:sea_lantern
fill -86 79 -139 -51 79 -139 minecraft:sea_lantern
fill -86 79 -135 -51 79 -135 minecraft:sea_lantern
fill -86 79 -131 -51 79 -131 minecraft:sea_lantern
fill -86 79 -127 -51 79 -127 minecraft:sea_lantern
fill -86 79 -123 -51 79 -123 minecraft:sea_lantern
fill -86 79 -119 -51 79 -119 minecraft:sea_lantern
fill -86 79 -115 -51 79 -115 minecraft:sea_lantern
fill -86 79 -111 -51 79 -111 minecraft:sea_lantern
fill -86 79 -107 -51 79 -107 minecraft:sea_lantern
fill -86 79 -103 -51 79 -103 minecraft:sea_lantern

# New outer west wall: white concrete, z=-100..-280, y=59..82 (roof parapet height)
fill -86 59 -280 -86 82 -100 minecraft:white_concrete

# =============================================================================
# STEP C — EAST OUTER SHELL EXPANSION (x=51..86, z=-100..-280)
# Mirror of Step B.
# =============================================================================

fill 51 59 -280 86 64 -225 minecraft:white_concrete
fill 51 59 -225 86 64 -170 minecraft:white_concrete
fill 51 59 -170 86 64 -115 minecraft:white_concrete
fill 51 59 -115 86 64 -100 minecraft:white_concrete

fill 51 65 -280 85 78 -235 minecraft:air
fill 51 65 -235 85 78 -190 minecraft:air
fill 51 65 -190 85 78 -145 minecraft:air
fill 51 65 -145 85 78 -100 minecraft:air

fill 51 79 -280 86 79 -100 minecraft:white_concrete
fill 51 79 -279 86 79 -279 minecraft:sea_lantern
fill 51 79 -275 86 79 -275 minecraft:sea_lantern
fill 51 79 -271 86 79 -271 minecraft:sea_lantern
fill 51 79 -267 86 79 -267 minecraft:sea_lantern
fill 51 79 -263 86 79 -263 minecraft:sea_lantern
fill 51 79 -259 86 79 -259 minecraft:sea_lantern
fill 51 79 -255 86 79 -255 minecraft:sea_lantern
fill 51 79 -251 86 79 -251 minecraft:sea_lantern
fill 51 79 -247 86 79 -247 minecraft:sea_lantern
fill 51 79 -243 86 79 -243 minecraft:sea_lantern
fill 51 79 -239 86 79 -239 minecraft:sea_lantern
fill 51 79 -235 86 79 -235 minecraft:sea_lantern
fill 51 79 -231 86 79 -231 minecraft:sea_lantern
fill 51 79 -227 86 79 -227 minecraft:sea_lantern
fill 51 79 -223 86 79 -223 minecraft:sea_lantern
fill 51 79 -219 86 79 -219 minecraft:sea_lantern
fill 51 79 -215 86 79 -215 minecraft:sea_lantern
fill 51 79 -211 86 79 -211 minecraft:sea_lantern
fill 51 79 -207 86 79 -207 minecraft:sea_lantern
fill 51 79 -203 86 79 -203 minecraft:sea_lantern
fill 51 79 -199 86 79 -199 minecraft:sea_lantern
fill 51 79 -195 86 79 -195 minecraft:sea_lantern
fill 51 79 -191 86 79 -191 minecraft:sea_lantern
fill 51 79 -187 86 79 -187 minecraft:sea_lantern
fill 51 79 -183 86 79 -183 minecraft:sea_lantern
fill 51 79 -179 86 79 -179 minecraft:sea_lantern
fill 51 79 -175 86 79 -175 minecraft:sea_lantern
fill 51 79 -171 86 79 -171 minecraft:sea_lantern
fill 51 79 -167 86 79 -167 minecraft:sea_lantern
fill 51 79 -163 86 79 -163 minecraft:sea_lantern
fill 51 79 -159 86 79 -159 minecraft:sea_lantern
fill 51 79 -155 86 79 -155 minecraft:sea_lantern
fill 51 79 -151 86 79 -151 minecraft:sea_lantern
fill 51 79 -147 86 79 -147 minecraft:sea_lantern
fill 51 79 -143 86 79 -143 minecraft:sea_lantern
fill 51 79 -139 86 79 -139 minecraft:sea_lantern
fill 51 79 -135 86 79 -135 minecraft:sea_lantern
fill 51 79 -131 86 79 -131 minecraft:sea_lantern
fill 51 79 -127 86 79 -127 minecraft:sea_lantern
fill 51 79 -123 86 79 -123 minecraft:sea_lantern
fill 51 79 -119 86 79 -119 minecraft:sea_lantern
fill 51 79 -115 86 79 -115 minecraft:sea_lantern
fill 51 79 -111 86 79 -111 minecraft:sea_lantern
fill 51 79 -107 86 79 -107 minecraft:sea_lantern
fill 51 79 -103 86 79 -103 minecraft:sea_lantern

fill 86 59 -280 86 82 -100 minecraft:white_concrete

# =============================================================================
# STEP D — NORTH SPINE EXTENSION (x=-86..86, z=-280..-420)
#
# Extends the mall 140 blocks north — adds ~140 meters / ~450 feet of spine.
# Gives the SEARZ anchor room to breathe and space for future stores north
# of existing SEARZ (z=-261..-279).
# Full width: 173 blocks. Split by y-level for void (173×1×141 = 24,393 ✓).
# =============================================================================

# Foundation + floor at y=59..64
fill -86 59 -420 86 64 -375 minecraft:white_concrete
fill -86 59 -375 86 64 -330 minecraft:white_concrete
fill -86 59 -330 86 64 -280 minecraft:white_concrete

# Promenade floor (x=-20..20): smooth quartz overrides white_concrete base
fill -20 64 -420 20 64 -280 minecraft:smooth_quartz
# Polished andesite grout lines every 4z (continued north from z=-280)
fill -20 64 -283 20 64 -283 minecraft:polished_andesite
fill -20 64 -287 20 64 -287 minecraft:polished_andesite
fill -20 64 -291 20 64 -291 minecraft:polished_andesite
fill -20 64 -295 20 64 -295 minecraft:polished_andesite
fill -20 64 -299 20 64 -299 minecraft:polished_andesite
fill -20 64 -303 20 64 -303 minecraft:polished_andesite
fill -20 64 -307 20 64 -307 minecraft:polished_andesite
fill -20 64 -311 20 64 -311 minecraft:polished_andesite
fill -20 64 -315 20 64 -315 minecraft:polished_andesite
fill -20 64 -319 20 64 -319 minecraft:polished_andesite
fill -20 64 -323 20 64 -323 minecraft:polished_andesite
fill -20 64 -327 20 64 -327 minecraft:polished_andesite
fill -20 64 -331 20 64 -331 minecraft:polished_andesite
fill -20 64 -335 20 64 -335 minecraft:polished_andesite
fill -20 64 -339 20 64 -339 minecraft:polished_andesite
fill -20 64 -343 20 64 -343 minecraft:polished_andesite
fill -20 64 -347 20 64 -347 minecraft:polished_andesite
fill -20 64 -351 20 64 -351 minecraft:polished_andesite
fill -20 64 -355 20 64 -355 minecraft:polished_andesite
fill -20 64 -359 20 64 -359 minecraft:polished_andesite
fill -20 64 -363 20 64 -363 minecraft:polished_andesite
fill -20 64 -367 20 64 -367 minecraft:polished_andesite
fill -20 64 -371 20 64 -371 minecraft:polished_andesite
fill -20 64 -375 20 64 -375 minecraft:polished_andesite
fill -20 64 -379 20 64 -379 minecraft:polished_andesite
fill -20 64 -383 20 64 -383 minecraft:polished_andesite
fill -20 64 -387 20 64 -387 minecraft:polished_andesite
fill -20 64 -391 20 64 -391 minecraft:polished_andesite
fill -20 64 -395 20 64 -395 minecraft:polished_andesite
fill -20 64 -399 20 64 -399 minecraft:polished_andesite
fill -20 64 -403 20 64 -403 minecraft:polished_andesite
fill -20 64 -407 20 64 -407 minecraft:polished_andesite
fill -20 64 -411 20 64 -411 minecraft:polished_andesite
fill -20 64 -415 20 64 -415 minecraft:polished_andesite
fill -20 64 -419 20 64 -419 minecraft:polished_andesite

# Interior void: per y-level to stay within 32,768 limit (173×1×141 = 24,393 each)
fill -86 65 -420 86 65 -280 minecraft:air
fill -86 66 -420 86 66 -280 minecraft:air
fill -86 67 -420 86 67 -280 minecraft:air
fill -86 68 -420 86 68 -280 minecraft:air
fill -86 69 -420 86 69 -280 minecraft:air
fill -86 70 -420 86 70 -280 minecraft:air
fill -86 71 -420 86 71 -280 minecraft:air
fill -86 72 -420 86 72 -280 minecraft:air
fill -86 73 -420 86 73 -280 minecraft:air
fill -86 74 -420 86 74 -280 minecraft:air
fill -86 75 -420 86 75 -280 minecraft:air
fill -86 76 -420 86 76 -280 minecraft:air
fill -86 77 -420 86 77 -280 minecraft:air
fill -86 78 -420 86 78 -280 minecraft:air

# Ceiling: white concrete base at y=79 with sea lantern troffers every 4z
fill -86 79 -420 86 79 -280 minecraft:white_concrete
# Promenade ceiling override: smooth stone slab drop-tile
fill -20 79 -420 20 79 -280 minecraft:smooth_stone_slab[type=top]
# Troffers every 4z in the north extension
fill -86 79 -283 86 79 -283 minecraft:sea_lantern
fill -86 79 -287 86 79 -287 minecraft:sea_lantern
fill -86 79 -291 86 79 -291 minecraft:sea_lantern
fill -86 79 -295 86 79 -295 minecraft:sea_lantern
fill -86 79 -299 86 79 -299 minecraft:sea_lantern
fill -86 79 -303 86 79 -303 minecraft:sea_lantern
fill -86 79 -307 86 79 -307 minecraft:sea_lantern
fill -86 79 -311 86 79 -311 minecraft:sea_lantern
fill -86 79 -315 86 79 -315 minecraft:sea_lantern
fill -86 79 -319 86 79 -319 minecraft:sea_lantern
fill -86 79 -323 86 79 -323 minecraft:sea_lantern
fill -86 79 -327 86 79 -327 minecraft:sea_lantern
fill -86 79 -331 86 79 -331 minecraft:sea_lantern
fill -86 79 -335 86 79 -335 minecraft:sea_lantern
fill -86 79 -339 86 79 -339 minecraft:sea_lantern
fill -86 79 -343 86 79 -343 minecraft:sea_lantern
fill -86 79 -347 86 79 -347 minecraft:sea_lantern
fill -86 79 -351 86 79 -351 minecraft:sea_lantern
fill -86 79 -355 86 79 -355 minecraft:sea_lantern
fill -86 79 -359 86 79 -359 minecraft:sea_lantern
fill -86 79 -363 86 79 -363 minecraft:sea_lantern
fill -86 79 -367 86 79 -367 minecraft:sea_lantern
fill -86 79 -371 86 79 -371 minecraft:sea_lantern
fill -86 79 -375 86 79 -375 minecraft:sea_lantern
fill -86 79 -379 86 79 -379 minecraft:sea_lantern
fill -86 79 -383 86 79 -383 minecraft:sea_lantern
fill -86 79 -387 86 79 -387 minecraft:sea_lantern
fill -86 79 -391 86 79 -391 minecraft:sea_lantern
fill -86 79 -395 86 79 -395 minecraft:sea_lantern
fill -86 79 -399 86 79 -399 minecraft:sea_lantern
fill -86 79 -403 86 79 -403 minecraft:sea_lantern
fill -86 79 -407 86 79 -407 minecraft:sea_lantern
fill -86 79 -411 86 79 -411 minecraft:sea_lantern
fill -86 79 -415 86 79 -415 minecraft:sea_lantern
fill -86 79 -419 86 79 -419 minecraft:sea_lantern

# North anchor wall at z=-420 (the far north end cap)
fill -86 59 -420 86 82 -420 minecraft:white_concrete

# West and east outer walls in north extension
fill -86 59 -420 -86 82 -280 minecraft:white_concrete
fill 86 59 -420 86 82 -280 minecraft:white_concrete

# =============================================================================
# STEP E — LATERAL WINGS (the "+" footprint — Galleria-style cross-axis)
#
# At z=-195..-245 (50-block span centered on mid-mall), two lateral wings extend
# beyond the main x=±86 walls to x=±150 — creating east and west store galleries
# visible from the promenade junction. This is the single most impactful scale
# element: the player sees the mall extend in all four cardinal directions.
#
# Wing dimensions: 65 blocks wide (x=87..150), 50 blocks deep (z=-195..-245).
# 65×14×25 = 22,750 per z-chunk ✓
# =============================================================================

# --- West wing ---
# Foundation + floor
fill -150 59 -245 -87 64 -220 minecraft:white_concrete
fill -150 59 -220 -87 64 -195 minecraft:white_concrete

# Void (cleared per z-chunk: 64×14×25 = 22,400 ✓)
fill -149 65 -245 -87 78 -220 minecraft:air
fill -149 65 -220 -87 78 -195 minecraft:air

# Ceiling + troffers every 4z
fill -150 79 -245 -87 79 -195 minecraft:white_concrete
fill -150 79 -243 -87 79 -243 minecraft:sea_lantern
fill -150 79 -239 -87 79 -239 minecraft:sea_lantern
fill -150 79 -235 -87 79 -235 minecraft:sea_lantern
fill -150 79 -231 -87 79 -231 minecraft:sea_lantern
fill -150 79 -227 -87 79 -227 minecraft:sea_lantern
fill -150 79 -223 -87 79 -223 minecraft:sea_lantern
fill -150 79 -219 -87 79 -219 minecraft:sea_lantern
fill -150 79 -215 -87 79 -215 minecraft:sea_lantern
fill -150 79 -211 -87 79 -211 minecraft:sea_lantern
fill -150 79 -207 -87 79 -207 minecraft:sea_lantern
fill -150 79 -203 -87 79 -203 minecraft:sea_lantern
fill -150 79 -199 -87 79 -199 minecraft:sea_lantern

# Wing walls: far west end, north wall, south wall
fill -150 59 -245 -150 82 -195 minecraft:white_concrete
fill -150 59 -245 -87 82 -245 minecraft:white_concrete
fill -150 59 -195 -87 82 -195 minecraft:white_concrete

# Open connection at x=-86 (gap in main west wall to enter the west wing)
fill -86 65 -245 -86 78 -195 minecraft:air

# Wing promenade floor (central east-west walkway inside the wing, z=-213..-227)
fill -149 64 -227 -87 64 -213 minecraft:smooth_quartz

# --- East wing ---
fill 87 59 -245 150 64 -220 minecraft:white_concrete
fill 87 59 -220 150 64 -195 minecraft:white_concrete

fill 87 65 -245 149 78 -220 minecraft:air
fill 87 65 -220 149 78 -195 minecraft:air

fill 87 79 -245 150 79 -195 minecraft:white_concrete
fill 87 79 -243 150 79 -243 minecraft:sea_lantern
fill 87 79 -239 150 79 -239 minecraft:sea_lantern
fill 87 79 -235 150 79 -235 minecraft:sea_lantern
fill 87 79 -231 150 79 -231 minecraft:sea_lantern
fill 87 79 -227 150 79 -227 minecraft:sea_lantern
fill 87 79 -223 150 79 -223 minecraft:sea_lantern
fill 87 79 -219 150 79 -219 minecraft:sea_lantern
fill 87 79 -215 150 79 -215 minecraft:sea_lantern
fill 87 79 -211 150 79 -211 minecraft:sea_lantern
fill 87 79 -207 150 79 -207 minecraft:sea_lantern
fill 87 79 -203 150 79 -203 minecraft:sea_lantern
fill 87 79 -199 150 79 -199 minecraft:sea_lantern

fill 150 59 -245 150 82 -195 minecraft:white_concrete
fill 87 59 -245 150 82 -245 minecraft:white_concrete
fill 87 59 -195 150 82 -195 minecraft:white_concrete

# Open connection at x=86
fill 86 65 -245 86 78 -195 minecraft:air

# Wing promenade floor
fill 87 64 -227 149 64 -213 minecraft:smooth_quartz

# =============================================================================
# STEP F — GRAND PROMENADE COLUMNS (structural scale anchors)
#
# Pairs of smooth quartz pillars every 16z along the promenade edge (x=±20).
# Each pillar: 1×1 base, floor to ceiling (y=64..78). Iron bars decorative
# bracket at capital (y=77). Creates the visual rhythm of a grand arcade.
# =============================================================================

# Column positions: z=-120, -136, -152, -168, -184, -200, -216, -232, -248, -264
setblock -20 64 -120 minecraft:smooth_quartz
setblock -20 65 -120 minecraft:quartz_pillar[axis=y]
setblock -20 66 -120 minecraft:quartz_pillar[axis=y]
setblock -20 67 -120 minecraft:quartz_pillar[axis=y]
setblock -20 68 -120 minecraft:quartz_pillar[axis=y]
setblock -20 69 -120 minecraft:quartz_pillar[axis=y]
setblock -20 70 -120 minecraft:quartz_pillar[axis=y]
setblock -20 71 -120 minecraft:quartz_pillar[axis=y]
setblock -20 72 -120 minecraft:quartz_pillar[axis=y]
setblock -20 73 -120 minecraft:quartz_pillar[axis=y]
setblock -20 74 -120 minecraft:quartz_pillar[axis=y]
setblock -20 75 -120 minecraft:quartz_pillar[axis=y]
setblock -20 76 -120 minecraft:quartz_pillar[axis=y]
setblock -20 77 -120 minecraft:iron_bars
setblock 20 64 -120 minecraft:smooth_quartz
setblock 20 65 -120 minecraft:quartz_pillar[axis=y]
setblock 20 66 -120 minecraft:quartz_pillar[axis=y]
setblock 20 67 -120 minecraft:quartz_pillar[axis=y]
setblock 20 68 -120 minecraft:quartz_pillar[axis=y]
setblock 20 69 -120 minecraft:quartz_pillar[axis=y]
setblock 20 70 -120 minecraft:quartz_pillar[axis=y]
setblock 20 71 -120 minecraft:quartz_pillar[axis=y]
setblock 20 72 -120 minecraft:quartz_pillar[axis=y]
setblock 20 73 -120 minecraft:quartz_pillar[axis=y]
setblock 20 74 -120 minecraft:quartz_pillar[axis=y]
setblock 20 75 -120 minecraft:quartz_pillar[axis=y]
setblock 20 76 -120 minecraft:quartz_pillar[axis=y]
setblock 20 77 -120 minecraft:iron_bars

setblock -20 64 -136 minecraft:smooth_quartz
setblock -20 65 -136 minecraft:quartz_pillar[axis=y]
setblock -20 66 -136 minecraft:quartz_pillar[axis=y]
setblock -20 67 -136 minecraft:quartz_pillar[axis=y]
setblock -20 68 -136 minecraft:quartz_pillar[axis=y]
setblock -20 69 -136 minecraft:quartz_pillar[axis=y]
setblock -20 70 -136 minecraft:quartz_pillar[axis=y]
setblock -20 71 -136 minecraft:quartz_pillar[axis=y]
setblock -20 72 -136 minecraft:quartz_pillar[axis=y]
setblock -20 73 -136 minecraft:quartz_pillar[axis=y]
setblock -20 74 -136 minecraft:quartz_pillar[axis=y]
setblock -20 75 -136 minecraft:quartz_pillar[axis=y]
setblock -20 76 -136 minecraft:quartz_pillar[axis=y]
setblock -20 77 -136 minecraft:iron_bars
setblock 20 64 -136 minecraft:smooth_quartz
setblock 20 65 -136 minecraft:quartz_pillar[axis=y]
setblock 20 66 -136 minecraft:quartz_pillar[axis=y]
setblock 20 67 -136 minecraft:quartz_pillar[axis=y]
setblock 20 68 -136 minecraft:quartz_pillar[axis=y]
setblock 20 69 -136 minecraft:quartz_pillar[axis=y]
setblock 20 70 -136 minecraft:quartz_pillar[axis=y]
setblock 20 71 -136 minecraft:quartz_pillar[axis=y]
setblock 20 72 -136 minecraft:quartz_pillar[axis=y]
setblock 20 73 -136 minecraft:quartz_pillar[axis=y]
setblock 20 74 -136 minecraft:quartz_pillar[axis=y]
setblock 20 75 -136 minecraft:quartz_pillar[axis=y]
setblock 20 76 -136 minecraft:quartz_pillar[axis=y]
setblock 20 77 -136 minecraft:iron_bars

setblock -20 64 -152 minecraft:smooth_quartz
setblock -20 65 -152 minecraft:quartz_pillar[axis=y]
setblock -20 66 -152 minecraft:quartz_pillar[axis=y]
setblock -20 67 -152 minecraft:quartz_pillar[axis=y]
setblock -20 68 -152 minecraft:quartz_pillar[axis=y]
setblock -20 69 -152 minecraft:quartz_pillar[axis=y]
setblock -20 70 -152 minecraft:quartz_pillar[axis=y]
setblock -20 71 -152 minecraft:quartz_pillar[axis=y]
setblock -20 72 -152 minecraft:quartz_pillar[axis=y]
setblock -20 73 -152 minecraft:quartz_pillar[axis=y]
setblock -20 74 -152 minecraft:quartz_pillar[axis=y]
setblock -20 75 -152 minecraft:quartz_pillar[axis=y]
setblock -20 76 -152 minecraft:quartz_pillar[axis=y]
setblock -20 77 -152 minecraft:iron_bars
setblock 20 64 -152 minecraft:smooth_quartz
setblock 20 65 -152 minecraft:quartz_pillar[axis=y]
setblock 20 66 -152 minecraft:quartz_pillar[axis=y]
setblock 20 67 -152 minecraft:quartz_pillar[axis=y]
setblock 20 68 -152 minecraft:quartz_pillar[axis=y]
setblock 20 69 -152 minecraft:quartz_pillar[axis=y]
setblock 20 70 -152 minecraft:quartz_pillar[axis=y]
setblock 20 71 -152 minecraft:quartz_pillar[axis=y]
setblock 20 72 -152 minecraft:quartz_pillar[axis=y]
setblock 20 73 -152 minecraft:quartz_pillar[axis=y]
setblock 20 74 -152 minecraft:quartz_pillar[axis=y]
setblock 20 75 -152 minecraft:quartz_pillar[axis=y]
setblock 20 76 -152 minecraft:quartz_pillar[axis=y]
setblock 20 77 -152 minecraft:iron_bars

setblock -20 64 -168 minecraft:smooth_quartz
setblock -20 65 -168 minecraft:quartz_pillar[axis=y]
setblock -20 66 -168 minecraft:quartz_pillar[axis=y]
setblock -20 67 -168 minecraft:quartz_pillar[axis=y]
setblock -20 68 -168 minecraft:quartz_pillar[axis=y]
setblock -20 69 -168 minecraft:quartz_pillar[axis=y]
setblock -20 70 -168 minecraft:quartz_pillar[axis=y]
setblock -20 71 -168 minecraft:quartz_pillar[axis=y]
setblock -20 72 -168 minecraft:quartz_pillar[axis=y]
setblock -20 73 -168 minecraft:quartz_pillar[axis=y]
setblock -20 74 -168 minecraft:quartz_pillar[axis=y]
setblock -20 75 -168 minecraft:quartz_pillar[axis=y]
setblock -20 76 -168 minecraft:quartz_pillar[axis=y]
setblock -20 77 -168 minecraft:iron_bars
setblock 20 64 -168 minecraft:smooth_quartz
setblock 20 65 -168 minecraft:quartz_pillar[axis=y]
setblock 20 66 -168 minecraft:quartz_pillar[axis=y]
setblock 20 67 -168 minecraft:quartz_pillar[axis=y]
setblock 20 68 -168 minecraft:quartz_pillar[axis=y]
setblock 20 69 -168 minecraft:quartz_pillar[axis=y]
setblock 20 70 -168 minecraft:quartz_pillar[axis=y]
setblock 20 71 -168 minecraft:quartz_pillar[axis=y]
setblock 20 72 -168 minecraft:quartz_pillar[axis=y]
setblock 20 73 -168 minecraft:quartz_pillar[axis=y]
setblock 20 74 -168 minecraft:quartz_pillar[axis=y]
setblock 20 75 -168 minecraft:quartz_pillar[axis=y]
setblock 20 76 -168 minecraft:quartz_pillar[axis=y]
setblock 20 77 -168 minecraft:iron_bars

setblock -20 64 -184 minecraft:smooth_quartz
setblock -20 65 -184 minecraft:quartz_pillar[axis=y]
setblock -20 66 -184 minecraft:quartz_pillar[axis=y]
setblock -20 67 -184 minecraft:quartz_pillar[axis=y]
setblock -20 68 -184 minecraft:quartz_pillar[axis=y]
setblock -20 69 -184 minecraft:quartz_pillar[axis=y]
setblock -20 70 -184 minecraft:quartz_pillar[axis=y]
setblock -20 71 -184 minecraft:quartz_pillar[axis=y]
setblock -20 72 -184 minecraft:quartz_pillar[axis=y]
setblock -20 73 -184 minecraft:quartz_pillar[axis=y]
setblock -20 74 -184 minecraft:quartz_pillar[axis=y]
setblock -20 75 -184 minecraft:quartz_pillar[axis=y]
setblock -20 76 -184 minecraft:quartz_pillar[axis=y]
setblock -20 77 -184 minecraft:iron_bars
setblock 20 64 -184 minecraft:smooth_quartz
setblock 20 65 -184 minecraft:quartz_pillar[axis=y]
setblock 20 66 -184 minecraft:quartz_pillar[axis=y]
setblock 20 67 -184 minecraft:quartz_pillar[axis=y]
setblock 20 68 -184 minecraft:quartz_pillar[axis=y]
setblock 20 69 -184 minecraft:quartz_pillar[axis=y]
setblock 20 70 -184 minecraft:quartz_pillar[axis=y]
setblock 20 71 -184 minecraft:quartz_pillar[axis=y]
setblock 20 72 -184 minecraft:quartz_pillar[axis=y]
setblock 20 73 -184 minecraft:quartz_pillar[axis=y]
setblock 20 74 -184 minecraft:quartz_pillar[axis=y]
setblock 20 75 -184 minecraft:quartz_pillar[axis=y]
setblock 20 76 -184 minecraft:quartz_pillar[axis=y]
setblock 20 77 -184 minecraft:iron_bars

setblock -20 64 -200 minecraft:smooth_quartz
setblock -20 65 -200 minecraft:quartz_pillar[axis=y]
setblock -20 66 -200 minecraft:quartz_pillar[axis=y]
setblock -20 67 -200 minecraft:quartz_pillar[axis=y]
setblock -20 68 -200 minecraft:quartz_pillar[axis=y]
setblock -20 69 -200 minecraft:quartz_pillar[axis=y]
setblock -20 70 -200 minecraft:quartz_pillar[axis=y]
setblock -20 71 -200 minecraft:quartz_pillar[axis=y]
setblock -20 72 -200 minecraft:quartz_pillar[axis=y]
setblock -20 73 -200 minecraft:quartz_pillar[axis=y]
setblock -20 74 -200 minecraft:quartz_pillar[axis=y]
setblock -20 75 -200 minecraft:quartz_pillar[axis=y]
setblock -20 76 -200 minecraft:quartz_pillar[axis=y]
setblock -20 77 -200 minecraft:iron_bars
setblock 20 64 -200 minecraft:smooth_quartz
setblock 20 65 -200 minecraft:quartz_pillar[axis=y]
setblock 20 66 -200 minecraft:quartz_pillar[axis=y]
setblock 20 67 -200 minecraft:quartz_pillar[axis=y]
setblock 20 68 -200 minecraft:quartz_pillar[axis=y]
setblock 20 69 -200 minecraft:quartz_pillar[axis=y]
setblock 20 70 -200 minecraft:quartz_pillar[axis=y]
setblock 20 71 -200 minecraft:quartz_pillar[axis=y]
setblock 20 72 -200 minecraft:quartz_pillar[axis=y]
setblock 20 73 -200 minecraft:quartz_pillar[axis=y]
setblock 20 74 -200 minecraft:quartz_pillar[axis=y]
setblock 20 75 -200 minecraft:quartz_pillar[axis=y]
setblock 20 76 -200 minecraft:quartz_pillar[axis=y]
setblock 20 77 -200 minecraft:iron_bars

setblock -20 64 -216 minecraft:smooth_quartz
setblock -20 65 -216 minecraft:quartz_pillar[axis=y]
setblock -20 66 -216 minecraft:quartz_pillar[axis=y]
setblock -20 67 -216 minecraft:quartz_pillar[axis=y]
setblock -20 68 -216 minecraft:quartz_pillar[axis=y]
setblock -20 69 -216 minecraft:quartz_pillar[axis=y]
setblock -20 70 -216 minecraft:quartz_pillar[axis=y]
setblock -20 71 -216 minecraft:quartz_pillar[axis=y]
setblock -20 72 -216 minecraft:quartz_pillar[axis=y]
setblock -20 73 -216 minecraft:quartz_pillar[axis=y]
setblock -20 74 -216 minecraft:quartz_pillar[axis=y]
setblock -20 75 -216 minecraft:quartz_pillar[axis=y]
setblock -20 76 -216 minecraft:quartz_pillar[axis=y]
setblock -20 77 -216 minecraft:iron_bars
setblock 20 64 -216 minecraft:smooth_quartz
setblock 20 65 -216 minecraft:quartz_pillar[axis=y]
setblock 20 66 -216 minecraft:quartz_pillar[axis=y]
setblock 20 67 -216 minecraft:quartz_pillar[axis=y]
setblock 20 68 -216 minecraft:quartz_pillar[axis=y]
setblock 20 69 -216 minecraft:quartz_pillar[axis=y]
setblock 20 70 -216 minecraft:quartz_pillar[axis=y]
setblock 20 71 -216 minecraft:quartz_pillar[axis=y]
setblock 20 72 -216 minecraft:quartz_pillar[axis=y]
setblock 20 73 -216 minecraft:quartz_pillar[axis=y]
setblock 20 74 -216 minecraft:quartz_pillar[axis=y]
setblock 20 75 -216 minecraft:quartz_pillar[axis=y]
setblock 20 76 -216 minecraft:quartz_pillar[axis=y]
setblock 20 77 -216 minecraft:iron_bars

setblock -20 64 -232 minecraft:smooth_quartz
setblock -20 65 -232 minecraft:quartz_pillar[axis=y]
setblock -20 66 -232 minecraft:quartz_pillar[axis=y]
setblock -20 67 -232 minecraft:quartz_pillar[axis=y]
setblock -20 68 -232 minecraft:quartz_pillar[axis=y]
setblock -20 69 -232 minecraft:quartz_pillar[axis=y]
setblock -20 70 -232 minecraft:quartz_pillar[axis=y]
setblock -20 71 -232 minecraft:quartz_pillar[axis=y]
setblock -20 72 -232 minecraft:quartz_pillar[axis=y]
setblock -20 73 -232 minecraft:quartz_pillar[axis=y]
setblock -20 74 -232 minecraft:quartz_pillar[axis=y]
setblock -20 75 -232 minecraft:quartz_pillar[axis=y]
setblock -20 76 -232 minecraft:quartz_pillar[axis=y]
setblock -20 77 -232 minecraft:iron_bars
setblock 20 64 -232 minecraft:smooth_quartz
setblock 20 65 -232 minecraft:quartz_pillar[axis=y]
setblock 20 66 -232 minecraft:quartz_pillar[axis=y]
setblock 20 67 -232 minecraft:quartz_pillar[axis=y]
setblock 20 68 -232 minecraft:quartz_pillar[axis=y]
setblock 20 69 -232 minecraft:quartz_pillar[axis=y]
setblock 20 70 -232 minecraft:quartz_pillar[axis=y]
setblock 20 71 -232 minecraft:quartz_pillar[axis=y]
setblock 20 72 -232 minecraft:quartz_pillar[axis=y]
setblock 20 73 -232 minecraft:quartz_pillar[axis=y]
setblock 20 74 -232 minecraft:quartz_pillar[axis=y]
setblock 20 75 -232 minecraft:quartz_pillar[axis=y]
setblock 20 76 -232 minecraft:quartz_pillar[axis=y]
setblock 20 77 -232 minecraft:iron_bars

setblock -20 64 -248 minecraft:smooth_quartz
setblock -20 65 -248 minecraft:quartz_pillar[axis=y]
setblock -20 66 -248 minecraft:quartz_pillar[axis=y]
setblock -20 67 -248 minecraft:quartz_pillar[axis=y]
setblock -20 68 -248 minecraft:quartz_pillar[axis=y]
setblock -20 69 -248 minecraft:quartz_pillar[axis=y]
setblock -20 70 -248 minecraft:quartz_pillar[axis=y]
setblock -20 71 -248 minecraft:quartz_pillar[axis=y]
setblock -20 72 -248 minecraft:quartz_pillar[axis=y]
setblock -20 73 -248 minecraft:quartz_pillar[axis=y]
setblock -20 74 -248 minecraft:quartz_pillar[axis=y]
setblock -20 75 -248 minecraft:quartz_pillar[axis=y]
setblock -20 76 -248 minecraft:quartz_pillar[axis=y]
setblock -20 77 -248 minecraft:iron_bars
setblock 20 64 -248 minecraft:smooth_quartz
setblock 20 65 -248 minecraft:quartz_pillar[axis=y]
setblock 20 66 -248 minecraft:quartz_pillar[axis=y]
setblock 20 67 -248 minecraft:quartz_pillar[axis=y]
setblock 20 68 -248 minecraft:quartz_pillar[axis=y]
setblock 20 69 -248 minecraft:quartz_pillar[axis=y]
setblock 20 70 -248 minecraft:quartz_pillar[axis=y]
setblock 20 71 -248 minecraft:quartz_pillar[axis=y]
setblock 20 72 -248 minecraft:quartz_pillar[axis=y]
setblock 20 73 -248 minecraft:quartz_pillar[axis=y]
setblock 20 74 -248 minecraft:quartz_pillar[axis=y]
setblock 20 75 -248 minecraft:quartz_pillar[axis=y]
setblock 20 76 -248 minecraft:quartz_pillar[axis=y]
setblock 20 77 -248 minecraft:iron_bars

setblock -20 64 -264 minecraft:smooth_quartz
setblock -20 65 -264 minecraft:quartz_pillar[axis=y]
setblock -20 66 -264 minecraft:quartz_pillar[axis=y]
setblock -20 67 -264 minecraft:quartz_pillar[axis=y]
setblock -20 68 -264 minecraft:quartz_pillar[axis=y]
setblock -20 69 -264 minecraft:quartz_pillar[axis=y]
setblock -20 70 -264 minecraft:quartz_pillar[axis=y]
setblock -20 71 -264 minecraft:quartz_pillar[axis=y]
setblock -20 72 -264 minecraft:quartz_pillar[axis=y]
setblock -20 73 -264 minecraft:quartz_pillar[axis=y]
setblock -20 74 -264 minecraft:quartz_pillar[axis=y]
setblock -20 75 -264 minecraft:quartz_pillar[axis=y]
setblock -20 76 -264 minecraft:quartz_pillar[axis=y]
setblock -20 77 -264 minecraft:iron_bars
setblock 20 64 -264 minecraft:smooth_quartz
setblock 20 65 -264 minecraft:quartz_pillar[axis=y]
setblock 20 66 -264 minecraft:quartz_pillar[axis=y]
setblock 20 67 -264 minecraft:quartz_pillar[axis=y]
setblock 20 68 -264 minecraft:quartz_pillar[axis=y]
setblock 20 69 -264 minecraft:quartz_pillar[axis=y]
setblock 20 70 -264 minecraft:quartz_pillar[axis=y]
setblock 20 71 -264 minecraft:quartz_pillar[axis=y]
setblock 20 72 -264 minecraft:quartz_pillar[axis=y]
setblock 20 73 -264 minecraft:quartz_pillar[axis=y]
setblock 20 74 -264 minecraft:quartz_pillar[axis=y]
setblock 20 75 -264 minecraft:quartz_pillar[axis=y]
setblock 20 76 -264 minecraft:quartz_pillar[axis=y]
setblock 20 77 -264 minecraft:iron_bars

# =============================================================================
# STEP G — WING JUNCTION: PROMENADE OPENING TO WINGS AT STORE-ROW LEVEL
#
# At z=-195..-245, the main promenade wall (at x=±21) opens to the lateral wings.
# Players walking down the promenade can see into both wing galleries.
# Clear a 3-block-wide viewing portal in the store-front wall at z=-210..-230.
# =============================================================================

# West wing junction: clear store-front wall at x=-21 across wing sight-line
fill -50 65 -230 -21 78 -210 minecraft:air

# East wing junction
fill 21 65 -230 50 78 -210 minecraft:air

# =============================================================================
# STEP H — SOUTH LOBBY EXPANSION (z=-100..-60, exterior-to-interior transition)
#
# The arrival court is currently 13 blocks wide. Widen to x=-20..20 and
# lay a proper arrival plaza approach matching promenade scale.
# =============================================================================

# Widen arrival approach floor from x=-6..6 to x=-20..20
fill -20 64 -99 20 64 -60 minecraft:smooth_quartz
fill -20 64 -99 -7 64 -60 minecraft:polished_andesite
fill 7 64 -99 20 64 -60 minecraft:polished_andesite
# Center stripe stays smooth quartz (already done by first fill above,
# then andesite overwrites wings — restore center)
fill -6 64 -99 6 64 -60 minecraft:smooth_quartz

# Clear terrain above the widened approach (14 wide each side, z=-60..-99, y=65..78)
fill -20 65 -99 -7 78 -60 minecraft:air
fill 7 65 -99 20 78 -60 minecraft:air

tellraw @a {"text":"[MOTFB] Sprawl expansion complete. Mall now x=-86..86, z=-100..-420, + lateral wings to x=±150.","color":"green","bold":true}
