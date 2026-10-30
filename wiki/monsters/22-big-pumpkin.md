---
kind: monster
id: 22
name: Big Pumpkin
status: in-game
level: 10
hp: 350
hp_per_level: 35
attack: 31
hit: 74
defence: 62
resistance: 27
avoid: 10
attack_speed: 85
attack_range: 2.3
damage: physical
walk_speed: 200
run_speed: 260
xp: 22
drop_item_rate: 60
drop_money_rate: 10
zones: 1
drops:
- item: '[[items/material/174-sticky-husk|Sticky Husk]]'
  slots_of_30: 4
- item: '[[items/material/173-limpid-skin|Limpid Skin]]'
  slots_of_30: 6
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/103-grapes|Grapes]]'
  slots_of_30: 1
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/material/196-bird-feather|Bird Feather]]'
  slots_of_30: 5
- item: '[[items/weapon/31-wooden-bat|Wooden Bat]]'
  slots_of_30: 0.6
- item: '[[items/hands/4-dumb-gloves|Dumb Gloves]]'
  slots_of_30: 0.8
- item: '[[items/feet/4-leather-shoes|Leather Shoes]]'
  slots_of_30: 0.6
- item: '[[items/weapon/231-bubble-gun|Bubble Gun]]'
  slots_of_30: 0.4
- item: '[[items/head/4-bandana-of-iguje|Bandana of Iguje]]'
  slots_of_30: 0.4
- item: '[[items/body/4-blue-wild-jeans|Blue Wild Jeans]]'
  slots_of_30: 0.4
- item: '[[items/weapon/2-short-sword|Short Sword]]'
  slots_of_30: 0.2
- item: '[[items/weapon/161-bamboo-spear|Bamboo Spear]]'
  slots_of_30: 0.2
- item: '[[items/weapon/201-toy-bow|Toy Bow]]'
  slots_of_30: 0.4
- item: '[[items/weapon/202-short-bow|Short Bow]]'
  slots_of_30: 0.4
- item: '[[items/subweapon/1-wooden-shield|Wooden Shield]]'
  slots_of_30: 0.2
- item: '[[items/weapon/401-knuckle|Knuckle]]'
  slots_of_30: 0.2
- item: '[[items/weapon/101-haedong-sword|Haedong Sword]]'
  slots_of_30: 0.2
quests:
- '[[quests/903-muse-job-change-quest|Muse Job Change Quest]]'
- '[[quests/2102-pumpkins-on-the-road|Pumpkins on the Road]]'
spawns:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5030
  y: 5460
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5030
  y: 5460
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5030
  y: 5460
  count: 1
  group: reinforcements
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5022
  y: 5300
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5022
  y: 5300
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5022
  y: 5300
  count: 1
  group: reinforcements
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5205
  y: 5318
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5205
  y: 5318
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5205
  y: 5318
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5260
  y: 5388
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5188
  y: 5208
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5188
  y: 5208
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5110
  y: 5100
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5110
  y: 5100
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5110
  y: 5100
  count: 1
  group: reinforcements
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5278
  y: 5118
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5278
  y: 5118
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5278
  y: 5118
  count: 1
  group: reinforcements
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5358
  y: 5088
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5358
  y: 5088
  count: 1
  group: reinforcements
source:
  data: godot/data/monsters/22-big-pumpkin.tres, drop table godot/data/drop_tables/109-big-pumpkin.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Big Pumpkin

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
