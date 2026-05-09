execute if score #unlock_fired mall.flag matches 1.. run return fail
scoreboard players set #unlock_fired mall.flag 1
fill -1 65 -230 1 79 -228 minecraft:air replace minecraft:bedrock
function motfb:pa/job_offer
setblock 0 81 -240 minecraft:redstone_lamp[lit=true]
setblock 0 84 -237 minecraft:redstone_lamp[lit=true]
setblock 0 88 -234 minecraft:redstone_lamp[lit=true]
setblock 0 92 -231 minecraft:redstone_lamp[lit=true]
setblock 0 96 -228 minecraft:redstone_lamp[lit=true]
playsound minecraft:block.beacon.activate ambient @a ~ ~ ~ 2 1
