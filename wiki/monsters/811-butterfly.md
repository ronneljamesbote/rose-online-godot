---
kind: monster
id: 811
name: ButterFly
status: in-game
level: 46
hp: 1154
attack: 131
hit: 142
defence: 112
resistance: 59
avoid: 95
attack_speed: 100
attack_range: 3.5
damage: physical
walk_speed: 200
run_speed: 450
xp: 34
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1151-call-butterfly|Call Butterfly]]'
source:
  data: LIST_NPC.STB row 811, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# ButterFly

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
