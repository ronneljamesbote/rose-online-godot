---
kind: monster
id: 837
name: Firegon
status: in-game
level: 78
hp: 162708
hp_per_level: 2086
attack: 260
hit: 234
defence: 213
resistance: 99
avoid: 115
attack_speed: 112
attack_range: 12.9
damage: magic
walk_speed: 256
run_speed: 562
xp: 156
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1191-call-firegon|Call Firegon]]'
source:
  data: godot/data/monsters/837-firegon.tres, drop table godot/data/drop_tables/3-bonfire-butterfly-phantomsword-and-others.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Firegon

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
