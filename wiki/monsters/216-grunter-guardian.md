---
kind: monster
id: 216
name: Grunter Guardian
status: in-game
level: 70
hp: 26250
hp_per_level: 375
attack: 391
hit: 289
defence: 266
resistance: 177
avoid: 109
attack_speed: 120
attack_range: 3.5
damage: physical
walk_speed: 300
run_speed: 650
xp: 1793
drop_item_rate: 82
drop_money_rate: 0
zones: 1
drops:
- item: '[[items/material/161-green-crystal|Green Crystal]]'
  slots_of_30: 4
- item: '[[items/material/162-blue-crystal|Blue Crystal]]'
  slots_of_30: 6
- item: '[[items/material/151-black-hearts|Black Hearts]]'
  slots_of_30: 2
- item: '[[items/material/163-red-crystal|Red Crystal]]'
  slots_of_30: 5
- item: '[[items/material/152-green-hearts|Green Hearts]]'
  slots_of_30: 2
- item: '[[items/material/153-blue-hearts|Blue Hearts]]'
  slots_of_30: 2
- item: '[[items/material/164-white-crystal|White Crystal]]'
  slots_of_30: 3
- item: '[[items/gem/331-topaz-1|Topaz 1]]'
  slots_of_30: 1
- item: '[[items/gem/341-emerald-1|Emerald 1]]'
  slots_of_30: 2
- item: '[[items/gem/351-peridot-1|Peridot 1]]'
  slots_of_30: 1
- item: '[[items/gem/361-diamond-1|Diamond 1]]'
  slots_of_30: 1
spawns:
- zone: '[[zones/11-junon-clan-field|Junon Clan Field]]'
  x: 5270.2
  y: 5058.9
  count: 1
  group: basic
- zone: '[[zones/11-junon-clan-field|Junon Clan Field]]'
  x: 5176.4
  y: 5004.4
  count: 1
  group: basic
- zone: '[[zones/11-junon-clan-field|Junon Clan Field]]'
  x: 5414.3
  y: 5047.7
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 216, ITEM_DROP.STB row 272
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Grunter Guardian

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
