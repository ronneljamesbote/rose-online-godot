---
kind: monster
id: 542
name: Sikuku Hunter
status: in-game
level: 143
hp: 31
attack: 756
hit: 421
defence: 437
resistance: 368
avoid: 226
attack_speed: 100
attack_range: 2.6
damage: physical
walk_speed: 300
run_speed: 650
xp: 140
drop_item_rate: 40
drop_money_rate: 25
zones: 1
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
- item: '[[items/jewellery/172-deadly-earring|Deadly Earring]]'
  slots_of_30: 1
- item: '[[items/weapon/71-mythril-bow-gun|Mythril Bow Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/19-black-shamshir|Black Shamshir]]'
  slots_of_30: 0.2
- item: '[[items/weapon/117-shion-s-guardian|Shion''s Guardian]]'
  slots_of_30: 0.2
- item: '[[items/weapon/218-bifenith-bow|Bifenith Bow]]'
  slots_of_30: 0.2
- item: '[[items/weapon/247-steam-shocker|Steam Shocker]]'
  slots_of_30: 0.2
spawns:
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5296.5
  y: 5311
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
  x: 5279.6
  y: 5270.2
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5351.2
  y: 5135.7
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5386.2
  y: 5160.5
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5532.8
  y: 5203.7
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5608.8
  y: 5188.5
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5107.1
  y: 5110.6
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
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5344.5
  y: 5050.4
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5317
  y: 5034.3
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5320.9
  y: 5063.3
  count: 1
  group: reinforcements
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5451.5
  y: 5083.8
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5492.9
  y: 5064.8
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5475.7
  y: 5094.6
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 542, ITEM_DROP.STB row 461
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Sikuku Hunter

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
