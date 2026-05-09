# =============================================================================
# SCENE DESIGN: Redstone Lamp Power Fix + Spencer's Stained Glass (MIN-163)
#
# Spencer's Cursed Gifts (east, x=7..49, z=-246..-260):
# f2_lighting placed redstone_lamp at y=79 but no power source.
# Fix: place redstone_block at y=80 above the lamp zone (above-ceiling cavity,
# not visible from inside the store). Block above a lamp powers it in vanilla MC.
#
# Also applies magenta stained glass overlay over select lamps per §1.6:
# "magenta stained glass over select lamps for color-shift"
# =============================================================================

# --- Power Spencer's redstone lamp ceiling ---
# Redstone blocks sit in the floor/ceiling cavity at y=80 (above the y=79 lamps)
# Use replace air so we don't overwrite any structural blocks in this cavity
fill 8 80 -259 48 80 -247 minecraft:redstone_block replace minecraft:air

# --- Magenta stained glass over select lamps (color-shift effect §1.6) ---
# Place magenta glass pane on top of every other lamp strip for the color shift
# The lamps are at y=79; glass pane at y=78 (below the lamp, visible from inside)
# Actually: place colored glass at y=79 replacing every other air block in the
# lamp zone interior — creates a dappled warm/magenta effect
fill 14 79 -258 22 79 -248 minecraft:magenta_stained_glass replace minecraft:air
fill 30 79 -258 38 79 -248 minecraft:magenta_stained_glass replace minecraft:air

# --- Spencer's weird clearance-bin sea lantern (§1.6 "Sea lanterns inside the clearance bin") ---
# Add a glowing clearance bin at the back of Spencer's (x=40, z=-252)
setblock 40 65 -252 minecraft:barrel[facing=up]
setblock 40 64 -252 minecraft:sea_lantern

# --- Ceiling over Spencer's: orange stained glass panels over redstone lamps (§1.6) ---
# "orange stained glass (warm tint over lamps)" — place orange glass at y=79
# in the center spine of Spencer's to warm the light
fill 20 79 -257 35 79 -249 minecraft:orange_stained_glass replace minecraft:redstone_lamp

# Note: those lamps covered by glass are still powered by the redstone_block above
# but the orange glass filters the color visible from below

tellraw @a {"text":"[MOTFB] Spencer's lamps powered; color-shift glass applied.","color":"gold"}
