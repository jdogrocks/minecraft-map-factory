execute as @a[x=-50,y=60,z=-245,dx=44,dy=20,dz=14] run tag @s add in_pixellich
execute as @a[x=-50,y=60,z=-245,dx=44,dy=20,dz=14] run tag @s add in_active_store
fill -6 62 -245 -6 79 -231 minecraft:bedrock
summon husk -28 65 -238 {Tags:["motfb_boss","motfb_pixellich_boss"],CustomName:'{"text":"The Pixel Lich, Champion of the Loading Screen","color":"dark_green"}',CustomNameVisible:1b,Health:80.0f,Attributes:[{Name:"minecraft:max_health",Base:80},{Name:"minecraft:attack_damage",Base:5},{Name:"minecraft:armor",Base:4},{Name:"minecraft:movement_speed",Base:0.26}],ArmorItems:[{id:"minecraft:chainmail_boots",Count:1},{id:"minecraft:chainmail_leggings",Count:1},{id:"minecraft:chainmail_chestplate",Count:1},{id:"minecraft:chainmail_helmet",Count:1}],HandItems:[{id:"minecraft:stone_sword",Count:1,components:{"minecraft:enchantments":{"levels":{"minecraft:sharpness":1}}}},{}],PersistenceRequired:1b}
bossbar add motfb:pixellich {"text":"The Pixel Lich — Champion of the Loading Screen","color":"green"}
bossbar set motfb:pixellich players @a[tag=in_pixellich]
bossbar set motfb:pixellich max 80
bossbar set motfb:pixellich value 80
bossbar set motfb:pixellich color green
bossbar set motfb:pixellich style notched_10
playsound minecraft:entity.husk.ambient hostile @a ~ ~ ~ 1 0.7
title @a[tag=in_pixellich] times 10 60 20
title @a[tag=in_pixellich] subtitle {"text":"CHAMPION OF THE LOADING SCREEN","color":"green"}
title @a[tag=in_pixellich] title {"text":"THE PIXEL LICH","color":"dark_green","bold":true}
tellraw @a [{"text":"[PA] ","color":"gray","italic":true},{"text":"Attention — GameStomp is now running a midnight tournament. Participation is not optional.","color":"gold","italic":true}]
