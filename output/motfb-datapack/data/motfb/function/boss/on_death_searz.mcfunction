scoreboard players set #searz_killed mall.flag 1
fill -50 62 -261 50 79 -261 minecraft:air replace minecraft:bedrock
tag @a[tag=in_searz] remove in_active_store
function motfb:utils/give_coupon
bossbar remove motfb:searz
tag @a[tag=in_searz] remove in_searz
tellraw @a [{"text":"[PA] ","color":"gray","italic":true},{"text":"SEARZ is closed for renovation across all three floors and all known dimensions.","color":"gold","italic":true}]
playsound minecraft:block.note_block.chime ambient @a ~ ~ ~ 1 1.2
