---
kind: monster
id: 571
name: Rotten Tree
status: in-game
level: 147
hp: 29
attack: 697
hit: 449
defence: 501
resistance: 469
avoid: 271
attack_speed: 100
attack_range: 13
damage: physical
walk_speed: 300
run_speed: 650
xp: 124
drop_item_rate: 25
drop_money_rate: 40
zones: 2
drops:
- item: '[[items/material/281-plasma|Plasma]]'
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
- item: '[[items/jewellery/101-mage-necklace|Mage Necklace]]'
  slots_of_30: 1
- item: '[[items/weapon/417-bloody-hook-knuckle|Bloody Hook Knuckle]]'
  slots_of_30: 0.2
- item: '[[items/weapon/48-nirvana-hammer|Nirvana Hammer]]'
  slots_of_30: 0.2
- item: '[[items/weapon/247-steam-shocker|Steam Shocker]]'
  slots_of_30: 0.2
- item: '[[items/weapon/445-glorious-dual-wield|Glorious Dual Wield]]'
  slots_of_30: 0.2
- item: '[[items/weapon/19-black-shamshir|Black Shamshir]]'
  slots_of_30: 0.2
spawns:
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5084.9
  y: 5336.4
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5128.6
  y: 5329.1
  count: 2
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5792.5
  y: 5336.8
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5846.6
  y: 5280.7
  count: 2
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5730.4
  y: 5153
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5110
  y: 4939.8
  count: 2
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5534.5
  y: 4687
  count: 2
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5534.5
  y: 4687
  count: 2
  group: reinforcements
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6223.1
  y: 5040.4
  count: 2
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5991.9
  y: 4869.6
  count: 2
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6476.1
  y: 4820.5
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5272
  y: 4713.3
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5209.8
  y: 4731.4
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5378
  y: 4708.7
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5350.6
  y: 4619.2
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5662.6
  y: 4589.1
  count: 2
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6250.4
  y: 4611.5
  count: 2
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6407.2
  y: 4628.3
  count: 2
  group: basic
source:
  data: LIST_NPC.STB row 571, ITEM_DROP.STB row 485
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Rotten Tree

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
