scoreboard players set #hottopical_killed mall.flag 1
fill -6 62 -200 -6 79 -186 minecraft:air replace minecraft:bedrock
tag @a[tag=in_hottopical] remove in_active_store
function motfb:utils/give_coupon
bossbar remove motfb:vampirequeen
tag @a[tag=in_hottopical] remove in_hottopical
tellraw @a [{"text":"[PA] ","color":"gray","italic":true},{"text":"Hot-Topical is closed for renovation. The darkness has been renovated.","color":"gold","italic":true}]
playsound minecraft:block.note_block.chime ambient @a ~ ~ ~ 1 1.2
