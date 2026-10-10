---
kind: monster
id: 831
name: Firegon
status: in-game
level: 72
hp: 1919
attack: 237
hit: 219
defence: 193
resistance: 90
avoid: 107
attack_speed: 100
attack_range: 12
damage: magic
walk_speed: 220
run_speed: 490
xp: 117
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1191-call-firegon|Call Firegon]]'
source:
  data: LIST_NPC.STB row 831, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Firegon

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
