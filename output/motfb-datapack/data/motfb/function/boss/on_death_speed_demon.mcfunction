scoreboard players set #spunky_killed mall.flag 1
fill 6 62 -215 6 79 -201 minecraft:air replace minecraft:bedrock
tag @a[tag=in_spunky] remove in_active_store
function motfb:utils/give_coupon
bossbar remove motfb:speeddemon
tag @a[tag=in_spunky] remove in_spunky
tellraw @a [{"text":"[PA] ","color":"gray","italic":true},{"text":"Spunky's is closed for renovation. We have located the Speed Demon's soul in aisle 4.","color":"gold","italic":true}]
playsound minecraft:block.note_block.chime ambient @a ~ ~ ~ 1 1.2
