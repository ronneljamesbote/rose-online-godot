---
kind: monster
id: 869
name: Mercenary Hunter
status: in-game
level: 72
hp: 1973
attack: 237
hit: 219
defence: 193
resistance: 90
avoid: 107
attack_speed: 116
attack_range: 19.5
damage: physical
walk_speed: 270
run_speed: 590
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/2361-hire-hunter|Hire Hunter]]'
source:
  data: LIST_NPC.STB row 869, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Mercenary Hunter

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
