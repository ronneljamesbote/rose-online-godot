---
kind: monster
id: 834
name: Firegon
status: in-game
level: 75
hp: 2002
attack: 249
hit: 227
defence: 203
resistance: 95
avoid: 111
attack_speed: 106
attack_range: 12.6
damage: magic
walk_speed: 238
run_speed: 526
xp: 133
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1191-call-firegon|Call Firegon]]'
source:
  data: LIST_NPC.STB row 834, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Firegon

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
