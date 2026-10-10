---
kind: monster
id: 936
name: Yeti Rider
status: in-game
level: 100
hp: 440900
hp_per_level: 4409
attack: 401
hit: 260
defence: 367
resistance: 170
avoid: 135
attack_speed: 100
attack_range: 3
damage: physical
walk_speed: 280
run_speed: 650
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2836
source:
  data: LIST_NPC.STB row 936, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Yeti Rider

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
