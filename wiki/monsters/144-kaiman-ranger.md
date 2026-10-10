---
kind: monster
id: 144
name: Kaiman Ranger
status: in-game
level: 61
hp: 28
attack: 210
hit: 144
defence: 133
resistance: 139
avoid: 115
attack_speed: 105
attack_range: 24
damage: physical
walk_speed: 220
run_speed: 550
xp: 47
drop_item_rate: 53
drop_money_rate: 16
zones: 1
drops:
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 7
- item: '[[items/consumable/164-mp-point-200|MP Point (+200)]]'
  slots_of_30: 1
- item: '[[items/consumable/5-health-bottle-m|Health Bottle (M)]]'
  slots_of_30: 1
- item: '[[items/consumable/155-hp-point-500|HP Point (+500)]]'
  slots_of_30: 1
- item: '[[items/material/192-animal-backbone|Animal Backbone]]'
  slots_of_30: 7
- item: '[[items/jewellery/261-socket-necklace|Socket Necklace]]'
  slots_of_30: 1
- item: '[[items/jewellery/89-solid-necklace|Solid Necklace]]'
  slots_of_30: 1
- item: '[[items/face/7-jolly-front-mask|Jolly Front Mask]]'
  slots_of_30: 0.2
- item: '[[items/weapon/236-sorden-gun|Sorden Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/135-orc-axe|Orc Axe]]'
  slots_of_30: 0.6
- item: '[[items/weapon/336-windphon-wand|Windphon Wand]]'
  slots_of_30: 0.6
- item: '[[items/weapon/434-twin-ghost-bat|Twin Ghost Bat]]'
  slots_of_30: 0.6
- item: '[[items/weapon/207-iron-bow|Iron Bow]]'
  slots_of_30: 0.2
- item: '[[items/feet/33-trunket-boots|Trunket Boots]]'
  slots_of_30: 0.2
- item: '[[items/head/63-purple-hat-of-witch|Purple Hat of Witch]]'
  slots_of_30: 0.2
- item: '[[items/body/93-ranger-chest|Ranger Chest]]'
  slots_of_30: 0.2
- item: '[[items/hands/123-tamiya-gloves|Tamiya Gloves]]'
  slots_of_30: 0.2
- item: '[[items/weapon/264-bronze-launcher|Bronze Launcher]]'
  slots_of_30: 0.4
- item: '[[items/subweapon/6-hoplon|Hoplon]]'
  slots_of_30: 0.4
spawns:
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5268
  y: 5470.3
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5657.5
  y: 5532.6
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5102.7
  y: 5345.5
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5147.5
  y: 5366.5
  count: 2
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
  x: 5418.6
  y: 5387.2
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5459.4
  y: 5347.3
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5518.5
  y: 5322.3
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5530
  y: 5415.3
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5505.9
  y: 5433
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5205.3
  y: 5270.5
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 144, ITEM_DROP.STB row 168
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Kaiman Ranger

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
