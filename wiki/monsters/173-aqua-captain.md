---
kind: monster
id: 173
name: Aqua Captain
status: in-game
level: 34
hp: 44
attack: 140
hit: 119
defence: 112
resistance: 70
avoid: 43
attack_speed: 95
attack_range: 3.8
damage: physical
walk_speed: 210
run_speed: 460
xp: 60
drop_item_rate: 70
drop_money_rate: 15
zones: 2
drops:
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 6
- item: '[[items/consumable/165-mp-point-300|MP Point (+300)]]'
  slots_of_30: 1
- item: '[[items/material/192-animal-backbone|Animal Backbone]]'
  slots_of_30: 8
- item: '[[items/consumable/172-stamina-75|Stamina (+75)]]'
  slots_of_30: 1
- item: '[[items/consumable/155-hp-point-500|HP Point (+500)]]'
  slots_of_30: 1
- item: '[[items/jewellery/261-socket-necklace|Socket Necklace]]'
  slots_of_30: 1
- item: '[[items/consumable/425-aqua-guard|Aqua Guard]]'
  slots_of_30: 1
- item: '[[items/material/189-predator-claw|Predator Claw]]'
  slots_of_30: 1
- item: '[[items/jewellery/4-fine-ring|Fine Ring]]'
  slots_of_30: 1
- item: '[[items/hands/32-iron-gauntlets|Iron Gauntlets]]'
  slots_of_30: 0.4
- item: '[[items/feet/62-green-sandals-of-witch|Green Sandals of Witch]]'
  slots_of_30: 0.4
- item: '[[items/weapon/131-woodman-axe|Woodman Axe]]'
  slots_of_30: 0.2
- item: '[[items/weapon/332-mage-s-wand|Mage''s Wand]]'
  slots_of_30: 0.4
- item: '[[items/weapon/163-round-spear|Round Spear]]'
  slots_of_30: 0.2
- item: '[[items/head/92-criker-hat|Criker Hat]]'
  slots_of_30: 0.2
- item: '[[items/body/122-vibe-vest|Vibe Vest]]'
  slots_of_30: 0.2
- item: '[[items/weapon/5-long-sword|Long Sword]]'
  slots_of_30: 0.4
- item: '[[items/weapon/103-sword-of-hardship|Sword of Hardship]]'
  slots_of_30: 0.4
- item: '[[items/weapon/204-orc-bow|Orc Bow]]'
  slots_of_30: 0.4
- item: '[[items/weapon/403-katar|Katar]]'
  slots_of_30: 0.4
- item: '[[items/subweapon/63-book-of-concentration|Book of Concentration]]'
  slots_of_30: 0.4
quests:
- '[[quests/859-hovert-s-mementos|Hovert''s Mementos]]'
spawns:
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5424.7
  y: 5346.1
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5328.3
  y: 5369.6
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5369.1
  y: 5324.3
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5281.4
  y: 5338.8
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5320.5
  y: 5320.7
  count: 1
  group: reinforcements
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5268
  y: 5316.7
  count: 1
  group: reinforcements
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5234.3
  y: 5312.8
  count: 1
  group: reinforcements
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5316.6
  y: 5329.4
  count: 1
  group: reinforcements
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5378.2
  y: 5301.8
  count: 1
  group: reinforcements
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5453.2
  y: 5294
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5453.2
  y: 5294
  count: 2
  group: reinforcements
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5275.2
  y: 5242.8
  count: 1
  group: reinforcements
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5297.7
  y: 5184
  count: 1
  group: reinforcements
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5461.4
  y: 5227.6
  count: 1
  group: reinforcements
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5481.3
  y: 5261.7
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5481.3
  y: 5261.7
  count: 2
  group: reinforcements
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5375.1
  y: 5092.9
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 173, ITEM_DROP.STB row 188
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Aqua Captain

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
