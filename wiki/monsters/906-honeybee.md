---
kind: monster
id: 906
name: HoneyBee
status: in-game
level: 32
hp: 38432
hp_per_level: 1201
attack: 100
hit: 92
defence: 104
resistance: 43
avoid: 42
attack_speed: 80
attack_range: 2
damage: physical
walk_speed: 220
run_speed: 300
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2806
source:
  data: godot/data/monsters/906-honeybee.tres, drop table godot/data/drop_tables/3-bonfire-butterfly-phantomsword-and-others.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# HoneyBee

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
