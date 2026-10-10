---
kind: monster
id: 886
name: Hawk
status: in-game
level: 77
hp: 1482
attack: 263
hit: 261
defence: 198
resistance: 133
avoid: 188
attack_speed: 120
attack_range: 1.8
damage: physical
walk_speed: 400
run_speed: 850
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1711-call-hawk|Call Hawk]]'
source:
  data: LIST_NPC.STB row 886, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Hawk

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
