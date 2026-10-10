---
kind: monster
id: 326
name: Green Gem
status: in-game
level: 80
hp: 54720
hp_per_level: 684
attack: 360
hit: 247
defence: 222
resistance: 234
avoid: 72
attack_speed: 100
attack_range: 0.1
damage: physical
walk_speed: 0
run_speed: 0
xp: 0
drop_item_rate: 140
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
- item: '[[items/material/161-green-crystal|Green Crystal]]'
  slots_of_30: 18
- item: '[[items/gem/341-emerald-1|Emerald 1]]'
  slots_of_30: 2
- item: '[[items/gem/351-peridot-1|Peridot 1]]'
  slots_of_30: 2
- item: '[[items/gem/342-emerald-2|Emerald 2]]'
  slots_of_30: 1
spawns:
- zone: '[[zones/52-mana-snowfields|Mana Snowfields]]'
  x: 5958.7
  y: 5154.5
  count: 1
  group: basic
- zone: '[[zones/52-mana-snowfields|Mana Snowfields]]'
  x: 5365.1
  y: 4972.1
  count: 1
  group: basic
- zone: '[[zones/52-mana-snowfields|Mana Snowfields]]'
  x: 5746.7
  y: 4985.5
  count: 1
  group: reinforcements
- zone: '[[zones/52-mana-snowfields|Mana Snowfields]]'
  x: 5986.8
  y: 5059.3
  count: 1
  group: reinforcements
- zone: '[[zones/52-mana-snowfields|Mana Snowfields]]'
  x: 5068.2
  y: 4857
  count: 1
  group: reinforcements
- zone: '[[zones/52-mana-snowfields|Mana Snowfields]]'
  x: 5102
  y: 4884.9
  count: 1
  group: reinforcements
- zone: '[[zones/52-mana-snowfields|Mana Snowfields]]'
  x: 5075.7
  y: 4791.1
  count: 1
  group: reinforcements
- zone: '[[zones/52-mana-snowfields|Mana Snowfields]]'
  x: 5614.4
  y: 4791.8
  count: 1
  group: basic
- zone: '[[zones/52-mana-snowfields|Mana Snowfields]]'
  x: 5536.9
  y: 4550.7
  count: 1
  group: basic
- zone: '[[zones/52-mana-snowfields|Mana Snowfields]]'
  x: 5469.3
  y: 4621.6
  count: 1
  group: reinforcements
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 6274
  y: 5336
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 6274
  y: 5336
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5883.6
  y: 5267
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5760
  y: 5000.7
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5333.2
  y: 4821.2
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 326, ITEM_DROP.STB row 351
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Green Gem

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
