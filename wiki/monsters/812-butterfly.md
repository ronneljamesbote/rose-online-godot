---
kind: monster
id: 812
name: ButterFly
status: in-game
level: 47
hp: 55319
hp_per_level: 1177
attack: 134
hit: 144
defence: 115
resistance: 60
avoid: 97
attack_speed: 102
attack_range: 3.5
damage: physical
walk_speed: 210
run_speed: 470
xp: 38
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1151-call-butterfly|Call Butterfly]]'
source:
  data: LIST_NPC.STB row 812, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# ButterFly

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
