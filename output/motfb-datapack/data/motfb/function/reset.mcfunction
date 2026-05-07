# --- score reset ---
scoreboard players set #party mall.coupons 0
scoreboard players set #party mall.journals 0
scoreboard players set #party mall.ending 0
scoreboard players set #party mall.bryan_phase 0
scoreboard players reset * mall.flag
scoreboard players reset @a mall.contract_use
scoreboard players reset @a mall.shears_use
scoreboard players reset @a mall.pa_cooldown
scoreboard players reset @a mall.lk_cooldown
scoreboard players reset @a mall.deaths

# --- tag reset (per-player) ---
tag @a remove has_contract
tag @a remove gave_contract
tag @a remove in_office
tag @a remove lk_following
tag @a remove journal1_found
tag @a remove journal2_found
tag @a remove journal3_found
tag @a remove in_kraw
tag @a remove in_searz
tag @a remove in_pixellich
tag @a remove in_cinnabog
tag @a remove in_buildaboss
tag @a remove in_hottopical
tag @a remove in_spencers
tag @a remove in_bathbody
tag @a remove in_pretzel
tag @a remove in_spunky

# --- entity cleanup ---
kill @e[tag=motfb_boss]
kill @e[tag=motfb_bryan]
kill @e[tag=motfb_lostkid]
kill @e[type=arrow,tag=motfb_bryan_attack]

# --- inventory cleanup ---
clear @a minecraft:paper[custom_name='{"text":"BOSS COUPON","color":"gold","italic":false}']
clear @a minecraft:carrot_on_a_stick[custom_model_data={floats:[1001.0f]}]

# --- restore store doors (remove bedrock seals) ---
fill -6 62 -260 -6 79 -246 minecraft:air replace minecraft:bedrock
fill 6 62 -260 6 79 -246 minecraft:air replace minecraft:bedrock
fill -6 62 -245 -6 79 -231 minecraft:air replace minecraft:bedrock
fill 6 62 -245 6 79 -231 minecraft:air replace minecraft:bedrock
fill -6 62 -230 -6 79 -216 minecraft:air replace minecraft:bedrock
fill 6 62 -230 6 79 -216 minecraft:air replace minecraft:bedrock
fill -6 62 -215 -6 79 -201 minecraft:air replace minecraft:bedrock
fill 6 62 -215 6 79 -201 minecraft:air replace minecraft:bedrock
fill -6 62 -200 -6 79 -186 minecraft:air replace minecraft:bedrock
fill -50 62 -260 50 79 -248 minecraft:air replace minecraft:bedrock

# --- restore escalator gate ---
fill -1 65 -230 1 79 -228 minecraft:bedrock replace minecraft:air

# --- restore storefronts from backup regions ---
clone -50 -37 -260 -6 -21 -246 -50 62 -260
clone 6 -37 -260 50 -21 -246 6 62 -260
clone -50 -37 -245 -6 -21 -231 -50 62 -245
clone 6 -37 -245 50 -21 -231 6 62 -245
clone -50 -37 -230 -6 -21 -216 -50 62 -230
clone 6 -37 -230 50 -21 -216 6 62 -230
clone -50 -37 -215 -6 -21 -201 -50 62 -215
clone 6 -37 -215 50 -21 -201 6 62 -215
clone -50 -37 -200 -6 -21 -186 -50 62 -200
clone -50 -37 -280 50 -21 -261 -50 62 -280

# --- restore office from backup ---
clone -12 -3 -220 12 11 -200 -12 97 -220

# --- re-summon static NPCs ---
function motfb:lostkid/spawn_at_arcade
function motfb:bryan/spawn_at_office

# --- teleport players to spawn ---
tp @a 0 65 -150

tellraw @a {"text":"The mall is open. Welcome, welcome, welcome.","color":"gold","italic":true}
