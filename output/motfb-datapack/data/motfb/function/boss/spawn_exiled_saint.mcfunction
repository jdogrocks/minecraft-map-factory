execute as @a[x=6,y=60,z=-245,dx=44,dy=20,dz=14] run tag @s add in_bathbody
execute as @a[x=6,y=60,z=-245,dx=44,dy=20,dz=14] run tag @s add in_active_store
fill 6 62 -245 6 79 -231 minecraft:bedrock
summon evoker 28 65 -238 {Tags:["motfb_boss","motfb_exiledsaint_boss"],CustomName:'{"text":"The Exiled Saint of Bath & Bodywork","color":"white"}',CustomNameVisible:1b,Health:100.0f,Attributes:[{Name:"minecraft:max_health",Base:100},{Name:"minecraft:movement_speed",Base:0.25}],PersistenceRequired:1b}
bossbar add motfb:exiledsaint {"text":"The Exiled Saint — Bath & Bodywork Sanctum","color":"white"}
bossbar set motfb:exiledsaint players @a[tag=in_bathbody]
bossbar set motfb:exiledsaint max 100
bossbar set motfb:exiledsaint value 100
bossbar set motfb:exiledsaint color white
bossbar set motfb:exiledsaint style notched_10
playsound minecraft:entity.evoker.ambient hostile @a ~ ~ ~ 1 0.9
title @a[tag=in_bathbody] times 10 60 20
title @a[tag=in_bathbody] subtitle {"text":"EXILED SAINT OF BATH & BODYWORK","color":"white"}
title @a[tag=in_bathbody] title {"text":"THE EXILED SAINT","color":"white","bold":true}
tellraw @a [{"text":"[PA] ","color":"gray","italic":true},{"text":"Bath and Bodywork Sanctum: all scents are complimentary. The vexes are not.","color":"gold","italic":true}]
