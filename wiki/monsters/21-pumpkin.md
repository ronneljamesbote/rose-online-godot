---
kind: monster
id: 21
name: Pumpkin
status: in-game
level: 9
hp: 207
hp_per_level: 23
attack: 32
hit: 65
defence: 39
resistance: 20
avoid: 25
attack_speed: 100
attack_range: 2.2
damage: physical
walk_speed: 200
run_speed: 260
xp: 18
drop_item_rate: 58
drop_money_rate: 10
zones: 1
drops:
- item: '[[items/material/173-limpid-skin|Limpid Skin]]'
  slots_of_30: 9
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/material/174-sticky-husk|Sticky Husk]]'
  slots_of_30: 6
- item: '[[items/consumable/404-pumpkin|Pumpkin]]'
  slots_of_30: 2
- item: '[[items/weapon/201-toy-bow|Toy Bow]]'
  slots_of_30: 0.8
- item: '[[items/weapon/231-bubble-gun|Bubble Gun]]'
  slots_of_30: 0.4
- item: '[[items/body/4-blue-wild-jeans|Blue Wild Jeans]]'
  slots_of_30: 0.6
- item: '[[items/head/4-bandana-of-iguje|Bandana of Iguje]]'
  slots_of_30: 0.4
- item: '[[items/hands/4-dumb-gloves|Dumb Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/4-leather-shoes|Leather Shoes]]'
  slots_of_30: 0.4
quests:
- '[[quests/105-healing-hands|Healing Hands]]'
- '[[quests/2102-pumpkins-on-the-road|Pumpkins on the Road]]'
spawns:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5022
  y: 5300
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5110
  y: 5100
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5110
  y: 5100
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5110
  y: 5100
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5022
  y: 5108
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5278
  y: 5118
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5278
  y: 5118
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5278
  y: 5118
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5358
  y: 5088
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5358
  y: 5088
  count: 2
  group: basic
source:
  data: godot/data/monsters/21-pumpkin.tres, drop table godot/data/drop_tables/108-pumpkin.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Pumpkin

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
