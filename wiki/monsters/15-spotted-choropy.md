---
kind: monster
id: 15
name: Spotted Choropy
status: in-game
level: 11
hp: 34
attack: 35
hit: 80
defence: 65
resistance: 28
avoid: 11
attack_speed: 85
attack_range: 2.3
damage: physical
walk_speed: 180
run_speed: 266
xp: 23
drop_item_rate: 65
drop_money_rate: 12
zones: 1
drops:
- item: '[[items/material/171-thin-insect-shell|Thin Insect Shell]]'
  slots_of_30: 9
- item: '[[items/material/176-insect-leg|Insect Leg]]'
  slots_of_30: 6
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/2-health-vial-m|Health Vial (M)]]'
  slots_of_30: 1
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/403-choropy|Choropy]]'
  slots_of_30: 2
- item: '[[items/material/177-insect-wing|Insect Wing]]'
  slots_of_30: 2
- item: '[[items/face/7-jolly-front-mask|Jolly Front Mask]]'
  slots_of_30: 0.2
- item: '[[items/weapon/201-toy-bow|Toy Bow]]'
  slots_of_30: 0.4
- item: '[[items/weapon/2-short-sword|Short Sword]]'
  slots_of_30: 0.2
- item: '[[items/head/5-islamic-bandana|Islamic Bandana]]'
  slots_of_30: 0.2
- item: '[[items/feet/6-shoes-of-iguje|Shoes of Iguje]]'
  slots_of_30: 0.2
- item: '[[items/weapon/161-bamboo-spear|Bamboo Spear]]'
  slots_of_30: 0.4
- item: '[[items/head/4-bandana-of-iguje|Bandana of Iguje]]'
  slots_of_30: 0.4
- item: '[[items/body/4-blue-wild-jeans|Blue Wild Jeans]]'
  slots_of_30: 0.4
- item: '[[items/hands/4-dumb-gloves|Dumb Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/4-leather-shoes|Leather Shoes]]'
  slots_of_30: 0.4
- item: '[[items/weapon/31-wooden-bat|Wooden Bat]]'
  slots_of_30: 0.8
- item: '[[items/weapon/101-haedong-sword|Haedong Sword]]'
  slots_of_30: 0.4
- item: '[[items/weapon/301-baobab-rod|Baobab Rod]]'
  slots_of_30: 0.2
- item: '[[items/weapon/401-knuckle|Knuckle]]'
  slots_of_30: 0.2
- item: '[[items/weapon/32-monkey-wrench|Monkey Wrench]]'
  slots_of_30: 0.2
quests:
- '[[quests/952-hawker-job-change-quest|Hawker Job Change Quest]]'
spawns:
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5473.1
  y: 5315.5
  count: 1
  group: reinforcements
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5492.6
  y: 5280.5
  count: 1
  group: reinforcements
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5233.6
  y: 5226.6
  count: 1
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5203.2
  y: 5181.3
  count: 1
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5203.2
  y: 5181.3
  count: 1
  group: reinforcements
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5176.1
  y: 5255.8
  count: 1
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5176.1
  y: 5255.8
  count: 1
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5145.7
  y: 5256.9
  count: 2
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5204.5
  y: 5221.1
  count: 2
  group: reinforcements
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5457.8
  y: 5245.1
  count: 1
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5457.8
  y: 5245.1
  count: 1
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5482.8
  y: 5249.6
  count: 1
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5482.8
  y: 5249.6
  count: 1
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5405.1
  y: 5107.8
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 15, ITEM_DROP.STB row 107
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Spotted Choropy

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
