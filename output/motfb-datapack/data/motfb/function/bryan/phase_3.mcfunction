scoreboard players set #party mall.bryan_phase 3
data merge entity @e[tag=motfb_bryan,limit=1] {NoAI:1b,Health:1.0f}
title @a times 10 100 30
title @a subtitle {"text":"\"...that's enough, sport.\"","color":"dark_purple","italic":true}
title @a title {"text":"THE ARCHITECT FALLS","color":"dark_red","bold":true}
playsound minecraft:entity.lightning_bolt.impact weather @a ~ ~ ~ 3 0.5
function motfb:ending/b_bosses_depart
function motfb:lostkid/wave_goodbye
schedule function motfb:utils/teleport_escape 200t
schedule function motfb:bryan/architect_collapse 300t
schedule function motfb:ending/credits 400t
