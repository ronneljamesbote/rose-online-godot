---
kind: monster
id: 317
name: ButterFly
status: in-game
level: 1
hp: 94
attack: 20
hit: 70
defence: 500
resistance: 500
avoid: 500
attack_speed: 15
attack_range: 1
damage: physical
walk_speed: 140
run_speed: 300
xp: 4
drop_item_rate: 1
drop_money_rate: 1
zones: 2
spawns:
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5255.6
  y: 5355.8
  count: 2
  group: basic
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5198.5
  y: 5155.9
  count: 3
  group: basic
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5269.2
  y: 5049.3
  count: 1
  group: basic
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5234.5
  y: 5055.6
  count: 1
  group: basic
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5387.3
  y: 5103.1
  count: 1
  group: basic
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5280.7
  y: 5046.1
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5495
  y: 5104.4
  count: 2
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5659.1
  y: 4440.1
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 317, ITEM_DROP.STB row 8
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# ButterFly

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
