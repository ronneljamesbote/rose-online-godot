---
kind: monster
id: 832
name: Firegon
status: in-game
level: 73
hp: 142058
hp_per_level: 1946
attack: 241
hit: 222
defence: 196
resistance: 92
avoid: 108
attack_speed: 102
attack_range: 12.2
damage: magic
walk_speed: 226
run_speed: 502
xp: 124
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1191-call-firegon|Call Firegon]]'
source:
  data: LIST_NPC.STB row 832, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Firegon

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
