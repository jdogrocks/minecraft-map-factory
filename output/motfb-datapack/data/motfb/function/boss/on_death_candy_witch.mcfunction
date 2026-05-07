scoreboard players set #cinnabog_killed mall.flag 1
fill -6 62 -230 -6 79 -216 minecraft:air replace minecraft:bedrock
tag @a[tag=in_cinnabog] remove in_active_store
function motfb:utils/give_coupon
bossbar remove motfb:candywitch
tag @a[tag=in_cinnabog] remove in_cinnabog
tellraw @a [{"text":"[PA] ","color":"gray","italic":true},{"text":"Cinnabog is closed for renovation. The frosting has been contained.","color":"gold","italic":true}]
playsound minecraft:block.note_block.chime ambient @a ~ ~ ~ 1 1.2
