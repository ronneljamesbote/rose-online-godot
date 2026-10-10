---
kind: monster
id: 819
name: ButterFly
status: in-game
level: 54
hp: 1344
attack: 156
hit: 158
defence: 133
resistance: 69
avoid: 109
attack_speed: 116
attack_range: 3.5
damage: physical
walk_speed: 280
run_speed: 610
xp: 65
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1151-call-butterfly|Call Butterfly]]'
source:
  data: LIST_NPC.STB row 819, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# ButterFly

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
