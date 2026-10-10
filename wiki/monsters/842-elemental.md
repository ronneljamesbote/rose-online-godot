---
kind: monster
id: 842
name: Elemental
status: in-game
level: 71
hp: 2335
attack: 254
hit: 217
defence: 211
resistance: 100
avoid: 90
attack_speed: 102
attack_range: 3
damage: physical
walk_speed: 207
run_speed: 464
xp: 40
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1181-call-elemental|Call Elemental]]'
source:
  data: LIST_NPC.STB row 842, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Elemental

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
