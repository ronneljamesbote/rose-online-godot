---
kind: monster
id: 516
name: Hook
status: in-game
level: 145
hp: 4495
hp_per_level: 31
attack: 713
hit: 416
defence: 394
resistance: 499
avoid: 243
attack_speed: 115
attack_range: 2.4
damage: physical
walk_speed: 300
run_speed: 650
xp: 122
drop_item_rate: 40
drop_money_rate: 20
zones: 2
drops:
- item: '[[items/material/280-transperant-wing|Transperant Wing]]'
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
- item: '[[items/jewellery/161-glamour-earring|Glamour Earring]]'
  slots_of_30: 1
- item: '[[items/weapon/317-oracle-staff|Oracle Staff]]'
  slots_of_30: 0.2
- item: '[[items/weapon/417-bloody-hook-knuckle|Bloody Hook Knuckle]]'
  slots_of_30: 0.2
- item: '[[items/weapon/48-nirvana-hammer|Nirvana Hammer]]'
  slots_of_30: 0.2
- item: '[[items/weapon/146-adamantium-axe|Adamantium Axe]]'
  slots_of_30: 0.2
- item: '[[items/weapon/275-spero-shooter|Spero Shooter]]'
  slots_of_30: 0.2
- item: '[[items/weapon/445-glorious-dual-wield|Glorious Dual Wield]]'
  slots_of_30: 0.4
- item: '[[items/weapon/178-nimrodpiece|Nimrodpiece]]'
  slots_of_30: 0.4
- item: '[[items/weapon/71-mythril-bow-gun|Mythril Bow Gun]]'
  slots_of_30: 0.4
- item: '[[items/weapon/177-longinus-spear|Longinus Spear]]'
  slots_of_30: 0.4
- item: '[[items/weapon/444-desperado-dual-wield|Desperado Dual Wield]]'
  slots_of_30: 0.4
spawns:
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5546.2
  y: 5346.4
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5546.2
  y: 5346.4
  count: 2
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5622.7
  y: 5350.5
  count: 2
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5838.8
  y: 4986.4
  count: 2
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5241.2
  y: 4852.4
  count: 2
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5190.1
  y: 4888.5
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5412.4
  y: 4936.5
  count: 2
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5182
  y: 4666.3
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5142
  y: 4648.2
  count: 2
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5761.9
  y: 4780.1
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5011.3
  y: 4596.2
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5048.4
  y: 4623.9
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6481.5
  y: 5024.6
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6476.1
  y: 4820.5
  count: 2
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6520.6
  y: 4920
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5186
  y: 4660
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5476.6
  y: 4680.5
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6163.3
  y: 4652.5
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6495.7
  y: 4703.3
  count: 2
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5350.6
  y: 4619.2
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5506.9
  y: 4612.6
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6407.2
  y: 4628.3
  count: 2
  group: basic
source:
  data: LIST_NPC.STB row 516, ITEM_DROP.STB row 441
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Hook

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
