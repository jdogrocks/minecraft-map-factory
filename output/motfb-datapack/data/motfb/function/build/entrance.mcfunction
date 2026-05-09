# Entrance approach floor — x=-20..20, z=-85..-101, y=64
# Lay solid smooth_quartz at y=64 (overwrites any slab or stride decoration)
fill -20 64 -101 20 64 -85 minecraft:smooth_quartz
# Remove any full blocks at y=65 that would force a jump (overlapping-fill artifact)
fill -20 65 -101 20 65 -85 minecraft:air replace minecraft:smooth_quartz
# Remove any slabs placed at y=65 by the stride-decor pass
fill -20 65 -101 20 65 -85 minecraft:air replace minecraft:smooth_quartz_slab
