tag @s add lk_following
clear @s minecraft:bread[custom_name='{"text":"Mall Pretzel","color":"yellow"}'] 1
tellraw @a [{"text":"The Lost Kid: ","color":"yellow","bold":true},{"text":"\"Sick. You're a real one. Lead the way, lowkey.\"","color":"white","italic":true}]
playsound minecraft:entity.villager.yes neutral @s
function motfb:lostkid/line_arcade
