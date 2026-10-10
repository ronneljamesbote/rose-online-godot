---
kind: monster
id: 143
name: Kaiman Warrior
status: in-game
level: 64
hp: 1920
hp_per_level: 30
attack: 264
hit: 175
defence: 201
resistance: 94
avoid: 66
attack_speed: 115
attack_range: 2.8
damage: physical
walk_speed: 220
run_speed: 480
xp: 44
drop_item_rate: 52
drop_money_rate: 20
zones: 1
drops:
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 7
- item: '[[items/material/192-animal-backbone|Animal Backbone]]'
  slots_of_30: 6
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/155-hp-point-500|HP Point (+500)]]'
  slots_of_30: 1
- item: '[[items/consumable/26-mana-bottle-l|Mana Bottle (L)]]'
  slots_of_30: 1
- item: '[[items/consumable/165-mp-point-300|MP Point (+300)]]'
  slots_of_30: 1
- item: '[[items/jewellery/251-socket-ring|Socket Ring]]'
  slots_of_30: 1
- item: '[[items/consumable/420-kaiman|Kaiman]]'
  slots_of_30: 1
- item: '[[items/material/189-predator-claw|Predator Claw]]'
  slots_of_30: 2
- item: '[[items/jewellery/88-pointed-necklace|Pointed Necklace]]'
  slots_of_30: 1
- item: '[[items/body/93-ranger-chest|Ranger Chest]]'
  slots_of_30: 0.2
- item: '[[items/weapon/37-stone-hammer|Stone Hammer]]'
  slots_of_30: 0.6
- item: '[[items/weapon/236-sorden-gun|Sorden Gun]]'
  slots_of_30: 0.4
- item: '[[items/weapon/135-orc-axe|Orc Axe]]'
  slots_of_30: 0.2
- item: '[[items/weapon/306-wisp-staff|Wisp Staff]]'
  slots_of_30: 0.2
- item: '[[items/weapon/406-bear-s-paw|Bear''s Paw]]'
  slots_of_30: 0.2
- item: '[[items/back/206-cutie-bag|Cutie Bag]]'
  slots_of_30: 0.2
- item: '[[items/weapon/106-bull-sword|Bull Sword]]'
  slots_of_30: 0.4
- item: '[[items/hands/33-trunket-gloves|Trunket Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/63-purple-sandals-of-witch|Purple Sandals of Witch]]'
  slots_of_30: 0.4
- item: '[[items/head/93-ranger-hat|Ranger Hat]]'
  slots_of_30: 0.4
- item: '[[items/body/123-tamiya-vest|Tamiya Vest]]'
  slots_of_30: 0.4
quests:
- '[[quests/131-eva-the-sorcerer|Eva the Sorcerer]]'
spawns:
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5268
  y: 5470.3
  count: 2
  group: reinforcements
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5375.1
  y: 5448.6
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5375.1
  y: 5448.6
  count: 1
  group: reinforcements
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5657.5
  y: 5532.6
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5102.7
  y: 5345.5
  count: 2
  group: reinforcements
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5147.5
  y: 5366.5
  count: 2
  group: reinforcements
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5146.9
  y: 5305.7
  count: 2
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5146.9
  y: 5305.7
  count: 4
  group: reinforcements
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5198.9
  y: 5381.2
  count: 2
  group: reinforcements
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5276.4
  y: 5420.3
  count: 2
  group: reinforcements
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5285.1
  y: 5294.3
  count: 3
  group: reinforcements
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5418.6
  y: 5387.2
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5323.3
  y: 5321.3
  count: 3
  group: reinforcements
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5459.4
  y: 5347.3
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5518.5
  y: 5322.3
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5530
  y: 5415.3
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5505.9
  y: 5433
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5205.3
  y: 5270.5
  count: 3
  group: reinforcements
source:
  data: LIST_NPC.STB row 143, ITEM_DROP.STB row 167
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Kaiman Warrior

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
