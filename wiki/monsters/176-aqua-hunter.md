---
kind: monster
id: 176
name: Aqua Hunter
status: in-game
level: 33
hp: 693
hp_per_level: 21
attack: 126
hit: 121
defence: 86
resistance: 40
avoid: 60
attack_speed: 82
attack_range: 20
damage: physical
walk_speed: 210
run_speed: 470
xp: 29
drop_item_rate: 65
drop_money_rate: 15
zones: 2
drops:
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 6
- item: '[[items/consumable/163-mp-point-100|MP Point (+100)]]'
  slots_of_30: 1
- item: '[[items/material/178-sticky-liquid|Sticky Liquid]]'
  slots_of_30: 1
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/material/192-animal-backbone|Animal Backbone]]'
  slots_of_30: 7
- item: '[[items/consumable/153-hp-point-200|HP Point (+200)]]'
  slots_of_30: 1
- item: '[[items/jewellery/261-socket-necklace|Socket Necklace]]'
  slots_of_30: 1
- item: '[[items/jewellery/5-pierced-ring|Pierced Ring]]'
  slots_of_30: 1
- item: '[[items/weapon/204-orc-bow|Orc Bow]]'
  slots_of_30: 0.2
- item: '[[items/weapon/261-wooden-launcher|Wooden Launcher]]'
  slots_of_30: 0.4
- item: '[[items/weapon/402-sword-knuckle|Sword Knuckle]]'
  slots_of_30: 0.2
- item: '[[items/weapon/332-mage-s-wand|Mage''s Wand]]'
  slots_of_30: 0.2
- item: '[[items/weapon/5-long-sword|Long Sword]]'
  slots_of_30: 0.6
- item: '[[items/weapon/34-ogre-mace|Ogre Mace]]'
  slots_of_30: 0.2
- item: '[[items/weapon/132-small-axe|Small Axe]]'
  slots_of_30: 0.2
- item: '[[items/weapon/333-sorcerer-s-wand|Sorcerer''s Wand]]'
  slots_of_30: 0.2
- item: '[[items/subweapon/3-round-shield|Round Shield]]'
  slots_of_30: 0.2
- item: '[[items/head/32-iron-helm|Iron Helm]]'
  slots_of_30: 0.4
- item: '[[items/body/62-green-vest-of-witch|Green Vest of Witch]]'
  slots_of_30: 0.4
- item: '[[items/hands/92-criker-gloves|Criker Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/122-vibe-shoes|Vibe Shoes]]'
  slots_of_30: 0.4
quests:
- '[[quests/858-hovert-s-mementos|Hovert''s Mementos]]'
spawns:
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5372.7
  y: 5429.7
  count: 2
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5423
  y: 5393.9
  count: 2
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5424.7
  y: 5346.1
  count: 2
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5328.3
  y: 5369.6
  count: 2
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5369.1
  y: 5324.3
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5281.4
  y: 5338.8
  count: 2
  group: reinforcements
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5268
  y: 5316.7
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5234.3
  y: 5312.8
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5234.3
  y: 5312.8
  count: 2
  group: reinforcements
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5316.6
  y: 5329.4
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5316.6
  y: 5329.4
  count: 2
  group: reinforcements
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5378.2
  y: 5301.8
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5378.2
  y: 5301.8
  count: 2
  group: reinforcements
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5453.2
  y: 5294
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5275.2
  y: 5242.8
  count: 1
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
  data: LIST_NPC.STB row 176, ITEM_DROP.STB row 190
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Aqua Hunter

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
