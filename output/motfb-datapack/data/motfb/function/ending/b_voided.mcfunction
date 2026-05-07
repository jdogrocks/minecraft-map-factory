scoreboard players set #party mall.bryan_phase 1
data merge entity @e[tag=motfb_bryan,limit=1] {Invulnerable:0b,NoAI:0b}
bossbar add motfb:bryan {"text":"Bryan — Mall Manager","color":"dark_purple"}
bossbar set motfb:bryan players @a[tag=in_office]
bossbar set motfb:bryan max 99
bossbar set motfb:bryan value 99
bossbar set motfb:bryan color purple
bossbar set motfb:bryan style notched_6
title @a times 10 60 20
title @a subtitle {"text":"\"This is really disappointing, sport.\"","color":"dark_purple","italic":true}
title @a title {"text":"ENDING B — THE VOID CONTRACT","color":"red","bold":true}
playsound minecraft:entity.wither.spawn hostile @a ~ ~ ~ 2 0.8
