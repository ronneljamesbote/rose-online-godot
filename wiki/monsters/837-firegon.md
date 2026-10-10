---
kind: monster
id: 837
name: Firegon
status: in-game
level: 78
hp: 2086
attack: 260
hit: 234
defence: 213
resistance: 99
avoid: 115
attack_speed: 112
attack_range: 12.9
damage: magic
walk_speed: 256
run_speed: 562
xp: 156
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1191-call-firegon|Call Firegon]]'
source:
  data: LIST_NPC.STB row 837, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Firegon

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
