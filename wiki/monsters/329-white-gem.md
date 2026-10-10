---
kind: monster
id: 329
name: White Gem
status: in-game
level: 89
hp: 706
attack: 405
hit: 271
defence: 255
resistance: 261
avoid: 80
attack_speed: 100
attack_range: 0.1
damage: physical
walk_speed: 0
run_speed: 0
xp: 0
drop_item_rate: 170
drop_money_rate: 0
zones: 1
drops:
- item: '[[items/material/76-orange-powder|Orange Powder]]'
  slots_of_30: 1
- item: '[[items/material/77-red-powder|Red Powder]]'
  slots_of_30: 1
- item: '[[items/material/79-transparent-powder|Transparent Powder]]'
  slots_of_30: 1
- item: '[[items/material/80-rainbow-powder|Rainbow Powder]]'
  slots_of_30: 1
- item: '[[items/material/81-lisent-fe|Lisent (Fe)]]'
  slots_of_30: 1
- item: '[[items/material/82-lisent-cu|Lisent (Cu)]]'
  slots_of_30: 1
- item: '[[items/material/164-white-crystal|White Crystal]]'
  slots_of_30: 18
- item: '[[items/gem/361-diamond-1|Diamond 1]]'
  slots_of_30: 4
- item: '[[items/gem/362-diamond-2|Diamond 2]]'
  slots_of_30: 1
spawns:
- zone: '[[zones/54-crystal-snowfields|Crystal Snowfields]]'
  x: 5312.3
  y: 5341
  count: 1
  group: reinforcements
- zone: '[[zones/54-crystal-snowfields|Crystal Snowfields]]'
  x: 5416.8
  y: 5188.9
  count: 1
  group: reinforcements
- zone: '[[zones/54-crystal-snowfields|Crystal Snowfields]]'
  x: 5626.7
  y: 4786
  count: 1
  group: reinforcements
- zone: '[[zones/54-crystal-snowfields|Crystal Snowfields]]'
  x: 5608.4
  y: 4540.2
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 329, ITEM_DROP.STB row 354
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# White Gem

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
