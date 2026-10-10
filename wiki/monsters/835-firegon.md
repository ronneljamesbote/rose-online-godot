---
kind: monster
id: 835
name: Firegon
status: in-game
level: 76
hp: 154280
hp_per_level: 2030
attack: 252
hit: 229
defence: 206
resistance: 96
avoid: 112
attack_speed: 108
attack_range: 12.7
damage: magic
walk_speed: 244
run_speed: 538
xp: 143
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1191-call-firegon|Call Firegon]]'
source:
  data: LIST_NPC.STB row 835, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Firegon

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
