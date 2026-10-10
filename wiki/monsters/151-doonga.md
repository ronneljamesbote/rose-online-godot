---
kind: monster
id: 151
name: Doonga
status: in-game
level: 56
hp: 1960
hp_per_level: 35
attack: 186
hit: 151
defence: 209
resistance: 123
avoid: 45
attack_speed: 90
attack_range: 1.9
damage: physical
walk_speed: 200
run_speed: 550
xp: 41
drop_item_rate: 50
drop_money_rate: 10
zones: 1
drops:
- item: '[[items/material/188-animal-tail-fur|Animal Tail Fur]]'
  slots_of_30: 8
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/material/187-black-animal-tail-fur|Black Animal Tail Fur]]'
  slots_of_30: 6
- item: '[[items/consumable/164-mp-point-200|MP Point (+200)]]'
  slots_of_30: 1
- item: '[[items/consumable/155-hp-point-500|HP Point (+500)]]'
  slots_of_30: 1
- item: '[[items/jewellery/271-socket-earring|Socket Earring]]'
  slots_of_30: 1
- item: '[[items/consumable/301-purify-scroll-solo|Purify Scroll (Solo)]]'
  slots_of_30: 1
- item: '[[items/face/8-iron-mask|Iron Mask]]'
  slots_of_30: 0.2
- item: '[[items/body/63-purple-vest-of-witch|Purple Vest of Witch]]'
  slots_of_30: 0.2
- item: '[[items/weapon/105-surado|Surado]]'
  slots_of_30: 0.2
- item: '[[items/weapon/206-elf-bow|Elf Bow]]'
  slots_of_30: 0.2
- item: '[[items/weapon/235-iron-shotgun|Iron Shotgun]]'
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
- item: '[[items/weapon/8-elven-sword|Elven Sword]]'
  slots_of_30: 0.4
- item: '[[items/body/33-trunket-armor|Trunket Armor]]'
  slots_of_30: 0.4
- item: '[[items/hands/63-purple-gloves-of-witch|Purple Gloves of Witch]]'
  slots_of_30: 0.4
- item: '[[items/feet/93-ranger-boots|Ranger Boots]]'
  slots_of_30: 0.4
- item: '[[items/head/123-tamiya-goggles|Tamiya Goggles]]'
  slots_of_30: 0.4
quests:
- '[[quests/3006-in-readiness-for-war|In Readiness for War]]'
spawns:
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5268
  y: 5470.3
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5102.7
  y: 5345.5
  count: 2
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5044
  y: 5334.4
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5044
  y: 5334.4
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5147.5
  y: 5366.5
  count: 2
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5279.8
  y: 5364.7
  count: 2
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5198.9
  y: 5381.2
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5276.4
  y: 5420.3
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5285.1
  y: 5294.3
  count: 2
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5323.3
  y: 5321.3
  count: 2
  group: basic
source:
  data: LIST_NPC.STB row 151, ITEM_DROP.STB row 170
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Doonga

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
