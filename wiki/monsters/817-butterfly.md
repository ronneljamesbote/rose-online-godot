---
kind: monster
id: 817
name: ButterFly
status: in-game
level: 52
hp: 1296
attack: 149
hit: 154
defence: 128
resistance: 66
avoid: 105
attack_speed: 112
attack_range: 3.5
damage: physical
walk_speed: 260
run_speed: 570
xp: 57
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1151-call-butterfly|Call Butterfly]]'
source:
  data: LIST_NPC.STB row 817, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# ButterFly

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
