---
kind: monster
id: 24
name: Cowboy Pumpkin
status: in-game
level: 14
hp: 350
hp_per_level: 25
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
quests:
- '[[quests/2105-stolen-supplies|Stolen Supplies]]'
spawns:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5558
  y: 5122
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5558
  y: 5122
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5558
  y: 5122
  count: 1
  group: reinforcements
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5430
  y: 5034
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5430
  y: 5034
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5430
  y: 5034
  count: 1
  group: reinforcements
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5492
  y: 5072
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5492
  y: 5072
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5492
  y: 5072
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5492
  y: 5072
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5492
  y: 5072
  count: 2
  group: reinforcements
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5525
  y: 5000
  count: 1
  group: basic
source:
  data: godot/data/monsters/24-cowboy-pumpkin.tres, drop table godot/data/drop_tables/111-cowboy-pumpkin.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Cowboy Pumpkin

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
