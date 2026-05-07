execute as @a[x=-50,y=60,z=-230,dx=44,dy=20,dz=14] run tag @s add in_cinnabog
execute as @a[x=-50,y=60,z=-230,dx=44,dy=20,dz=14] run tag @s add in_active_store
fill -6 62 -230 -6 79 -216 minecraft:bedrock
summon witch -28 65 -223 {Tags:["motfb_boss","motfb_candywitch_boss"],CustomName:'{"text":"The Candy Witch of Cinnabog","color":"light_purple"}',CustomNameVisible:1b,Health:90.0f,Attributes:[{Name:"minecraft:max_health",Base:90},{Name:"minecraft:movement_speed",Base:0.22}],PersistenceRequired:1b}
bossbar add motfb:candywitch {"text":"The Candy Witch of Cinnabog","color":"dark_purple"}
bossbar set motfb:candywitch players @a[tag=in_cinnabog]
bossbar set motfb:candywitch max 90
bossbar set motfb:candywitch value 90
bossbar set motfb:candywitch color purple
bossbar set motfb:candywitch style notched_10
playsound minecraft:entity.witch.ambient hostile @a ~ ~ ~ 1 0.8
title @a[tag=in_cinnabog] times 10 60 20
title @a[tag=in_cinnabog] subtitle {"text":"SWEETER THAN SIN","color":"light_purple"}
title @a[tag=in_cinnabog] title {"text":"THE CANDY WITCH","color":"dark_purple","bold":true}
tellraw @a [{"text":"[PA] ","color":"gray","italic":true},{"text":"Cinnabog special: the frosting is complimentary. Everything else costs a piece of you.","color":"gold","italic":true}]
