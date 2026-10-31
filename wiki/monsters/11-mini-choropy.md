---
kind: monster
id: 11
name: Mini-Choropy
status: in-game
level: 4
hp: 124
hp_per_level: 31
attack: 6
hit: 59
defence: 29
resistance: 13
avoid: 6
attack_speed: 90
attack_range: 1.8
damage: physical
walk_speed: 160
run_speed: 242
xp: 10
drop_item_rate: 60
drop_money_rate: 5
zones: 1
drops:
- item: '[[items/material/171-thin-insect-shell|Thin Insect Shell]]'
  slots_of_30: 5
- item: '[[items/material/176-insect-leg|Insect Leg]]'
  slots_of_30: 9
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 2
- item: '[[items/consumable/403-choropy|Choropy]]'
  slots_of_30: 2
- item: '[[items/weapon/2-short-sword|Short Sword]]'
  slots_of_30: 0.4
- item: '[[items/feet/2-red-shoes|Red Shoes]]'
  slots_of_30: 0.4
- item: '[[items/weapon/161-bamboo-spear|Bamboo Spear]]'
  slots_of_30: 0.4
- item: '[[items/hands/4-dumb-gloves|Dumb Gloves]]'
  slots_of_30: 0.4
- item: '[[items/head/3-luxurious-green-hat|Luxurious Green Hat]]'
  slots_of_30: 1
- item: '[[items/weapon/31-wooden-bat|Wooden Bat]]'
  slots_of_30: 0.6
- item: '[[items/body/3-leopard-pants|Leopard Pants]]'
  slots_of_30: 0.6
- item: '[[items/hands/3-leather-gloves|Leather Gloves]]'
  slots_of_30: 0.6
- item: '[[items/feet/3-waterproof-shoes|Waterproof Shoes]]'
  slots_of_30: 0.6
quests:
- '[[quests/5052-hunting-practice-2|Hunting Practice (2)]]'
spawns:
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5620
  y: 5150
  count: 3
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5660
  y: 5160
  count: 2
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5565
  y: 5075
  count: 3
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5680
  y: 5115
  count: 3
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5605
  y: 5085
  count: 2
  group: basic
source:
  data: godot/data/monsters/11-mini-choropy.tres, drop table godot/data/drop_tables/104-mini-choropy.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Mini-Choropy

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].

## Changed from iROSE

The Adventurer's Plain redesign moved its spawn points there (see [[zones/22-adventurer-s-plain|Adventurer's Plain]]).
