---
kind: monster
id: 814
name: ButterFly
status: in-game
level: 49
hp: 1224
attack: 140
hit: 148
defence: 120
resistance: 62
avoid: 100
attack_speed: 106
attack_range: 3.5
damage: physical
walk_speed: 230
run_speed: 510
xp: 45
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1151-call-butterfly|Call Butterfly]]'
source:
  data: LIST_NPC.STB row 814, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# ButterFly

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
