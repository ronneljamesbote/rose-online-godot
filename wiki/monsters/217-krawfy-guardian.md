---
kind: monster
id: 217
name: Krawfy Guardian
status: in-game
level: 85
hp: 33745
hp_per_level: 397
attack: 466
hit: 341
defence: 339
resistance: 214
avoid: 147
attack_speed: 130
attack_range: 2.5
damage: physical
walk_speed: 330
run_speed: 710
xp: 2280
drop_item_rate: 83
drop_money_rate: 0
zones: 1
drops:
- item: '[[items/material/161-green-crystal|Green Crystal]]'
  slots_of_30: 3
- item: '[[items/material/162-blue-crystal|Blue Crystal]]'
  slots_of_30: 7
- item: '[[items/material/152-green-hearts|Green Hearts]]'
  slots_of_30: 2
- item: '[[items/material/163-red-crystal|Red Crystal]]'
  slots_of_30: 5
- item: '[[items/material/153-blue-hearts|Blue Hearts]]'
  slots_of_30: 2
- item: '[[items/material/154-pink-hearts|Pink Hearts]]'
  slots_of_30: 2
- item: '[[items/material/164-white-crystal|White Crystal]]'
  slots_of_30: 3
- item: '[[items/gem/321-sapphire-1|Sapphire 1]]'
  slots_of_30: 1
- item: '[[items/gem/351-peridot-1|Peridot 1]]'
  slots_of_30: 1
- item: '[[items/gem/312-ruby-2|Ruby 2]]'
  slots_of_30: 1
- item: '[[items/gem/322-sapphire-2|Sapphire 2]]'
  slots_of_30: 1
- item: '[[items/gem/332-topaz-2|Topaz 2]]'
  slots_of_30: 1
spawns:
- zone: '[[zones/11-junon-clan-field|Junon Clan Field]]'
  x: 4993
  y: 5140.5
  count: 1
  group: basic
- zone: '[[zones/11-junon-clan-field|Junon Clan Field]]'
  x: 5075.3
  y: 5049
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 217, ITEM_DROP.STB row 273
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Krawfy Guardian

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
