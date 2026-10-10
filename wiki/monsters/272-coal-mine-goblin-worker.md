---
kind: monster
id: 272
name: Coal Mine Goblin Worker
status: in-game
level: 55
hp: 35
attack: 182
hit: 149
defence: 205
resistance: 120
avoid: 44
attack_speed: 90
attack_range: 2.5
damage: physical
walk_speed: 220
run_speed: 480
xp: 35
drop_item_rate: 55
drop_money_rate: 12
zones: 1
drops:
- item: '[[items/material/192-animal-backbone|Animal Backbone]]'
  slots_of_30: 6
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 7
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/153-hp-point-200|HP Point (+200)]]'
  slots_of_30: 1
- item: '[[items/consumable/163-mp-point-100|MP Point (+100)]]'
  slots_of_30: 1
- item: '[[items/jewellery/271-socket-earring|Socket Earring]]'
  slots_of_30: 1
- item: '[[items/consumable/428-goblin-worker|Goblin Worker]]'
  slots_of_30: 1
- item: '[[items/material/161-green-crystal|Green Crystal]]'
  slots_of_30: 1
- item: '[[items/head/93-ranger-hat|Ranger Hat]]'
  slots_of_30: 0.2
- item: '[[items/weapon/206-elf-bow|Elf Bow]]'
  slots_of_30: 0.2
- item: '[[items/weapon/305-white-staff|White Staff]]'
  slots_of_30: 0.2
- item: '[[items/weapon/235-iron-shotgun|Iron Shotgun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/762-mage-s-rod|Mage''s Rod]]'
  slots_of_30: 0.2
- item: '[[items/weapon/105-surado|Surado]]'
  slots_of_30: 0.2
- item: '[[items/head/13-luxurious-venetian-hat|Luxurious Venetian Hat]]'
  slots_of_30: 0.2
- item: '[[items/hands/13-luxurious-venetian-gloves|Luxurious Venetian Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/13-luxurious-marble-shoes|Luxurious Marble Shoes]]'
  slots_of_30: 0.2
- item: '[[items/body/13-fancy-blue|Fancy Blue]]'
  slots_of_30: 0.2
- item: '[[items/weapon/335-silence-wand|Silence Wand]]'
  slots_of_30: 0.4
- item: '[[items/weapon/433-dual-bushido|Dual Bushido]]'
  slots_of_30: 0.4
- item: '[[items/weapon/166-iron-spear|Iron Spear]]'
  slots_of_30: 0.4
- item: '[[items/weapon/62-cutter-bow-gun|Cutter Bow Gun]]'
  slots_of_30: 0.4
- item: '[[items/subweapon/65-mythical-flute|Mythical Flute]]'
  slots_of_30: 0.4
quests:
- '[[quests/805-secretly-learning-to-drive|Secretly Learning to Drive]]'
spawns:
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5072
  y: 5414
  count: 1
  group: basic
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5072
  y: 5414
  count: 1
  group: basic
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5061.9
  y: 5282.6
  count: 1
  group: basic
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5080.6
  y: 5281.5
  count: 1
  group: basic
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5069.4
  y: 5307.6
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
  count: 1
  group: basic
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5378.2
  y: 5133.6
  count: 2
  group: basic
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
  x: 5162.2
  y: 5057.7
  count: 2
  group: basic
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5162.2
  y: 5057.7
  count: 2
  group: reinforcements
source:
  data: LIST_NPC.STB row 272, ITEM_DROP.STB row 227
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Coal Mine Goblin Worker

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
