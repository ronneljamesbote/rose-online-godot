---
kind: monster
id: 24
name: Cowboy Pumpkin
status: in-game
level: 14
hp: 25
attack: 55
hit: 85
defence: 56
resistance: 18
avoid: 18
attack_speed: 110
attack_range: 2.5
damage: physical
walk_speed: 200
run_speed: 260
xp: 22
drop_item_rate: 65
drop_money_rate: 15
zones: 1
drops:
- item: '[[items/material/173-limpid-skin|Limpid Skin]]'
  slots_of_30: 9
- item: '[[items/material/174-sticky-husk|Sticky Husk]]'
  slots_of_30: 5
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/1-health-vial-s|Health Vial (S)]]'
  slots_of_30: 1
- item: '[[items/consumable/172-stamina-75|Stamina (+75)]]'
  slots_of_30: 1
- item: '[[items/consumable/404-pumpkin|Pumpkin]]'
  slots_of_30: 2
- item: '[[items/material/184-plant-seed|Plant Seed]]'
  slots_of_30: 1
- item: '[[items/face/7-jolly-front-mask|Jolly Front Mask]]'
  slots_of_30: 0.2
- item: '[[items/feet/121-brown-shoes|Brown Shoes]]'
  slots_of_30: 0.2
- item: '[[items/weapon/231-bubble-gun|Bubble Gun]]'
  slots_of_30: 0.4
- item: '[[items/weapon/2-short-sword|Short Sword]]'
  slots_of_30: 0.2
- item: '[[items/weapon/162-javelin|Javelin]]'
  slots_of_30: 0.2
- item: '[[items/weapon/101-haedong-sword|Haedong Sword]]'
  slots_of_30: 0.4
- item: '[[items/head/5-islamic-bandana|Islamic Bandana]]'
  slots_of_30: 0.4
- item: '[[items/body/5-yellow-wild-jeans|Yellow Wild Jeans]]'
  slots_of_30: 0.4
- item: '[[items/hands/5-safe-gloves|Safe Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/5-dash-shoes|Dash Shoes]]'
  slots_of_30: 0.4
- item: '[[items/weapon/301-baobab-rod|Baobab Rod]]'
  slots_of_30: 0.2
- item: '[[items/weapon/401-knuckle|Knuckle]]'
  slots_of_30: 0.6
- item: '[[items/weapon/32-monkey-wrench|Monkey Wrench]]'
  slots_of_30: 0.6
- item: '[[items/subweapon/1-wooden-shield|Wooden Shield]]'
  slots_of_30: 0.2
- item: '[[items/weapon/331-baobab-wand|Baobab Wand]]'
  slots_of_30: 0.2
spawns:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5118
  y: 5298.5
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5189.8
  y: 5391.3
  count: 2
  group: reinforcements
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5406.9
  y: 5322.7
  count: 1
  group: reinforcements
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5382.3
  y: 5300.6
  count: 1
  group: reinforcements
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5104.5
  y: 5167.8
  count: 2
  group: reinforcements
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5133.9
  y: 5125.1
  count: 1
  group: reinforcements
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5396.5
  y: 5258.3
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5340.9
  y: 5112.8
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 24, ITEM_DROP.STB row 111
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Cowboy Pumpkin

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
