---
kind: monster
id: 103
name: Woopie Glutton
status: in-game
level: 18
hp: 34
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
zones: 2
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
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5441.3
  y: 5448.9
  count: 2
  group: reinforcements
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5245.2
  y: 5390.4
  count: 1
  group: reinforcements
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5194.9
  y: 5354.2
  count: 1
  group: reinforcements
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5261.4
  y: 5361.4
  count: 1
  group: basic
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5301.3
  y: 5395.8
  count: 1
  group: reinforcements
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5332
  y: 5339.6
  count: 2
  group: basic
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5377
  y: 5394.4
  count: 1
  group: reinforcements
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5423.8
  y: 5418.3
  count: 1
  group: reinforcements
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5399.8
  y: 5165.5
  count: 2
  group: reinforcements
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5382.9
  y: 5244.3
  count: 2
  group: basic
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5393.8
  y: 5191.6
  count: 1
  group: reinforcements
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5449.5
  y: 5197.6
  count: 1
  group: reinforcements
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
  data: LIST_NPC.STB row 103, ITEM_DROP.STB row 149
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Woopie Glutton

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
