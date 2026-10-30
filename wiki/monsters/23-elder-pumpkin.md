---
kind: monster
id: 23
name: Elder Pumpkin
status: in-game
level: 13
hp: 364
hp_per_level: 28
attack: 48
hit: 65
defence: 39
resistance: 28
avoid: 30
attack_speed: 95
attack_range: 2.5
damage: physical
walk_speed: 230
run_speed: 400
xp: 22
drop_item_rate: 60
drop_money_rate: 12
zones: 1
drops:
- item: '[[items/material/173-limpid-skin|Limpid Skin]]'
  slots_of_30: 8
- item: '[[items/material/174-sticky-husk|Sticky Husk]]'
  slots_of_30: 7
- item: '[[items/consumable/1-health-vial-s|Health Vial (S)]]'
  slots_of_30: 1
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/404-pumpkin|Pumpkin]]'
  slots_of_30: 2
- item: '[[items/feet/91-hunter-boots|Hunter Boots]]'
  slots_of_30: 0.2
- item: '[[items/weapon/2-short-sword|Short Sword]]'
  slots_of_30: 0.2
- item: '[[items/weapon/301-baobab-rod|Baobab Rod]]'
  slots_of_30: 0.4
- item: '[[items/weapon/162-javelin|Javelin]]'
  slots_of_30: 0.2
- item: '[[items/weapon/3-rapier|Rapier]]'
  slots_of_30: 0.4
- item: '[[items/head/5-islamic-bandana|Islamic Bandana]]'
  slots_of_30: 0.4
- item: '[[items/body/5-yellow-wild-jeans|Yellow Wild Jeans]]'
  slots_of_30: 0.4
- item: '[[items/hands/5-safe-gloves|Safe Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/5-dash-shoes|Dash Shoes]]'
  slots_of_30: 0.4
- item: '[[items/weapon/31-wooden-bat|Wooden Bat]]'
  slots_of_30: 0.2
- item: '[[items/weapon/201-toy-bow|Toy Bow]]'
  slots_of_30: 0.2
- item: '[[items/weapon/101-haedong-sword|Haedong Sword]]'
  slots_of_30: 0.6
- item: '[[items/weapon/231-bubble-gun|Bubble Gun]]'
  slots_of_30: 0.6
- item: '[[items/subweapon/1-wooden-shield|Wooden Shield]]'
  slots_of_30: 0.2
- item: '[[items/weapon/401-knuckle|Knuckle]]'
  slots_of_30: 0.2
quests:
- '[[quests/853-solider-job-change-quest|Solider Job Change Quest]]'
spawns:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5205
  y: 5318
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5205
  y: 5318
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5205
  y: 5318
  count: 1
  group: reinforcements
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5260
  y: 5388
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5260
  y: 5388
  count: 1
  group: reinforcements
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5188
  y: 5208
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5188
  y: 5208
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5188
  y: 5208
  count: 1
  group: reinforcements
source:
  data: godot/data/monsters/23-elder-pumpkin.tres, drop table godot/data/drop_tables/110-elder-pumpkin.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Elder Pumpkin

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
