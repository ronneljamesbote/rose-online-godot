---
kind: monster
id: 816
name: ButterFly
status: in-game
level: 51
hp: 1272
attack: 146
hit: 152
defence: 125
resistance: 65
avoid: 103
attack_speed: 110
attack_range: 3.5
damage: physical
walk_speed: 250
run_speed: 550
xp: 53
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1151-call-butterfly|Call Butterfly]]'
source:
  data: LIST_NPC.STB row 816, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# ButterFly

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
