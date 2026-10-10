---
kind: monster
id: 327
name: Blue Gem
status: in-game
level: 83
hp: 57353
hp_per_level: 691
attack: 375
hit: 255
defence: 233
resistance: 243
avoid: 75
attack_speed: 100
attack_range: 0.1
damage: physical
walk_speed: 0
run_speed: 0
xp: 0
drop_item_rate: 150
drop_money_rate: 0
zones: 2
drops:
- item: '[[items/material/75-pink-powder|Pink Powder]]'
  slots_of_30: 1
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
- item: '[[items/material/162-blue-crystal|Blue Crystal]]'
  slots_of_30: 18
- item: '[[items/gem/321-sapphire-1|Sapphire 1]]'
  slots_of_30: 2
- item: '[[items/gem/331-topaz-1|Topaz 1]]'
  slots_of_30: 2
- item: '[[items/gem/322-sapphire-2|Sapphire 2]]'
  slots_of_30: 1
spawns:
- zone: '[[zones/52-mana-snowfields|Mana Snowfields]]'
  x: 5376.1
  y: 5183.9
  count: 1
  group: basic
- zone: '[[zones/52-mana-snowfields|Mana Snowfields]]'
  x: 5720.2
  y: 4909.9
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5156.5
  y: 5324.1
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5604.3
  y: 5332.5
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 6274
  y: 5336
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5713.4
  y: 5235.6
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5744.2
  y: 4972.6
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5788.7
  y: 5009.7
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5190.1
  y: 4874.3
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5245.6
  y: 4797.7
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 327, ITEM_DROP.STB row 352
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Blue Gem

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
