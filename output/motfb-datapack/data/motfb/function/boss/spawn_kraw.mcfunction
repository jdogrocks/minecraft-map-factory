execute as @a[x=-50,y=60,z=-260,dx=44,dy=20,dz=14] run tag @s add in_kraw
execute as @a[x=-50,y=60,z=-260,dx=44,dy=20,dz=14] run tag @s add in_active_store
fill -6 62 -260 -6 79 -246 minecraft:bedrock
summon ghast -28 72 -253 {Tags:["motfb_boss","motfb_kraw_boss"],CustomName:'{"text":"Colonel Kraw, Wyvern Tyrant of the Drive-Thru","color":"red"}',CustomNameVisible:1b,Health:120.0f,Attributes:[{Name:"minecraft:max_health",Base:120},{Name:"minecraft:movement_speed",Base:0.06}],PersistenceRequired:1b}
bossbar add motfb:kraw {"text":"Colonel Kraw — Wyvern Tyrant of the Drive-Thru","color":"red"}
bossbar set motfb:kraw players @a[tag=in_kraw]
bossbar set motfb:kraw max 120
bossbar set motfb:kraw value 120
bossbar set motfb:kraw color red
bossbar set motfb:kraw style notched_10
playsound minecraft:entity.ghast.warn hostile @a ~ ~ ~ 1 0.6
title @a[tag=in_kraw] times 10 60 20
title @a[tag=in_kraw] subtitle {"text":"WYVERN TYRANT OF THE DRIVE-THRU","color":"yellow"}
title @a[tag=in_kraw] title {"text":"COLONEL KRAW","color":"red","bold":true}
tellraw @a [{"text":"[PA] ","color":"gray","italic":true},{"text":"BOGO Cursed Nuggets! Limit one hero per coupon.","color":"gold","italic":true}]
