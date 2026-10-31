---
kind: monster
id: 41
name: Flanae
status: in-game
level: 7
hp: 175
hp_per_level: 25
attack: 20
hit: 63
defence: 35
resistance: 17
avoid: 22
attack_speed: 90
attack_range: 17.5
damage: magic
walk_speed: 0
run_speed: 0
xp: 12
drop_item_rate: 55
drop_money_rate: 1
zones: 1
drops:
- item: '[[items/material/180-weed-stem|Weed Stem]]'
  slots_of_30: 6
- item: '[[items/material/184-plant-seed|Plant Seed]]'
  slots_of_30: 3
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/103-grapes|Grapes]]'
  slots_of_30: 1
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/material/179-weed-root|Weed Root]]'
  slots_of_30: 6
- item: '[[items/body/2-flower-shorts|Flower Shorts]]'
  slots_of_30: 0.4
- item: '[[items/weapon/101-haedong-sword|Haedong Sword]]'
  slots_of_30: 0.4
- item: '[[items/body/3-leopard-pants|Leopard Pants]]'
  slots_of_30: 0.8
- item: '[[items/head/2-green-hat|Green Hat]]'
  slots_of_30: 0.4
- item: '[[items/weapon/2-short-sword|Short Sword]]'
  slots_of_30: 0.4
- item: '[[items/weapon/161-bamboo-spear|Bamboo Spear]]'
  slots_of_30: 0.4
- item: '[[items/head/3-luxurious-green-hat|Luxurious Green Hat]]'
  slots_of_30: 0.4
- item: '[[items/hands/3-leather-gloves|Leather Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/3-waterproof-shoes|Waterproof Shoes]]'
  slots_of_30: 0.4
quests:
- '[[quests/2206-tibbs-farm|Tibbs'' Farm]]'
- '[[quests/902-muse-job-change-quest|Muse Job Change Quest]]'
spawns:
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5260
  y: 5478
  count: 3
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5222
  y: 5455
  count: 2
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5205
  y: 5480
  count: 3
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5290
  y: 5468
  count: 3
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5180
  y: 5420
  count: 2
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5150
  y: 5395
  count: 2
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5300
  y: 5355
  count: 3
  group: basic
source:
  data: godot/data/monsters/41-flanae.tres, drop table godot/data/drop_tables/116-flanae.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Flanae

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].

## Changed from iROSE

The Adventurer's Plain redesign moved its spawn points there (see [[zones/22-adventurer-s-plain|Adventurer's Plain]]). Its kills also count for [[quests/2206-tibbs-farm|Tibbs' Farm]].
