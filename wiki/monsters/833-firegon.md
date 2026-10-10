---
kind: monster
id: 833
name: Firegon
status: in-game
level: 74
hp: 1974
attack: 245
hit: 224
defence: 199
resistance: 93
avoid: 110
attack_speed: 104
attack_range: 12.4
damage: magic
walk_speed: 232
run_speed: 514
xp: 127
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1191-call-firegon|Call Firegon]]'
source:
  data: LIST_NPC.STB row 833, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Firegon

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
