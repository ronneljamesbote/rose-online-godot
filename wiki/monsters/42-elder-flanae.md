---
kind: monster
id: 42
name: Elder Flanae
status: in-game
level: 8
hp: 224
hp_per_level: 28
attack: 30
hit: 59
defence: 30
resistance: 20
avoid: 24
attack_speed: 85
attack_range: 17.5
damage: magic
walk_speed: 0
run_speed: 0
xp: 12
drop_item_rate: 60
drop_money_rate: 1
zones: 2
drops:
- item: '[[items/material/180-weed-stem|Weed Stem]]'
  slots_of_30: 6
- item: '[[items/material/184-plant-seed|Plant Seed]]'
  slots_of_30: 5
- item: '[[items/material/179-weed-root|Weed Root]]'
  slots_of_30: 6
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/body/2-flower-shorts|Flower Shorts]]'
  slots_of_30: 0.4
- item: '[[items/weapon/31-wooden-bat|Wooden Bat]]'
  slots_of_30: 0.4
- item: '[[items/hands/5-safe-gloves|Safe Gloves]]'
  slots_of_30: 0.8
- item: '[[items/weapon/2-short-sword|Short Sword]]'
  slots_of_30: 0.4
- item: '[[items/weapon/201-toy-bow|Toy Bow]]'
  slots_of_30: 0.4
- item: '[[items/head/4-bandana-of-iguje|Bandana of Iguje]]'
  slots_of_30: 0.4
- item: '[[items/body/4-blue-wild-jeans|Blue Wild Jeans]]'
  slots_of_30: 0.4
- item: '[[items/hands/4-dumb-gloves|Dumb Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/4-leather-shoes|Leather Shoes]]'
  slots_of_30: 0.4
quests:
- '[[quests/2206-tibbs-farm|Tibbs'' Farm]]'
- '[[quests/902-muse-job-change-quest|Muse Job Change Quest]]'
spawns:
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5260
  y: 5478
  count: 1
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5222
  y: 5455
  count: 2
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5205
  y: 5480
  count: 1
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5290
  y: 5468
  count: 1
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5056.89
  y: 5438.41
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
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5068.3
  y: 5300.7
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5253.1
  y: 5294.2
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5133.1
  y: 5194.1
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5286.1
  y: 5278.2
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5329.8
  y: 4978.3
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5440.1
  y: 5035.2
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5477.7
  y: 4990.3
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5449.8
  y: 5011
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5491.1
  y: 4960.3
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5268.6
  y: 4903.1
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5356.6
  y: 4958.1
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5311.4
  y: 4910.5
  count: 1
  group: basic
source:
  data: godot/data/monsters/42-elder-flanae.tres, drop table godot/data/drop_tables/117-elder-flanae.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Elder Flanae

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].

## Changed from iROSE

The Adventurer's Plain redesign moved its spawn points there (see [[zones/22-adventurer-s-plain|Adventurer's Plain]]). Its kills also count for [[quests/2206-tibbs-farm|Tibbs' Farm]].
