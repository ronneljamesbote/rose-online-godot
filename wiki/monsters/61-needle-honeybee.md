---
kind: monster
id: 61
name: Needle HoneyBee
status: changed-from-irose
level: 11
hp: 275
hp_per_level: 25
attack: 38
hit: 68
defence: 43
resistance: 23
avoid: 28
attack_speed: 90
attack_range: 2
damage: physical
walk_speed: 200
run_speed: 350
xp: 20
drop_item_rate: 62
drop_money_rate: 8
zones: 1
drops:
- item: '[[items/material/176-insect-leg|Insect Leg]]'
  slots_of_30: 7
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/103-grapes|Grapes]]'
  slots_of_30: 1
- item: '[[items/material/177-insect-wing|Insect Wing]]'
  slots_of_30: 7
- item: '[[items/consumable/161-mp-point-30|MP Point (+30)]]'
  slots_of_30: 1
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/406-honey-bee|Honey Bee]]'
  slots_of_30: 2
- item: '[[items/body/121-brown-vest|Brown Vest]]'
  slots_of_30: 0.6
- item: '[[items/hands/31-soldier-gloves|Soldier Gloves]]'
  slots_of_30: 0.6
- item: '[[items/hands/91-hunter-gloves|Hunter Gloves]]'
  slots_of_30: 0.6
- item: '[[items/body/61-spiritual-vest|Spiritual Vest]]'
  slots_of_30: 0.6
- item: '[[items/feet/121-brown-shoes|Brown Shoes]]'
  slots_of_30: 0.6
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
quests:
- '[[quests/1003-dealer-job-change-quest|Dealer Job Change Quest]]'
- '[[quests/2103-glowing-stingers|Glowing Stingers]]'
spawns:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5294
  y: 5338
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5038
  y: 5216
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5222
  y: 5172
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5146
  y: 5148
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5022
  y: 5108
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5022
  y: 5108
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5022
  y: 5108
  count: 1
  group: reinforcements
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5352
  y: 5002
  count: 1
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5352
  y: 5002
  count: 2
  group: basic
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5352
  y: 5002
  count: 1
  group: reinforcements
source:
  data: godot/data/monsters/61-needle-honeybee.tres, drop table godot/data/drop_tables/124-needle-honeybee.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Needle HoneyBee

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].

## Changed from iROSE

The Adventurer's Plain redesign took it out of that zone.

The Zant city hub redesign makes [[zones/1-canyon-city-of-zant|Canyon City of Zant]] a safe city: this monster no longer spawns in or around Zant.
