# =============================================================================
# Journal placement — 3 readable lecterns required for Ending C (ANNUL)
# Players collect a journal just by approaching each lectern.
# Tick detection zones match these coords (see tick.mcfunction).
# =============================================================================

# Journal 1 — Food Court (z=-134): lore about the mall before it changed
setblock 0 65 -134 minecraft:oak_log[axis=y]
setblock 0 66 -134 minecraft:oak_log[axis=y]
setblock 0 67 -134 minecraft:lectern[facing=south]
setblock 0 68 -134 minecraft:end_rod[facing=up]
setblock 0 69 -134 minecraft:sea_lantern
setblock -1 67 -134 minecraft:oak_wall_sign[facing=east]{front_text:{messages:['{"text":"JOURNAL","color":"gold","bold":true}','{"text":"\"...they opened late,","color":"white"}','{"text":"they opened always...\"","color":"white"}','{"text":"[Approach to read]","color":"gray","italic":true}'],has_glowing_text:1b,color:"black"},is_waxed:1b}

# Journal 2 — Inside SEARZ (z=-272): found after Mama SEARZ is defeated
setblock 0 69 -272 minecraft:oak_log[axis=y]
setblock 0 70 -272 minecraft:oak_log[axis=y]
setblock 0 71 -272 minecraft:lectern[facing=south]
setblock 0 72 -272 minecraft:end_rod[facing=up]
setblock 0 73 -272 minecraft:sea_lantern
setblock -1 71 -272 minecraft:oak_wall_sign[facing=east]{front_text:{messages:['{"text":"JOURNAL","color":"gold","bold":true}','{"text":"\"Clearance on all","color":"white"}','{"text":"warranties. All of them.\"","color":"white"}','{"text":"[Approach to read]","color":"gray","italic":true}'],has_glowing_text:1b,color:"black"},is_waxed:1b}

# Journal 3 — Office antechamber (y=98, z=-218): discovered before meeting Bryan
setblock -5 98 -218 minecraft:oak_log[axis=y]
setblock -5 99 -218 minecraft:lectern[facing=east]
setblock -5 100 -218 minecraft:end_rod[facing=up]
setblock -5 101 -218 minecraft:sea_lantern
setblock -6 99 -218 minecraft:oak_wall_sign[facing=east]{front_text:{messages:['{"text":"JOURNAL","color":"gold","bold":true}','{"text":"\"Per the Original","color":"white"}','{"text":"Contract, §7.4...\"","color":"white"}','{"text":"[Approach to read]","color":"gray","italic":true}'],has_glowing_text:1b,color:"black"},is_waxed:1b}
