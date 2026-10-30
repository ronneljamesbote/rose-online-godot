---
kind: monster
id: 8
name: ButterFly
status: in-game
level: 1
hp: 54
hp_per_level: 54
attack: 1
hit: 55
defence: 23
resistance: 10
avoid: 3
attack_speed: 20
attack_range: 1.5
damage: physical
walk_speed: 110
run_speed: 150
xp: 6
drop_item_rate: 40
drop_money_rate: 1
zones: 1
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
spawns:
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5216.3
  y: 4428.8
  count: 2
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5619.1
  y: 4454.2
  count: 1
  group: basic
source:
  data: godot/data/monsters/8-butterfly.tres, drop table godot/data/drop_tables/100-mini-jelly-bean-butterfly.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# ButterFly

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
