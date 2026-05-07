execute as @a[x=6,y=60,z=-230,dx=44,dy=20,dz=14] run tag @s add in_pretzel
execute as @a[x=6,y=60,z=-230,dx=44,dy=20,dz=14] run tag @s add in_active_store
fill 6 62 -230 6 79 -216 minecraft:bedrock
summon iron_golem 28 65 -223 {Tags:["motfb_boss","motfb_knotgod_boss"],CustomName:'{"text":"Janice, the Knot God","color":"gold"}',CustomNameVisible:1b,Health:140.0f,Attributes:[{Name:"minecraft:max_health",Base:140},{Name:"minecraft:attack_damage",Base:9},{Name:"minecraft:movement_speed",Base:0.18}],PersistenceRequired:1b}
bossbar add motfb:knotgod {"text":"Janice — The Knot God of Pretzel-Pretzel","color":"gold"}
bossbar set motfb:knotgod players @a[tag=in_pretzel]
bossbar set motfb:knotgod max 140
bossbar set motfb:knotgod value 140
bossbar set motfb:knotgod color yellow
bossbar set motfb:knotgod style notched_10
playsound minecraft:entity.iron_golem.attack hostile @a ~ ~ ~ 1 0.7
title @a[tag=in_pretzel] times 10 60 20
title @a[tag=in_pretzel] subtitle {"text":"THE KNOT GOD OF PRETZEL-PRETZEL PRETZEL","color":"gold"}
title @a[tag=in_pretzel] title {"text":"JANICE","color":"yellow","bold":true}
tellraw @a [{"text":"[PA] ","color":"gray","italic":true},{"text":"Pretzel-Pretzel Pretzel: the pretzels are free. Janice is not.","color":"gold","italic":true}]
