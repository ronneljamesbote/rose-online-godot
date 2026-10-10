---
kind: monster
id: 553
name: Sikuku Shaman
status: in-game
level: 150
hp: 4200
hp_per_level: 28
attack: 714
hit: 443
defence: 386
resistance: 721
avoid: 239
attack_speed: 100
attack_range: 15
damage: magic
walk_speed: 200
run_speed: 450
xp: 169
drop_item_rate: 60
drop_money_rate: 50
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
- item: '[[items/jewellery/103-gorgeous-necklace|Gorgeous Necklace]]'
  slots_of_30: 1
- item: '[[items/material/157-white-hearts|White Hearts]]'
  slots_of_30: 1
- item: '[[items/weapon/247-steam-shocker|Steam Shocker]]'
  slots_of_30: 0.2
- item: '[[items/weapon/317-oracle-staff|Oracle Staff]]'
  slots_of_30: 0.2
- item: '[[items/weapon/275-spero-shooter|Spero Shooter]]'
  slots_of_30: 0.2
- item: '[[items/weapon/445-glorious-dual-wield|Glorious Dual Wield]]'
  slots_of_30: 0.2
- item: '[[items/weapon/178-nimrodpiece|Nimrodpiece]]'
  slots_of_30: 0.2
spawns:
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5294.5
  y: 5295.4
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5344.2
  y: 5299.4
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5135.2
  y: 5125.8
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5235.5
  y: 5226
  count: 1
  group: reinforcements
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5344.9
  y: 5230.9
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5433.1
  y: 5189.5
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5568.6
  y: 5143.3
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5278.8
  y: 5029.8
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5353.1
  y: 5067.4
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5385.1
  y: 5084.9
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5298.7
  y: 5118.1
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5451.5
  y: 5083.8
  count: 2
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5492.9
  y: 5064.8
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5528
  y: 5086.6
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5386.9
  y: 4971.8
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6015.1
  y: 4975.2
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 553, ITEM_DROP.STB row 472
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Sikuku Shaman

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
