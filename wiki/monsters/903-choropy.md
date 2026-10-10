---
kind: monster
id: 903
name: Choropy
status: in-game
level: 27
hp: 28566
hp_per_level: 1058
attack: 81
hit: 80
defence: 100
resistance: 46
avoid: 35
attack_speed: 90
attack_range: 2
damage: physical
walk_speed: 160
run_speed: 240
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2803
source:
  data: LIST_NPC.STB row 903, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Choropy

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
