---
kind: monster
id: 271
name: Goblin Worker
status: in-game
level: 51
hp: 28
attack: 185
hit: 131
defence: 146
resistance: 98
avoid: 78
attack_speed: 105
attack_range: 2.5
damage: physical
walk_speed: 210
run_speed: 530
xp: 35
drop_item_rate: 58
drop_money_rate: 10
zones: 1
drops:
- item: '[[items/material/192-animal-backbone|Animal Backbone]]'
  slots_of_30: 9
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 5
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/6-health-bottle-l|Health Bottle (L)]]'
  slots_of_30: 1
- item: '[[items/consumable/154-hp-point-300|HP Point (+300)]]'
  slots_of_30: 1
- item: '[[items/consumable/163-mp-point-100|MP Point (+100)]]'
  slots_of_30: 1
- item: '[[items/jewellery/261-socket-necklace|Socket Necklace]]'
  slots_of_30: 1
- item: '[[items/consumable/428-goblin-worker|Goblin Worker]]'
  slots_of_30: 1
- item: '[[items/weapon/234-iron-rifle|Iron Rifle]]'
  slots_of_30: 0.2
- item: '[[items/body/63-purple-vest-of-witch|Purple Vest of Witch]]'
  slots_of_30: 0.2
- item: '[[items/weapon/133-battle-axe|Battle Axe]]'
  slots_of_30: 0.2
- item: '[[items/weapon/334-elven-wand|Elven Wand]]'
  slots_of_30: 0.2
- item: '[[items/weapon/6-bushido|Bushido]]'
  slots_of_30: 0.2
- item: '[[items/weapon/104-cutter-edge|Cutter Edge]]'
  slots_of_30: 0.2
- item: '[[items/head/13-luxurious-venetian-hat|Luxurious Venetian Hat]]'
  slots_of_30: 0.2
- item: '[[items/hands/13-luxurious-venetian-gloves|Luxurious Venetian Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/13-luxurious-marble-shoes|Luxurious Marble Shoes]]'
  slots_of_30: 0.2
- item: '[[items/body/13-fancy-blue|Fancy Blue]]'
  slots_of_30: 0.2
- item: '[[items/weapon/36-ghost-bat|Ghost Bat]]'
  slots_of_30: 0.4
- item: '[[items/weapon/263-hard-launcher|Hard Launcher]]'
  slots_of_30: 0.4
- item: '[[items/weapon/433-dual-bushido|Dual Bushido]]'
  slots_of_30: 0.4
- item: '[[items/weapon/432-long-sword-mace|Long Sword & Mace]]'
  slots_of_30: 0.4
- item: '[[items/subweapon/5-aspis|Aspis]]'
  slots_of_30: 0.4
quests:
- '[[quests/5014-goblin-cave-expedition-1st-level|Goblin Cave Expedition (1st Level)]]'
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
  x: 5069.4
  y: 5307.6
  count: 1
  group: basic
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5424.4
  y: 5435
  count: 1
  group: basic
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
  x: 5378.2
  y: 5133.6
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
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 271, ITEM_DROP.STB row 226
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Goblin Worker

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
