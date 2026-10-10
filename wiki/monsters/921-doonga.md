---
kind: monster
id: 921
name: Doonga
status: in-game
level: 67
hp: 186327
hp_per_level: 2781
attack: 218
hit: 158
defence: 210
resistance: 73
avoid: 69
attack_speed: 90
attack_range: 2.5
damage: physical
walk_speed: 200
run_speed: 400
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2821
source:
  data: LIST_NPC.STB row 921, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Doonga

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
