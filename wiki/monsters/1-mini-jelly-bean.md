---
kind: monster
id: 1
name: Mini-Jelly Bean
status: in-game
level: 2
hp: 88
hp_per_level: 44
attack: 2
hit: 56
defence: 25
resistance: 11
avoid: 5
attack_speed: 80
attack_range: 1.9
damage: physical
walk_speed: 160
run_speed: 370
xp: 9
drop_item_rate: 70
drop_money_rate: 6
zones: 2
drops:
- item: '[[items/material/171-thin-insect-shell|Thin Insect Shell]]'
  slots_of_30: 5
- item: '[[items/material/173-limpid-skin|Limpid Skin]]'
  slots_of_30: 8
- item: '[[items/material/178-sticky-liquid|Sticky Liquid]]'
  slots_of_30: 3
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/401-jelly-bean|Jelly Bean]]'
  slots_of_30: 2
- item: '[[items/weapon/2-short-sword|Short Sword]]'
  slots_of_30: 1.2
- item: '[[items/hands/3-leather-gloves|Leather Gloves]]'
  slots_of_30: 1.2
- item: '[[items/feet/2-red-shoes|Red Shoes]]'
  slots_of_30: 0.6
- item: '[[items/weapon/161-bamboo-spear|Bamboo Spear]]'
  slots_of_30: 0.6
- item: '[[items/weapon/1-wooden-sword|Wooden Sword]]'
  slots_of_30: 0.6
- item: '[[items/head/3-luxurious-green-hat|Luxurious Green Hat]]'
  slots_of_30: 0.6
- item: '[[items/body/3-leopard-pants|Leopard Pants]]'
  slots_of_30: 0.6
- item: '[[items/feet/3-waterproof-shoes|Waterproof Shoes]]'
  slots_of_30: 0.6
quests:
- '[[quests/2202-jelly-on-the-jetty|Jelly on the Jetty]]'
- '[[quests/5033-stockpiling-jelly-bean-hides|Stockpiling Jelly Bean Hides]]'
spawns:
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5752
  y: 5455
  count: 4
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5790
  y: 5490
  count: 4
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5765
  y: 5415
  count: 4
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5770
  y: 5370
  count: 4
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5745
  y: 5085
  count: 4
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5741
  y: 5050
  count: 4
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5329.9
  y: 5070.4
  count: 0
  group: reinforcements
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5329.9
  y: 5070.4
  count: 0
  group: reinforcements
source:
  data: godot/data/monsters/1-mini-jelly-bean.tres, drop table godot/data/drop_tables/100-mini-jelly-bean-butterfly.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Mini-Jelly Bean

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].

## Changed from iROSE

The Adventurer's Plain redesign moved its spawn points there (see [[zones/22-adventurer-s-plain|Adventurer's Plain]]). Its kills also count for [[quests/2202-jelly-on-the-jetty|Jelly on the Jetty]].
