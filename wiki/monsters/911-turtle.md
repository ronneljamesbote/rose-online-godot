---
kind: monster
id: 911
name: Turtle
status: in-game
level: 45
hp: 76950
hp_per_level: 1710
attack: 144
hit: 116
defence: 142
resistance: 58
avoid: 56
attack_speed: 110
attack_range: 2
damage: physical
walk_speed: 180
run_speed: 410
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2811
source:
  data: LIST_NPC.STB row 911, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Turtle

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
