---
kind: monster
id: 103
name: Woopie Glutton
status: changed-from-irose
level: 18
hp: 612
hp_per_level: 34
attack: 47
hit: 89
defence: 66
resistance: 26
avoid: 39
attack_speed: 95
attack_range: 2.6
damage: physical
walk_speed: 170
run_speed: 390
xp: 30
drop_item_rate: 65
drop_money_rate: 35
zones: 1
drops:
- item: '[[items/material/185-white-animal-tail-fur|White Animal Tail Fur]]'
  slots_of_30: 5
- item: '[[items/material/186-red-animal-tail-fur|Red Animal Tail Fur]]'
  slots_of_30: 7
- item: '[[items/consumable/161-mp-point-30|MP Point (+30)]]'
  slots_of_30: 1
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/414-woopy|Woopy]]'
  slots_of_30: 2
- item: '[[items/material/188-animal-tail-fur|Animal Tail Fur]]'
  slots_of_30: 2
- item: '[[items/weapon/331-baobab-wand|Baobab Wand]]'
  slots_of_30: 0.8
- item: '[[items/weapon/401-knuckle|Knuckle]]'
  slots_of_30: 0.6
- item: '[[items/weapon/162-javelin|Javelin]]'
  slots_of_30: 0.6
- item: '[[items/weapon/32-monkey-wrench|Monkey Wrench]]'
  slots_of_30: 0.8
- item: '[[items/head/7-red-leather-beret|Red Leather Beret]]'
  slots_of_30: 0.2
- item: '[[items/body/7-yellow-denim-look|Yellow Denim Look]]'
  slots_of_30: 0.2
- item: '[[items/hands/7-full-action-gloves|Full Action Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/7-islamic-shoes|Islamic Shoes]]'
  slots_of_30: 0.2
- item: '[[items/subweapon/1-wooden-shield|Wooden Shield]]'
  slots_of_30: 0.4
spawns:
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5119.9
  y: 5164.1
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5072.8
  y: 5184.9
  count: 1
  group: basic
source:
  data: godot/data/monsters/103-woopie-glutton.tres, drop table godot/data/drop_tables/149-woopie-glutton.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Woopie Glutton

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].

## Changed from iROSE

The Zant city hub redesign makes [[zones/1-canyon-city-of-zant|Canyon City of Zant]] a safe city: this monster no longer spawns in or around Zant.
