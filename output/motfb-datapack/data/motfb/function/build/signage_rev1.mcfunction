# =============================================================================
# MOTFB Storefront Signage Rev 1 — Channel-Letter Facades (MIN-209)
#
# Replaces single oak_wall_sign stubs from scene_details (at y=70)
# with full-width 3-block-tall channel-letter fascia bands (y=71-73).
#
# Structure per store:
#   y=71 (bottom trim): store-color concrete — full entry z-width at x=+/-7
#   y=72 (center glow): continuous light-source strip per store palette
#   y=73 (top cap):     store-color concrete — full entry z-width at x=+/-7
#   Sign block: x=+/-6, y=72, z=midpoint, facing corridor (glowing text)
#
# Store entry z-ranges (from scene_details coordinate reference):
#   Hot-Topical W:   x=-7, z=-186 to z=-200 (mid -193)
#   Build-A-Boss W:  x=-7, z=-201 to z=-215 (mid -208)
#   Cinnabog W:      x=-7, z=-216 to z=-230 (mid -223)
#   GameStomp W:     x=-7, z=-231 to z=-245 (mid -238)
#   Cluck-O-Mart W:  x=-7, z=-246 to z=-260 (mid -253)
#   Spunky's E:      x=7,  z=-201 to z=-215 (mid -208)
#   Pretzel E:       x=7,  z=-216 to z=-230 (mid -223)
#   Bath+Body E:     x=7,  z=-231 to z=-245 (mid -238)
#   Spencer's E:     x=7,  z=-246 to z=-260 (mid -253)
#   SEARZ both:      x=+/-7, z=-261 to z=-279 (mid -270) -- 4-row grand treatment
# =============================================================================

tellraw @a {"text":"[MOTFB] Applying channel-letter facades (MIN-209)...","color":"aqua","italic":true}

# =============================================================================
# STEP 1: Remove stale y=70 single-block sign stubs from scene_details
# =============================================================================

setblock -6 70 -193 minecraft:air
setblock -6 70 -208 minecraft:air
setblock -6 70 -223 minecraft:air
setblock -6 70 -238 minecraft:air
setblock -6 70 -253 minecraft:air
setblock -6 70 -270 minecraft:air
setblock 6 70 -253 minecraft:air
setblock 6 70 -238 minecraft:air
setblock 6 70 -223 minecraft:air
setblock 6 70 -208 minecraft:air
setblock 6 70 -270 minecraft:air

# =============================================================================
# HOT-TOPICAL  (west, x=-7, z=-186 to z=-200, mid z=-193)
# Frame: black_concrete | Glow: soul_lantern (cold blue, sec.1.5)
# =============================================================================

fill -7 71 -200 -7 71 -186 minecraft:black_concrete
fill -7 72 -200 -7 72 -186 minecraft:soul_lantern
fill -7 73 -200 -7 73 -186 minecraft:black_concrete
setblock -6 72 -193 minecraft:oak_wall_sign[facing=east]
data merge block -6 72 -193 {front_text:{messages:[{text:""},{text:"HOT-TOPICAL",color:"light_purple",bold:1b},{text:"alt fashion & more",color:"dark_gray",italic:1b},{text:""}],has_glowing_text:1b,color:"black"},is_waxed:1b}

# =============================================================================
# BUILD-A-BOSS WORKSHOP  (west, x=-7, z=-201 to z=-215, mid z=-208)
# Frame: pink_concrete | Glow: sea_lantern (cold clinical white, sec.1.1)
# =============================================================================

fill -7 71 -215 -7 71 -201 minecraft:pink_concrete
fill -7 72 -215 -7 72 -201 minecraft:sea_lantern
fill -7 73 -215 -7 73 -201 minecraft:pink_concrete
setblock -6 72 -208 minecraft:oak_wall_sign[facing=east]
data merge block -6 72 -208 {front_text:{messages:[{text:""},{text:"BUILD-A-BOSS",color:"light_blue",bold:1b},{text:"custom creatures",color:"pink",italic:1b},{text:""}],has_glowing_text:1b,color:"pink"},is_waxed:1b}

# =============================================================================
# CINNABOG & CO.  (west, x=-7, z=-216 to z=-230, mid z=-223)
# Frame: orange_concrete | Glow: lantern (warm amber, sec.1.3 iron lanterns)
# =============================================================================

