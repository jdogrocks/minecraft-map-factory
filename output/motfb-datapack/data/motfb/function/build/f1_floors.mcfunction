# =============================================================================
# F1 — Per-Store Floor Differentiation (MIN-160)
# Each of the 10 store bays gets a distinct floor material matching §1.x palette.
# Corridor floor (y=64, x=-6..6) keeps smooth quartz per §1.0.
# Store interiors fill at y=64 only (no structural risk).
# =============================================================================

# --- WEST STORES (x=-49..-7) ---

# Cluck-O-Mart / Colonel Kraw (z=-246..-260): yellow terracotta dining, red concrete kitchen §1.2
fill -49 64 -260 -7 64 -260 minecraft:yellow_terracotta
fill -49 64 -259 -7 64 -253 minecraft:yellow_terracotta
fill -49 64 -252 -7 64 -246 minecraft:red_concrete
# Kitchen fryer pit trace (red concrete back section behind boss spawn z=-256..-258)
fill -35 64 -258 -20 64 -253 minecraft:red_concrete

# GameStomp / Pixel Lich (z=-231..-245): dark oak planks §1.4
fill -49 64 -245 -7 64 -231 minecraft:dark_oak_planks
# Checkout area (light gray concrete near corridor entrance x=-15..-7)
fill -15 64 -245 -7 64 -231 minecraft:gray_concrete

# Cinnabog & Co. / Candy Witch (z=-216..-230): birch planks with brown terracotta accents §1.3
fill -49 64 -230 -7 64 -216 minecraft:birch_planks
# Counter zone oak trapdoor mat (near entrance x=-10..-7)
fill -12 64 -225 -7 64 -221 minecraft:oak_trapdoor[facing=north,open=false,half=bottom]

# Build-A-Boss / Stitch Lord (z=-201..-215): pink concrete with lime inset dots §1.1
fill -49 64 -215 -7 64 -201 minecraft:pink_concrete
# Lime dot pattern every 4 blocks (playmat feel)
fill -45 64 -215 -45 64 -215 minecraft:lime_concrete
fill -41 64 -211 -41 64 -211 minecraft:lime_concrete
fill -37 64 -207 -37 64 -207 minecraft:lime_concrete
fill -33 64 -203 -33 64 -203 minecraft:lime_concrete
fill -29 64 -215 -29 64 -215 minecraft:lime_concrete
fill -25 64 -211 -25 64 -211 minecraft:lime_concrete
fill -21 64 -207 -21 64 -207 minecraft:lime_concrete
fill -17 64 -203 -17 64 -203 minecraft:lime_concrete
fill -13 64 -211 -13 64 -211 minecraft:lime_concrete
fill -9 64 -205 -9 64 -205 minecraft:lime_concrete

# Hot-Topical / Vampire Queen (z=-186..-200): black concrete with deepslate tile §1.5
fill -49 64 -200 -7 64 -186 minecraft:black_concrete
# Fitting room threshold: crimson planks strip (near back of store x=-35..-20)
fill -35 64 -196 -20 64 -190 minecraft:crimson_planks

# --- EAST STORES (x=7..49) ---

# Spencer's Cursed Gifts / Imp Swarm (z=-246..-260): multi-color chaos per §1.6
# Rotating concrete colors (3-block chunks)
fill 7 64 -260 17 64 -246 minecraft:orange_concrete
fill 18 64 -260 27 64 -246 minecraft:yellow_concrete
fill 28 64 -260 37 64 -246 minecraft:cyan_concrete
fill 38 64 -260 49 64 -246 minecraft:lime_concrete
# Center aisle stays light gray
fill 7 64 -255 20 64 -252 minecraft:light_gray_concrete
# Pentagram center (single lime block where Imp Swarm spawns)
fill 25 64 -253 25 64 -253 minecraft:lime_concrete

# Bath & Bodywork Sanctum / Exiled Saint (z=-231..-245): white concrete with quartz, pink insets §1.7
fill 7 64 -245 49 64 -231 minecraft:white_concrete
# Light pink terracotta insets (1×1 every 4 blocks)
fill 11 64 -245 11 64 -245 minecraft:pink_terracotta
fill 15 64 -241 15 64 -241 minecraft:pink_terracotta
fill 19 64 -237 19 64 -237 minecraft:pink_terracotta
fill 23 64 -233 23 64 -233 minecraft:pink_terracotta
fill 27 64 -245 27 64 -245 minecraft:pink_terracotta
fill 31 64 -241 31 64 -241 minecraft:pink_terracotta
fill 35 64 -237 35 64 -237 minecraft:pink_terracotta
fill 39 64 -233 39 64 -233 minecraft:pink_terracotta
fill 43 64 -241 43 64 -241 minecraft:pink_terracotta

