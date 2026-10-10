---
kind: monster
id: 67
name: Queen HoneyBee
status: in-game
level: 26
hp: 520
hp_per_level: 20
attack: 85
hit: 90
defence: 51
resistance: 100
avoid: 45
attack_speed: 80
attack_range: 8
damage: magic
walk_speed: 240
run_speed: 320
xp: 26
drop_item_rate: 60
drop_money_rate: 10
zones: 1
drops:
- item: '[[items/material/177-insect-wing|Insect Wing]]'
  slots_of_30: 6
- item: '[[items/material/181-pollen|Pollen]]'
  slots_of_30: 8
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/21-mana-vial-s|Mana Vial (S)]]'
  slots_of_30: 1
- item: '[[items/consumable/161-mp-point-30|MP Point (+30)]]'
  slots_of_30: 1
- item: '[[items/weapon/4-khukuri|Khukuri]]'
  slots_of_30: 0.4
- item: '[[items/weapon/302-lemmings-rod|Lemmings Rod]]'
  slots_of_30: 0.4
- item: '[[items/weapon/102-nameless-sword|Nameless Sword]]'
  slots_of_30: 0.4
- item: '[[items/weapon/232-air-gun|Air Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/402-sword-knuckle|Sword Knuckle]]'
  slots_of_30: 0.2
- item: '[[items/weapon/203-long-bow|Long Bow]]'
  slots_of_30: 0.2
- item: '[[items/subweapon/61-book-of-standards|Book of Standards]]'
  slots_of_30: 0.2
- item: '[[items/weapon/163-round-spear|Round Spear]]'
  slots_of_30: 0.4
- item: '[[items/head/9-luxurious-rodeo-hat|Luxurious Rodeo Hat]]'
  slots_of_30: 0.4
- item: '[[items/body/9-brown-denim-look|Brown Denim Look]]'
  slots_of_30: 0.4
- item: '[[items/hands/9-hill-gray-gloves|Hill Gray Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/9-wild-walkers|Wild Walkers]]'
  slots_of_30: 0.4
quests:
- '[[quests/118-forbidden-potion|Forbidden Potion]]'
spawns:
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5630.2
  y: 5460.2
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5630.2
  y: 5460.2
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5084.7
  y: 5379.9
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5110.9
  y: 5395.6
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5135
  y: 5426.3
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5195.6
  y: 5436.6
  count: 2
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5119.9
  y: 5164.1
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5098.8
  y: 5188.9
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5072.8
  y: 5184.9
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5045.6
  y: 5239.9
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5283.7
  y: 5142.1
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5342.8
  y: 5142.1
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5246.1
  y: 5119
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5246.1
  y: 4983.5
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5434.3
  y: 5009.9
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5338.9
  y: 5075.1
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5336.3
  y: 4953.1
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 67, ITEM_DROP.STB row 128
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Queen HoneyBee

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
