---
kind: monster
id: 142
name: Kaiman Guard
status: in-game
level: 60
hp: 35
attack: 201
hit: 159
defence: 224
resistance: 133
avoid: 48
attack_speed: 90
attack_range: 2.6
damage: physical
walk_speed: 240
run_speed: 600
xp: 43
drop_item_rate: 49
drop_money_rate: 16
zones: 1
drops:
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 8
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/164-mp-point-200|MP Point (+200)]]'
  slots_of_30: 1
- item: '[[items/consumable/26-mana-bottle-l|Mana Bottle (L)]]'
  slots_of_30: 1
- item: '[[items/consumable/155-hp-point-500|HP Point (+500)]]'
  slots_of_30: 1
- item: '[[items/material/192-animal-backbone|Animal Backbone]]'
  slots_of_30: 7
- item: '[[items/jewellery/261-socket-necklace|Socket Necklace]]'
  slots_of_30: 1
- item: '[[items/feet/123-tamiya-shoes|Tamiya Shoes]]'
  slots_of_30: 0.2
- item: '[[items/weapon/106-bull-sword|Bull Sword]]'
  slots_of_30: 0.4
- item: '[[items/weapon/433-dual-bushido|Dual Bushido]]'
  slots_of_30: 0.2
- item: '[[items/weapon/8-elven-sword|Elven Sword]]'
  slots_of_30: 0.8
- item: '[[items/weapon/306-wisp-staff|Wisp Staff]]'
  slots_of_30: 0.2
- item: '[[items/weapon/62-cutter-bow-gun|Cutter Bow Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/207-iron-bow|Iron Bow]]'
  slots_of_30: 0.2
- item: '[[items/subweapon/6-hoplon|Hoplon]]'
  slots_of_30: 0.2
- item: '[[items/head/13-luxurious-venetian-hat|Luxurious Venetian Hat]]'
  slots_of_30: 0.4
- item: '[[items/body/13-fancy-blue|Fancy Blue]]'
  slots_of_30: 0.4
- item: '[[items/hands/13-luxurious-venetian-gloves|Luxurious Venetian Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/13-luxurious-marble-shoes|Luxurious Marble Shoes]]'
  slots_of_30: 0.4
spawns:
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5268
  y: 5470.3
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5375.1
  y: 5448.6
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5102.7
  y: 5345.5
  count: 2
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5147.5
  y: 5366.5
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5146.9
  y: 5305.7
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5198.9
  y: 5381.2
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5276.4
  y: 5420.3
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5205.3
  y: 5270.5
  count: 2
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5205.3
  y: 5270.5
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 142, ITEM_DROP.STB row 166
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Kaiman Guard

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
