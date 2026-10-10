---
kind: monster
id: 844
name: Elemental
status: in-game
level: 73
hp: 2400
attack: 262
hit: 222
defence: 218
resistance: 103
avoid: 93
attack_speed: 106
attack_range: 3
damage: physical
walk_speed: 221
run_speed: 492
xp: 49
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1181-call-elemental|Call Elemental]]'
source:
  data: LIST_NPC.STB row 844, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Elemental

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
