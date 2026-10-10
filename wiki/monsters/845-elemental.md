---
kind: monster
id: 845
name: Elemental
status: in-game
level: 74
hp: 179968
hp_per_level: 2432
attack: 266
hit: 225
defence: 222
resistance: 105
avoid: 94
attack_speed: 108
attack_range: 3
damage: physical
walk_speed: 228
run_speed: 506
xp: 53
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1181-call-elemental|Call Elemental]]'
source:
  data: LIST_NPC.STB row 845, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Elemental

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
