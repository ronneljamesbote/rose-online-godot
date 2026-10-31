---
kind: monster
id: 315
name: FireFly
status: in-game
level: 1
hp: 94
hp_per_level: 94
attack: 20
hit: 70
defence: 500
resistance: 500
avoid: 500
attack_speed: 10
attack_range: 1
damage: physical
walk_speed: 100
run_speed: 250
xp: 4
drop_item_rate: 1
drop_money_rate: 1
zones: 1
spawns:
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5594
  y: 5170
  count: 2
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5638
  y: 5103.46
  count: 2
  group: basic
source:
  data: godot/data/monsters/315-firefly.tres, drop table godot/data/drop_tables/8-mini-bee-firefly-butterfly-and-others.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# FireFly

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].

## Changed from iROSE

iROSE never spawns it. The Adventurer's Plain redesign places it there.
