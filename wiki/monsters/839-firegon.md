---
kind: monster
id: 839
name: Firegon
status: in-game
level: 80
hp: 171440
hp_per_level: 2143
attack: 267
hit: 239
defence: 220
resistance: 102
avoid: 118
attack_speed: 116
attack_range: 13.1
damage: magic
walk_speed: 268
run_speed: 586
xp: 171
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1191-call-firegon|Call Firegon]]'
source:
  data: LIST_NPC.STB row 839, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Firegon

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
