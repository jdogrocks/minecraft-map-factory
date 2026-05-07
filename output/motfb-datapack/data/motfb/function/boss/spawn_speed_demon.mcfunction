execute as @a[x=6,y=60,z=-215,dx=44,dy=20,dz=14] run tag @s add in_spunky
execute as @a[x=6,y=60,z=-215,dx=44,dy=20,dz=14] run tag @s add in_active_store
fill 6 62 -215 6 79 -201 minecraft:bedrock
summon vindicator 28 65 -208 {Tags:["motfb_boss","motfb_speeddemon_boss"],CustomName:'{"text":"The Speed Demon, Floor Manager of Spunkys","color":"aqua"}',CustomNameVisible:1b,Health:70.0f,Attributes:[{Name:"minecraft:max_health",Base:70},{Name:"minecraft:attack_damage",Base:6},{Name:"minecraft:movement_speed",Base:0.5}],HandItems:[{id:"minecraft:iron_sword",Count:1},{}],PersistenceRequired:1b}
effect give @e[tag=motfb_speeddemon_boss] minecraft:speed 999999 2 true
bossbar add motfb:speeddemon {"text":"The Speed Demon - Spunky's Sneakers","color":"aqua"}
bossbar set motfb:speeddemon players @a[tag=in_spunky]
bossbar set motfb:speeddemon max 70
bossbar set motfb:speeddemon value 70
bossbar set motfb:speeddemon color blue
bossbar set motfb:speeddemon style notched_6
playsound minecraft:entity.vindicator.ambient hostile @a ~ ~ ~ 1 1.5
title @a[tag=in_spunky] times 10 60 20
title @a[tag=in_spunky] subtitle {"text":"FLOOR MANAGER OF SPUNKYS SNEAKERS","color":"aqua"}
title @a[tag=in_spunky] title {"text":"THE SPEED DEMON","color":"blue","bold":true}
tellraw @a [{"text":"[PA] ","color":"gray","italic":true},{"text":"Spunkys Sneakers: these shoes were made for... this, specifically.","color":"gold","italic":true}]
