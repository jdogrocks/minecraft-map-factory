# --- difficulty (must be Hard so summoned hostiles persist; Peaceful despawns them) ---
difficulty hard

# --- gamerules: mall-feel ---
gamerule keep_inventory true
gamerule send_command_feedback false
gamerule immediate_respawn true
gamerule spawn_mobs false
gamerule mob_griefing false

# --- daylight/weather control (commands since gamerule names differ per version) ---
time set 13000
weather clear 999999

# --- objectives (add idempotently) ---
scoreboard objectives add mall.coupons dummy {"text":"Boss Coupons","color":"gold"}
scoreboard objectives add mall.journals dummy {"text":"Journals","color":"light_purple"}
scoreboard objectives add mall.ending dummy "Ending"
scoreboard objectives add mall.bryan_phase dummy "Bryan Phase"
scoreboard objectives add mall.bryan_hp dummy "Bryan HP"
scoreboard objectives add mall.contract_use minecraft.used:minecraft.carrot_on_a_stick
scoreboard objectives add mall.shears_use minecraft.used:minecraft.shears
scoreboard objectives add mall.pa_cooldown dummy
scoreboard objectives add mall.lk_cooldown dummy
scoreboard objectives add mall.flag dummy
scoreboard objectives add mall.deaths deathCount

# --- HUD ---
scoreboard objectives setdisplay sidebar mall.coupons
scoreboard objectives setdisplay list mall.journals

# --- teams ---
team add motfb_boss
team modify motfb_boss color red
team modify motfb_boss collisionRule pushOtherTeams
team add motfb_bryan
team modify motfb_bryan color dark_purple

# --- fake-player init ---
execute unless score #party mall.coupons matches 0.. run scoreboard players set #party mall.coupons 0
execute unless score #party mall.journals matches 0.. run scoreboard players set #party mall.journals 0
execute unless score #party mall.ending matches 0.. run scoreboard players set #party mall.ending 0
execute unless score #party mall.bryan_phase matches 0.. run scoreboard players set #party mall.bryan_phase 0
scoreboard players set #party mall.bryan_hp 99

# --- set spawn at exterior entrance ---
setworldspawn 0 65 -90
spawnpoint @a 0 65 -90

# --- place journal lecterns and spawn boss entities + static NPCs ---
function motfb:build/setup_journals
function motfb:boss/init_all
function motfb:lostkid/spawn_at_arcade
function motfb:bryan/spawn_at_office

tellraw @a {"text":"Liminal Lakes Mall — datapack loaded.","color":"dark_gray","italic":true}
