---
kind: monster
id: 207
name: Aqua King
status: in-game
level: 40
hp: 160
attack: 197
hit: 150
defence: 155
resistance: 106
avoid: 49
attack_speed: 105
attack_range: 3.5
damage: physical
walk_speed: 290
run_speed: 630
xp: 527
drop_item_rate: 80
drop_money_rate: 15
zones: 2
drops:
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 6
- item: '[[items/material/192-animal-backbone|Animal Backbone]]'
  slots_of_30: 5
- item: '[[items/consumable/173-stamina-100|Stamina (+100)]]'
  slots_of_30: 1
- item: '[[items/material/162-blue-crystal|Blue Crystal]]'
  slots_of_30: 3
- item: '[[items/consumable/25-mana-bottle-m|Mana Bottle (M)]]'
  slots_of_30: 1
- item: '[[items/consumable/58-vital-jam-5|Vital Jam (+5)]]'
  slots_of_30: 2
- item: '[[items/consumable/308-dexterity-scroll-solo|Dexterity Scroll (Solo)]]'
  slots_of_30: 1
- item: '[[items/weapon/103-sword-of-hardship|Sword of Hardship]]'
  slots_of_30: 0.4
- item: '[[items/weapon/132-small-axe|Small Axe]]'
  slots_of_30: 0.4
- item: '[[items/weapon/164-scimitar|Scimitar]]'
  slots_of_30: 1.2
- item: '[[items/weapon/431-khukuri-long-sword|Khukuri & Long Sword]]'
  slots_of_30: 0.4
- item: '[[items/head/92-criker-hat|Criker Hat]]'
  slots_of_30: 0.4
- item: '[[items/body/122-vibe-vest|Vibe Vest]]'
  slots_of_30: 0.4
- item: '[[items/hands/32-iron-gauntlets|Iron Gauntlets]]'
  slots_of_30: 0.4
- item: '[[items/feet/62-green-sandals-of-witch|Green Sandals of Witch]]'
  slots_of_30: 0.4
- item: '[[items/weapon/104-cutter-edge|Cutter Edge]]'
  slots_of_30: 0.4
- item: '[[items/weapon/205-white-wing-bow|White Wing Bow]]'
  slots_of_30: 0.4
- item: '[[items/weapon/6-bushido|Bushido]]'
  slots_of_30: 0.4
- item: '[[items/weapon/234-iron-rifle|Iron Rifle]]'
  slots_of_30: 0.4
- item: '[[items/weapon/304-mage-s-rod|Mage''s Rod]]'
  slots_of_30: 0.4
- item: '[[items/weapon/641-bardiche|Bardiche]]'
  slots_of_30: 0.4
- item: '[[items/head/411-iron-helm|Iron Helm]]'
  slots_of_30: 0.4
- item: '[[items/body/611-vibe-vest|Vibe Vest]]'
  slots_of_30: 0.4
- item: '[[items/feet/612-vibe-shoes|Vibe Shoes]]'
  slots_of_30: 0.4
- item: '[[items/hands/312-iron-gauntlets|Iron Gauntlets]]'
  slots_of_30: 0.4
spawns:
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5369.1
  y: 5324.3
  count: 1
  group: reinforcements
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5268
  y: 5316.7
  count: 1
  group: reinforcements
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5453.2
  y: 5294
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
  group: reinforcements
source:
  data: LIST_NPC.STB row 207, ITEM_DROP.STB row 202
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Aqua King

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
