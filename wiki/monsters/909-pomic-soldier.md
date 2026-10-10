---
kind: monster
id: 909
name: Pomic Soldier
status: in-game
level: 36
hp: 44496
hp_per_level: 1236
attack: 121
hit: 115
defence: 107
resistance: 39
avoid: 43
attack_speed: 100
attack_range: 3.5
damage: physical
walk_speed: 180
run_speed: 410
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2809
source:
  data: LIST_NPC.STB row 909, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Pomic Soldier

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
