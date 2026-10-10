---
kind: monster
id: 284
name: Goblin Leader
status: in-game
level: 77
hp: 3696
hp_per_level: 48
attack: 335
hit: 202
defence: 253
resistance: 166
avoid: 95
attack_speed: 115
attack_range: 3.5
damage: physical
walk_speed: 240
run_speed: 650
xp: 70
drop_item_rate: 60
drop_money_rate: 15
zones: 1
drops:
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 7
- item: '[[items/consumable/172-stamina-75|Stamina (+75)]]'
  slots_of_30: 1
- item: '[[items/material/192-animal-backbone|Animal Backbone]]'
  slots_of_30: 7
- item: '[[items/jewellery/261-socket-necklace|Socket Necklace]]'
  slots_of_30: 1
- item: '[[items/material/161-green-crystal|Green Crystal]]'
  slots_of_30: 1
- item: '[[items/material/162-blue-crystal|Blue Crystal]]'
  slots_of_30: 1
- item: '[[items/hands/64-violet-gloves|Violet Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/94-black-pirate-boots|Black Pirate Boots]]'
  slots_of_30: 0.2
- item: '[[items/weapon/64-crossbow-gun|Crossbow Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/168-death-spear|Death Spear]]'
  slots_of_30: 0.2
- item: '[[items/weapon/435-saber-elven-sword|Saber & Elven Sword]]'
  slots_of_30: 0.2
- item: '[[items/weapon/265-marble-launcher|Marble Launcher]]'
  slots_of_30: 0.2
- item: '[[items/hands/34-chrome-gauntlets|Chrome Gauntlets]]'
  slots_of_30: 0.2
- item: '[[items/feet/64-violet-boots|Violet Boots]]'
  slots_of_30: 0.2
- item: '[[items/head/94-black-pirate-hat|Black Pirate Hat]]'
  slots_of_30: 0.2
- item: '[[items/body/124-semiya-vest|Semiya Vest]]'
  slots_of_30: 0.2
- item: '[[items/weapon/108-after-blade|After Blade]]'
  slots_of_30: 0.2
- item: '[[items/weapon/209-rider-bow|Rider Bow]]'
  slots_of_30: 0.2
- item: '[[items/weapon/238-showdown-gun|Showdown Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/308-golden-staff|Golden Staff]]'
  slots_of_30: 0.2
- item: '[[items/subweapon/7-kite-shield|Kite Shield]]'
  slots_of_30: 0.2
- item: '[[items/weapon/169-halberd|Halberd]]'
  slots_of_30: 0.2
- item: '[[items/weapon/266-orc-launcher|Orc Launcher]]'
  slots_of_30: 0.2
- item: '[[items/weapon/10-shark-blade|Shark Blade]]'
  slots_of_30: 0.2
- item: '[[items/weapon/825-crescent-knuckle|Crescent Knuckle]]'
  slots_of_30: 0.2
- item: '[[items/weapon/505-shark-blade|Shark Blade]]'
  slots_of_30: 0.2
spawns:
- zone: '[[zones/32-goblin-cave-b2|Goblin Cave (B2)]]'
  x: 5115.6
  y: 5471.8
  count: 1
  group: reinforcements
- zone: '[[zones/32-goblin-cave-b2|Goblin Cave (B2)]]'
  x: 5148.6
  y: 5501.6
  count: 1
  group: reinforcements
- zone: '[[zones/32-goblin-cave-b2|Goblin Cave (B2)]]'
  x: 5406.2
  y: 5512.5
  count: 1
  group: reinforcements
- zone: '[[zones/32-goblin-cave-b2|Goblin Cave (B2)]]'
  x: 5117
  y: 5135.6
  count: 1
  group: reinforcements
- zone: '[[zones/32-goblin-cave-b2|Goblin Cave (B2)]]'
  x: 5091.4
  y: 5133.8
  count: 1
  group: reinforcements
- zone: '[[zones/32-goblin-cave-b2|Goblin Cave (B2)]]'
  x: 5142.8
  y: 5118.9
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 284, ITEM_DROP.STB row 239
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Goblin Leader

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
