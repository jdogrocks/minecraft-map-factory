execute as @s at @s unless block ~ ~-1 ~ minecraft:redstone_block run return fail
execute as @s unless score #party mall.ending matches 0 run return fail
execute if score #party mall.journals matches ..2 run tellraw @s [{"text":"[PA] ","color":"gray"},{"text":"Now sport, that's not how we do things at Liminal Lakes. You haven't read the fine print.","color":"dark_purple","italic":true}]
execute if score #party mall.journals matches ..2 run playsound minecraft:block.note_block.didgeridoo ambient @s ~ ~ ~ 1 0.8
execute if score #party mall.journals matches ..2 run return fail
scoreboard players set #party mall.ending 3
clear @s minecraft:carrot_on_a_stick[custom_model_data={floats:[1001.0f]}] 1
particle minecraft:explosion ~ ~1 ~ 0.3 0.3 0.3 0.05 20
playsound minecraft:entity.item.break ambient @a
function motfb:ending/c_annulled
