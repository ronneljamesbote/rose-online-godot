---
kind: monster
id: 923
name: Doonga Captain
status: in-game
level: 79
hp: 270970
hp_per_level: 3430
attack: 291
hit: 218
defence: 268
resistance: 112
avoid: 100
attack_speed: 110
attack_range: 5
damage: physical
walk_speed: 230
run_speed: 510
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2823
source:
  data: LIST_NPC.STB row 923, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Doonga Captain

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
