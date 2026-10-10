---
kind: monster
id: 537
name: Sikuku Leader
status: in-game
level: 151
hp: 44
attack: 684
hit: 490
defence: 551
resistance: 409
avoid: 278
attack_speed: 110
attack_range: 3.2
damage: physical
walk_speed: 400
run_speed: 850
xp: 176
drop_item_rate: 60
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
- item: '[[items/jewellery/261-socket-necklace|Socket Necklace]]'
  slots_of_30: 1
- item: '[[items/gem/343-emerald-3|Emerald 3]]'
  slots_of_30: 1
- item: '[[items/jewellery/98-fascinating-necklace|Fascinating Necklace]]'
  slots_of_30: 1
- item: '[[items/material/154-pink-hearts|Pink Hearts]]'
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
spawns:
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5532.8
  y: 5203.7
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5209
  y: 5067.9
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5358.8
  y: 5052.6
  count: 1
  group: reinforcements
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5353.1
  y: 5067.4
  count: 1
  group: reinforcements
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5337.3
  y: 5165.3
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5329.9
  y: 5070.4
  count: 2
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5881.8
  y: 5019.1
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5380.4
  y: 4940.6
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5756.1
  y: 4876.5
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5827.8
  y: 4944.1
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 537, ITEM_DROP.STB row 455
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Sikuku Leader

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
