---
kind: monster
id: 914
name: Woopie
status: in-game
level: 38
hp: 54378
hp_per_level: 1431
attack: 120
hit: 103
defence: 121
resistance: 49
avoid: 48
attack_speed: 90
attack_range: 3.5
damage: physical
walk_speed: 160
run_speed: 370
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2814
source:
  data: LIST_NPC.STB row 914, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Woopie

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
