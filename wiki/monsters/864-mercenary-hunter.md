---
kind: monster
id: 864
name: Mercenary Hunter
status: in-game
level: 67
hp: 1833
attack: 219
hit: 207
defence: 177
resistance: 84
avoid: 99
attack_speed: 106
attack_range: 17
damage: physical
walk_speed: 245
run_speed: 540
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/2361-hire-hunter|Hire Hunter]]'
source:
  data: LIST_NPC.STB row 864, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Mercenary Hunter

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
