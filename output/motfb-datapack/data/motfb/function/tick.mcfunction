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

# --- Boss bar: continuous presence detection + HP value + player list updates ---
# Pattern per boss: add tag for entrants, remove for leavers, then update bar value + players.
# on_death_* handles tag removal and bossbar remove when the boss dies.

# kraw (Cluck-O-Mart left)
execute if entity @e[tag=motfb_kraw_boss] as @a[x=-50,y=60,z=-260,dx=44,dy=20,dz=14,tag=!in_kraw] run tag @s add in_kraw
execute as @a[tag=in_kraw] unless entity @s[x=-50,y=60,z=-260,dx=44,dy=20,dz=14] run tag @s remove in_kraw
execute if entity @e[tag=motfb_kraw_boss] run execute store result bossbar motfb:kraw value run data get entity @e[tag=motfb_kraw_boss,limit=1] Health
execute if entity @e[tag=motfb_kraw_boss] run bossbar set motfb:kraw players @a[tag=in_kraw]

# searz (SEARZ dept-store upper)
execute if entity @e[tag=motfb_searz_boss] as @a[x=-50,y=60,z=-280,dx=100,dy=20,dz=19,tag=!in_searz] run tag @s add in_searz
execute as @a[tag=in_searz] unless entity @s[x=-50,y=60,z=-280,dx=100,dy=20,dz=19] run tag @s remove in_searz
execute if entity @e[tag=motfb_searz_boss] run execute store result bossbar motfb:searz value run data get entity @e[tag=motfb_searz_boss,limit=1] Health
execute if entity @e[tag=motfb_searz_boss] run bossbar set motfb:searz players @a[tag=in_searz]

# pixellich (GameZone left)
execute if entity @e[tag=motfb_pixellich_boss] as @a[x=-50,y=60,z=-245,dx=44,dy=20,dz=14,tag=!in_pixellich] run tag @s add in_pixellich
execute as @a[tag=in_pixellich] unless entity @s[x=-50,y=60,z=-245,dx=44,dy=20,dz=14] run tag @s remove in_pixellich
execute if entity @e[tag=motfb_pixellich_boss] run execute store result bossbar motfb:pixellich value run data get entity @e[tag=motfb_pixellich_boss,limit=1] Health
execute if entity @e[tag=motfb_pixellich_boss] run bossbar set motfb:pixellich players @a[tag=in_pixellich]

# candywitch (Cinnabog left)
execute if entity @e[tag=motfb_candywitch_boss] as @a[x=-50,y=60,z=-230,dx=44,dy=20,dz=14,tag=!in_cinnabog] run tag @s add in_cinnabog
execute as @a[tag=in_cinnabog] unless entity @s[x=-50,y=60,z=-230,dx=44,dy=20,dz=14] run tag @s remove in_cinnabog
execute if entity @e[tag=motfb_candywitch_boss] run execute store result bossbar motfb:candywitch value run data get entity @e[tag=motfb_candywitch_boss,limit=1] Health
execute if entity @e[tag=motfb_candywitch_boss] run bossbar set motfb:candywitch players @a[tag=in_cinnabog]

# stitchlord (Build-A-Boss left)
execute if entity @e[tag=motfb_stitchlord_boss] as @a[x=-50,y=60,z=-215,dx=44,dy=20,dz=14,tag=!in_buildaboss] run tag @s add in_buildaboss
execute as @a[tag=in_buildaboss] unless entity @s[x=-50,y=60,z=-215,dx=44,dy=20,dz=14] run tag @s remove in_buildaboss
execute if entity @e[tag=motfb_stitchlord_boss] run execute store result bossbar motfb:stitchlord value run data get entity @e[tag=motfb_stitchlord_boss,limit=1] Health
execute if entity @e[tag=motfb_stitchlord_boss] run bossbar set motfb:stitchlord players @a[tag=in_buildaboss]

