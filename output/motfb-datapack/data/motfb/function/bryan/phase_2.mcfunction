scoreboard players set #party mall.bryan_phase 2
execute as @a run effect give @s minecraft:darkness 5 0 true
weather thunder 30
playsound minecraft:entity.lightning_bolt.thunder weather @a ~ ~ ~ 4 0.5
title @a times 10 60 20
title @a subtitle {"text":"Do you have ANY IDEA how tired I am?","color":"red","italic":true}
title @a title {"text":"THE ARCHITECT OF ALL ENDINGS","color":"dark_red","bold":true}
execute as @e[tag=motfb_bryan_phase1,limit=1] at @s run summon wither_skeleton ~ ~ ~ {Tags:["motfb_bryan","motfb_bryan_phase2"],CustomName:'{"text":"The Architect of All Endings","color":"dark_red","bold":true}',CustomNameVisible:1b,Health:66.0f,Attributes:[{Name:"minecraft:max_health",Base:66},{Name:"minecraft:attack_damage",Base:9},{Name:"minecraft:armor",Base:6},{Name:"minecraft:movement_speed",Base:0.32}],HandItems:[{id:"minecraft:netherite_sword",Count:1,components:{"minecraft:enchantments":{"levels":{"minecraft:sharpness":3}}}},{}],ArmorItems:[{id:"minecraft:netherite_boots",Count:1},{id:"minecraft:netherite_leggings",Count:1},{id:"minecraft:netherite_chestplate",Count:1},{id:"minecraft:netherite_helmet",Count:1}],PersistenceRequired:1b}
kill @e[tag=motfb_bryan_phase1]
bossbar remove motfb:bryan
bossbar add motfb:bryan {"text":"The Architect of All Endings","color":"red"}
bossbar set motfb:bryan players @a[tag=in_office]
bossbar set motfb:bryan max 66
bossbar set motfb:bryan value 66
bossbar set motfb:bryan color red
bossbar set motfb:bryan style notched_6
schedule function motfb:bryan/monologue 40t
