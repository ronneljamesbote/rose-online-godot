---
kind: monster
id: 81
name: Beetle
status: in-game
level: 20
hp: 25
attack: 69
hit: 81
defence: 63
resistance: 37
avoid: 38
attack_speed: 95
attack_range: 1.8
damage: physical
walk_speed: 150
run_speed: 420
xp: 25
drop_item_rate: 58
drop_money_rate: 15
zones: 1
drops:
- item: '[[items/material/176-insect-leg|Insect Leg]]'
  slots_of_30: 6
- item: '[[items/material/178-sticky-liquid|Sticky Liquid]]'
  slots_of_30: 1
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/161-mp-point-30|MP Point (+30)]]'
  slots_of_30: 1
- item: '[[items/material/172-thick-insect-shell|Thick Insect Shell]]'
  slots_of_30: 8
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/410-beetle|Beetle]]'
  slots_of_30: 2
- item: '[[items/face/8-iron-mask|Iron Mask]]'
  slots_of_30: 0.2
- item: '[[items/weapon/401-knuckle|Knuckle]]'
  slots_of_30: 0.4
- item: '[[items/weapon/231-bubble-gun|Bubble Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/301-baobab-rod|Baobab Rod]]'
  slots_of_30: 0.2
- item: '[[items/weapon/32-monkey-wrench|Monkey Wrench]]'
  slots_of_30: 0.4
- item: '[[items/weapon/162-javelin|Javelin]]'
  slots_of_30: 0.2
- item: '[[items/weapon/331-baobab-wand|Baobab Wand]]'
  slots_of_30: 0.2
- item: '[[items/subweapon/2-tarz|Tarz]]'
  slots_of_30: 0.2
- item: '[[items/weapon/101-haedong-sword|Haedong Sword]]'
  slots_of_30: 0.4
- item: '[[items/body/31-soldier-armor|Soldier Armor]]'
  slots_of_30: 0.4
- item: '[[items/hands/61-spiritual-gloves|Spiritual Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/91-hunter-boots|Hunter Boots]]'
  slots_of_30: 0.4
- item: '[[items/head/121-brown-turban|Brown Turban]]'
  slots_of_30: 0.4
quests:
- '[[quests/855-living-as-a-true-soldier|Living as a True Soldier]]'
spawns:
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5479.9
  y: 5384.7
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5495.1
  y: 5245.4
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5495.1
  y: 5245.4
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5443.2
  y: 5223.3
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5386.9
  y: 4966.3
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5386.9
  y: 4966.3
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5440.1
  y: 5035.2
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5477.7
  y: 4990.3
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5495.8
  y: 5076.3
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5495.8
  y: 5076.3
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5449.8
  y: 5011
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5491.1
  y: 4960.3
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5249
  y: 4903.9
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5249
  y: 4903.9
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5422.5
  y: 4941.1
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5422.5
  y: 4941.1
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5436.3
  y: 4871.8
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5436.3
  y: 4871.8
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5481.3
  y: 4881.9
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5481.3
  y: 4881.9
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 81, ITEM_DROP.STB row 136
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Beetle

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
