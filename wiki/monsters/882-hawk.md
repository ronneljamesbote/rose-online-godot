---
kind: monster
id: 882
name: Hawk
status: in-game
level: 73
hp: 102127
hp_per_level: 1399
attack: 247
hit: 250
defence: 186
resistance: 125
avoid: 176
attack_speed: 112
attack_range: 1.8
damage: physical
walk_speed: 368
run_speed: 786
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1711-call-hawk|Call Hawk]]'
source:
  data: godot/data/monsters/882-hawk.tres, drop table godot/data/drop_tables/3-bonfire-butterfly-phantomsword-and-others.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Hawk

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
