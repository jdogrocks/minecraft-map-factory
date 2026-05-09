scoreboard players set #pixellich_killed mall.flag 1
fill -6 62 -245 -6 79 -231 minecraft:air replace minecraft:bedrock
tag @a[tag=in_pixellich] remove in_active_store
function motfb:utils/give_coupon
bossbar remove motfb:pixellich
tag @a[tag=in_pixellich] remove in_pixellich
tellraw @a [{"text":"[PA] ","color":"gray","italic":true},{"text":"GameStomp is closed for renovation. Please check your achievements. You have no achievements.","color":"gold","italic":true}]
playsound minecraft:block.note_block.chime ambient @a ~ ~ ~ 1 1.2