# Pretzel-Pretzel Pretzel / Knot God (z=-216..-230): birch planks with stone brick border §1.8
fill 7 64 -230 49 64 -216 minecraft:birch_planks
# Kiosk border: dark oak around the kiosk zone (center area)
fill 22 64 -228 32 64 -218 minecraft:dark_oak_planks
fill 24 64 -226 30 64 -220 minecraft:birch_planks

# Spunky's Sneakers / Speed Demon (z=-201..-215): white concrete with orange center logo §1.9
fill 7 64 -215 49 64 -201 minecraft:white_concrete
# Light gray concrete grid lines (4-block intervals)
fill 7 64 -211 49 64 -211 minecraft:light_gray_concrete
fill 7 64 -207 49 64 -207 minecraft:light_gray_concrete
fill 7 64 -203 49 64 -203 minecraft:light_gray_concrete
fill 25 64 -215 25 64 -201 minecraft:light_gray_concrete
fill 35 64 -215 35 64 -201 minecraft:light_gray_concrete
# Orange concrete inset logo panel (center of store)
fill 27 64 -213 33 64 -203 minecraft:orange_concrete

# --- SEARZ Anchor (full width x=-49..49, z=-261..-279): polished granite §1.10 ---
fill -49 64 -279 49 64 -261 minecraft:polished_granite
# Cracked stone bricks scattered (damage aesthetic — only in SEARZ)
fill -40 64 -275 -35 64 -270 minecraft:cracked_stone_bricks
fill 30 64 -275 38 64 -268 minecraft:cracked_stone_bricks
fill -20 64 -265 -10 64 -261 minecraft:cracked_stone_bricks

# Corridor base floor §1.0 (was missing — caused entrance-corridor gaps)
fill -6 64 -185 6 64 -102 minecraft:smooth_quartz

# --- Corridor floor (x=-6..6): keep smooth quartz per §1.0 but add polished andesite grout lines ---
# Grout lines every 4 blocks in z
fill -6 64 -279 6 64 -279 minecraft:polished_andesite
fill -6 64 -275 6 64 -275 minecraft:polished_andesite
fill -6 64 -271 6 64 -271 minecraft:polished_andesite
fill -6 64 -267 6 64 -267 minecraft:polished_andesite
fill -6 64 -263 6 64 -263 minecraft:polished_andesite
fill -6 64 -259 6 64 -259 minecraft:polished_andesite
fill -6 64 -255 6 64 -255 minecraft:polished_andesite
fill -6 64 -251 6 64 -251 minecraft:polished_andesite
fill -6 64 -247 6 64 -247 minecraft:polished_andesite
fill -6 64 -243 6 64 -243 minecraft:polished_andesite
fill -6 64 -239 6 64 -239 minecraft:polished_andesite
fill -6 64 -235 6 64 -235 minecraft:polished_andesite
fill -6 64 -231 6 64 -231 minecraft:polished_andesite
fill -6 64 -227 6 64 -227 minecraft:polished_andesite
fill -6 64 -223 6 64 -223 minecraft:polished_andesite
fill -6 64 -219 6 64 -219 minecraft:polished_andesite
fill -6 64 -215 6 64 -215 minecraft:polished_andesite
fill -6 64 -211 6 64 -211 minecraft:polished_andesite
fill -6 64 -207 6 64 -207 minecraft:polished_andesite
fill -6 64 -203 6 64 -203 minecraft:polished_andesite
fill -6 64 -199 6 64 -199 minecraft:polished_andesite
fill -6 64 -195 6 64 -195 minecraft:polished_andesite
fill -6 64 -191 6 64 -191 minecraft:polished_andesite
fill -6 64 -187 6 64 -187 minecraft:polished_andesite
fill -6 64 -183 6 64 -183 minecraft:polished_andesite
fill -6 64 -179 6 64 -179 minecraft:polished_andesite
fill -6 64 -175 6 64 -175 minecraft:polished_andesite
fill -6 64 -171 6 64 -171 minecraft:polished_andesite
fill -6 64 -167 6 64 -167 minecraft:polished_andesite
fill -6 64 -163 6 64 -163 minecraft:polished_andesite
fill -6 64 -159 6 64 -159 minecraft:polished_andesite
fill -6 64 -155 6 64 -155 minecraft:polished_andesite
fill -6 64 -151 6 64 -151 minecraft:polished_andesite
fill -6 64 -147 6 64 -147 minecraft:polished_andesite
fill -6 64 -143 6 64 -143 minecraft:polished_andesite
fill -6 64 -139 6 64 -139 minecraft:polished_andesite
fill -6 64 -135 6 64 -135 minecraft:polished_andesite
fill -6 64 -131 6 64 -131 minecraft:polished_andesite
fill -6 64 -127 6 64 -127 minecraft:polished_andesite
fill -6 64 -123 6 64 -123 minecraft:polished_andesite
fill -6 64 -119 6 64 -119 minecraft:polished_andesite
fill -6 64 -115 6 64 -115 minecraft:polished_andesite
fill -6 64 -111 6 64 -111 minecraft:polished_andesite
fill -6 64 -107 6 64 -107 minecraft:polished_andesite
fill -6 64 -103 6 64 -103 minecraft:polished_andesite

