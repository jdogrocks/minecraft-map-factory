execute as @s at @s unless block ~ ~-1 ~ minecraft:lectern run return fail
execute as @s unless score #party mall.ending matches 0 run return fail
scoreboard players set #party mall.ending 1
function motfb:ending/a_honored
