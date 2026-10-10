---
kind: monster
id: 175
name: Aqua Ranger
status: in-game
level: 32
hp: 24
attack: 104
hit: 96
defence: 66
resistance: 70
avoid: 68
attack_speed: 95
attack_range: 22
damage: physical
walk_speed: 240
run_speed: 530
xp: 32
drop_item_rate: 62
drop_money_rate: 12
zones: 2
drops:
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 7
- item: '[[items/material/192-animal-backbone|Animal Backbone]]'
  slots_of_30: 8
- item: '[[items/material/178-sticky-liquid|Sticky Liquid]]'
  slots_of_30: 1
- item: '[[items/consumable/154-hp-point-300|HP Point (+300)]]'
  slots_of_30: 1
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/163-mp-point-100|MP Point (+100)]]'
  slots_of_30: 1
- item: '[[items/jewellery/251-socket-ring|Socket Ring]]'
  slots_of_30: 1
- item: '[[items/jewellery/3-faint-ring|Faint Ring]]'
  slots_of_30: 1
- item: '[[items/weapon/261-wooden-launcher|Wooden Launcher]]'
  slots_of_30: 0.2
- item: '[[items/weapon/233-gloria-gun|Gloria Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/4-khukuri|Khukuri]]'
  slots_of_30: 0.2
- item: '[[items/weapon/102-nameless-sword|Nameless Sword]]'
  slots_of_30: 0.2
- item: '[[items/weapon/163-round-spear|Round Spear]]'
  slots_of_30: 0.6
- item: '[[items/weapon/403-katar|Katar]]'
  slots_of_30: 0.2
- item: '[[items/weapon/5-long-sword|Long Sword]]'
  slots_of_30: 0.2
- item: '[[items/weapon/332-mage-s-wand|Mage''s Wand]]'
  slots_of_30: 0.2
- item: '[[items/head/9-luxurious-rodeo-hat|Luxurious Rodeo Hat]]'
  slots_of_30: 0.4
- item: '[[items/body/9-brown-denim-look|Brown Denim Look]]'
  slots_of_30: 0.4
- item: '[[items/hands/9-hill-gray-gloves|Hill Gray Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/9-wild-walkers|Wild Walkers]]'
  slots_of_30: 0.4
- item: '[[items/subweapon/3-round-shield|Round Shield]]'
  slots_of_30: 0.4
quests:
- '[[quests/858-hovert-s-mementos|Hovert''s Mementos]]'
spawns:
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5247
  y: 5412.1
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5301
  y: 5438.1
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5372.7
  y: 5429.7
  count: 1
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
  x: 5369.1
  y: 5324.3
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5281.4
  y: 5338.8
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5320.5
  y: 5320.7
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5268
  y: 5316.7
  count: 2
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5234.3
  y: 5312.8
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5316.6
  y: 5329.4
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5378.2
  y: 5301.8
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5453.2
  y: 5294
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5275.2
  y: 5242.8
  count: 2
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5297.7
  y: 5184
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5461.4
  y: 5227.6
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5481.3
  y: 5261.7
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5375.1
  y: 5092.9
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 175, ITEM_DROP.STB row 189
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Aqua Ranger

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
