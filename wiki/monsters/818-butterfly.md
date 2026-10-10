---
kind: monster
id: 818
name: ButterFly
status: in-game
level: 53
hp: 69960
hp_per_level: 1320
attack: 152
hit: 156
defence: 131
resistance: 67
avoid: 107
attack_speed: 114
attack_range: 3.5
damage: physical
walk_speed: 270
run_speed: 590
xp: 61
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1151-call-butterfly|Call Butterfly]]'
source:
  data: LIST_NPC.STB row 818, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# ButterFly

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
