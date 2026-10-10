---
kind: monster
id: 813
name: ButterFly
status: in-game
level: 48
hp: 1200
attack: 137
hit: 146
defence: 117
resistance: 61
avoid: 98
attack_speed: 104
attack_range: 3.5
damage: physical
walk_speed: 220
run_speed: 490
xp: 41
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1151-call-butterfly|Call Butterfly]]'
source:
  data: LIST_NPC.STB row 813, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# ButterFly

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
