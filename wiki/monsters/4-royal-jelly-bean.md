---
kind: monster
id: 4
name: Royal Jelly Bean
status: in-game
level: 4
hp: 120
hp_per_level: 30
attack: 7
hit: 78
defence: 34
resistance: 5
avoid: 10
attack_speed: 110
attack_range: 2
damage: physical
walk_speed: 220
run_speed: 490
xp: 14
drop_item_rate: 73
drop_money_rate: 15
zones: 1
spawns:
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5640
  y: 5290
  count: 1
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5600
  y: 5385
  count: 1
  group: basic
source:
  data: godot/data/monsters/4-royal-jelly-bean.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Royal Jelly Bean

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].

## Changed from iROSE

iROSE never spawns it. The Adventurer's Plain redesign places it there.
