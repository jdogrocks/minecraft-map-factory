execute as @a[x=6,y=60,z=-260,dx=44,dy=20,dz=14] run tag @s add in_spencers
execute as @a[x=6,y=60,z=-260,dx=44,dy=20,dz=14] run tag @s add in_active_store
fill 6 62 -260 6 79 -246 minecraft:bedrock
summon vex 22 67 -253 {Tags:["motfb_boss","motfb_impswarm_boss","motfb_imp_1"],CustomName:'{"text":"Imp Swarm [1/5]","color":"aqua"}',CustomNameVisible:1b,Health:25.0f,Attributes:[{Name:"minecraft:max_health",Base:25}],Lifetime:-1,PersistenceRequired:1b}
summon vex 28 69 -253 {Tags:["motfb_boss","motfb_imp_2"],CustomName:'{"text":"Imp Swarm [2/5]","color":"aqua"}',CustomNameVisible:1b,Health:25.0f,Attributes:[{Name:"minecraft:max_health",Base:25}],Lifetime:-1,PersistenceRequired:1b}
summon vex 22 71 -256 {Tags:["motfb_boss","motfb_imp_3"],CustomName:'{"text":"Imp Swarm [3/5]","color":"aqua"}',CustomNameVisible:1b,Health:25.0f,Attributes:[{Name:"minecraft:max_health",Base:25}],Lifetime:-1,PersistenceRequired:1b}
summon vex 28 67 -256 {Tags:["motfb_boss","motfb_imp_4"],CustomName:'{"text":"Imp Swarm [4/5]","color":"aqua"}',CustomNameVisible:1b,Health:25.0f,Attributes:[{Name:"minecraft:max_health",Base:25}],Lifetime:-1,PersistenceRequired:1b}
summon vex 25 69 -250 {Tags:["motfb_boss","motfb_imp_5"],CustomName:'{"text":"Imp Swarm [5/5]","color":"aqua"}',CustomNameVisible:1b,Health:25.0f,Attributes:[{Name:"minecraft:max_health",Base:25}],Lifetime:-1,PersistenceRequired:1b}
bossbar add motfb:impswarm {"text":"Imp Swarm — Spencer's Cursed Gifts","color":"blue"}
bossbar set motfb:impswarm players @a[tag=in_spencers]
bossbar set motfb:impswarm max 125
bossbar set motfb:impswarm value 125
bossbar set motfb:impswarm color blue
bossbar set motfb:impswarm style notched_10
playsound minecraft:entity.vex.ambient hostile @a ~ ~ ~ 1 1.2
title @a[tag=in_spencers] times 10 60 20
title @a[tag=in_spencers] subtitle {"text":"CURSED GIFTS — WHILE SUPPLIES LAST","color":"aqua"}
title @a[tag=in_spencers] title {"text":"IMP SWARM","color":"blue","bold":true}
tellraw @a [{"text":"[PA] ","color":"gray","italic":true},{"text":"Spencer's Cursed Gifts is having a five-for-one special. All of them are angry.","color":"gold","italic":true}]
