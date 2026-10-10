---
kind: monster
id: 5
name: Red Jelly Bean
status: in-game
level: 6
hp: 31
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
spawns:
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5260.4
  y: 5363.6
  count: 2
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5335.4
  y: 5371.2
  count: 1
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5320.2
  y: 5331.5
  count: 2
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5296.6
  y: 5332.9
  count: 2
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5346.3
  y: 5337.4
  count: 1
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5346.3
  y: 5337.4
  count: 2
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5480.4
  y: 5378.7
  count: 2
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5575.2
  y: 5319
  count: 1
  group: reinforcements
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5496.2
  y: 5398.8
  count: 2
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5608.5
  y: 5294.6
  count: 1
  group: reinforcements
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5629.5
  y: 5348.9
  count: 1
  group: reinforcements
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5792.3
  y: 5439.4
  count: 1
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5204.5
  y: 5221.1
  count: 1
  group: basic
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5665.1
  y: 5279.8
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 5, ITEM_DROP.STB row 103
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Red Jelly Bean

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
