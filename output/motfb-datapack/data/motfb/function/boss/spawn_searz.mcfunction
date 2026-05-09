kill @e[tag=motfb_searz_boss]
execute as @a[x=-50,y=60,z=-280,dx=100,dy=20,dz=19] run tag @s add in_searz
execute as @a[x=-50,y=60,z=-280,dx=100,dy=20,dz=19] run tag @s add in_active_store
fill -50 62 -261 50 79 -261 minecraft:bedrock
summon wither 0 68 -270 {Tags:["motfb_boss","motfb_searz_boss"],CustomName:'{"text":"Mama SEARZ, Forsaken Department Goddess (Floor 1)","color":"dark_red"}',CustomNameVisible:1b,Health:100.0f,Attributes:[{Name:"minecraft:max_health",Base:100},{Name:"minecraft:movement_speed",Base:0.1}],Invulnerable:0b,PersistenceRequired:1b,NoAI:0b}
bossbar add motfb:searz {"text":"Mama SEARZ — Forsaken Department Goddess","color":"red"}
bossbar set motfb:searz players @a[tag=in_searz]
bossbar set motfb:searz max 300
bossbar set motfb:searz value 300
bossbar set motfb:searz color red
bossbar set motfb:searz style notched_10
playsound minecraft:entity.wither.spawn hostile @a ~ ~ ~ 2 0.5
title @a[tag=in_searz] times 10 60 20
title @a[tag=in_searz] subtitle {"text":"FORSAKEN DEPARTMENT GODDESS","color":"dark_red"}
title @a[tag=in_searz] title {"text":"MAMA SEARZ","color":"red","bold":true}
tellraw @a [{"text":"[PA] ","color":"gray","italic":true},{"text":"SEARZ is having a clearance event. All floors. All departments. She is all departments.","color":"gold","italic":true}]
