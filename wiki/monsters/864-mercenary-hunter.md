---
kind: monster
id: 864
name: Mercenary Hunter
status: in-game
level: 67
hp: 122811
hp_per_level: 1833
attack: 219
hit: 207
defence: 177
resistance: 84
avoid: 99
attack_speed: 106
attack_range: 17
damage: physical
walk_speed: 245
run_speed: 540
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/2361-hire-hunter|Hire Hunter]]'
source:
  data: godot/data/monsters/864-mercenary-hunter.tres, drop table godot/data/drop_tables/3-bonfire-butterfly-phantomsword-and-others.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Mercenary Hunter

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
