scoreboard players set #bathbody_killed mall.flag 1
fill 6 62 -245 6 79 -231 minecraft:air replace minecraft:bedrock
tag @a[tag=in_bathbody] remove in_active_store
function motfb:utils/give_coupon
bossbar remove motfb:exiledsaint
tag @a[tag=in_bathbody] remove in_bathbody
tellraw @a [{"text":"[PA] ","color":"gray","italic":true},{"text":"Bath and Bodywork is closed for renovation. The sanctum is at peace.","color":"gold","italic":true}]
playsound minecraft:block.note_block.chime ambient @a ~ ~ ~ 1 1.2
