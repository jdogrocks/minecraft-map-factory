# =============================================================================
# Boss entity init: summon all 10 bosses in display mode (NoAI, Invulnerable)
# so they are visible in their storefronts on world load.
# The actual fight sequence (arena seal, bossbar, sounds) fires from each
# spawn_<boss>.mcfunction when a player steps into the storefront zone.
# =============================================================================

kill @e[tag=motfb_boss]

# --- West side stores ---

# Cluck-O-Mart (z=-260..-246): Colonel Kraw — Ghast
summon ghast -28 72 -253 {Tags:["motfb_boss","motfb_kraw_boss"],CustomName:'{"text":"Colonel Kraw, Wyvern Tyrant of the Drive-Thru","color":"red"}',CustomNameVisible:1b,Health:120.0f,Attributes:[{Name:"minecraft:max_health",Base:120},{Name:"minecraft:movement_speed",Base:0.06}],PersistenceRequired:1b,NoAI:1b,Invulnerable:1b,Silent:1b}

# SEARZ dept store (z=-261..-280): Mama SEARZ — Wither (Silent to suppress spawn boom)
summon wither 0 68 -270 {Tags:["motfb_boss","motfb_searz_boss"],CustomName:'{"text":"Mama SEARZ, Forsaken Department Goddess","color":"dark_red"}',CustomNameVisible:1b,Health:100.0f,Attributes:[{Name:"minecraft:max_health",Base:100},{Name:"minecraft:movement_speed",Base:0.1}],Invulnerable:1b,PersistenceRequired:1b,NoAI:1b,Silent:1b}

# GameZone (z=-245..-231): The Pixel Lich — Husk
summon husk -28 65 -238 {Tags:["motfb_boss","motfb_pixellich_boss"],CustomName:'{"text":"The Pixel Lich, Champion of the Loading Screen","color":"dark_green"}',CustomNameVisible:1b,Health:80.0f,Attributes:[{Name:"minecraft:max_health",Base:80},{Name:"minecraft:movement_speed",Base:0.26}],PersistenceRequired:1b,NoAI:1b,Invulnerable:1b,Silent:1b}

# Cinnabog (z=-230..-216): The Candy Witch — Witch
summon witch -28 65 -223 {Tags:["motfb_boss","motfb_candywitch_boss"],CustomName:'{"text":"The Candy Witch of Cinnabog","color":"light_purple"}',CustomNameVisible:1b,Health:90.0f,Attributes:[{Name:"minecraft:max_health",Base:90},{Name:"minecraft:movement_speed",Base:0.22}],PersistenceRequired:1b,NoAI:1b,Invulnerable:1b,Silent:1b}

# Build-A-Boss (z=-215..-201): The Stitch Lord — Vindicator
summon vindicator -28 65 -208 {Tags:["motfb_boss","motfb_stitchlord_boss"],CustomName:'{"text":"The Stitch Lord, Plushie Overlord","color":"yellow"}',CustomNameVisible:1b,Health:100.0f,Attributes:[{Name:"minecraft:max_health",Base:100},{Name:"minecraft:movement_speed",Base:0.3}],HandItems:[{id:"minecraft:iron_axe",Count:1,components:{"minecraft:enchantments":{"levels":{"minecraft:sharpness":2}}}},{}],PersistenceRequired:1b,NoAI:1b,Invulnerable:1b,Silent:1b}

# Hot-Topical (z=-200..-186): The Vampire Queen — Wither Skeleton
summon wither_skeleton -28 65 -193 {Tags:["motfb_boss","motfb_vampirequeen_boss"],CustomName:'{"text":"The Vampire Queen of Hot-Topical","color":"dark_red"}',CustomNameVisible:1b,Health:100.0f,Attributes:[{Name:"minecraft:max_health",Base:100},{Name:"minecraft:movement_speed",Base:0.28}],HandItems:[{id:"minecraft:stone_sword",Count:1,components:{"minecraft:enchantments":{"levels":{"minecraft:fire_aspect":1}}}},{}],PersistenceRequired:1b,NoAI:1b,Invulnerable:1b,Silent:1b}

# --- East side stores ---

# Spencer's (z=-260..-246): Imp Swarm — Vex x2 display representatives
summon vex 22 67 -253 {Tags:["motfb_boss","motfb_impswarm_boss","motfb_imp_1"],CustomName:'{"text":"Imp Swarm [1/5]","color":"aqua"}',CustomNameVisible:1b,Health:25.0f,Attributes:[{Name:"minecraft:max_health",Base:25}],Lifetime:-1,PersistenceRequired:1b,NoAI:1b,Invulnerable:1b,Silent:1b}
summon vex 28 67 -256 {Tags:["motfb_boss","motfb_imp_2"],CustomName:'{"text":"Imp Swarm [2/5]","color":"aqua"}',CustomNameVisible:1b,Health:25.0f,Attributes:[{Name:"minecraft:max_health",Base:25}],Lifetime:-1,PersistenceRequired:1b,NoAI:1b,Invulnerable:1b,Silent:1b}

# Bath & Body (z=-245..-231): The Exiled Saint — Evoker
summon evoker 28 65 -238 {Tags:["motfb_boss","motfb_exiledsaint_boss"],CustomName:'{"text":"The Exiled Saint of Bath & Bodywork","color":"white"}',CustomNameVisible:1b,Health:100.0f,Attributes:[{Name:"minecraft:max_health",Base:100},{Name:"minecraft:movement_speed",Base:0.25}],PersistenceRequired:1b,NoAI:1b,Invulnerable:1b,Silent:1b}

# Pretzel-Pretzel Pretzel (z=-230..-216): Janice, the Knot God — Iron Golem
summon iron_golem 28 65 -223 {Tags:["motfb_boss","motfb_knotgod_boss"],CustomName:'{"text":"Janice, the Knot God","color":"gold"}',CustomNameVisible:1b,Health:140.0f,Attributes:[{Name:"minecraft:max_health",Base:140},{Name:"minecraft:attack_damage",Base:9},{Name:"minecraft:movement_speed",Base:0.18}],PersistenceRequired:1b,NoAI:1b,Invulnerable:1b,Silent:1b}

# Spunky's Footwear (z=-215..-201): The Speed Demon — Vindicator
summon vindicator 28 65 -208 {Tags:["motfb_boss","motfb_speeddemon_boss"],CustomName:'{"text":"The Speed Demon, Floor Manager of Spunkys","color":"aqua"}',CustomNameVisible:1b,Health:70.0f,Attributes:[{Name:"minecraft:max_health",Base:70},{Name:"minecraft:movement_speed",Base:0.5}],HandItems:[{id:"minecraft:iron_sword",Count:1},{}],PersistenceRequired:1b,NoAI:1b,Invulnerable:1b,Silent:1b}
