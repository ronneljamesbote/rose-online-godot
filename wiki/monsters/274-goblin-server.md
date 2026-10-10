---
kind: monster
id: 274
name: Goblin Server
status: in-game
level: 52
hp: 1456
hp_per_level: 28
attack: 189
hit: 132
defence: 150
resistance: 100
avoid: 79
attack_speed: 95
attack_range: 8
damage: physical
walk_speed: 200
run_speed: 500
xp: 34
drop_item_rate: 57
drop_money_rate: 15
zones: 1
drops:
- item: '[[items/material/192-animal-backbone|Animal Backbone]]'
  slots_of_30: 9
- item: '[[items/consumable/163-mp-point-100|MP Point (+100)]]'
  slots_of_30: 1
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 6
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/153-hp-point-200|HP Point (+200)]]'
  slots_of_30: 1
- item: '[[items/jewellery/251-socket-ring|Socket Ring]]'
  slots_of_30: 1
- item: '[[items/head/93-ranger-hat|Ranger Hat]]'
  slots_of_30: 0.2
- item: '[[items/body/123-tamiya-vest|Tamiya Vest]]'
  slots_of_30: 0.2
- item: '[[items/weapon/305-white-staff|White Staff]]'
  slots_of_30: 0.2
- item: '[[items/weapon/206-elf-bow|Elf Bow]]'
  slots_of_30: 0.2
- item: '[[items/weapon/405-wolf-s-paw|Wolf''s Paw]]'
  slots_of_30: 0.2
- item: '[[items/weapon/7-saber|Saber]]'
  slots_of_30: 0.2
- item: '[[items/head/13-luxurious-venetian-hat|Luxurious Venetian Hat]]'
  slots_of_30: 0.2
- item: '[[items/hands/13-luxurious-venetian-gloves|Luxurious Venetian Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/13-luxurious-marble-shoes|Luxurious Marble Shoes]]'
  slots_of_30: 0.2
- item: '[[items/body/13-fancy-blue|Fancy Blue]]'
  slots_of_30: 0.2
- item: '[[items/weapon/166-iron-spear|Iron Spear]]'
  slots_of_30: 0.4
- item: '[[items/weapon/433-dual-bushido|Dual Bushido]]'
  slots_of_30: 0.4
- item: '[[items/weapon/335-silence-wand|Silence Wand]]'
  slots_of_30: 0.4
- item: '[[items/weapon/134-tomahawk|Tomahawk]]'
  slots_of_30: 0.4
- item: '[[items/back/245-lion-backshield|Lion Backshield]]'
  slots_of_30: 0.4
quests:
- '[[quests/5014-goblin-cave-expedition-1st-level|Goblin Cave Expedition (1st Level)]]'
spawns:
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5080.6
  y: 5281.5
  count: 1
  group: basic
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5080.6
  y: 5281.5
  count: 2
  group: reinforcements
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5069.4
  y: 5307.6
  count: 1
  group: basic
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5069.4
  y: 5307.6
  count: 2
  group: reinforcements
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5424.4
  y: 5435
  count: 1
  group: basic
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5383
  y: 5152.8
  count: 1
  group: basic
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5383
  y: 5152.8
  count: 2
  group: reinforcements
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5378.2
  y: 5133.6
  count: 1
  group: basic
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5378.2
  y: 5133.6
  count: 2
  group: reinforcements
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5385.4
  y: 5170.7
  count: 1
  group: basic
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5385.4
  y: 5170.7
  count: 1
  group: basic
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5385.4
  y: 5170.7
  count: 2
  group: reinforcements
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5162.2
  y: 5057.7
  count: 1
  group: basic
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5162.2
  y: 5057.7
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 274, ITEM_DROP.STB row 229
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Goblin Server

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
