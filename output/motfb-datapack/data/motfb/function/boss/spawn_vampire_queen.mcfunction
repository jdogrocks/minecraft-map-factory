kill @e[tag=motfb_vampirequeen_boss]
execute as @a[x=-50,y=60,z=-200,dx=44,dy=20,dz=14] run tag @s add in_hottopical
execute as @a[x=-50,y=60,z=-200,dx=44,dy=20,dz=14] run tag @s add in_active_store
fill -6 62 -200 -6 79 -186 minecraft:bedrock
summon wither_skeleton -28 65 -193 {Tags:["motfb_boss","motfb_vampirequeen_boss"],CustomName:'{"text":"The Vampire Queen of Hot-Topical","color":"dark_red"}',CustomNameVisible:1b,Health:100.0f,Attributes:[{Name:"minecraft:max_health",Base:100},{Name:"minecraft:attack_damage",Base:7},{Name:"minecraft:armor",Base:4},{Name:"minecraft:movement_speed",Base:0.28}],HandItems:[{id:"minecraft:stone_sword",Count:1,components:{"minecraft:enchantments":{"levels":{"minecraft:fire_aspect":1}}}},{}],PersistenceRequired:1b}
bossbar add motfb:vampirequeen {"text":"The Vampire Queen — Hot-Topical","color":"red"}
bossbar set motfb:vampirequeen players @a[tag=in_hottopical]
bossbar set motfb:vampirequeen max 100
bossbar set motfb:vampirequeen value 100
bossbar set motfb:vampirequeen color red
bossbar set motfb:vampirequeen style notched_10
playsound minecraft:entity.wither_skeleton.ambient hostile @a ~ ~ ~ 1 0.9
title @a[tag=in_hottopical] times 10 60 20
title @a[tag=in_hottopical] subtitle {"text":"QUEEN OF HOT-TOPICAL","color":"dark_red"}
title @a[tag=in_hottopical] title {"text":"THE VAMPIRE QUEEN","color":"red","bold":true}
tellraw @a [{"text":"[PA] ","color":"gray","italic":true},{"text":"Hot-Topical clearance event — all darkness, half off. The darkness costs extra.","color":"gold","italic":true}]
