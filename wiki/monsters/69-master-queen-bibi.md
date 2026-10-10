---
kind: monster
id: 69
name: Master Queen Bibi
status: in-game
level: 33
hp: 1419
hp_per_level: 43
attack: 136
hit: 117
defence: 109
resistance: 68
avoid: 42
attack_speed: 90
attack_range: 15
damage: magic
walk_speed: 300
run_speed: 450
xp: 65
drop_item_rate: 67
drop_money_rate: 12
zones: 1
drops:
- item: '[[items/material/177-insect-wing|Insect Wing]]'
  slots_of_30: 6
- item: '[[items/material/181-pollen|Pollen]]'
  slots_of_30: 4
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/23-mana-vial-l|Mana Vial (L)]]'
  slots_of_30: 1
- item: '[[items/consumable/162-mp-point-50|MP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/material/196-bird-feather|Bird Feather]]'
  slots_of_30: 3
- item: '[[items/consumable/152-hp-point-100|HP Point (+100)]]'
  slots_of_30: 1
- item: '[[items/jewellery/271-socket-earring|Socket Earring]]'
  slots_of_30: 1
- item: '[[items/material/197-nymph-powder|Nymph Powder]]'
  slots_of_30: 3
- item: '[[items/body/92-criker-chest|Criker Chest]]'
  slots_of_30: 0.4
- item: '[[items/hands/122-vibe-gloves|Vibe Gloves]]'
  slots_of_30: 0.4
- item: '[[items/weapon/103-sword-of-hardship|Sword of Hardship]]'
  slots_of_30: 0.6
- item: '[[items/weapon/403-katar|Katar]]'
  slots_of_30: 0.4
- item: '[[items/weapon/303-animal-rod|Animal Rod]]'
  slots_of_30: 0.2
- item: '[[items/feet/32-iron-boots|Iron Boots]]'
  slots_of_30: 0.2
- item: '[[items/head/62-green-hat-of-witch|Green Hat of Witch]]'
  slots_of_30: 0.2
- item: '[[items/weapon/163-round-spear|Round Spear]]'
  slots_of_30: 0.4
- item: '[[items/weapon/5-long-sword|Long Sword]]'
  slots_of_30: 0.4
- item: '[[items/weapon/204-orc-bow|Orc Bow]]'
  slots_of_30: 0.4
- item: '[[items/back/222-nymph-wing|Nymph Wing]]'
  slots_of_30: 0.4
spawns:
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5480.2
  y: 5442
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5590.8
  y: 5481.7
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5542.3
  y: 5459.6
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5630.2
  y: 5460.2
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5193.7
  y: 5312.1
  count: 2
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5649.6
  y: 5416.4
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5650.1
  y: 5372.7
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5648.3
  y: 5319.4
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5665
  y: 5299.8
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5072.8
  y: 5184.9
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5136.1
  y: 5245.9
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5246.1
  y: 4983.5
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5434.3
  y: 5009.9
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5338.9
  y: 5075.1
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5336.3
  y: 4953.1
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 69, ITEM_DROP.STB row 130
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Master Queen Bibi

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
