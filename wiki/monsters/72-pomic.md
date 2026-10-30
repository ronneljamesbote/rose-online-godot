---
kind: monster
id: 72
name: Pomic
status: in-game
level: 13
hp: 325
hp_per_level: 25
attack: 45
hit: 71
defence: 47
resistance: 26
avoid: 30
attack_speed: 90
attack_range: 2.2
damage: physical
walk_speed: 140
run_speed: 420
xp: 21
drop_item_rate: 65
drop_money_rate: 20
zones: 2
drops:
- item: '[[items/material/175-insect-feeler|Insect Feeler]]'
  slots_of_30: 7
- item: '[[items/material/176-insect-leg|Insect Leg]]'
  slots_of_30: 8
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/161-mp-point-30|MP Point (+30)]]'
  slots_of_30: 1
- item: '[[items/consumable/408-pomic|Pomic]]'
  slots_of_30: 2
- item: '[[items/head/91-hunter-hat|Hunter Hat]]'
  slots_of_30: 0.4
- item: '[[items/weapon/101-haedong-sword|Haedong Sword]]'
  slots_of_30: 0.6
- item: '[[items/weapon/162-javelin|Javelin]]'
  slots_of_30: 0.8
- item: '[[items/feet/31-soldier-boots|Soldier Boots]]'
  slots_of_30: 0.4
- item: '[[items/weapon/401-knuckle|Knuckle]]'
  slots_of_30: 0.2
- item: '[[items/head/5-islamic-bandana|Islamic Bandana]]'
  slots_of_30: 0.2
- item: '[[items/body/5-yellow-wild-jeans|Yellow Wild Jeans]]'
  slots_of_30: 0.2
- item: '[[items/hands/5-safe-gloves|Safe Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/5-dash-shoes|Dash Shoes]]'
  slots_of_30: 0.2
- item: '[[items/weapon/31-wooden-bat|Wooden Bat]]'
  slots_of_30: 0.2
- item: '[[items/weapon/3-rapier|Rapier]]'
  slots_of_30: 0.2
- item: '[[items/weapon/202-short-bow|Short Bow]]'
  slots_of_30: 0.2
- item: '[[items/subweapon/1-wooden-shield|Wooden Shield]]'
  slots_of_30: 0.2
quests:
- '[[quests/5002-pomic-hunt-fetch-quest|Pomic Hunt (Fetch Quest)]]'
- '[[quests/2106-restless-stones|Restless Stones]]'
spawns:
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5120.4
  y: 5012.5
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5099.4
  y: 4901.7
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5072
  y: 5394
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5260
  y: 5308
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5260
  y: 5308
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5260
  y: 5308
  count: 2
  group: reinforcements
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5326
  y: 5300
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5326
  y: 5300
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5326
  y: 5300
  count: 1
  group: reinforcements
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5362
  y: 5198
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5362
  y: 5198
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5362
  y: 5198
  count: 1
  group: reinforcements
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5465
  y: 5148
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5465
  y: 5148
  count: 2
  group: basic
source:
  data: godot/data/monsters/72-pomic.tres, drop table godot/data/drop_tables/132-pomic.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Pomic

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
