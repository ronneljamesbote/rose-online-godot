---
kind: monster
id: 901
name: Jelly Bean
status: in-game
level: 26
hp: 980
attack: 80
hit: 81
defence: 87
resistance: 36
avoid: 36
attack_speed: 80
attack_range: 2
damage: physical
walk_speed: 170
run_speed: 390
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2801
source:
  data: LIST_NPC.STB row 901, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Jelly Bean

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
