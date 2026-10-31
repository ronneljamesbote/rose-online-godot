---
kind: monster
id: 13
name: Mother Choropy
status: in-game
level: 10
hp: 350
hp_per_level: 35
attack: 32
hit: 78
defence: 62
resistance: 27
avoid: 10
attack_speed: 80
attack_range: 2.2
damage: physical
walk_speed: 180
run_speed: 266
xp: 22
drop_item_rate: 62
drop_money_rate: 10
zones: 1
drops:
- item: '[[items/material/171-thin-insect-shell|Thin Insect Shell]]'
  slots_of_30: 7
- item: '[[items/material/176-insect-leg|Insect Leg]]'
  slots_of_30: 8
- item: '[[items/consumable/103-grapes|Grapes]]'
  slots_of_30: 1
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/2-health-vial-m|Health Vial (M)]]'
  slots_of_30: 1
- item: '[[items/consumable/403-choropy|Choropy]]'
  slots_of_30: 2
- item: '[[items/material/177-insect-wing|Insect Wing]]'
  slots_of_30: 2
- item: '[[items/weapon/31-wooden-bat|Wooden Bat]]'
  slots_of_30: 0.6
- item: '[[items/hands/5-safe-gloves|Safe Gloves]]'
  slots_of_30: 0.2
- item: '[[items/body/4-blue-wild-jeans|Blue Wild Jeans]]'
  slots_of_30: 0.6
- item: '[[items/weapon/2-short-sword|Short Sword]]'
  slots_of_30: 0.6
- item: '[[items/feet/5-dash-shoes|Dash Shoes]]'
  slots_of_30: 0.2
- item: '[[items/head/4-bandana-of-iguje|Bandana of Iguje]]'
  slots_of_30: 0.4
- item: '[[items/hands/4-dumb-gloves|Dumb Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/4-leather-shoes|Leather Shoes]]'
  slots_of_30: 0.4
- item: '[[items/weapon/201-toy-bow|Toy Bow]]'
  slots_of_30: 0.4
- item: '[[items/weapon/3-rapier|Rapier]]'
  slots_of_30: 0.6
- item: '[[items/weapon/101-haedong-sword|Haedong Sword]]'
  slots_of_30: 0.2
- item: '[[items/weapon/231-bubble-gun|Bubble Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/401-knuckle|Knuckle]]'
  slots_of_30: 0.2
quests:
- '[[quests/2208-poisoned-seed|Poisoned Seed]]'
- '[[quests/952-hawker-job-change-quest|Hawker Job Change Quest]]'
spawns:
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5019.8
  y: 5499.8
  count: 3
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5400
  y: 5440
  count: 3
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5052
  y: 5390
  count: 3
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5045.59
  y: 5309
  count: 2
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5050.95
  y: 5224
  count: 1
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5230
  y: 5240
  count: 2
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5215
  y: 5150
  count: 1
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5150
  y: 5215
  count: 2
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5253
  y: 5189.8
  count: 1
  group: basic
source:
  data: godot/data/monsters/13-mother-choropy.tres, drop table godot/data/drop_tables/106-mother-choropy.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Mother Choropy

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].

## Changed from iROSE

The Adventurer's Plain redesign moved its spawn points there (see [[zones/22-adventurer-s-plain|Adventurer's Plain]]). Its kills also count for [[quests/2208-poisoned-seed|Poisoned Seed]].
