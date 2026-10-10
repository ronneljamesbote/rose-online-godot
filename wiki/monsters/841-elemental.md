---
kind: monster
id: 841
name: Elemental
status: in-game
level: 70
hp: 161210
hp_per_level: 2303
attack: 250
hit: 215
defence: 208
resistance: 99
avoid: 89
attack_speed: 100
attack_range: 3
damage: physical
walk_speed: 200
run_speed: 450
xp: 36
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1181-call-elemental|Call Elemental]]'
source:
  data: LIST_NPC.STB row 841, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Elemental

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
