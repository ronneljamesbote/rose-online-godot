---
kind: monster
id: 548
name: Sikuku Assassine
status: in-game
level: 155
hp: 35
attack: 773
hit: 524
defence: 399
resistance: 443
avoid: 301
attack_speed: 90
attack_range: 15
damage: physical
walk_speed: 300
run_speed: 650
xp: 148
drop_item_rate: 50
drop_money_rate: 30
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
- item: '[[items/jewellery/251-socket-ring|Socket Ring]]'
  slots_of_30: 1
- item: '[[items/jewellery/24-dazzling-ring|Dazzling Ring]]'
  slots_of_30: 1
- item: '[[items/weapon/219-zephyr-long-bow|Zephyr Long Bow]]'
  slots_of_30: 0.2
- item: '[[items/weapon/248-shok-impact-gun|Shok Impact Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/276-breakdown-canon|Breakdown Canon]]'
  slots_of_30: 0.2
- item: '[[items/weapon/318-twinkle-staff|Twinkle Staff]]'
  slots_of_30: 0.2
- item: '[[items/weapon/347-snake-wand|Snake Wand]]'
  slots_of_30: 0.2
spawns:
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5367.3
  y: 5170.9
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5740.5
  y: 4961.7
  count: 1
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
  x: 5612.3
  y: 4850.4
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5828.3
  y: 4902.4
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 548, ITEM_DROP.STB row 467
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Sikuku Assassine

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
