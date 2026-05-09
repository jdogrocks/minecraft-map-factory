kill @e[tag=motfb_lostkid]
summon villager -30 65 -238 {Tags:["motfb_lostkid"],CustomName:'{"text":"The Lost Kid","color":"yellow","italic":false}',CustomNameVisible:1b,VillagerData:{type:"plains",profession:"none",level:1},NoAI:0b,Silent:1b,Invulnerable:1b,PersistenceRequired:1b,Health:20.0f,Attributes:[{Name:"minecraft:movement_speed",Base:0.5},{Name:"minecraft:follow_range",Base:64}]}

# directional sign on concourse wall (x=-6, facing the corridor)
setblock -6 67 -238 minecraft:oak_wall_sign[facing=west]{front_text:{messages:['{"text":"THE LOST KID","color":"yellow","bold":true}','{"text":"is in the Arcade >>","color":"white"}','{"text":"Give a Mall Pretzel","color":"gold"}','{"text":"to recruit them.","color":"gold"}'],has_glowing_text:1b,color:"black"},is_waxed:1b}
