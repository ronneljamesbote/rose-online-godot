---
kind: monster
id: 321
name: Tropical Fish
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
walk_speed: 120
run_speed: 330
xp: 4
drop_item_rate: 1
drop_money_rate: 1
zones: 2
spawns:
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5294
  y: 5350.8
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5357.1
  y: 5357.9
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5427.4
  y: 5332.8
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5250.4
  y: 5298.2
  count: 2
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5284.6
  y: 5299.9
  count: 2
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5438.2
  y: 5301.4
  count: 2
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5429.4
  y: 5253.7
  count: 2
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5323.4
  y: 5131.7
  count: 2
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5458.4
  y: 5195.1
  count: 2
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5360.8
  y: 5085.9
  count: 2
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5365
  y: 5116.1
  count: 2
  group: basic
source:
  data: LIST_NPC.STB row 321, ITEM_DROP.STB row 8
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Tropical Fish

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
