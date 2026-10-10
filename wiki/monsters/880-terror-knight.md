---
kind: monster
id: 880
name: Terror Knight
status: in-game
level: 88
hp: 2450
attack: 295
hit: 225
defence: 226
resistance: 111
avoid: 92
attack_speed: 109
attack_range: 3
damage: physical
walk_speed: 205
run_speed: 460
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/2371-terror-knight|Terror Knight]]'
source:
  data: LIST_NPC.STB row 880, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Terror Knight

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
