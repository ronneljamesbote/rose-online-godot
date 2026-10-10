---
kind: monster
id: 218
name: Golem Guardian
status: in-game
level: 95
hp: 39140
hp_per_level: 412
attack: 530
hit: 376
defence: 381
resistance: 246
avoid: 163
attack_speed: 110
attack_range: 2.5
damage: physical
walk_speed: 320
run_speed: 690
xp: 2135
drop_item_rate: 84
drop_money_rate: 0
zones: 1
drops:
- item: '[[items/material/161-green-crystal|Green Crystal]]'
  slots_of_30: 3
- item: '[[items/material/162-blue-crystal|Blue Crystal]]'
  slots_of_30: 5
- item: '[[items/material/163-red-crystal|Red Crystal]]'
  slots_of_30: 5
- item: '[[items/material/153-blue-hearts|Blue Hearts]]'
  slots_of_30: 2
- item: '[[items/material/154-pink-hearts|Pink Hearts]]'
  slots_of_30: 2
- item: '[[items/material/155-red-hearts|Red Hearts]]'
  slots_of_30: 2
- item: '[[items/material/164-white-crystal|White Crystal]]'
  slots_of_30: 5
- item: '[[items/gem/341-emerald-1|Emerald 1]]'
  slots_of_30: 1
- item: '[[items/gem/351-peridot-1|Peridot 1]]'
  slots_of_30: 1
- item: '[[items/gem/302-garnet-2|Garnet 2]]'
  slots_of_30: 1
- item: '[[items/gem/332-topaz-2|Topaz 2]]'
  slots_of_30: 1
- item: '[[items/gem/352-peridot-2|Peridot 2]]'
  slots_of_30: 1
spawns:
- zone: '[[zones/11-junon-clan-field|Junon Clan Field]]'
  x: 5084.3
  y: 5378.7
  count: 1
  group: basic
- zone: '[[zones/11-junon-clan-field|Junon Clan Field]]'
  x: 5184.6
  y: 5367.4
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 218, ITEM_DROP.STB row 274
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Golem Guardian

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
