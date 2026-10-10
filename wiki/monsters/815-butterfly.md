---
kind: monster
id: 815
name: ButterFly
status: in-game
level: 50
hp: 1248
attack: 143
hit: 150
defence: 123
resistance: 64
avoid: 102
attack_speed: 108
attack_range: 3.5
damage: physical
walk_speed: 240
run_speed: 530
xp: 49
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1151-call-butterfly|Call Butterfly]]'
source:
  data: LIST_NPC.STB row 815, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# ButterFly

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
