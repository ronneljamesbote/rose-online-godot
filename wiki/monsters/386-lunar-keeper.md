---
kind: monster
id: 386
name: Lunar Keeper
status: in-game
level: 131
hp: 5764
hp_per_level: 44
attack: 564
hit: 304
defence: 322
resistance: 508
avoid: 204
attack_speed: 110
attack_range: 15
damage: magic
walk_speed: 280
run_speed: 610
xp: 88
drop_item_rate: 33
drop_money_rate: 65
zones: 1
spawns:
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5530.9
  y: 5005.1
  count: 1
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5530.9
  y: 5005.1
  count: 2
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5442.4
  y: 5007.8
  count: 1
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5442.4
  y: 5007.8
  count: 2
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5618.6
  y: 5047.7
  count: 1
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5618.6
  y: 5047.7
  count: 2
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5785.3
  y: 4982.2
  count: 1
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5785.3
  y: 4982.2
  count: 2
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5422.4
  y: 4935.9
  count: 2
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5771.5
  y: 4928.5
  count: 1
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5771.5
  y: 4928.5
  count: 2
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5784.4
  y: 4950
  count: 1
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5784.4
  y: 4950
  count: 2
  group: basic
source:
  data: LIST_NPC.STB row 386, ITEM_DROP.STB row 0
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Lunar Keeper

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
