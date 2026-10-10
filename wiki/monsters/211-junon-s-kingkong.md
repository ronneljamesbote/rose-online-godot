---
kind: monster
id: 211
name: Junon's KingKong
status: in-game
level: 70
hp: 182
attack: 333
hit: 214
defence: 266
resistance: 191
avoid: 86
attack_speed: 90
attack_range: 2.5
damage: physical
walk_speed: 340
run_speed: 730
xp: 578
drop_item_rate: 80
drop_money_rate: 1
zones: 1
drops:
- item: '[[items/material/187-black-animal-tail-fur|Black Animal Tail Fur]]'
  slots_of_30: 5
- item: '[[items/material/162-blue-crystal|Blue Crystal]]'
  slots_of_30: 4
- item: '[[items/material/188-animal-tail-fur|Animal Tail Fur]]'
  slots_of_30: 5
- item: '[[items/consumable/165-mp-point-300|MP Point (+300)]]'
  slots_of_30: 1
- item: '[[items/consumable/173-stamina-100|Stamina (+100)]]'
  slots_of_30: 1
- item: '[[items/consumable/58-vital-jam-5|Vital Jam (+5)]]'
  slots_of_30: 2
- item: '[[items/weapon/38-skeleton-hammer|Skeleton Hammer]]'
  slots_of_30: 0.4
- item: '[[items/weapon/435-saber-elven-sword|Saber & Elven Sword]]'
  slots_of_30: 0.4
- item: '[[items/weapon/9-squid-sword|Squid Sword]]'
  slots_of_30: 0.4
- item: '[[items/weapon/136-tower-axe|Tower Axe]]'
  slots_of_30: 0.4
- item: '[[items/weapon/265-marble-launcher|Marble Launcher]]'
  slots_of_30: 0.4
- item: '[[items/head/64-violet-beret|Violet Beret]]'
  slots_of_30: 0.4
- item: '[[items/hands/124-semiya-gloves|Semiya Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/94-black-pirate-boots|Black Pirate Boots]]'
  slots_of_30: 0.4
- item: '[[items/body/34-chrome-armor|Chrome Armor]]'
  slots_of_30: 0.4
- item: '[[items/weapon/765-shadow-staff|Shadow Staff]]'
  slots_of_30: 1
- item: '[[items/weapon/854-saber-elven-sword|Saber & Elven Sword]]'
  slots_of_30: 0.8
- item: '[[items/weapon/504-squid-sword|Squid Sword]]'
  slots_of_30: 0.4
- item: '[[items/weapon/795-flame-wand|Flame Wand]]'
  slots_of_30: 0.4
- item: '[[items/weapon/824-spike-knuckle|Spike Knuckle]]'
  slots_of_30: 0.4
- item: '[[items/weapon/535-skeleton-hammer|Skeleton Hammer]]'
  slots_of_30: 0.6
- item: '[[items/weapon/583-two-handed-sword|Two-Handed Sword]]'
  slots_of_30: 0.6
- item: '[[items/head/624-black-pirate-hat|Black Pirate Hat]]'
  slots_of_30: 0.6
- item: '[[items/body/624-semiya-vest|Semiya Vest]]'
  slots_of_30: 0.6
- item: '[[items/material/221-cart-engine-schematic|Cart Engine Schematic]]'
  slots_of_30: 1
quests:
- '[[quests/1062-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]'
spawns:
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5058.4
  y: 5506.6
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5102.7
  y: 5345.5
  count: 1
  group: reinforcements
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5026.6
  y: 5281.9
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5044
  y: 5334.4
  count: 1
  group: reinforcements
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5279.8
  y: 5364.7
  count: 1
  group: reinforcements
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5144.6
  y: 5323.6
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5198.9
  y: 5381.2
  count: 1
  group: reinforcements
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5140.1
  y: 5340.9
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 211, ITEM_DROP.STB row 206
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Junon's KingKong

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
