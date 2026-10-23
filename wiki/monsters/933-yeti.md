---
kind: monster
id: 933
name: Yeti
status: in-game
level: 91
hp: 357812
hp_per_level: 3932
attack: 361
hit: 238
defence: 328
resistance: 153
avoid: 122
attack_speed: 100
attack_range: 3
damage: physical
walk_speed: 220
run_speed: 550
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2833
source:
  data: godot/data/monsters/933-yeti.tres, drop table godot/data/drop_tables/3-bonfire-butterfly-phantomsword-and-others.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Yeti

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
