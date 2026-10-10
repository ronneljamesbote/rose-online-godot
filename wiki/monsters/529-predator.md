---
kind: monster
id: 529
name: Predator
status: in-game
level: 158
hp: 42
attack: 821
hit: 455
defence: 497
resistance: 402
avoid: 270
attack_speed: 110
attack_range: 2.3
damage: physical
walk_speed: 300
run_speed: 650
xp: 140
drop_item_rate: 60
drop_money_rate: 30
zones: 1
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
- item: '[[items/gem/303-garnet-3|Garnet 3]]'
  slots_of_30: 1
- item: '[[items/gem/313-ruby-3|Ruby 3]]'
  slots_of_30: 1
- item: '[[items/jewellery/91-glamour-necklace|Glamour Necklace]]'
  slots_of_30: 1
- item: '[[items/material/155-red-hearts|Red Hearts]]'
  slots_of_30: 1
spawns:
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6349.6
  y: 5258.3
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6124.4
  y: 5103.1
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6249
  y: 4912
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 529, ITEM_DROP.STB row 451
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Predator

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
