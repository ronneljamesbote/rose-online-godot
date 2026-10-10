---
kind: monster
id: 928
name: Goblin Worker
status: in-game
level: 62
hp: 151094
hp_per_level: 2437
attack: 206
hit: 149
defence: 199
resistance: 79
avoid: 75
attack_speed: 100
attack_range: 2.6
damage: physical
walk_speed: 180
run_speed: 410
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2828
source:
  data: LIST_NPC.STB row 928, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Goblin Worker

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
