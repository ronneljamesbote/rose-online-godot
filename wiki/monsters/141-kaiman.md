---
kind: monster
id: 141
name: Kaiman
status: in-game
level: 59
hp: 29
attack: 217
hit: 145
defence: 171
resistance: 116
avoid: 89
attack_speed: 100
attack_range: 2.5
damage: physical
walk_speed: 220
run_speed: 540
xp: 45
drop_item_rate: 48
drop_money_rate: 16
zones: 1
drops:
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 7
- item: '[[items/consumable/165-mp-point-300|MP Point (+300)]]'
  slots_of_30: 1
- item: '[[items/material/192-animal-backbone|Animal Backbone]]'
  slots_of_30: 7
- item: '[[items/consumable/155-hp-point-500|HP Point (+500)]]'
  slots_of_30: 1
- item: '[[items/jewellery/271-socket-earring|Socket Earring]]'
  slots_of_30: 1
- item: '[[items/consumable/420-kaiman|Kaiman]]'
  slots_of_30: 1
- item: '[[items/jewellery/86-glass-necklace|Glass Necklace]]'
  slots_of_30: 1
- item: '[[items/hands/123-tamiya-gloves|Tamiya Gloves]]'
  slots_of_30: 0.2
- item: '[[items/weapon/434-twin-ghost-bat|Twin Ghost Bat]]'
  slots_of_30: 0.2
- item: '[[items/weapon/105-surado|Surado]]'
  slots_of_30: 0.2
- item: '[[items/weapon/206-elf-bow|Elf Bow]]'
  slots_of_30: 0.2
- item: '[[items/weapon/405-wolf-s-paw|Wolf''s Paw]]'
  slots_of_30: 0.2
- item: '[[items/weapon/134-tomahawk|Tomahawk]]'
  slots_of_30: 0.2
- item: '[[items/weapon/263-hard-launcher|Hard Launcher]]'
  slots_of_30: 0.2
- item: '[[items/weapon/335-silence-wand|Silence Wand]]'
  slots_of_30: 0.2
- item: '[[items/weapon/433-dual-bushido|Dual Bushido]]'
  slots_of_30: 0.2
- item: '[[items/subweapon/65-mythical-flute|Mythical Flute]]'
  slots_of_30: 0.2
- item: '[[items/weapon/7-saber|Saber]]'
  slots_of_30: 0.4
- item: '[[items/head/33-trunket-helm|Trunket Helm]]'
  slots_of_30: 0.4
- item: '[[items/body/63-purple-vest-of-witch|Purple Vest of Witch]]'
  slots_of_30: 0.4
- item: '[[items/hands/93-ranger-gloves|Ranger Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/123-tamiya-shoes|Tamiya Shoes]]'
  slots_of_30: 0.4
spawns:
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5268
  y: 5470.3
  count: 2
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5102.7
  y: 5345.5
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5147.5
  y: 5366.5
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5146.9
  y: 5305.7
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5279.8
  y: 5364.7
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5198.9
  y: 5381.2
  count: 2
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5276.4
  y: 5420.3
  count: 2
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5205.3
  y: 5270.5
  count: 2
  group: basic
source:
  data: LIST_NPC.STB row 141, ITEM_DROP.STB row 165
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Kaiman

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
