---
kind: monster
id: 318
name: ButterFly
status: in-game
level: 1
hp: 94
hp_per_level: 94
attack: 20
hit: 70
defence: 500
resistance: 500
avoid: 500
attack_speed: 15
attack_range: 1
damage: physical
walk_speed: 150
run_speed: 310
xp: 4
drop_item_rate: 1
drop_money_rate: 1
zones: 1
spawns:
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5255.6
  y: 5355.8
  count: 2
  group: basic
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5198.5
  y: 5155.9
  count: 2
  group: basic
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5198.5
  y: 5155.9
  count: 2
  group: reinforcements
source:
  data: LIST_NPC.STB row 318, ITEM_DROP.STB row 8
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# ButterFly

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
