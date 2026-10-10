---
kind: monster
id: 917
name: Porkie
status: in-game
level: 50
hp: 1979
attack: 158
hit: 121
defence: 174
resistance: 76
avoid: 57
attack_speed: 100
attack_range: 2.5
damage: physical
walk_speed: 170
run_speed: 390
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2817
source:
  data: LIST_NPC.STB row 917, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Porkie

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
