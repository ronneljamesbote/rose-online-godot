---
kind: monster
id: 3
name: JellyNut
status: in-game
level: 5
hp: 140
hp_per_level: 28
attack: 8
hit: 60
defence: 31
resistance: 15
avoid: 9
attack_speed: 90
attack_range: 2.2
damage: physical
walk_speed: 175
run_speed: 400
xp: 11
drop_item_rate: 70
drop_money_rate: 12
zones: 1
drops:
- item: '[[items/material/171-thin-insect-shell|Thin Insect Shell]]'
  slots_of_30: 7
- item: '[[items/material/173-limpid-skin|Limpid Skin]]'
  slots_of_30: 8
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/1-health-vial-s|Health Vial (S)]]'
  slots_of_30: 1
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/402-jelly-nut|Jelly Nut]]'
  slots_of_30: 2
- item: '[[items/weapon/161-bamboo-spear|Bamboo Spear]]'
  slots_of_30: 1.8
- item: '[[items/head/3-luxurious-green-hat|Luxurious Green Hat]]'
  slots_of_30: 1.2
- item: '[[items/weapon/2-short-sword|Short Sword]]'
  slots_of_30: 1.2
- item: '[[items/body/3-leopard-pants|Leopard Pants]]'
  slots_of_30: 0.6
- item: '[[items/hands/3-leather-gloves|Leather Gloves]]'
  slots_of_30: 0.6
- item: '[[items/feet/3-waterproof-shoes|Waterproof Shoes]]'
  slots_of_30: 0.6
quests:
- '[[quests/2204-the-humming-stones|The Humming Stones]]'
- '[[quests/5033-stockpiling-jelly-bean-hides|Stockpiling Jelly Bean Hides]]'
spawns:
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5672
  y: 5395
  count: 4
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5702
  y: 5355
  count: 4
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5668
  y: 5360
  count: 4
  group: basic
source:
  data: godot/data/monsters/3-jellynut.tres, drop table godot/data/drop_tables/102-jellynut.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# JellyNut

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].

## Changed from iROSE

The Adventurer's Plain redesign moved its spawn points there (see [[zones/22-adventurer-s-plain|Adventurer's Plain]]). Its kills also count for [[quests/2204-the-humming-stones|The Humming Stones]].
