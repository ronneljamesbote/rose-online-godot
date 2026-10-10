---
kind: monster
id: 204
name: Wandering Rackie
status: in-game
level: 20
hp: 34
attack: 57
hit: 94
defence: 71
resistance: 28
avoid: 42
attack_speed: 90
attack_range: 2.5
damage: physical
walk_speed: 160
run_speed: 540
xp: 33
drop_item_rate: 60
drop_money_rate: 25
zones: 2
drops:
- item: '[[items/material/185-white-animal-tail-fur|White Animal Tail Fur]]'
  slots_of_30: 7
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/163-mp-point-100|MP Point (+100)]]'
  slots_of_30: 1
- item: '[[items/material/187-black-animal-tail-fur|Black Animal Tail Fur]]'
  slots_of_30: 5
- item: '[[items/consumable/22-mana-vial-m|Mana Vial (M)]]'
  slots_of_30: 1
- item: '[[items/material/161-green-crystal|Green Crystal]]'
  slots_of_30: 3
- item: '[[items/consumable/308-dexterity-scroll-solo|Dexterity Scroll (Solo)]]'
  slots_of_30: 1
- item: '[[items/head/31-soldier-helm|Soldier Helm]]'
  slots_of_30: 0.4
- item: '[[items/hands/121-brown-gloves|Brown Gloves]]'
  slots_of_30: 0.4
- item: '[[items/weapon/331-baobab-wand|Baobab Wand]]'
  slots_of_30: 1
- item: '[[items/weapon/301-baobab-rod|Baobab Rod]]'
  slots_of_30: 0.6
- item: '[[items/weapon/401-knuckle|Knuckle]]'
  slots_of_30: 0.6
- item: '[[items/head/7-red-leather-beret|Red Leather Beret]]'
  slots_of_30: 0.2
- item: '[[items/body/7-yellow-denim-look|Yellow Denim Look]]'
  slots_of_30: 0.2
- item: '[[items/hands/7-full-action-gloves|Full Action Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/7-islamic-shoes|Islamic Shoes]]'
  slots_of_30: 0.2
- item: '[[items/weapon/3-rapier|Rapier]]'
  slots_of_30: 0.2
- item: '[[items/weapon/202-short-bow|Short Bow]]'
  slots_of_30: 0.4
- item: '[[items/subweapon/2-tarz|Tarz]]'
  slots_of_30: 0.2
- item: '[[items/weapon/162-javelin|Javelin]]'
  slots_of_30: 0.2
- item: '[[items/weapon/4-khukuri|Khukuri]]'
  slots_of_30: 0.2
spawns:
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5261.4
  y: 5361.4
  count: 1
  group: reinforcements
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5410.4
  y: 5212.4
  count: 1
  group: reinforcements
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5275.6
  y: 5454.8
  count: 1
  group: reinforcements
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5189.8
  y: 5391.3
  count: 1
  group: reinforcements
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5133.7
  y: 5303.3
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 204, ITEM_DROP.STB row 199
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Wandering Rackie

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
