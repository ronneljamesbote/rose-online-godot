---
kind: monster
id: 85
name: Turtle Guard
status: in-game
level: 29
hp: 928
hp_per_level: 32
attack: 91
hit: 104
defence: 116
resistance: 62
avoid: 23
attack_speed: 90
attack_range: 2.2
damage: physical
walk_speed: 180
run_speed: 470
xp: 30
drop_item_rate: 60
drop_money_rate: 23
zones: 1
drops:
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 6
- item: '[[items/material/192-animal-backbone|Animal Backbone]]'
  slots_of_30: 8
- item: '[[items/consumable/161-mp-point-30|MP Point (+30)]]'
  slots_of_30: 1
- item: '[[items/consumable/101-apple|Apple]]'
  slots_of_30: 1
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/411-turtle|Turtle]]'
  slots_of_30: 2
- item: '[[items/head/310-luxurious-rodeo-hat|Luxurious Rodeo Hat]]'
  slots_of_30: 0.2
- item: '[[items/weapon/332-mage-s-wand|Mage''s Wand]]'
  slots_of_30: 0.2
- item: '[[items/weapon/32-monkey-wrench|Monkey Wrench]]'
  slots_of_30: 0.2
- item: '[[items/weapon/131-woodman-axe|Woodman Axe]]'
  slots_of_30: 0.2
- item: '[[items/weapon/33-buffoon-mace|Buffoon Mace]]'
  slots_of_30: 0.2
- item: '[[items/body/31-soldier-armor|Soldier Armor]]'
  slots_of_30: 0.2
- item: '[[items/hands/61-spiritual-gloves|Spiritual Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/91-hunter-boots|Hunter Boots]]'
  slots_of_30: 0.2
- item: '[[items/head/121-brown-turban|Brown Turban]]'
  slots_of_30: 0.2
- item: '[[items/subweapon/3-round-shield|Round Shield]]'
  slots_of_30: 0.2
- item: '[[items/weapon/4-khukuri|Khukuri]]'
  slots_of_30: 0.4
- item: '[[items/weapon/102-nameless-sword|Nameless Sword]]'
  slots_of_30: 0.4
- item: '[[items/weapon/203-long-bow|Long Bow]]'
  slots_of_30: 0.4
- item: '[[items/weapon/232-air-gun|Air Gun]]'
  slots_of_30: 0.4
- item: '[[items/weapon/302-lemmings-rod|Lemmings Rod]]'
  slots_of_30: 0.4
quests:
- '[[quests/5008-stockpiling-silver-fragments|Stockpiling Silver Fragments]]'
spawns:
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5590.8
  y: 5481.7
  count: 3
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5542.3
  y: 5459.6
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5542.3
  y: 5459.6
  count: 2
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5630.2
  y: 5460.2
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5649.6
  y: 5416.4
  count: 2
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5649.6
  y: 5416.4
  count: 3
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5650.1
  y: 5372.7
  count: 2
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5650.1
  y: 5372.7
  count: 3
  group: reinforcements
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
  count: 2
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5613.6
  y: 5305.4
  count: 2
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5613.6
  y: 5305.4
  count: 2
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5440.8
  y: 5012.2
  count: 2
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5498.9
  y: 5024.4
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 85, ITEM_DROP.STB row 139
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Turtle Guard

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