fill -7 71 -230 -7 71 -216 minecraft:orange_concrete
fill -7 72 -230 -7 72 -216 minecraft:lantern[hanging=false]
fill -7 73 -230 -7 73 -216 minecraft:orange_concrete
setblock -6 72 -223 minecraft:oak_wall_sign[facing=east]
data merge block -6 72 -223 {front_text:{messages:[{text:""},{text:"CINNABOG & CO.",color:"yellow",bold:1b},{text:"fresh baked horrors",color:"orange",italic:1b},{text:""}],has_glowing_text:1b,color:"orange"},is_waxed:1b}

# =============================================================================
# GAMESTOMP  (west, x=-7, z=-231 to z=-245, mid z=-238)
# Frame: blackstone | Glow: sea_lantern + cyan glass cap (arcade neon, sec.1.4)
# =============================================================================

fill -7 71 -245 -7 71 -231 minecraft:blackstone
fill -7 72 -245 -7 72 -231 minecraft:sea_lantern
fill -7 73 -245 -7 73 -231 minecraft:blackstone
fill -6 73 -245 -6 73 -231 minecraft:cyan_stained_glass_pane
setblock -6 72 -238 minecraft:oak_wall_sign[facing=east]
data merge block -6 72 -238 {front_text:{messages:[{text:""},{text:"GAMESTOMP",color:"dark_green",bold:1b},{text:"find the lost kid inside",color:"gray",italic:1b},{text:""}],has_glowing_text:1b,color:"green"},is_waxed:1b}

# =============================================================================
# CLUCK-O-MART  (west, x=-7, z=-246 to z=-260, mid z=-253)
# Frame: yellow bottom / red top | Glow: glowstone (hot grease-glazed, sec.1.2)
# =============================================================================

fill -7 71 -260 -7 71 -246 minecraft:yellow_concrete
fill -7 72 -260 -7 72 -246 minecraft:glowstone
fill -7 73 -260 -7 73 -246 minecraft:red_concrete
setblock -6 72 -253 minecraft:oak_wall_sign[facing=east]
data merge block -6 72 -253 {front_text:{messages:[{text:""},{text:"CLUCK-O-MART",color:"yellow",bold:1b},{text:"nugget dynasty",color:"red",italic:1b},{text:""}],has_glowing_text:1b,color:"red"},is_waxed:1b}

# =============================================================================
# SPUNKY'S SNEAKERS  (east, x=7, z=-201 to z=-215, mid z=-208)
# Frame: white_concrete + orange bottom accent | Glow: sea_lantern (sec.1.9)
# =============================================================================

fill 7 71 -215 7 71 -201 minecraft:white_concrete
fill 7 71 -213 7 71 -203 minecraft:orange_concrete
fill 7 72 -215 7 72 -201 minecraft:sea_lantern
fill 7 73 -215 7 73 -201 minecraft:white_concrete
setblock 6 72 -208 minecraft:oak_wall_sign[facing=west]
data merge block 6 72 -208 {front_text:{messages:[{text:""},{text:"SPUNKY'S SHOES",color:"orange",bold:1b},{text:"run while you can",color:"white",italic:1b},{text:""}],has_glowing_text:1b,color:"cyan"},is_waxed:1b}

# =============================================================================
# PRETZEL-PRETZEL PRETZEL  (east, x=7, z=-216 to z=-230, mid z=-223)
# Frame: smooth_sandstone + yellow cap strip | Glow: lantern (warm amber, sec.1.8)
# =============================================================================

fill 7 71 -230 7 71 -216 minecraft:smooth_sandstone
fill 7 72 -230 7 72 -216 minecraft:lantern[hanging=false]
fill 7 73 -230 7 73 -216 minecraft:smooth_sandstone
fill 7 73 -228 7 73 -218 minecraft:yellow_concrete
setblock 6 72 -223 minecraft:oak_wall_sign[facing=west]
data merge block 6 72 -223 {front_text:{messages:[{text:""},{text:"PRETZEL PRETZEL",color:"gold",bold:1b},{text:"pretzel",color:"yellow",italic:1b},{text:""}],has_glowing_text:1b,color:"orange"},is_waxed:1b}

# =============================================================================
# BATH & BODYWORK SANCTUM  (east, x=7, z=-231 to z=-245, mid z=-238)
# Frame: white_concrete | Glow: sea_lantern + lavender glass cap (sec.1.7)
# =============================================================================

