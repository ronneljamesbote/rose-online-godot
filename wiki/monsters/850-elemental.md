---
kind: monster
id: 850
name: Elemental
status: in-game
level: 80
hp: 2631
attack: 290
hit: 239
defence: 243
resistance: 114
avoid: 101
attack_speed: 118
attack_range: 3
damage: physical
walk_speed: 263
run_speed: 576
xp: 79
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1181-call-elemental|Call Elemental]]'
source:
  data: LIST_NPC.STB row 850, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Elemental

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
