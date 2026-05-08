# =============================================================================
# F2 — Interior Lighting Rework (MIN-160)
# Removes Jason's sea-lantern stop-gap (~34,600 blocks at y=79 and y=96)
# and replaces with intentional period-appropriate recessed troffer lighting.
#
# Floor 1 ceiling (y=79): white concrete base with sea lanterns every 4 z blocks
# — reads as long-run fluorescent troffers running east-west per mall aesthetic.
# Floor 2 mezzanine (y=96): same pattern, sparser.
# Per-store ceiling overrides follow the base pass.
# =============================================================================

# --- Floor 1: clear stop-gap sea lanterns at y=78 and y=79 ---
fill -49 79 -279 49 79 -101 minecraft:white_concrete replace minecraft:sea_lantern
fill -49 78 -279 49 78 -101 minecraft:white_concrete replace minecraft:sea_lantern

# --- Floor 1: corridor ceiling grid (smooth_stone_slab as drop-ceiling tiles) ---
# Central corridor x=-6..6: smooth stone slab grid with sea lantern troffers
fill -6 79 -279 6 79 -101 minecraft:smooth_stone_slab[type=top]

# --- Floor 1: troffer strip rows every 4 z blocks in the full interior ---
# These replace the slab grid back to sea_lantern at the row positions,
# giving a strip-of-troffers look (each strip is 1 block wide, full mall width)
fill -49 79 -279 49 79 -279 minecraft:sea_lantern
fill -49 79 -275 49 79 -275 minecraft:sea_lantern
fill -49 79 -271 49 79 -271 minecraft:sea_lantern
fill -49 79 -267 49 79 -267 minecraft:sea_lantern
fill -49 79 -263 49 79 -263 minecraft:sea_lantern
fill -49 79 -259 49 79 -259 minecraft:sea_lantern
fill -49 79 -255 49 79 -255 minecraft:sea_lantern
fill -49 79 -251 49 79 -251 minecraft:sea_lantern
fill -49 79 -247 49 79 -247 minecraft:sea_lantern
fill -49 79 -243 49 79 -243 minecraft:sea_lantern
fill -49 79 -239 49 79 -239 minecraft:sea_lantern
fill -49 79 -235 49 79 -235 minecraft:sea_lantern
fill -49 79 -231 49 79 -231 minecraft:sea_lantern
fill -49 79 -227 49 79 -227 minecraft:sea_lantern
fill -49 79 -223 49 79 -223 minecraft:sea_lantern
fill -49 79 -219 49 79 -219 minecraft:sea_lantern
fill -49 79 -215 49 79 -215 minecraft:sea_lantern
fill -49 79 -211 49 79 -211 minecraft:sea_lantern
fill -49 79 -207 49 79 -207 minecraft:sea_lantern
fill -49 79 -203 49 79 -203 minecraft:sea_lantern
fill -49 79 -199 49 79 -199 minecraft:sea_lantern
fill -49 79 -195 49 79 -195 minecraft:sea_lantern
fill -49 79 -191 49 79 -191 minecraft:sea_lantern
fill -49 79 -187 49 79 -187 minecraft:sea_lantern
fill -49 79 -183 49 79 -183 minecraft:sea_lantern
fill -49 79 -179 49 79 -179 minecraft:sea_lantern
fill -49 79 -175 49 79 -175 minecraft:sea_lantern
fill -49 79 -171 49 79 -171 minecraft:sea_lantern
fill -49 79 -167 49 79 -167 minecraft:sea_lantern
fill -49 79 -163 49 79 -163 minecraft:sea_lantern
fill -49 79 -159 49 79 -159 minecraft:sea_lantern
fill -49 79 -155 49 79 -155 minecraft:sea_lantern
fill -49 79 -151 49 79 -151 minecraft:sea_lantern
fill -49 79 -147 49 79 -147 minecraft:sea_lantern
fill -49 79 -143 49 79 -143 minecraft:sea_lantern
fill -49 79 -139 49 79 -139 minecraft:sea_lantern
fill -49 79 -135 49 79 -135 minecraft:sea_lantern
fill -49 79 -131 49 79 -131 minecraft:sea_lantern
fill -49 79 -127 49 79 -127 minecraft:sea_lantern
fill -49 79 -123 49 79 -123 minecraft:sea_lantern
fill -49 79 -119 49 79 -119 minecraft:sea_lantern
fill -49 79 -115 49 79 -115 minecraft:sea_lantern
fill -49 79 -111 49 79 -111 minecraft:sea_lantern
fill -49 79 -107 49 79 -107 minecraft:sea_lantern
fill -49 79 -103 49 79 -103 minecraft:sea_lantern

