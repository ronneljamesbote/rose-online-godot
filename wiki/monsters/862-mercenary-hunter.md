---
kind: monster
id: 862
name: Mercenary Hunter
status: in-game
level: 65
hp: 1778
attack: 212
hit: 202
defence: 171
resistance: 81
avoid: 97
attack_speed: 102
attack_range: 16
damage: physical
walk_speed: 235
run_speed: 520
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/2361-hire-hunter|Hire Hunter]]'
source:
  data: LIST_NPC.STB row 862, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Mercenary Hunter

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
