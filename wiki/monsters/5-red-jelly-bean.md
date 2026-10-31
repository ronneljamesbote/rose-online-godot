---
kind: monster
id: 5
name: Red Jelly Bean
status: in-game
level: 6
hp: 186
hp_per_level: 31
attack: 12
hit: 56
defence: 27
resistance: 17
avoid: 15
attack_speed: 95
attack_range: 2.5
damage: physical
walk_speed: 200
run_speed: 450
xp: 13
drop_item_rate: 75
drop_money_rate: 10
zones: 1
drops:
- item: '[[items/material/171-thin-insect-shell|Thin Insect Shell]]'
  slots_of_30: 7
- item: '[[items/consumable/102-banana|Banana]]'
  slots_of_30: 1
- item: '[[items/material/173-limpid-skin|Limpid Skin]]'
  slots_of_30: 5
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/172-stamina-75|Stamina (+75)]]'
  slots_of_30: 1
- item: '[[items/consumable/401-jelly-bean|Jelly Bean]]'
  slots_of_30: 2
- item: '[[items/material/178-sticky-liquid|Sticky Liquid]]'
  slots_of_30: 1
- item: '[[items/body/2-flower-shorts|Flower Shorts]]'
  slots_of_30: 0.6
- item: '[[items/head/2-green-hat|Green Hat]]'
  slots_of_30: 0.6
- item: '[[items/weapon/31-wooden-bat|Wooden Bat]]'
  slots_of_30: 0.6
- item: '[[items/feet/3-waterproof-shoes|Waterproof Shoes]]'
  slots_of_30: 1.2
- item: '[[items/weapon/2-short-sword|Short Sword]]'
  slots_of_30: 0.6
- item: '[[items/weapon/201-toy-bow|Toy Bow]]'
  slots_of_30: 0.6
- item: '[[items/head/3-luxurious-green-hat|Luxurious Green Hat]]'
  slots_of_30: 0.6
- item: '[[items/body/3-leopard-pants|Leopard Pants]]'
  slots_of_30: 0.6
- item: '[[items/hands/3-leather-gloves|Leather Gloves]]'
  slots_of_30: 0.6
quests:
- '[[quests/2205-ford-watch|Ford Watch]]'
spawns:
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5397.17
  y: 5407.83
  count: 4
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5387.27
  y: 5337.07
  count: 4
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5330
  y: 5300
  count: 1
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5537
  y: 5398.86
  count: 4
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5544
  y: 5320
  count: 4
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5370
  y: 5235
  count: 1
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5571.59
  y: 5243.11
  count: 4
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5450
  y: 5250
  count: 1
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5488.27
  y: 5199
  count: 1
  group: basic
source:
  data: godot/data/monsters/5-red-jelly-bean.tres, drop table godot/data/drop_tables/103-red-jelly-bean.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Red Jelly Bean

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].

## Changed from iROSE

The Adventurer's Plain redesign moved its spawn points there (see [[zones/22-adventurer-s-plain|Adventurer's Plain]]). Its kills also count for [[quests/2205-ford-watch|Ford Watch]].
