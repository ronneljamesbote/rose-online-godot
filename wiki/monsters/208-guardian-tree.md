---
kind: monster
id: 208
name: Guardian Tree
status: in-game
level: 57
hp: 9804
hp_per_level: 172
attack: 272
hit: 185
defence: 216
resistance: 153
avoid: 69
attack_speed: 90
attack_range: 15
damage: magic
walk_speed: 240
run_speed: 360
xp: 424
drop_item_rate: 75
drop_money_rate: 1
zones: 1
drops:
- item: '[[items/material/176-insect-leg|Insect Leg]]'
  slots_of_30: 9
- item: '[[items/material/163-red-crystal|Red Crystal]]'
  slots_of_30: 5
- item: '[[items/consumable/26-mana-bottle-l|Mana Bottle (L)]]'
  slots_of_30: 1
- item: '[[items/consumable/153-hp-point-200|HP Point (+200)]]'
  slots_of_30: 1
- item: '[[items/consumable/58-vital-jam-5|Vital Jam (+5)]]'
  slots_of_30: 2
- item: '[[items/weapon/206-elf-bow|Elf Bow]]'
  slots_of_30: 0.4
- item: '[[items/weapon/35-onion-mace|Onion Mace]]'
  slots_of_30: 0.4
- item: '[[items/weapon/134-tomahawk|Tomahawk]]'
  slots_of_30: 0.4
- item: '[[items/weapon/263-hard-launcher|Hard Launcher]]'
  slots_of_30: 0.4
- item: '[[items/weapon/433-dual-bushido|Dual Bushido]]'
  slots_of_30: 0.4
- item: '[[items/head/33-trunket-helm|Trunket Helm]]'
  slots_of_30: 0.4
- item: '[[items/body/63-purple-vest-of-witch|Purple Vest of Witch]]'
  slots_of_30: 0.4
- item: '[[items/hands/93-ranger-gloves|Ranger Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/123-tamiya-shoes|Tamiya Shoes]]'
  slots_of_30: 0.4
- item: '[[items/weapon/335-silence-wand|Silence Wand]]'
  slots_of_30: 0.4
- item: '[[items/weapon/793-silence-wand|Silence Wand]]'
  slots_of_30: 0.4
- item: '[[items/weapon/672-white-wing-bow|White Wing Bow]]'
  slots_of_30: 0.4
- item: '[[items/weapon/581-cutter-edge|Cutter Edge]]'
  slots_of_30: 0.4
- item: '[[items/weapon/642-iron-spear|Iron Spear]]'
  slots_of_30: 0.4
- item: '[[items/back/223-butterfly-wing|Butterfly Wing]]'
  slots_of_30: 0.4
- item: '[[items/head/518-purple-hat-of-witch|Purple Hat of Witch]]'
  slots_of_30: 0.4
- item: '[[items/body/223-fancy-blue|Fancy Blue]]'
  slots_of_30: 0.4
- item: '[[items/hands/318-trunket-gloves|Trunket Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/317-trunket-boots|Trunket Boots]]'
  slots_of_30: 0.4
- item: '[[items/material/155-red-hearts|Red Hearts]]'
  slots_of_30: 0.4
quests:
- '[[quests/128-the-owl-eye|The Owl Eye]]'
spawns:
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5446.7
  y: 5177.5
  count: 1
  group: reinforcements
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5442.1
  y: 5161.7
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5442.1
  y: 5161.7
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5442.1
  y: 5161.7
  count: 1
  group: reinforcements
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5588
  y: 5014.1
  count: 1
  group: reinforcements
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5614.2
  y: 5002.6
  count: 1
  group: reinforcements
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5608.7
  y: 5009.2
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5608.7
  y: 5009.2
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5608.7
  y: 5009.2
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5641.8
  y: 4930.2
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 208, ITEM_DROP.STB row 203
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Guardian Tree

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
