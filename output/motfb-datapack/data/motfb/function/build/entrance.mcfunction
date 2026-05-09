# =============================================================================
# ENTRANCE — South exterior approach and guaranteed spawn platform
#
# Spawn is set to 0 65 -90 by init.mcfunction. This function guarantees solid
# floor exists at y=64 beneath that spawn point regardless of world terrain.
# Also builds a visible exterior approach path (z=-86..-100) so the player can
# see they are OUTSIDE the mall before walking north through the entrance.
#
# South mall wall is at z=-101; the entrance opening in that wall is at x=-7..7.
# =============================================================================

# --- Guaranteed spawn floor: smooth quartz platform at y=64 ---
# Player spawns at y=65 and drops 1 block onto this platform
fill -8 64 -100 8 64 -85 minecraft:smooth_quartz
fill -8 65 -100 8 65 -85 minecraft:air
fill -8 66 -100 8 66 -85 minecraft:air

# --- Center approach stripe: polished andesite carpet down the middle ---
fill -2 64 -100 2 64 -85 minecraft:polished_andesite

# --- Entrance-marker lantern posts flanking the path ---
setblock -5 64 -93 minecraft:smooth_stone
setblock -5 65 -93 minecraft:smooth_stone_slab[type=top]
setblock -5 66 -93 minecraft:iron_bars
setblock -5 67 -93 minecraft:iron_bars
setblock -5 68 -93 minecraft:lantern
setblock 5 64 -93 minecraft:smooth_stone
setblock 5 65 -93 minecraft:smooth_stone_slab[type=top]
setblock 5 66 -93 minecraft:iron_bars
setblock 5 67 -93 minecraft:iron_bars
setblock 5 68 -93 minecraft:lantern

# --- South facade: ensure the entrance opening is clear at y=65..76 ---
# The mall south wall should already exist in the world. Clear the opening so the
# player can walk north through the entrance corridor.
fill -6 65 -101 6 76 -101 minecraft:air

# --- Welcome banner above entrance opening (z=-101, y=77..79) ---
# Gold block header bar signals "this is the entrance"
fill -7 77 -101 7 77 -101 minecraft:gold_block
fill -6 78 -101 6 78 -101 minecraft:smooth_quartz
fill -5 79 -101 5 79 -101 minecraft:smooth_quartz

tellraw @a {"text":"[MOTFB] Entrance platform and approach path built.","color":"dark_green"}