fill 7 71 -245 7 71 -231 minecraft:white_concrete
fill 7 72 -245 7 72 -231 minecraft:sea_lantern
fill 7 73 -245 7 73 -231 minecraft:white_concrete
fill 6 72 -243 6 72 -233 minecraft:purple_stained_glass_pane
setblock 6 72 -238 minecraft:oak_wall_sign[facing=west]
data merge block 6 72 -238 {front_text:{messages:[{text:""},{text:"BATH + BODY",color:"white",bold:1b},{text:"sanctum of scents",color:"light_purple",italic:1b},{text:""}],has_glowing_text:1b,color:"white"},is_waxed:1b}

# =============================================================================
# SPENCER'S CURSED GIFTS  (east, x=7, z=-246 to z=-260, mid z=-253)
# Frame: orange bottom / lime top + random spot accents | Glow: sea_lantern (sec.1.6)
# =============================================================================

fill 7 71 -260 7 71 -246 minecraft:orange_concrete
fill 7 72 -260 7 72 -246 minecraft:sea_lantern
fill 7 73 -260 7 73 -246 minecraft:lime_concrete
setblock 7 71 -256 minecraft:cyan_concrete
setblock 7 71 -250 minecraft:magenta_concrete
setblock 7 73 -256 minecraft:yellow_concrete
setblock 7 73 -250 minecraft:orange_concrete
setblock 6 72 -253 minecraft:oak_wall_sign[facing=west]
data merge block 6 72 -253 {front_text:{messages:[{text:""},{text:"SPENCER'S GIFTS",color:"orange",bold:1b},{text:"cursed collectibles",color:"yellow",italic:1b},{text:""}],has_glowing_text:1b,color:"orange"},is_waxed:1b}

# =============================================================================
# SEARZ DEPARTMENT STORE  (both sides, x=+/-7, z=-261 to z=-279, mid z=-270)
# 4-row grand anchor treatment (y=71-74)
# Frame: smooth_quartz + red_concrete cap | Glow: sea_lantern (sec.1.10)
# =============================================================================

# West face
fill -7 71 -279 -7 71 -261 minecraft:smooth_quartz
fill -7 72 -279 -7 72 -261 minecraft:sea_lantern
fill -7 73 -279 -7 73 -261 minecraft:smooth_quartz
fill -7 74 -279 -7 74 -261 minecraft:smooth_quartz
fill -7 74 -277 -7 74 -263 minecraft:red_concrete

# East face
fill 7 71 -279 7 71 -261 minecraft:smooth_quartz
fill 7 72 -279 7 72 -261 minecraft:sea_lantern
fill 7 73 -279 7 73 -261 minecraft:smooth_quartz
fill 7 74 -279 7 74 -261 minecraft:smooth_quartz
fill 7 74 -277 7 74 -263 minecraft:red_concrete

# Grand SEARZ nameplates (west and east facing corridor)
setblock -6 72 -270 minecraft:oak_wall_sign[facing=east]
data merge block -6 72 -270 {front_text:{messages:[{text:""},{text:"SEARZ",color:"dark_red",bold:1b},{text:"DEPARTMENT STORE",color:"red"},{text:""}],has_glowing_text:1b,color:"red"},is_waxed:1b}
setblock 6 72 -270 minecraft:oak_wall_sign[facing=west]
data merge block 6 72 -270 {front_text:{messages:[{text:""},{text:"SEARZ",color:"dark_red",bold:1b},{text:"DEPARTMENT STORE",color:"red"},{text:""}],has_glowing_text:1b,color:"red"},is_waxed:1b}

# SEARZ floor-directory sub-signs (ground and level 2)
setblock -6 72 -264 minecraft:oak_wall_sign[facing=east]
data merge block -6 72 -264 {front_text:{messages:[{text:"SEARZ"},{text:"Housewares & Home"},{text:"Ground Floor",color:"dark_gray",italic:1b},{text:""}],has_glowing_text:0b,color:"gray"},is_waxed:1b}
setblock -6 72 -276 minecraft:oak_wall_sign[facing=east]
data merge block -6 72 -276 {front_text:{messages:[{text:"SEARZ"},{text:"Apparel / Footwear"},{text:"Level 2",color:"dark_gray",italic:1b},{text:""}],has_glowing_text:0b,color:"gray"},is_waxed:1b}

tellraw @a {"text":"[MOTFB] Channel-letter facades installed at all 10 store entries.","color":"aqua","bold":true}
