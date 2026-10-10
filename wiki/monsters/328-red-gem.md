---
kind: monster
id: 328
name: Red Gem
status: in-game
level: 86
hp: 60028
hp_per_level: 698
attack: 390
hit: 263
defence: 244
resistance: 252
avoid: 77
attack_speed: 100
attack_range: 0.1
damage: physical
walk_speed: 0
run_speed: 0
xp: 0
drop_item_rate: 160
drop_money_rate: 0
zones: 2
drops:
- item: '[[items/material/76-orange-powder|Orange Powder]]'
  slots_of_30: 1
- item: '[[items/material/77-red-powder|Red Powder]]'
  slots_of_30: 1
- item: '[[items/material/78-golden-powder|Golden Powder]]'
  slots_of_30: 1
- item: '[[items/material/79-transparent-powder|Transparent Powder]]'
  slots_of_30: 1
- item: '[[items/material/80-rainbow-powder|Rainbow Powder]]'
  slots_of_30: 1
- item: '[[items/material/81-lisent-fe|Lisent (Fe)]]'
  slots_of_30: 1
- item: '[[items/material/163-red-crystal|Red Crystal]]'
  slots_of_30: 18
- item: '[[items/gem/301-garnet-1|Garnet 1]]'
  slots_of_30: 2
- item: '[[items/gem/311-ruby-1|Ruby 1]]'
  slots_of_30: 2
- item: '[[items/gem/302-garnet-2|Garnet 2]]'
  slots_of_30: 1
spawns:
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 6274
  y: 5336
  count: 1
  group: reinforcements
- zone: '[[zones/54-crystal-snowfields|Crystal Snowfields]]'
  x: 5285.7
  y: 5309.2
  count: 1
  group: basic
- zone: '[[zones/54-crystal-snowfields|Crystal Snowfields]]'
  x: 5140.7
  y: 5167.9
  count: 1
  group: basic
- zone: '[[zones/54-crystal-snowfields|Crystal Snowfields]]'
  x: 5048.5
  y: 5025.4
  count: 1
  group: basic
- zone: '[[zones/54-crystal-snowfields|Crystal Snowfields]]'
  x: 5474.7
  y: 4866.2
  count: 1
  group: reinforcements
- zone: '[[zones/54-crystal-snowfields|Crystal Snowfields]]'
  x: 5190.1
  y: 4557
  count: 1
  group: basic
- zone: '[[zones/54-crystal-snowfields|Crystal Snowfields]]'
  x: 5304.3
  y: 4619.7
  count: 1
  group: basic
- zone: '[[zones/54-crystal-snowfields|Crystal Snowfields]]'
  x: 5333.7
  y: 4419.6
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 328, ITEM_DROP.STB row 353
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Red Gem

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
