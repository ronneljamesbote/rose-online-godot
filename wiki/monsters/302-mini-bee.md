---
kind: monster
id: 302
name: Mini Bee
status: in-game
level: 13
hp: 16
attack: 20
hit: 70
defence: 51
resistance: 28
avoid: 90
attack_speed: 100
attack_range: 2.5
damage: physical
walk_speed: 280
run_speed: 400
xp: 30
drop_item_rate: 20
drop_money_rate: 1
zones: 2
spawns:
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5538.5
  y: 4941.6
  count: 2
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5377.3
  y: 4614.8
  count: 2
  group: basic
source:
  data: LIST_NPC.STB row 302, ITEM_DROP.STB row 8
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Mini Bee

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
