---
kind: monster
id: 543
name: Sikuku Tracer
status: in-game
level: 158
hp: 6004
hp_per_level: 38
attack: 852
hit: 468
defence: 497
resistance: 420
avoid: 254
attack_speed: 100
attack_range: 2.7
damage: physical
walk_speed: 300
run_speed: 650
xp: 169
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
- item: '[[items/jewellery/251-socket-ring|Socket Ring]]'
  slots_of_30: 1
- item: '[[items/jewellery/23-gorgeous-ring|Gorgeous Ring]]'
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
  x: 5418.5
  y: 5182.8
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5329.9
  y: 5070.4
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6019.8
  y: 5079.1
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6015.1
  y: 4975.2
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5343.9
  y: 4934.8
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5418.5
  y: 4909.6
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
  data: LIST_NPC.STB row 543, ITEM_DROP.STB row 462
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Sikuku Tracer

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
