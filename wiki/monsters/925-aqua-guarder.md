---
kind: monster
id: 925
name: Aqua Guarder
status: in-game
level: 48
hp: 1894
attack: 151
hit: 118
defence: 167
resistance: 73
avoid: 55
attack_speed: 95
attack_range: 3.5
damage: physical
walk_speed: 180
run_speed: 410
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2825
source:
  data: LIST_NPC.STB row 925, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Aqua Guarder

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
