---
kind: monster
id: 62
name: HoneyBee
status: changed-from-irose
level: 12
hp: 300
hp_per_level: 25
attack: 42
hit: 70
defence: 45
resistance: 24
avoid: 29
attack_speed: 93
attack_range: 2.1
damage: physical
walk_speed: 220
run_speed: 440
xp: 22
drop_item_rate: 60
drop_money_rate: 10
zones: 2
drops:
- item: '[[items/material/176-insect-leg|Insect Leg]]'
  slots_of_30: 6
- item: '[[items/material/177-insect-wing|Insect Wing]]'
  slots_of_30: 9
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/406-honey-bee|Honey Bee]]'
  slots_of_30: 2
- item: '[[items/head/91-hunter-hat|Hunter Hat]]'
  slots_of_30: 0.6
- item: '[[items/body/121-brown-vest|Brown Vest]]'
  slots_of_30: 0.6
- item: '[[items/hands/91-hunter-gloves|Hunter Gloves]]'
  slots_of_30: 1.2
- item: '[[items/body/61-spiritual-vest|Spiritual Vest]]'
  slots_of_30: 0.6
- item: '[[items/weapon/32-monkey-wrench|Monkey Wrench]]'
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
- '[[quests/852-solider-job-change-quest|Solider Job Change Quest]]'
- '[[quests/2103-glowing-stingers|Glowing Stingers]]'
spawns:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5294
  y: 5338
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5294
  y: 5338
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5294
  y: 5338
  count: 1
  group: reinforcements
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5038
  y: 5216
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5038
  y: 5216
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5038
  y: 5216
  count: 1
  group: reinforcements
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5222
  y: 5172
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5222
  y: 5172
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5222
  y: 5172
  count: 1
  group: reinforcements
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5146
  y: 5148
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5146
  y: 5148
  count: 1
  group: reinforcements
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5230.0
  y: 5372.0
  count: 4
  group: basic
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5300.0
  y: 5398.0
  count: 4
  group: basic
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5346.0
  y: 5352.0
  count: 4
  group: basic
source:
  data: godot/data/monsters/62-honeybee.tres, drop table godot/data/drop_tables/125-honeybee.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# HoneyBee

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].

## Changed from iROSE

The Zant city hub redesign makes the HoneyBees the only fighting monster of [[zones/1-canyon-city-of-zant|Canyon City of Zant]]'s fields: three spawn points of 4 on the Training Grounds north of the walls (around 5230-5346, 5352-5398), coming back 20 seconds after they fall. They never attack first, so new arrivals can practise on them.
