# --- decay PA + lost-kid cooldowns ---
execute as @a if score @s mall.pa_cooldown matches 1.. run scoreboard players remove @s mall.pa_cooldown 1
execute as @a if score @s mall.lk_cooldown matches 1.. run scoreboard players remove @s mall.lk_cooldown 1

# --- contract item use → fan out to ending handlers ---
execute as @a[tag=has_contract] if score @s mall.contract_use matches 1.. run function motfb:contract/sign
execute as @a[tag=has_contract] if score @s mall.shears_use matches 1.. run function motfb:contract/tear
scoreboard players reset @a mall.contract_use
scoreboard players reset @a mall.shears_use

# --- re-give lost contract ---
execute as @a[tag=has_contract] run execute store result score @s mall.contract_held run clear @s minecraft:carrot_on_a_stick[custom_model_data={floats:[1001.0f]}] 0
execute as @a[tag=has_contract] if score @s mall.contract_held matches 0 run function motfb:contract/give

# --- Bryan vulnerability gate: flip off invulnerability once any player has contract ---
execute if entity @a[tag=has_contract] if entity @e[tag=motfb_bryan] run data merge entity @e[tag=motfb_bryan,limit=1] {Invulnerable:0b,NoAI:0b}

# --- Bryan HP probe (only while fight active) ---
execute if score #party mall.bryan_phase matches 1..2 run function motfb:bryan/hp_probe

# --- Bryan attack clock (phase 1 only) ---
execute if score #party mall.bryan_phase matches 1 run scoreboard players add #bryan_atk_clk mall.flag 1
execute if score #party mall.bryan_phase matches 1 if score #bryan_atk_clk mall.flag matches 60.. as @e[tag=motfb_bryan,tag=motfb_bryan_phase1,limit=1] at @s run summon arrow ~ ~1.6 ~ {Tags:["motfb_bryan_attack"],damage:6.0d,pickup:2b}
execute if score #party mall.bryan_phase matches 1 if score #bryan_atk_clk mall.flag matches 60.. run scoreboard players set #bryan_atk_clk mall.flag 0

# --- Boss bar updates while bosses alive ---
execute if entity @e[tag=motfb_kraw_boss] run execute store result bossbar motfb:kraw value run data get entity @e[tag=motfb_kraw_boss,limit=1] Health
execute if entity @e[tag=motfb_searz_boss] run execute store result bossbar motfb:searz value run data get entity @e[tag=motfb_searz_boss,limit=1] Health
execute if entity @e[tag=motfb_pixellich_boss] run execute store result bossbar motfb:pixellich value run data get entity @e[tag=motfb_pixellich_boss,limit=1] Health
execute if entity @e[tag=motfb_candywitch_boss] run execute store result bossbar motfb:candywitch value run data get entity @e[tag=motfb_candywitch_boss,limit=1] Health
execute if entity @e[tag=motfb_stitchlord_boss] run execute store result bossbar motfb:stitchlord value run data get entity @e[tag=motfb_stitchlord_boss,limit=1] Health
execute if entity @e[tag=motfb_vampirequeen_boss] run execute store result bossbar motfb:vampirequeen value run data get entity @e[tag=motfb_vampirequeen_boss,limit=1] Health
execute if entity @e[tag=motfb_impswarm_boss] run execute store result bossbar motfb:impswarm value run data get entity @e[tag=motfb_impswarm_boss,limit=1] Health
execute if entity @e[tag=motfb_exiledsaint_boss] run execute store result bossbar motfb:exiledsaint value run data get entity @e[tag=motfb_exiledsaint_boss,limit=1] Health
execute if entity @e[tag=motfb_knotgod_boss] run execute store result bossbar motfb:knotgod value run data get entity @e[tag=motfb_knotgod_boss,limit=1] Health
execute if entity @e[tag=motfb_speeddemon_boss] run execute store result bossbar motfb:speeddemon value run data get entity @e[tag=motfb_speeddemon_boss,limit=1] Health
execute if entity @e[tag=motfb_bryan] run execute store result bossbar motfb:bryan value run data get entity @e[tag=motfb_bryan,limit=1] Health

# --- Lost Kid follow tick ---
execute if entity @e[tag=motfb_lostkid] run function motfb:lostkid/follow_tick

# --- visit-end title on death ---
execute as @a if score @s mall.deaths matches 1.. run function motfb:utils/visit_ended
