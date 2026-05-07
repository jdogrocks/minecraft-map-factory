tellraw @a [{"text":"The Lost Kid: ","color":"yellow","bold":true},{"text":"\"Tell my mom I said hi. Also tell her I figured it out. She'll know what that means.\"","color":"white","italic":true}]
playsound minecraft:entity.villager.yes ambient @a
schedule function motfb:lostkid/despawn 80t
