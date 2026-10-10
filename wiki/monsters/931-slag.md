---
kind: monster
id: 931
name: Slag
status: in-game
level: 82
hp: 2768
attack: 321
hit: 191
defence: 224
resistance: 170
avoid: 132
attack_speed: 100
attack_range: 3.5
damage: physical
walk_speed: 250
run_speed: 560
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2831
source:
  data: LIST_NPC.STB row 931, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Slag

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
