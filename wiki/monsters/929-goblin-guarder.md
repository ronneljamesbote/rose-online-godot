---
kind: monster
id: 929
name: Goblin Guarder
status: in-game
level: 75
hp: 227700
hp_per_level: 3036
attack: 256
hit: 176
defence: 248
resistance: 97
avoid: 91
attack_speed: 100
attack_range: 3
damage: physical
walk_speed: 210
run_speed: 470
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2829
source:
  data: LIST_NPC.STB row 929, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Goblin Guarder

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
