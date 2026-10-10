---
kind: monster
id: 902
name: JellyNut
status: in-game
level: 28
hp: 960
attack: 94
hit: 100
defence: 86
resistance: 32
avoid: 36
attack_speed: 90
attack_range: 2.3
damage: physical
walk_speed: 180
run_speed: 410
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2802
source:
  data: LIST_NPC.STB row 902, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# JellyNut

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
