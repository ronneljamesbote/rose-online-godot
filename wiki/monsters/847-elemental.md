---
kind: monster
id: 847
name: Elemental
status: in-game
level: 76
hp: 2498
attack: 274
hit: 229
defence: 229
resistance: 108
avoid: 96
attack_speed: 112
attack_range: 3
damage: physical
walk_speed: 242
run_speed: 534
xp: 63
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1181-call-elemental|Call Elemental]]'
source:
  data: LIST_NPC.STB row 847, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Elemental

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
