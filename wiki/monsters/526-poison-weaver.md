---
kind: monster
id: 526
name: Poison Weaver
status: in-game
level: 156
hp: 35
attack: 752
hit: 441
defence: 473
resistance: 329
avoid: 239
attack_speed: 100
attack_range: 2.2
damage: physical
walk_speed: 240
run_speed: 530
xp: 126
drop_item_rate: 40
drop_money_rate: 25
zones: 1
drops:
- item: '[[items/material/278-spider-web|Spider Web]]'
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
- item: '[[items/jewellery/170-lizard-earring|Lizard Earring]]'
  slots_of_30: 1
- item: '[[items/material/152-green-hearts|Green Hearts]]'
  slots_of_30: 1
- item: '[[items/weapon/20-death-bringer|Death Bringer]]'
  slots_of_30: 0.2
- item: '[[items/weapon/49-doom-hammer|Doom Hammer]]'
  slots_of_30: 0.2
- item: '[[items/weapon/118-sacred-hander|Sacred Hander]]'
  slots_of_30: 0.2
- item: '[[items/weapon/147-titan-axe|Titan Axe]]'
  slots_of_30: 0.2
- item: '[[items/weapon/179-fury-spantun|Fury Spantun]]'
  slots_of_30: 0.2
spawns:
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6392.9
  y: 5277.1
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6325.1
  y: 5245.2
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6106.6
  y: 5087.7
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6137.7
  y: 5107.5
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6296.2
  y: 5111.1
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6230.9
  y: 4882.4
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6243.4
  y: 4949.5
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6028.4
  y: 4687.9
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6056.4
  y: 4582.3
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 526, ITEM_DROP.STB row 448
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Poison Weaver

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
