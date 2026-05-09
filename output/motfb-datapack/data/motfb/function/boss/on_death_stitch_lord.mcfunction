scoreboard players set #buildaboss_killed mall.flag 1
fill -6 62 -215 -6 79 -201 minecraft:air replace minecraft:bedrock
tag @a[tag=in_buildaboss] remove in_active_store
function motfb:utils/give_coupon
bossbar remove motfb:stitchlord
tag @a[tag=in_buildaboss] remove in_buildaboss
tellraw @a [{"text":"[PA] ","color":"gray","italic":true},{"text":"Build-A-Boss is closed for renovation. The workshop is at peace.","color":"gold","italic":true}]
playsound minecraft:block.note_block.chime ambient @a ~ ~ ~ 1 1.2
