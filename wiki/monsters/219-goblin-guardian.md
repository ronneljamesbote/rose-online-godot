---
kind: monster
id: 219
name: Goblin Guardian
status: in-game
level: 105
hp: 426
attack: 599
hit: 410
defence: 449
resistance: 289
avoid: 185
attack_speed: 110
attack_range: 2.5
damage: physical
walk_speed: 310
run_speed: 730
xp: 2217
drop_item_rate: 85
drop_money_rate: 0
zones: 1
drops:
- item: '[[items/material/161-green-crystal|Green Crystal]]'
  slots_of_30: 2
- item: '[[items/material/162-blue-crystal|Blue Crystal]]'
  slots_of_30: 6
- item: '[[items/material/163-red-crystal|Red Crystal]]'
  slots_of_30: 5
- item: '[[items/material/155-red-hearts|Red Hearts]]'
  slots_of_30: 2
- item: '[[items/material/156-golden-hearts|Golden Hearts]]'
  slots_of_30: 2
- item: '[[items/material/164-white-crystal|White Crystal]]'
  slots_of_30: 5
- item: '[[items/material/157-white-hearts|White Hearts]]'
  slots_of_30: 2
- item: '[[items/gem/351-peridot-1|Peridot 1]]'
  slots_of_30: 1
- item: '[[items/gem/361-diamond-1|Diamond 1]]'
  slots_of_30: 1
- item: '[[items/gem/352-peridot-2|Peridot 2]]'
  slots_of_30: 2
- item: '[[items/gem/362-diamond-2|Diamond 2]]'
  slots_of_30: 1
spawns:
- zone: '[[zones/11-junon-clan-field|Junon Clan Field]]'
  x: 5193.5
  y: 5197.6
  count: 1
  group: basic
- zone: '[[zones/11-junon-clan-field|Junon Clan Field]]'
  x: 5147.7
  y: 5175.4
  count: 1
  group: basic
- zone: '[[zones/11-junon-clan-field|Junon Clan Field]]'
  x: 5224.6
  y: 5248.1
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 219, ITEM_DROP.STB row 275
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Goblin Guardian

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
