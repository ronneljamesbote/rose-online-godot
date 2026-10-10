---
kind: monster
id: 538
name: Sikuku Fighter
status: in-game
level: 147
hp: 5880
hp_per_level: 40
attack: 718
hit: 433
defence: 476
resistance: 382
avoid: 233
attack_speed: 110
attack_range: 3.1
damage: physical
walk_speed: 400
run_speed: 850
xp: 148
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
- item: '[[items/jewellery/271-socket-earring|Socket Earring]]'
  slots_of_30: 1
- item: '[[items/jewellery/168-fascinating-earring|Fascinating Earring]]'
  slots_of_30: 1
- item: '[[items/weapon/177-longinus-spear|Longinus Spear]]'
  slots_of_30: 0.2
- item: '[[items/weapon/71-mythril-bow-gun|Mythril Bow Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/19-black-shamshir|Black Shamshir]]'
  slots_of_30: 0.2
- item: '[[items/weapon/117-shion-s-guardian|Shion''s Guardian]]'
  slots_of_30: 0.2
- item: '[[items/weapon/218-bifenith-bow|Bifenith Bow]]'
  slots_of_30: 0.2
spawns:
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5291.9
  y: 5280.9
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5344.2
  y: 5299.4
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5535.8
  y: 5141.8
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5532.8
  y: 5203.7
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5320.9
  y: 5063.3
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5492.9
  y: 5064.8
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5367.3
  y: 5170.9
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5411
  y: 5146.1
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5337.3
  y: 5165.3
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5380.4
  y: 4940.6
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5418.5
  y: 4909.6
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5386.3
  y: 4802.5
  count: 2
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5561.8
  y: 4844
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
  data: LIST_NPC.STB row 538, ITEM_DROP.STB row 456
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Sikuku Fighter

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
