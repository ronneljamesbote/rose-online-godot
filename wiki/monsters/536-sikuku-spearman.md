---
kind: monster
id: 536
name: Sikuku Spearman
status: in-game
level: 140
hp: 4760
hp_per_level: 34
attack: 677
hit: 412
defence: 447
resistance: 351
avoid: 220
attack_speed: 110
attack_range: 3.1
damage: physical
walk_speed: 400
run_speed: 850
xp: 121
drop_item_rate: 35
drop_money_rate: 30
zones: 2
drops:
- item: '[[items/material/282-sikuku-costume|Sikuku Costume]]'
  slots_of_30: 14
- item: '[[items/consumable/10-vital-water-s|Vital Water (S)]]'
  slots_of_30: 1
- item: '[[items/consumable/29-spiritual-water-s|Spiritual Water (S)]]'
  slots_of_30: 1
- item: '[[items/material/161-green-crystal|Green Crystal]]'
  slots_of_30: 1
- item: '[[items/material/162-blue-crystal|Blue Crystal]]'
  slots_of_30: 1
- item: '[[items/material/163-red-crystal|Red Crystal]]'
  slots_of_30: 1
- item: '[[items/material/164-white-crystal|White Crystal]]'
  slots_of_30: 1
- item: '[[items/jewellery/251-socket-ring|Socket Ring]]'
  slots_of_30: 1
- item: '[[items/jewellery/18-fascinating-ring|Fascinating Ring]]'
  slots_of_30: 1
- item: '[[items/material/153-blue-hearts|Blue Hearts]]'
  slots_of_30: 1
- item: '[[items/weapon/47-labor-hammer|Labor Hammer]]'
  slots_of_30: 0.2
- item: '[[items/weapon/177-longinus-spear|Longinus Spear]]'
  slots_of_30: 0.2
- item: '[[items/weapon/444-desperado-dual-wield|Desperado Dual Wield]]'
  slots_of_30: 0.2
- item: '[[items/weapon/346-twister-wand|Twister Wand]]'
  slots_of_30: 0.2
- item: '[[items/weapon/274-ferrell-cannon|Ferrell Cannon]]'
  slots_of_30: 0.2
spawns:
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5274.2
  y: 5283.7
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5108.8
  y: 5123.9
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5196.1
  y: 5177.5
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5235.5
  y: 5226
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5344.9
  y: 5230.9
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5608.8
  y: 5188.5
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5278.8
  y: 5029.8
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5209
  y: 5067.9
  count: 1
  group: reinforcements
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5358.8
  y: 5052.6
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5385.1
  y: 5084.9
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5547.6
  y: 5097.1
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5528
  y: 5086.6
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5475.7
  y: 5094.6
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5543.8
  y: 4969
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 536, ITEM_DROP.STB row 454
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Sikuku Spearman

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
