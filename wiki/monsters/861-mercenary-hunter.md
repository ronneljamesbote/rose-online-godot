---
kind: monster
id: 861
name: Mercenary Hunter
status: in-game
level: 64
hp: 1750
attack: 208
hit: 199
defence: 168
resistance: 80
avoid: 95
attack_speed: 100
attack_range: 15.5
damage: physical
walk_speed: 230
run_speed: 510
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/2361-hire-hunter|Hire Hunter]]'
source:
  data: LIST_NPC.STB row 861, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Mercenary Hunter

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
