---
kind: monster
id: 838
name: Firegon
status: in-game
level: 79
hp: 167085
hp_per_level: 2115
attack: 264
hit: 237
defence: 216
resistance: 100
avoid: 117
attack_speed: 114
attack_range: 13
damage: magic
walk_speed: 262
run_speed: 574
xp: 164
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1191-call-firegon|Call Firegon]]'
source:
  data: LIST_NPC.STB row 838, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Firegon

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
