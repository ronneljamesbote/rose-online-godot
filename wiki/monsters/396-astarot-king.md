---
kind: monster
id: 396
name: Astarot King
status: in-game
level: 155
hp: 609
attack: 886
hit: 569
defence: 693
resistance: 571
avoid: 235
attack_speed: 110
attack_range: 2.5
damage: physical
walk_speed: 300
run_speed: 710
xp: 3615
drop_item_rate: 85
drop_money_rate: 0
zones: 1
drops:
- item: '[[items/material/161-green-crystal|Green Crystal]]'
  slots_of_30: 2
- item: '[[items/consumable/183-clan-point-3|Clan Point (+3)]]'
  slots_of_30: 2
- item: '[[items/material/162-blue-crystal|Blue Crystal]]'
  slots_of_30: 4
- item: '[[items/consumable/184-clan-point-5|Clan Point (+5)]]'
  slots_of_30: 1
- item: '[[items/consumable/185-clan-point-7|Clan Point (+7)]]'
  slots_of_30: 1
- item: '[[items/material/163-red-crystal|Red Crystal]]'
  slots_of_30: 1
- item: '[[items/consumable/186-clan-point-10|Clan Point (+10)]]'
  slots_of_30: 1
- item: '[[items/material/164-white-crystal|White Crystal]]'
  slots_of_30: 2
- item: '[[items/consumable/187-clan-point-15|Clan Point (+15)]]'
  slots_of_30: 3
- item: '[[items/gem/321-sapphire-1|Sapphire 1]]'
  slots_of_30: 1
- item: '[[items/gem/351-peridot-1|Peridot 1]]'
  slots_of_30: 1
- item: '[[items/gem/331-topaz-1|Topaz 1]]'
  slots_of_30: 1
- item: '[[items/gem/341-emerald-1|Emerald 1]]'
  slots_of_30: 1
- item: '[[items/gem/361-diamond-1|Diamond 1]]'
  slots_of_30: 1
- item: '[[items/material/153-blue-hearts|Blue Hearts]]'
  slots_of_30: 1
- item: '[[items/material/154-pink-hearts|Pink Hearts]]'
  slots_of_30: 1
- item: '[[items/material/155-red-hearts|Red Hearts]]'
  slots_of_30: 1
- item: '[[items/material/156-golden-hearts|Golden Hearts]]'
  slots_of_30: 1
- item: '[[items/material/157-white-hearts|White Hearts]]'
  slots_of_30: 1
- item: '[[items/weapon/901-falchion|Falchion]]'
  slots_of_30: 1
- item: '[[items/back/233-astarot-wing|Astarot Wing]]'
  slots_of_30: 1
spawns:
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5318.8
  y: 5129.5
  count: 1
  group: basic
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5318.8
  y: 5129.5
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 396, ITEM_DROP.STB row 278
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Astarot King

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