# vampirequeen (Hot-Topical left)
execute if entity @e[tag=motfb_vampirequeen_boss] as @a[x=-50,y=60,z=-200,dx=44,dy=20,dz=14,tag=!in_hottopical] run tag @s add in_hottopical
execute as @a[tag=in_hottopical] unless entity @s[x=-50,y=60,z=-200,dx=44,dy=20,dz=14] run tag @s remove in_hottopical
execute if entity @e[tag=motfb_vampirequeen_boss] run execute store result bossbar motfb:vampirequeen value run data get entity @e[tag=motfb_vampirequeen_boss,limit=1] Health
execute if entity @e[tag=motfb_vampirequeen_boss] run bossbar set motfb:vampirequeen players @a[tag=in_hottopical]

# impswarm (Spencer's right)
execute if entity @e[tag=motfb_impswarm_boss] as @a[x=6,y=60,z=-260,dx=44,dy=20,dz=14,tag=!in_spencers] run tag @s add in_spencers
execute as @a[tag=in_spencers] unless entity @s[x=6,y=60,z=-260,dx=44,dy=20,dz=14] run tag @s remove in_spencers
execute if entity @e[tag=motfb_impswarm_boss] run execute store result bossbar motfb:impswarm value run data get entity @e[tag=motfb_impswarm_boss,limit=1] Health
execute if entity @e[tag=motfb_impswarm_boss] run bossbar set motfb:impswarm players @a[tag=in_spencers]

# exiledsaint (Bath & Body right)
execute if entity @e[tag=motfb_exiledsaint_boss] as @a[x=6,y=60,z=-245,dx=44,dy=20,dz=14,tag=!in_bathbody] run tag @s add in_bathbody
execute as @a[tag=in_bathbody] unless entity @s[x=6,y=60,z=-245,dx=44,dy=20,dz=14] run tag @s remove in_bathbody
execute if entity @e[tag=motfb_exiledsaint_boss] run execute store result bossbar motfb:exiledsaint value run data get entity @e[tag=motfb_exiledsaint_boss,limit=1] Health
execute if entity @e[tag=motfb_exiledsaint_boss] run bossbar set motfb:exiledsaint players @a[tag=in_bathbody]

# knotgod (Pretzel-Pretzel Pretzel right)
execute if entity @e[tag=motfb_knotgod_boss] as @a[x=6,y=60,z=-230,dx=44,dy=20,dz=14,tag=!in_pretzel] run tag @s add in_pretzel
execute as @a[tag=in_pretzel] unless entity @s[x=6,y=60,z=-230,dx=44,dy=20,dz=14] run tag @s remove in_pretzel
execute if entity @e[tag=motfb_knotgod_boss] run execute store result bossbar motfb:knotgod value run data get entity @e[tag=motfb_knotgod_boss,limit=1] Health
execute if entity @e[tag=motfb_knotgod_boss] run bossbar set motfb:knotgod players @a[tag=in_pretzel]

# speeddemon (Spunky's Footwear right)
execute if entity @e[tag=motfb_speeddemon_boss] as @a[x=6,y=60,z=-215,dx=44,dy=20,dz=14,tag=!in_spunky] run tag @s add in_spunky
execute as @a[tag=in_spunky] unless entity @s[x=6,y=60,z=-215,dx=44,dy=20,dz=14] run tag @s remove in_spunky
execute if entity @e[tag=motfb_speeddemon_boss] run execute store result bossbar motfb:speeddemon value run data get entity @e[tag=motfb_speeddemon_boss,limit=1] Health
execute if entity @e[tag=motfb_speeddemon_boss] run bossbar set motfb:speeddemon players @a[tag=in_spunky]

# bryan (office fight — no bounding box; all players see the bar)
execute if entity @e[tag=motfb_bryan] run execute store result bossbar motfb:bryan value run data get entity @e[tag=motfb_bryan,limit=1] Health

# --- Lost Kid follow tick ---
execute if entity @e[tag=motfb_lostkid] run function motfb:lostkid/follow_tick

# --- visit-end title on death ---
execute as @a if score @s mall.deaths matches 1.. run function motfb:utils/visit_ended
