# =============================================================================
# SCENE DESIGN: Fountain Plaza Glass Dome Atrium (MIN-163)
# §3.3 Setpiece 1 — "the lungs of the map"
#
# Fountain center: x=0, z=-162 (water pool at x=-1..1, z=-161..-163, y=64)
# Dome footprint: 20×20 = x=-10..10, z=-172..-152
# Dome top: y=88 (24 blocks above fountain base y=64)
#
# Approach: clear the troffer grid (sea_lantern, smooth_stone_slab) from the
# 20×20 fountain zone at y=79 so the space reads as an open 24-block atrium,
# then place the concentric glass-ring ceiling at y=88.
# Uses replace filters only — will not disturb structural wall blocks.
# =============================================================================

# --- Clear the troffer ceiling from the fountain plaza zone ---
# Remove sea lantern strips and smooth_stone_slab drop ceiling (both placed by f2_lighting)
# leave white_concrete and other structural blocks intact
fill -10 79 -172 10 79 -152 minecraft:air replace minecraft:sea_lantern
fill -10 79 -172 10 79 -152 minecraft:air replace minecraft:smooth_stone_slab

# --- Glass dome ceiling at y=88 — concentric ring pattern (§3.3) ---
# Outer band: orange stained glass (amber substitute — "last light of 8:47 PM sun")
fill -10 88 -172 10 88 -152 minecraft:orange_stained_glass
# Middle band: cyan stained glass (teal substitute)
fill -8 88 -170 8 88 -154 minecraft:cyan_stained_glass
# Inner band: light blue stained glass
fill -6 88 -168 6 88 -156 minecraft:light_blue_stained_glass
# Center: clear glass (lets sky through directly above fountain)
fill -4 88 -166 4 88 -158 minecraft:glass

# --- Slight dome curve: outer ring one block lower (y=87) for visual depth ---
# Perimeter strip of orange at y=87 (1-block border of the 20×20 zone)
fill -10 87 -172 10 87 -172 minecraft:orange_stained_glass
fill -10 87 -152 10 87 -152 minecraft:orange_stained_glass
fill -10 87 -172 -10 87 -152 minecraft:orange_stained_glass
fill 10 87 -172 10 87 -152 minecraft:orange_stained_glass

# --- Vertical atrium glass on the dome perimeter (y=80..86) ---
# These give the atrium volume visible from the plaza floor looking up
# North wall glass strip (at z=-172, the north edge)
fill -9 80 -172 9 86 -172 minecraft:glass_pane
# South wall glass strip (at z=-152, the south edge)
fill -9 80 -152 9 86 -152 minecraft:glass_pane
# West wall glass strip (at x=-10)
fill -10 80 -171 -10 86 -153 minecraft:glass_pane
# East wall glass strip (at x=10)
fill 10 80 -171 10 86 -153 minecraft:glass_pane

# --- Fountain sea lanterns already placed by f1_decor at y=63 ---
# No changes needed to the fountain pool itself

tellraw @a {"text":"[MOTFB] Setpiece 1: Fountain Plaza glass dome built.","color":"aqua"}
