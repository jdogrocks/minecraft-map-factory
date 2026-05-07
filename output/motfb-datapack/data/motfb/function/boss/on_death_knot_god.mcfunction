scoreboard players set #pretzel_killed mall.flag 1
fill 6 62 -230 6 79 -216 minecraft:air replace minecraft:bedrock
tag @a[tag=in_pretzel] remove in_active_store
function motfb:utils/give_coupon
bossbar remove motfb:knotgod
tag @a[tag=in_pretzel] remove in_pretzel
tellraw @a [{"text":"[PA] ","color":"gray","italic":true},{"text":"Pretzel-Pretzel is closed for renovation. Janice is resting. Do not disturb Janice.","color":"gold","italic":true}]
playsound minecraft:block.note_block.chime ambient @a ~ ~ ~ 1 1.2
