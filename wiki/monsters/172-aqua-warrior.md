---
kind: monster
id: 172
name: Aqua Warrior
status: in-game
level: 31
hp: 26
attack: 122
hit: 114
defence: 99
resistance: 39
avoid: 33
attack_speed: 110
attack_range: 3.6
damage: physical
walk_speed: 220
run_speed: 430
xp: 29
drop_item_rate: 63
drop_money_rate: 12
zones: 2
drops:
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 5
- item: '[[items/material/192-animal-backbone|Animal Backbone]]'
  slots_of_30: 10
- item: '[[items/material/178-sticky-liquid|Sticky Liquid]]'
  slots_of_30: 1
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/155-hp-point-500|HP Point (+500)]]'
  slots_of_30: 1
- item: '[[items/consumable/165-mp-point-300|MP Point (+300)]]'
  slots_of_30: 1
- item: '[[items/jewellery/271-socket-earring|Socket Earring]]'
  slots_of_30: 1
- item: '[[items/weapon/163-round-spear|Round Spear]]'
  slots_of_30: 0.8
- item: '[[items/weapon/302-lemmings-rod|Lemmings Rod]]'
  slots_of_30: 0.2
- item: '[[items/weapon/232-air-gun|Air Gun]]'
  slots_of_30: 0.2
- item: '[[items/head/9-luxurious-rodeo-hat|Luxurious Rodeo Hat]]'
  slots_of_30: 0.2
- item: '[[items/body/9-brown-denim-look|Brown Denim Look]]'
  slots_of_30: 0.2
- item: '[[items/hands/9-hill-gray-gloves|Hill Gray Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/9-wild-walkers|Wild Walkers]]'
  slots_of_30: 0.2
- item: '[[items/subweapon/63-book-of-concentration|Book of Concentration]]'
  slots_of_30: 0.2
- item: '[[items/weapon/131-woodman-axe|Woodman Axe]]'
  slots_of_30: 0.8
- item: '[[items/weapon/332-mage-s-wand|Mage''s Wand]]'
  slots_of_30: 0.4
- item: '[[items/weapon/33-buffoon-mace|Buffoon Mace]]'
  slots_of_30: 0.4
quests:
- '[[quests/858-hovert-s-mementos|Hovert''s Mementos]]'
spawns:
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5424.7
  y: 5346.1
  count: 2
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5328.3
  y: 5369.6
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5369.1
  y: 5324.3
  count: 3
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
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5297.7
  y: 5184
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
  count: 1
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
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5375.1
  y: 5092.9
  count: 2
  group: basic
source:
  data: LIST_NPC.STB row 172, ITEM_DROP.STB row 187
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Aqua Warrior

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
