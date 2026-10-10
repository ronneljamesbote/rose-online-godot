---
kind: monster
id: 253
name: Zombie
status: in-game
level: 35
hp: 1540
hp_per_level: 44
attack: 145
hit: 121
defence: 114
resistance: 72
avoid: 44
attack_speed: 95
attack_range: 2.5
damage: physical
walk_speed: 166
run_speed: 382
xp: 57
drop_item_rate: 0
drop_money_rate: 0
quests:
- '[[quests/122-resurrection|Resurrection]]'
source:
  data: LIST_NPC.STB row 253, ITEM_DROP.STB row 0
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Zombie

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