# --- Per-store ceiling overrides (F1 ceiling level y=79) ---

# GameStomp (west, z=-231..-245): intentionally no overhead ambient per §1.4
# Only motivated light sources in the store — remove troffers from the store interior
fill -49 79 -245 -7 79 -231 minecraft:blackstone replace minecraft:sea_lantern
fill -49 79 -245 -7 79 -231 minecraft:blackstone replace minecraft:white_concrete
fill -49 79 -245 -7 79 -231 minecraft:blackstone replace minecraft:smooth_stone_slab

# Hot-Topical (west, z=-186..-200): soul lanterns only per §1.5
fill -49 79 -200 -7 79 -186 minecraft:soul_lantern replace minecraft:sea_lantern
fill -49 79 -200 -7 79 -186 minecraft:black_concrete replace minecraft:white_concrete
fill -49 79 -200 -7 79 -186 minecraft:black_concrete replace minecraft:smooth_stone_slab

# Spencer's Cursed Gifts (east, z=-246..-260): redstone lamps per §1.6
fill 7 79 -260 49 79 -246 minecraft:redstone_lamp replace minecraft:sea_lantern
fill 7 79 -260 49 79 -246 minecraft:yellow_concrete replace minecraft:white_concrete

# Bath & Bodywork Sanctum (east, z=-231..-245): end rods flush per §1.7
fill 7 79 -245 49 79 -231 minecraft:end_rod[facing=down] replace minecraft:sea_lantern
fill 7 79 -245 49 79 -231 minecraft:white_concrete replace minecraft:smooth_stone_slab

# --- Floor 2 mezzanine ceiling (y=96): clear stop-gap sea lanterns ---
fill -49 96 -279 49 96 -101 minecraft:white_concrete replace minecraft:sea_lantern
fill -49 95 -279 49 95 -101 minecraft:white_concrete replace minecraft:sea_lantern

# Floor 2: iron lanterns on posts and wall sconces (use sea_lantern sparser grid)
# Strip troffers every 6 blocks (mezzanine gets dimmer lighting per spec §3.2)
fill -49 96 -279 49 96 -279 minecraft:sea_lantern
fill -49 96 -273 49 96 -273 minecraft:sea_lantern
fill -49 96 -267 49 96 -267 minecraft:sea_lantern
fill -49 96 -261 49 96 -261 minecraft:sea_lantern
fill -49 96 -255 49 96 -255 minecraft:sea_lantern
fill -49 96 -249 49 96 -249 minecraft:sea_lantern
fill -49 96 -243 49 96 -243 minecraft:sea_lantern
fill -49 96 -237 49 96 -237 minecraft:sea_lantern
fill -49 96 -231 49 96 -231 minecraft:sea_lantern
fill -49 96 -225 49 96 -225 minecraft:sea_lantern
fill -49 96 -219 49 96 -219 minecraft:sea_lantern
fill -49 96 -213 49 96 -213 minecraft:sea_lantern
fill -49 96 -207 49 96 -207 minecraft:sea_lantern
fill -49 96 -201 49 96 -201 minecraft:sea_lantern
fill -49 96 -195 49 96 -195 minecraft:sea_lantern
fill -49 96 -189 49 96 -189 minecraft:sea_lantern
fill -49 96 -183 49 96 -183 minecraft:sea_lantern
fill -49 96 -177 49 96 -177 minecraft:sea_lantern
fill -49 96 -171 49 96 -171 minecraft:sea_lantern
fill -49 96 -165 49 96 -165 minecraft:sea_lantern
fill -49 96 -159 49 96 -159 minecraft:sea_lantern
fill -49 96 -153 49 96 -153 minecraft:sea_lantern
fill -49 96 -147 49 96 -147 minecraft:sea_lantern
fill -49 96 -141 49 96 -141 minecraft:sea_lantern
fill -49 96 -135 49 96 -135 minecraft:sea_lantern
fill -49 96 -129 49 96 -129 minecraft:sea_lantern
fill -49 96 -123 49 96 -123 minecraft:sea_lantern
fill -49 96 -117 49 96 -117 minecraft:sea_lantern
fill -49 96 -111 49 96 -111 minecraft:sea_lantern
fill -49 96 -105 49 96 -105 minecraft:sea_lantern

# --- Reset difficulty if Jason had overridden it ---
difficulty normal
