---
kind: monster
id: 885
name: Hawk
status: in-game
level: 76
hp: 111036
hp_per_level: 1461
attack: 259
hit: 258
defence: 195
resistance: 131
avoid: 185
attack_speed: 118
attack_range: 1.8
damage: physical
walk_speed: 392
run_speed: 834
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1711-call-hawk|Call Hawk]]'
source:
  data: LIST_NPC.STB row 885, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Hawk

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
