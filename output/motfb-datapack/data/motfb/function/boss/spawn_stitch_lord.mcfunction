kill @e[tag=motfb_stitchlord_boss]
execute as @a[x=-50,y=60,z=-215,dx=44,dy=20,dz=14] run tag @s add in_buildaboss
execute as @a[x=-50,y=60,z=-215,dx=44,dy=20,dz=14] run tag @s add in_active_store
fill -6 62 -215 -6 79 -201 minecraft:bedrock
summon vindicator -28 65 -208 {Tags:["motfb_boss","motfb_stitchlord_boss"],CustomName:'{"text":"The Stitch Lord, Plushie Overlord","color":"yellow"}',CustomNameVisible:1b,Health:100.0f,Attributes:[{Name:"minecraft:max_health",Base:100},{Name:"minecraft:attack_damage",Base:7},{Name:"minecraft:armor",Base:2},{Name:"minecraft:movement_speed",Base:0.3}],HandItems:[{id:"minecraft:iron_axe",Count:1,components:{"minecraft:enchantments":{"levels":{"minecraft:sharpness":2}}}},{}],PersistenceRequired:1b}
bossbar add motfb:stitchlord {"text":"The Stitch Lord — Plushie Overlord","color":"yellow"}
bossbar set motfb:stitchlord players @a[tag=in_buildaboss]
bossbar set motfb:stitchlord max 100
bossbar set motfb:stitchlord value 100
bossbar set motfb:stitchlord color yellow
bossbar set motfb:stitchlord style notched_10
playsound minecraft:entity.vindicator.ambient hostile @a ~ ~ ~ 1 0.9
title @a[tag=in_buildaboss] times 10 60 20
title @a[tag=in_buildaboss] subtitle {"text":"PLUSHIE OVERLORD","color":"yellow"}
title @a[tag=in_buildaboss] title {"text":"THE STITCH LORD","color":"gold","bold":true}
tellraw @a [{"text":"[PA] ","color":"gray","italic":true},{"text":"Build-A-Boss — customize your doom. Choose wisely. They chose wisely.","color":"gold","italic":true}]
