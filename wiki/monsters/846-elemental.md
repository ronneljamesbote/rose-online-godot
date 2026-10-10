---
kind: monster
id: 846
name: Elemental
status: in-game
level: 75
hp: 2465
attack: 270
hit: 227
defence: 225
resistance: 106
avoid: 95
attack_speed: 110
attack_range: 3
damage: physical
walk_speed: 235
run_speed: 520
xp: 59
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1181-call-elemental|Call Elemental]]'
source:
  data: LIST_NPC.STB row 846, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Elemental

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
