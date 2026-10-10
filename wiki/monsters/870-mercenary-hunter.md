---
kind: monster
id: 870
name: Mercenary Hunter
status: in-game
level: 74
hp: 150220
hp_per_level: 2030
attack: 245
hit: 224
defence: 199
resistance: 93
avoid: 110
attack_speed: 118
attack_range: 20
damage: physical
walk_speed: 275
run_speed: 600
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/2361-hire-hunter|Hire Hunter]]'
source:
  data: LIST_NPC.STB row 870, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Mercenary Hunter

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
