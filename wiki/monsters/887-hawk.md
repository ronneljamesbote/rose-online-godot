---
kind: monster
id: 887
name: Hawk
status: in-game
level: 78
hp: 117156
hp_per_level: 1502
attack: 267
hit: 263
defence: 201
resistance: 134
avoid: 191
attack_speed: 122
attack_range: 1.8
damage: physical
walk_speed: 408
run_speed: 866
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1711-call-hawk|Call Hawk]]'
source:
  data: godot/data/monsters/887-hawk.tres, drop table godot/data/drop_tables/3-bonfire-butterfly-phantomsword-and-others.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Hawk

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
