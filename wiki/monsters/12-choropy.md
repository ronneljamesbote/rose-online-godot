---
kind: monster
id: 12
name: Choropy
status: in-game
level: 6
hp: 156
hp_per_level: 26
attack: 10
hit: 61
defence: 33
resistance: 16
avoid: 8
attack_speed: 90
attack_range: 2
damage: physical
walk_speed: 175
run_speed: 260
xp: 11
drop_item_rate: 62
drop_money_rate: 8
zones: 1
drops:
- item: '[[items/material/171-thin-insect-shell|Thin Insect Shell]]'
  slots_of_30: 6
- item: '[[items/material/176-insect-leg|Insect Leg]]'
  slots_of_30: 7
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/403-choropy|Choropy]]'
  slots_of_30: 2
- item: '[[items/weapon/161-bamboo-spear|Bamboo Spear]]'
  slots_of_30: 0.4
- item: '[[items/weapon/2-short-sword|Short Sword]]'
  slots_of_30: 1.2
- item: '[[items/body/5-yellow-wild-jeans|Yellow Wild Jeans]]'
  slots_of_30: 0.4
- item: '[[items/weapon/1-wooden-sword|Wooden Sword]]'
  slots_of_30: 0.6
- item: '[[items/head/3-luxurious-green-hat|Luxurious Green Hat]]'
  slots_of_30: 0.6
- item: '[[items/body/3-leopard-pants|Leopard Pants]]'
  slots_of_30: 0.6
- item: '[[items/hands/3-leather-gloves|Leather Gloves]]'
  slots_of_30: 0.6
- item: '[[items/feet/3-waterproof-shoes|Waterproof Shoes]]'
  slots_of_30: 0.6
quests:
- '[[quests/5053-hunting-practice-3|Hunting Practice (3)]]'
spawns:
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5330
  y: 5300
  count: 3
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5370
  y: 5235
  count: 3
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5450
  y: 5250
  count: 3
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5488.27
  y: 5199
  count: 3
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5620
  y: 5150
  count: 1
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5660
  y: 5160
  count: 2
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5565
  y: 5075
  count: 1
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5680
  y: 5115
  count: 1
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5605
  y: 5085
  count: 2
  group: basic
source:
  data: godot/data/monsters/12-choropy.tres, drop table godot/data/drop_tables/105-choropy.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Choropy

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].

## Changed from iROSE

The Adventurer's Plain redesign moved its spawn points there (see [[zones/22-adventurer-s-plain|Adventurer's Plain]]).
