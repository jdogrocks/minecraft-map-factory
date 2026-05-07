execute as @a[tag=lk_following,limit=1] at @s as @e[tag=motfb_lostkid,limit=1] at @s if entity @p[tag=lk_following,distance=8..] run tp @s @p[tag=lk_following,limit=1]
execute as @a[tag=lk_following] at @s unless entity @s[x=-52,y=60,z=-282,dx=104,dy=60,dz=184] run tag @s remove lk_following
execute as @a[tag=lk_following] at @s unless entity @s[x=-52,y=60,z=-282,dx=104,dy=60,dz=184] run tellraw @s [{"text":"The Lost Kid: ","color":"yellow","bold":true},{"text":"\"Nah dude, I'm staying. Mall hours forever, lowkey.\"","color":"white","italic":true}]
execute as @a[tag=lk_following] if score @s mall.lk_cooldown matches ..0 if score #party mall.coupons matches 5..9 run function motfb:lostkid/line_bryan
execute as @a[tag=lk_following] if score @s mall.lk_cooldown matches ..0 if score #searz_killed mall.flag matches 1 run function motfb:lostkid/line_searz_post
