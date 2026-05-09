data merge entity @e[tag=motfb_bryan,limit=1] {NoAI:1b}
stopsound @a music
title @a times 10 200 30
title @a subtitle {"text":"\"...thank you, champ.\"","color":"dark_purple","italic":true}
title @a title {"text":"ENDING C — THE CONTRACT IS TORN","color":"gold","bold":true}
playsound minecraft:block.note_block.harp ambient @a ~ ~ ~ 2 0.5
schedule function motfb:ending/c_step2 80t
schedule function motfb:ending/c_step3 240t
schedule function motfb:ending/c_step4 360t
schedule function motfb:ending/credits 440t
