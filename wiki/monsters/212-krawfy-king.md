---
kind: monster
id: 212
name: Krawfy King
status: in-game
level: 75
hp: 13950
hp_per_level: 186
attack: 356
hit: 225
defence: 286
resistance: 207
avoid: 92
attack_speed: 95
attack_range: 2.5
damage: physical
walk_speed: 330
run_speed: 710
xp: 598
drop_item_rate: 82
drop_money_rate: 15
zones: 1
drops:
- item: '[[items/material/187-black-animal-tail-fur|Black Animal Tail Fur]]'
  slots_of_30: 7
- item: '[[items/material/162-blue-crystal|Blue Crystal]]'
  slots_of_30: 6
- item: '[[items/consumable/173-stamina-100|Stamina (+100)]]'
  slots_of_30: 2
- item: '[[items/consumable/165-mp-point-300|MP Point (+300)]]'
  slots_of_30: 1
- item: '[[items/consumable/58-vital-jam-5|Vital Jam (+5)]]'
  slots_of_30: 2
- item: '[[items/weapon/407-spike-knuckle|Spike Knuckle]]'
  slots_of_30: 0.4
- item: '[[items/weapon/307-shadow-staff|Shadow Staff]]'
  slots_of_30: 0.4
- item: '[[items/weapon/107-two-handed-sword|Two-Handed Sword]]'
  slots_of_30: 0.4
- item: '[[items/weapon/10-shark-blade|Shark Blade]]'
  slots_of_30: 0.4
- item: '[[items/weapon/108-after-blade|After Blade]]'
  slots_of_30: 0.4
- item: '[[items/hands/34-chrome-gauntlets|Chrome Gauntlets]]'
  slots_of_30: 0.4
- item: '[[items/feet/64-violet-boots|Violet Boots]]'
  slots_of_30: 0.4
- item: '[[items/head/94-black-pirate-hat|Black Pirate Hat]]'
  slots_of_30: 0.4
- item: '[[items/body/124-semiya-vest|Semiya Vest]]'
  slots_of_30: 0.4
- item: '[[items/weapon/583-two-handed-sword|Two-Handed Sword]]'
  slots_of_30: 0.4
- item: '[[items/weapon/821-rake-hand|Rake Hand]]'
  slots_of_30: 0.4
- item: '[[items/weapon/615-tower-axe|Tower Axe]]'
  slots_of_30: 0.4
- item: '[[items/weapon/644-death-spear|Death Spear]]'
  slots_of_30: 0.4
- item: '[[items/weapon/505-shark-blade|Shark Blade]]'
  slots_of_30: 0.4
- item: '[[items/weapon/563-crossbow-gun|Crossbow Gun]]'
  slots_of_30: 0.4
- item: '[[items/weapon/825-crescent-knuckle|Crescent Knuckle]]'
  slots_of_30: 0.6
- item: '[[items/head/424-chrome-helm|Chrome Helm]]'
  slots_of_30: 0.6
- item: '[[items/body/324-chrome-armor|Chrome Armor]]'
  slots_of_30: 0.6
- item: '[[items/hands/424-violet-gloves|Violet Gloves]]'
  slots_of_30: 0.6
- item: '[[items/feet/524-black-pirate-boots|Black Pirate Boots]]'
  slots_of_30: 0.6
- item: '[[items/material/211-wooden-cart-schematic|Wooden Cart Schematic]]'
  slots_of_30: 1
quests:
- '[[quests/1055-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]'
spawns:
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5552
  y: 5122.9
  count: 1
  group: reinforcements
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5475.9
  y: 5177.8
  count: 1
  group: reinforcements
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5503.9
  y: 5156.6
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5481.5
  y: 5117.4
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 212, ITEM_DROP.STB row 207
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Krawfy King

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
