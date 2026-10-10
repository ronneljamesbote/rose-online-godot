---
kind: monster
id: 934
name: Yeti Hunter
status: in-game
level: 94
hp: 4089
attack: 374
hit: 265
defence: 311
resistance: 158
avoid: 140
attack_speed: 100
attack_range: 16
damage: physical
walk_speed: 210
run_speed: 530
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2834
source:
  data: LIST_NPC.STB row 934, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Yeti Hunter

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
