---
kind: monster
id: 888
name: Hawk
status: in-game
level: 79
hp: 1523
attack: 270
hit: 266
defence: 205
resistance: 136
avoid: 194
attack_speed: 124
attack_range: 1.8
damage: physical
walk_speed: 416
run_speed: 882
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1711-call-hawk|Call Hawk]]'
source:
  data: LIST_NPC.STB row 888, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Hawk

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
