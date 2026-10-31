---
kind: monster
id: 301
name: Mini Bee
status: in-game
level: 9
hp: 153
hp_per_level: 17
attack: 10
hit: 34
defence: 45
resistance: 24
avoid: 60
attack_speed: 100
attack_range: 2.5
damage: physical
walk_speed: 280
run_speed: 400
xp: 18
drop_item_rate: 15
drop_money_rate: 1
zones: 3
spawns:
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5179.2
  y: 5556.55
  count: 3
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5255
  y: 5545
  count: 3
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5410
  y: 5470
  count: 3
  group: basic
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5238.4
  y: 5171.1
  count: 2
  group: basic
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5120.8
  y: 5135.7
  count: 3
  group: basic
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5139.8
  y: 5194.5
  count: 3
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5414
  y: 4528.9
  count: 3
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5433.5
  y: 4639.1
  count: 2
  group: basic
source:
  data: godot/data/monsters/301-mini-bee.tres, drop table godot/data/drop_tables/8-mini-bee-firefly-butterfly-and-others.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Mini Bee

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].

## Changed from iROSE

The Adventurer's Plain redesign moved its spawn points there (see [[zones/22-adventurer-s-plain|Adventurer's Plain]]).
