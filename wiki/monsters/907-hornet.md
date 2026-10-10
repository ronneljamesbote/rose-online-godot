---
kind: monster
id: 907
name: Hornet
status: in-game
level: 38
hp: 1308
attack: 128
hit: 119
defence: 113
resistance: 41
avoid: 45
attack_speed: 80
attack_range: 2.5
damage: physical
walk_speed: 260
run_speed: 360
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2807
source:
  data: LIST_NPC.STB row 907, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Hornet

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
