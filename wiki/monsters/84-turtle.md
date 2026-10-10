---
kind: monster
id: 84
name: Turtle
status: in-game
level: 27
hp: 675
hp_per_level: 25
attack: 94
hit: 92
defence: 80
resistance: 49
avoid: 46
attack_speed: 100
attack_range: 2
damage: physical
walk_speed: 160
run_speed: 370
xp: 28
drop_item_rate: 58
drop_money_rate: 20
zones: 1
drops:
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 6
- item: '[[items/consumable/161-mp-point-30|MP Point (+30)]]'
  slots_of_30: 1
- item: '[[items/consumable/2-health-vial-m|Health Vial (M)]]'
  slots_of_30: 1
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/material/192-animal-backbone|Animal Backbone]]'
  slots_of_30: 7
- item: '[[items/consumable/411-turtle|Turtle]]'
  slots_of_30: 2
- item: '[[items/jewellery/88-pointed-necklace|Pointed Necklace]]'
  slots_of_30: 1
- item: '[[items/hands/8-gray-gloves|Gray Gloves]]'
  slots_of_30: 0.2
- item: '[[items/weapon/4-khukuri|Khukuri]]'
  slots_of_30: 0.6
- item: '[[items/weapon/401-knuckle|Knuckle]]'
  slots_of_30: 0.2
- item: '[[items/weapon/203-long-bow|Long Bow]]'
  slots_of_30: 0.2
- item: '[[items/weapon/402-sword-knuckle|Sword Knuckle]]'
  slots_of_30: 0.2
- item: '[[items/head/31-soldier-helm|Soldier Helm]]'
  slots_of_30: 0.2
- item: '[[items/body/61-spiritual-vest|Spiritual Vest]]'
  slots_of_30: 0.2
- item: '[[items/hands/91-hunter-gloves|Hunter Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/121-brown-shoes|Brown Shoes]]'
  slots_of_30: 0.2
- item: '[[items/subweapon/2-tarz|Tarz]]'
  slots_of_30: 0.2
- item: '[[items/weapon/231-bubble-gun|Bubble Gun]]'
  slots_of_30: 0.4
- item: '[[items/weapon/32-monkey-wrench|Monkey Wrench]]'
  slots_of_30: 0.4
- item: '[[items/weapon/162-javelin|Javelin]]'
  slots_of_30: 0.4
- item: '[[items/weapon/101-haedong-sword|Haedong Sword]]'
  slots_of_30: 0.4
quests:
- '[[quests/5008-stockpiling-silver-fragments|Stockpiling Silver Fragments]]'
spawns:
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5590.8
  y: 5481.7
  count: 2
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5590.8
  y: 5481.7
  count: 2
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5542.3
  y: 5459.6
  count: 2
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5542.3
  y: 5459.6
  count: 2
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5630.2
  y: 5460.2
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5630.2
  y: 5460.2
  count: 2
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5649.6
  y: 5416.4
  count: 2
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5649.6
  y: 5416.4
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5650.1
  y: 5372.7
  count: 2
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5650.1
  y: 5372.7
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5648.3
  y: 5319.4
  count: 2
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5648.3
  y: 5319.4
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5665
  y: 5299.8
  count: 2
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5665
  y: 5299.8
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5613.6
  y: 5305.4
  count: 2
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5613.6
  y: 5305.4
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5440.8
  y: 5012.2
  count: 2
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5498.9
  y: 5024.4
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 84, ITEM_DROP.STB row 138
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Turtle

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
