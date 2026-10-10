---
kind: monster
id: 868
name: Mercenary Hunter
status: in-game
level: 71
hp: 1945
attack: 234
hit: 217
defence: 190
resistance: 89
avoid: 105
attack_speed: 114
attack_range: 19
damage: physical
walk_speed: 265
run_speed: 580
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/2361-hire-hunter|Hire Hunter]]'
source:
  data: LIST_NPC.STB row 868, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Mercenary Hunter

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
