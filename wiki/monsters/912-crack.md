---
kind: monster
id: 912
name: Crack
status: in-game
level: 39
hp: 57330
hp_per_level: 1470
attack: 123
hit: 104
defence: 124
resistance: 51
avoid: 50
attack_speed: 90
attack_range: 2.5
damage: physical
walk_speed: 170
run_speed: 390
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2812
source:
  data: LIST_NPC.STB row 912, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Crack

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
