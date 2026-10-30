---
kind: monster
id: 71
name: Needle Pomic
status: in-game
level: 12
hp: 300
hp_per_level: 25
attack: 42
hit: 70
defence: 45
resistance: 24
avoid: 29
attack_speed: 85
attack_range: 2
damage: physical
walk_speed: 140
run_speed: 400
xp: 20
drop_item_rate: 62
drop_money_rate: 20
zones: 1
drops:
- item: '[[items/material/175-insect-feeler|Insect Feeler]]'
  slots_of_30: 6
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/101-apple|Apple]]'
  slots_of_30: 1
- item: '[[items/material/176-insect-leg|Insect Leg]]'
  slots_of_30: 8
- item: '[[items/consumable/161-mp-point-30|MP Point (+30)]]'
  slots_of_30: 1
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/408-pomic|Pomic]]'
  slots_of_30: 2
- item: '[[items/weapon/162-javelin|Javelin]]'
  slots_of_30: 0.6
- item: '[[items/weapon/32-monkey-wrench|Monkey Wrench]]'
  slots_of_30: 0.6
- item: '[[items/feet/121-brown-shoes|Brown Shoes]]'
  slots_of_30: 0.6
- item: '[[items/head/31-soldier-helm|Soldier Helm]]'
  slots_of_30: 0.6
- item: '[[items/body/91-hunter-chest|Hunter Chest]]'
  slots_of_30: 0.6
- item: '[[items/weapon/301-baobab-rod|Baobab Rod]]'
  slots_of_30: 0.2
- item: '[[items/head/5-islamic-bandana|Islamic Bandana]]'
  slots_of_30: 0.2
- item: '[[items/body/5-yellow-wild-jeans|Yellow Wild Jeans]]'
  slots_of_30: 0.2
- item: '[[items/hands/5-safe-gloves|Safe Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/5-dash-shoes|Dash Shoes]]'
  slots_of_30: 0.2
quests:
- '[[quests/5006-stockpiling-bronze-fragments|Stockpiling Bronze Fragments]]'
- '[[quests/2106-restless-stones|Restless Stones]]'
spawns:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5072
  y: 5394
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5072
  y: 5394
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5072
  y: 5394
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5072
  y: 5394
  count: 2
  group: reinforcements
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5260
  y: 5308
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5260
  y: 5308
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5326
  y: 5300
  count: 2
  group: basic
source:
  data: godot/data/monsters/71-needle-pomic.tres, drop table godot/data/drop_tables/131-needle-pomic.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Needle Pomic

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
