scoreboard players set #kraw_killed mall.flag 1
fill -6 62 -260 -6 79 -246 minecraft:air replace minecraft:bedrock
tag @a[tag=in_kraw] remove in_active_store
function motfb:utils/give_coupon
bossbar remove motfb:kraw
tag @a[tag=in_kraw] remove in_kraw
tellraw @a [{"text":"[PA] ","color":"gray","italic":true},{"text":"Cluck-O-Mart is now closed for renovation. Thank you for shopping with us, sport.","color":"gold","italic":true}]
playsound minecraft:block.note_block.chime ambient @a ~ ~ ~ 1 1.2
