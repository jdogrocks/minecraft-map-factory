execute as @a[distance=..4,tag=!has_contract] at @s run tag @s add has_contract
execute as @a[distance=..4,tag=!has_contract] at @s run tag @s add in_office
give @a[tag=has_contract,tag=!gave_contract] minecraft:carrot_on_a_stick[custom_model_data={floats:[1001.0f]},custom_name='{"text":"The Original Contract","color":"dark_purple","italic":false,"bold":true}',lore=['{"text":"PARTY OF THE FIRST PART:","color":"dark_gray","italic":false}','{"text":"  The Architect of All Endings","color":"gray","italic":true}','{"text":"","color":"gray"}','{"text":"Right-click on the Signing Lectern: ACCEPT","color":"gold","italic":false}','{"text":"Strike Bryan with a weapon: VOID","color":"red","italic":false}','{"text":"Use Shears while standing on the Tearing Pad: ANNUL","color":"light_purple","italic":false}','{"text":"  (3 journals required)","color":"light_purple","italic":false}']] 1
tag @a[tag=has_contract,tag=!gave_contract] add gave_contract
function motfb:pa/job_offer
