---
kind: monster
id: 843
name: Elemental
status: in-game
level: 72
hp: 2367
attack: 258
hit: 220
defence: 214
resistance: 102
avoid: 91
attack_speed: 104
attack_range: 3
damage: physical
walk_speed: 214
run_speed: 478
xp: 45
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1181-call-elemental|Call Elemental]]'
source:
  data: LIST_NPC.STB row 843, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Elemental

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
