scoreboard players add #party mall.coupons 1

give @a[tag=in_active_store] minecraft:paper[custom_name='{"text":"BOSS COUPON","color":"gold","italic":false}',lore=['{"text":"Liminal Lakes Mall","color":"gray","italic":false}','{"text":"Authorized signature on file","color":"dark_gray","italic":true}']] 1

playsound minecraft:block.note_block.chime ambient @a ~ ~ ~ 1 1.4

execute as @a[tag=in_active_store] at @s run title @s times 10 60 20
execute as @a[tag=in_active_store] at @s run title @s subtitle {"text":"+1 Coupon","color":"yellow"}
execute as @a[tag=in_active_store] at @s run title @s title {"text":"BOSS DEFEATED","color":"gold"}

execute if score #party mall.coupons matches 9.. run function motfb:contract/unlock_office
