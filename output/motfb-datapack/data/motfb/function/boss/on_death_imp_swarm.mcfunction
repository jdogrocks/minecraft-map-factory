execute if entity @e[tag=motfb_imp_2] run return fail
execute if entity @e[tag=motfb_imp_3] run return fail
execute if entity @e[tag=motfb_imp_4] run return fail
execute if entity @e[tag=motfb_imp_5] run return fail
scoreboard players set #spencers_killed mall.flag 1
fill 6 62 -260 6 79 -246 minecraft:air replace minecraft:bedrock
tag @a[tag=in_spencers] remove in_active_store
function motfb:utils/give_coupon
bossbar remove motfb:impswarm
tag @a[tag=in_spencers] remove in_spencers
tellraw @a [{"text":"[PA] ","color":"gray","italic":true},{"text":"Spencer's is closed for renovation. All five imps have been returned to the gift bags.","color":"gold","italic":true}]
playsound minecraft:block.note_block.chime ambient @a ~ ~ ~ 1 1.2
