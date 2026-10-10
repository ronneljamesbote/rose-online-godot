---
kind: monster
id: 803
name: Bonfire
status: in-game
level: 32
hp: 470
attack: 10
hit: 10
defence: 60
resistance: 43
avoid: 46
attack_speed: 106
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
  data: LIST_NPC.STB row 803, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Bonfire

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
