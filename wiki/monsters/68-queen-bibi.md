---
kind: monster
id: 68
name: Queen Bibi
status: in-game
level: 27
hp: 918
hp_per_level: 34
attack: 104
hit: 103
defence: 88
resistance: 38
avoid: 51
attack_speed: 95
attack_range: 9
damage: magic
walk_speed: 240
run_speed: 350
xp: 45
drop_item_rate: 62
drop_money_rate: 10
zones: 1
drops:
- item: '[[items/material/181-pollen|Pollen]]'
  slots_of_30: 10
- item: '[[items/material/177-insect-wing|Insect Wing]]'
  slots_of_30: 4
- item: '[[items/consumable/161-mp-point-30|MP Point (+30)]]'
  slots_of_30: 1
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/22-mana-vial-m|Mana Vial (M)]]'
  slots_of_30: 1
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/weapon/302-lemmings-rod|Lemmings Rod]]'
  slots_of_30: 0.4
- item: '[[items/head/62-green-hat-of-witch|Green Hat of Witch]]'
  slots_of_30: 0.2
- item: '[[items/weapon/203-long-bow|Long Bow]]'
  slots_of_30: 0.2
- item: '[[items/weapon/33-buffoon-mace|Buffoon Mace]]'
  slots_of_30: 0.6
- item: '[[items/weapon/5-long-sword|Long Sword]]'
  slots_of_30: 0.2
- item: '[[items/hands/31-soldier-gloves|Soldier Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/61-spiritual-shoes|Spiritual Shoes]]'
  slots_of_30: 0.2
- item: '[[items/head/91-hunter-hat|Hunter Hat]]'
  slots_of_30: 0.2
- item: '[[items/body/121-brown-vest|Brown Vest]]'
  slots_of_30: 0.2
- item: '[[items/weapon/402-sword-knuckle|Sword Knuckle]]'
  slots_of_30: 0.4
- item: '[[items/weapon/131-woodman-axe|Woodman Axe]]'
  slots_of_30: 0.4
- item: '[[items/weapon/332-mage-s-wand|Mage''s Wand]]'
  slots_of_30: 0.4
- item: '[[items/subweapon/62-oriental-drum|Oriental Drum]]'
  slots_of_30: 0.4
quests:
- '[[quests/857-living-as-a-true-soldier|Living as a True Soldier]]'
spawns:
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5590.8
  y: 5481.7
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5542.3
  y: 5459.6
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5630.2
  y: 5460.2
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5195.6
  y: 5436.6
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5178.9
  y: 5398.1
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5649.6
  y: 5416.4
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5650.1
  y: 5372.7
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5665
  y: 5299.8
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5613.6
  y: 5305.4
  count: 1
  group: basic
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
  x: 5045.6
  y: 5239.9
  count: 1
  group: reinforcements
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5136.1
  y: 5245.9
  count: 1
  group: basic
- zone: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
  x: 5283.7
  y: 5142.1
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
  data: LIST_NPC.STB row 68, ITEM_DROP.STB row 129
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Queen Bibi

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
