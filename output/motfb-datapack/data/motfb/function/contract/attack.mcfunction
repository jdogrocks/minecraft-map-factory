execute unless score #party mall.ending matches 0 run return fail
execute if score #party mall.bryan_phase matches 0 if score #party mall.bryan_hp matches ..98 run scoreboard players set #party mall.ending 2
execute if score #party mall.ending matches 2 if score #party mall.bryan_phase matches 0 run function motfb:ending/b_voided
