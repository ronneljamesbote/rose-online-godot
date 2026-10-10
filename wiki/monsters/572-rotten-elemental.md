---
kind: monster
id: 572
name: Rotten Elemental
status: in-game
level: 157
hp: 35
attack: 754
hit: 481
defence: 502
resistance: 509
avoid: 293
attack_speed: 100
attack_range: 2.2
damage: physical
walk_speed: 300
run_speed: 650
xp: 139
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
- item: '[[items/jewellery/102-lucky-necklace|Lucky Necklace]]'
  slots_of_30: 1
- item: '[[items/material/156-golden-hearts|Golden Hearts]]'
  slots_of_30: 1
- item: '[[items/weapon/318-twinkle-staff|Twinkle Staff]]'
  slots_of_30: 0.2
- item: '[[items/weapon/419-raven-knuckle|Raven Knuckle]]'
  slots_of_30: 0.2
- item: '[[items/weapon/418-hawk-knuckle|Hawk Knuckle]]'
  slots_of_30: 0.2
- item: '[[items/weapon/277-reverse-canon|Reverse Canon]]'
  slots_of_30: 0.2
- item: '[[items/weapon/446-demise-dual-weapon|Demise Dual Weapon]]'
  slots_of_30: 0.2
spawns:
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5846.6
  y: 5280.7
  count: 1
  group: reinforcements
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5534.5
  y: 4687
  count: 1
  group: reinforcements
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6173
  y: 5223.1
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6206.1
  y: 5121.6
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6345.2
  y: 5176.3
  count: 1
  group: reinforcements
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6223.1
  y: 5040.4
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6326.4
  y: 5034.4
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6019.3
  y: 4873.8
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 572, ITEM_DROP.STB row 486
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Rotten Elemental

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