# --- Hot-Topical CORRIDOR section (x=-6..6, z=-186..-200): eggplant floor transition ---
fill -5 64 -200 5 64 -186 minecraft:purple_concrete

# --- Store wall lower accents (baseboard level y=65-66) ---
# Hot-Topical west store: black concrete lower walls + crimson trim
fill -49 65 -200 -49 66 -186 minecraft:black_concrete
fill -49 65 -186 -7 66 -186 minecraft:black_concrete
fill -49 65 -200 -7 66 -200 minecraft:black_concrete
fill -7 65 -200 -7 66 -186 minecraft:black_concrete
# Purple terracotta accent (lower 2-block band inside Hot-Topical)
fill -48 65 -199 -8 66 -187 minecraft:purple_terracotta

# Build-A-Boss west store: light blue concrete walls
fill -49 65 -215 -49 66 -201 minecraft:light_blue_concrete
fill -49 65 -201 -7 66 -201 minecraft:light_blue_concrete
fill -49 65 -215 -7 66 -215 minecraft:light_blue_concrete
fill -7 65 -215 -7 66 -201 minecraft:light_blue_concrete

# GameStomp west store: dark oak planks lower walls
fill -49 65 -245 -49 66 -231 minecraft:dark_oak_planks
fill -49 65 -231 -7 66 -231 minecraft:dark_oak_planks
fill -49 65 -245 -7 66 -245 minecraft:dark_oak_planks
fill -7 65 -245 -7 66 -231 minecraft:dark_oak_planks

# Cluck-O-Mart west store: red concrete lower walls
fill -49 65 -260 -49 66 -246 minecraft:red_concrete
fill -49 65 -246 -7 66 -246 minecraft:red_concrete
fill -49 65 -260 -7 66 -260 minecraft:red_concrete
fill -7 65 -260 -7 66 -246 minecraft:red_concrete

# --- EAST STORE WALL BASEBOARDS (y=65-66) ---
# Spencer's Cursed Gifts east store: orange/yellow concrete lower walls §1.6
fill 49 65 -260 49 66 -246 minecraft:orange_concrete
fill 49 65 -246 7 66 -246 minecraft:orange_concrete
fill 49 65 -260 7 66 -260 minecraft:orange_concrete
fill 7 65 -260 7 66 -246 minecraft:orange_concrete

# Bath & Bodywork Sanctum east store: white concrete lower walls §1.7
fill 49 65 -245 49 66 -231 minecraft:white_concrete
fill 49 65 -231 7 66 -231 minecraft:white_concrete
fill 49 65 -245 7 66 -245 minecraft:white_concrete
fill 7 65 -245 7 66 -231 minecraft:white_concrete
# Lavender stained glass accent (inner band §1.7)
fill 48 65 -244 8 66 -232 minecraft:smooth_quartz

# Pretzel-Pretzel Pretzel east store: smooth sandstone lower walls §1.8
fill 49 65 -230 49 66 -216 minecraft:smooth_sandstone
fill 49 65 -216 7 66 -216 minecraft:smooth_sandstone
fill 49 65 -230 7 66 -230 minecraft:smooth_sandstone
fill 7 65 -230 7 66 -216 minecraft:smooth_sandstone

# Spunky's Sneakers east store: white concrete lower walls §1.9
fill 49 65 -215 49 66 -201 minecraft:white_concrete
fill 49 65 -201 7 66 -201 minecraft:white_concrete
fill 49 65 -215 7 66 -215 minecraft:white_concrete
fill 7 65 -215 7 66 -201 minecraft:white_concrete
# Orange accent stripe at mid-wall height §1.9
fill 48 65 -214 8 66 -202 minecraft:orange_concrete
