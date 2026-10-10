---
kind: monster
id: 22
name: Big Pumpkin
status: in-game
level: 10
hp: 35
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
spawns:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5118
  y: 5298.5
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5163.8
  y: 5329
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5189.8
  y: 5391.3
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5189.8
  y: 5391.3
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5133.7
  y: 5303.3
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5104.5
  y: 5167.8
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5104.5
  y: 5167.8
  count: 3
  group: reinforcements
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5109.5
  y: 5211.2
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5115.3
  y: 5143.7
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5115.3
  y: 5143.7
  count: 1
  group: reinforcements
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5133.9
  y: 5125.1
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5133.9
  y: 5125.1
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5133.9
  y: 5125.1
  count: 2
  group: reinforcements
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5183.3
  y: 5173.9
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5275.6
  y: 5208.7
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5120.6
  y: 5274.2
  count: 1
  group: reinforcements
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5282
  y: 5138.2
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5282
  y: 5138.2
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5295.8
  y: 5106.9
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5295.8
  y: 5106.9
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5340.9
  y: 5112.8
  count: 2
  group: basic
source:
  data: LIST_NPC.STB row 22, ITEM_DROP.STB row 109
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Big Pumpkin

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
