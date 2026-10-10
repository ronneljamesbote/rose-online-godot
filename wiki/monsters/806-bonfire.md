---
kind: monster
id: 806
name: Bonfire
status: in-game
level: 35
hp: 22225
hp_per_level: 635
attack: 10
hit: 10
defence: 75
resistance: 46
avoid: 49
attack_speed: 100
attack_range: 0
damage: physical
walk_speed: 0
run_speed: 0
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1161-bonfire|Bonfire]]'
source:
  data: LIST_NPC.STB row 806, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Bonfire

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
