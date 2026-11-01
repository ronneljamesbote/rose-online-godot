---
kind: monster
id: 102
name: Woopie Chef
status: changed-from-irose
level: 17
hp: 391
hp_per_level: 23
attack: 54
hit: 74
defence: 37
resistance: 40
avoid: 46
attack_speed: 85
attack_range: 15
damage: physical
walk_speed: 190
run_speed: 480
xp: 23
drop_item_rate: 63
drop_money_rate: 30
zones: 1
drops:
- item: '[[items/material/185-white-animal-tail-fur|White Animal Tail Fur]]'
  slots_of_30: 7
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/material/186-red-animal-tail-fur|Red Animal Tail Fur]]'
  slots_of_30: 7
- item: '[[items/consumable/22-mana-vial-m|Mana Vial (M)]]'
  slots_of_30: 1
- item: '[[items/consumable/161-mp-point-30|MP Point (+30)]]'
  slots_of_30: 1
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/414-woopy|Woopy]]'
  slots_of_30: 2
- item: '[[items/weapon/301-baobab-rod|Baobab Rod]]'
  slots_of_30: 0.4
- item: '[[items/weapon/331-baobab-wand|Baobab Wand]]'
  slots_of_30: 0.4
- item: '[[items/body/203-natalie-look|Natalie Look]]'
  slots_of_30: 0.2
- item: '[[items/feet/203-shoes-of-iguje|Shoes of Iguje]]'
  slots_of_30: 0.2
- item: '[[items/hands/203-action-gloves|Action Gloves]]'
  slots_of_30: 0.2
- item: '[[items/weapon/401-knuckle|Knuckle]]'
  slots_of_30: 0.6
- item: '[[items/head/7-red-leather-beret|Red Leather Beret]]'
  slots_of_30: 0.4
- item: '[[items/body/7-yellow-denim-look|Yellow Denim Look]]'
  slots_of_30: 0.4
- item: '[[items/hands/7-full-action-gloves|Full Action Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/7-islamic-shoes|Islamic Shoes]]'
  slots_of_30: 0.4
- item: '[[items/weapon/32-monkey-wrench|Monkey Wrench]]'
  slots_of_30: 0.2
- item: '[[items/subweapon/1-wooden-shield|Wooden Shield]]'
  slots_of_30: 0.2
spawns:
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5119.9
  y: 5164.1
  count: 2
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5072.8
  y: 5184.9
  count: 3
  group: basic
source:
  data: godot/data/monsters/102-woopie-chef.tres, drop table godot/data/drop_tables/148-woopie-chef.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Woopie Chef

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].

## Changed from iROSE

The Zant city hub redesign makes [[zones/1-canyon-city-of-zant|Canyon City of Zant]] a safe city: this monster no longer spawns in or around Zant.
