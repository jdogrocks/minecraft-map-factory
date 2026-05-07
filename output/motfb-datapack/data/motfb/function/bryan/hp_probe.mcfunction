execute store result score #party mall.bryan_hp run data get entity @e[tag=motfb_bryan,limit=1] Health
execute if score #party mall.bryan_hp matches ..66 if score #party mall.bryan_phase matches 1 run function motfb:bryan/phase_2
execute if score #party mall.bryan_hp matches ..33 if score #party mall.bryan_phase matches 2 run function motfb:bryan/phase_3
execute if score #party mall.bryan_hp matches ..0 if score #party mall.bryan_phase matches 3 run function motfb:ending/b_voided_finalize
execute if score #party mall.bryan_phase matches 0 if score #party mall.bryan_hp matches ..98 if entity @a[tag=has_contract] run function motfb:contract/attack
