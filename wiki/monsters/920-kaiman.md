---
kind: monster
id: 920
name: Kaiman
status: in-game
level: 70
hp: 2572
attack: 249
hit: 184
defence: 215
resistance: 77
avoid: 79
attack_speed: 100
attack_range: 3.5
damage: physical
walk_speed: 200
run_speed: 450
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2820
source:
  data: LIST_NPC.STB row 920, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Kaiman

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
