---
kind: monster
id: 849
name: Elemental
status: in-game
level: 78
hp: 2564
attack: 282
hit: 234
defence: 236
resistance: 111
avoid: 99
attack_speed: 116
attack_range: 3
damage: physical
walk_speed: 256
run_speed: 562
xp: 74
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1181-call-elemental|Call Elemental]]'
source:
  data: LIST_NPC.STB row 849, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Elemental

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
