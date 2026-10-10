---
kind: monster
id: 215
name: Aqua Guardian
status: in-game
level: 55
hp: 352
attack: 304
hit: 241
defence: 205
resistance: 143
avoid: 89
attack_speed: 125
attack_range: 3.5
damage: physical
walk_speed: 290
run_speed: 630
xp: 1722
drop_item_rate: 80
drop_money_rate: 0
zones: 1
drops:
- item: '[[items/material/161-green-crystal|Green Crystal]]'
  slots_of_30: 6
- item: '[[items/material/162-blue-crystal|Blue Crystal]]'
  slots_of_30: 6
- item: '[[items/material/151-black-hearts|Black Hearts]]'
  slots_of_30: 2
- item: '[[items/material/152-green-hearts|Green Hearts]]'
  slots_of_30: 2
- item: '[[items/material/163-red-crystal|Red Crystal]]'
  slots_of_30: 4
- item: '[[items/material/153-blue-hearts|Blue Hearts]]'
  slots_of_30: 2
- item: '[[items/gem/301-garnet-1|Garnet 1]]'
  slots_of_30: 2
- item: '[[items/gem/311-ruby-1|Ruby 1]]'
  slots_of_30: 2
- item: '[[items/material/164-white-crystal|White Crystal]]'
  slots_of_30: 2
- item: '[[items/gem/321-sapphire-1|Sapphire 1]]'
  slots_of_30: 1
spawns:
- zone: '[[zones/11-junon-clan-field|Junon Clan Field]]'
  x: 5345.8
  y: 5334
  count: 1
  group: basic
- zone: '[[zones/11-junon-clan-field|Junon Clan Field]]'
  x: 5432.6
  y: 5355.9
  count: 1
  group: basic
- zone: '[[zones/11-junon-clan-field|Junon Clan Field]]'
  x: 5425.5
  y: 5194.1
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 215, ITEM_DROP.STB row 271
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Aqua Guardian

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
