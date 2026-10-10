---
kind: monster
id: 916
name: Krawfy
status: in-game
level: 74
hp: 203130
hp_per_level: 2745
attack: 265
hit: 193
defence: 230
resistance: 82
avoid: 84
attack_speed: 110
attack_range: 3.5
damage: physical
walk_speed: 195
run_speed: 440
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2816
source:
  data: LIST_NPC.STB row 916, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Krawfy

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
