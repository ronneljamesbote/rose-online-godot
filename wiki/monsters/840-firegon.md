---
kind: monster
id: 840
name: Firegon
status: in-game
level: 82
hp: 180482
hp_per_level: 2201
attack: 275
hit: 244
defence: 226
resistance: 104
avoid: 121
attack_speed: 118
attack_range: 13.2
damage: magic
walk_speed: 274
run_speed: 598
xp: 179
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1191-call-firegon|Call Firegon]]'
source:
  data: LIST_NPC.STB row 840, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Firegon

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
