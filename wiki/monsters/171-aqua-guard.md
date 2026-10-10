---
kind: monster
id: 171
name: Aqua Guard
status: in-game
level: 30
hp: 32
attack: 95
hit: 105
defence: 120
resistance: 64
avoid: 24
attack_speed: 90
attack_range: 3.5
damage: physical
walk_speed: 180
run_speed: 480
xp: 31
drop_item_rate: 61
drop_money_rate: 10
zones: 2
drops:
- item: '[[items/material/192-animal-backbone|Animal Backbone]]'
  slots_of_30: 8
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 5
- item: '[[items/consumable/165-mp-point-300|MP Point (+300)]]'
  slots_of_30: 1
- item: '[[items/consumable/155-hp-point-500|HP Point (+500)]]'
  slots_of_30: 1
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/jewellery/251-socket-ring|Socket Ring]]'
  slots_of_30: 1
- item: '[[items/consumable/425-aqua-guard|Aqua Guard]]'
  slots_of_30: 1
- item: '[[items/jewellery/1-shining-ring|Shining Ring]]'
  slots_of_30: 1
- item: '[[items/hands/31-soldier-gloves|Soldier Gloves]]'
  slots_of_30: 0.2
- item: '[[items/head/121-brown-turban|Brown Turban]]'
  slots_of_30: 0.2
- item: '[[items/weapon/332-mage-s-wand|Mage''s Wand]]'
  slots_of_30: 0.6
- item: '[[items/weapon/33-buffoon-mace|Buffoon Mace]]'
  slots_of_30: 1
- item: '[[items/weapon/131-woodman-axe|Woodman Axe]]'
  slots_of_30: 0.6
- item: '[[items/head/9-luxurious-rodeo-hat|Luxurious Rodeo Hat]]'
  slots_of_30: 0.2
- item: '[[items/body/9-brown-denim-look|Brown Denim Look]]'
  slots_of_30: 0.2
- item: '[[items/hands/9-hill-gray-gloves|Hill Gray Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/9-wild-walkers|Wild Walkers]]'
  slots_of_30: 0.2
- item: '[[items/subweapon/3-round-shield|Round Shield]]'
  slots_of_30: 0.2
- item: '[[items/weapon/402-sword-knuckle|Sword Knuckle]]'
  slots_of_30: 0.4
spawns:
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5372.7
  y: 5429.7
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5423
  y: 5393.9
  count: 2
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5424.7
  y: 5346.1
  count: 2
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5424.7
  y: 5346.1
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5328.3
  y: 5369.6
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5328.3
  y: 5369.6
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5369.1
  y: 5324.3
  count: 2
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5281.4
  y: 5338.8
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5281.4
  y: 5338.8
  count: 2
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5320.5
  y: 5320.7
  count: 2
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5320.5
  y: 5320.7
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5320.5
  y: 5320.7
  count: 2
  group: reinforcements
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5268
  y: 5316.7
  count: 2
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5234.3
  y: 5312.8
  count: 2
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5316.6
  y: 5329.4
  count: 2
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5378.2
  y: 5301.8
  count: 2
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5453.2
  y: 5294
  count: 2
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5275.2
  y: 5242.8
  count: 2
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5297.7
  y: 5184
  count: 2
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5461.4
  y: 5227.6
  count: 2
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5481.3
  y: 5261.7
  count: 2
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5375.1
  y: 5092.9
  count: 2
  group: basic
source:
  data: LIST_NPC.STB row 171, ITEM_DROP.STB row 186
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Aqua Guard

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
