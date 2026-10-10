---
kind: monster
id: 908
name: Pomic
status: in-game
level: 35
hp: 1315
attack: 110
hit: 97
defence: 112
resistance: 46
avoid: 45
attack_speed: 100
attack_range: 3
damage: physical
walk_speed: 170
run_speed: 390
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2808
source:
  data: LIST_NPC.STB row 908, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Pomic

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
