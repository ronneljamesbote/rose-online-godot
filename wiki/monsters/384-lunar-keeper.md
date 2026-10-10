---
kind: monster
id: 384
name: Lunar Keeper
status: in-game
level: 122
hp: 5246
hp_per_level: 43
attack: 518
hit: 281
defence: 293
resistance: 466
avoid: 188
attack_speed: 100
attack_range: 13.5
damage: magic
walk_speed: 230
run_speed: 510
xp: 77
drop_item_rate: 30
drop_money_rate: 50
zones: 2
spawns:
- zone: '[[zones/54-crystal-snowfields|Crystal Snowfields]]'
  x: 5179.6
  y: 4992.4
  count: 1
  group: basic
- zone: '[[zones/54-crystal-snowfields|Crystal Snowfields]]'
  x: 5179.6
  y: 4992.4
  count: 2
  group: basic
- zone: '[[zones/54-crystal-snowfields|Crystal Snowfields]]'
  x: 5179.6
  y: 4992.4
  count: 1
  group: basic
- zone: '[[zones/54-crystal-snowfields|Crystal Snowfields]]'
  x: 5179.6
  y: 4992.4
  count: 2
  group: reinforcements
- zone: '[[zones/54-crystal-snowfields|Crystal Snowfields]]'
  x: 5204.1
  y: 5013.3
  count: 2
  group: basic
- zone: '[[zones/54-crystal-snowfields|Crystal Snowfields]]'
  x: 5204.1
  y: 5013.3
  count: 1
  group: basic
- zone: '[[zones/54-crystal-snowfields|Crystal Snowfields]]'
  x: 5204.1
  y: 5013.3
  count: 2
  group: reinforcements
- zone: '[[zones/54-crystal-snowfields|Crystal Snowfields]]'
  x: 5255.2
  y: 5037.8
  count: 2
  group: basic
- zone: '[[zones/54-crystal-snowfields|Crystal Snowfields]]'
  x: 5255.2
  y: 5037.8
  count: 1
  group: basic
- zone: '[[zones/54-crystal-snowfields|Crystal Snowfields]]'
  x: 5255.2
  y: 5037.8
  count: 2
  group: reinforcements
- zone: '[[zones/54-crystal-snowfields|Crystal Snowfields]]'
  x: 5334.6
  y: 5058.3
  count: 1
  group: basic
- zone: '[[zones/54-crystal-snowfields|Crystal Snowfields]]'
  x: 5334.6
  y: 5058.3
  count: 2
  group: basic
- zone: '[[zones/54-crystal-snowfields|Crystal Snowfields]]'
  x: 5334.6
  y: 5058.3
  count: 2
  group: reinforcements
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5492.5
  y: 4934.7
  count: 1
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5517.3
  y: 4891.4
  count: 1
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5393.6
  y: 4641
  count: 3
  group: basic
source:
  data: LIST_NPC.STB row 384, ITEM_DROP.STB row 0
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Lunar Keeper

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
