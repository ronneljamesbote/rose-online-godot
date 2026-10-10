---
kind: monster
id: 913
name: Rackie
status: in-game
level: 44
hp: 1473
attack: 160
hit: 116
defence: 119
resistance: 56
avoid: 57
attack_speed: 90
attack_range: 3
damage: physical
walk_speed: 145
run_speed: 330
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2813
source:
  data: LIST_NPC.STB row 913, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Rackie

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
