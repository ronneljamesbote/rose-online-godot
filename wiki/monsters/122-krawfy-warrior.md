---
kind: monster
id: 122
name: Krawfy Warrior
status: in-game
level: 65
hp: 30
attack: 268
hit: 177
defence: 205
resistance: 96
avoid: 67
attack_speed: 115
attack_range: 2.4
damage: physical
walk_speed: 200
run_speed: 450
xp: 43
drop_item_rate: 57
drop_money_rate: 17
zones: 1
drops:
- item: '[[items/material/192-animal-backbone|Animal Backbone]]'
  slots_of_30: 7
- item: '[[items/material/178-sticky-liquid|Sticky Liquid]]'
  slots_of_30: 1
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 8
- item: '[[items/consumable/154-hp-point-300|HP Point (+300)]]'
  slots_of_30: 1
- item: '[[items/consumable/5-health-bottle-m|Health Bottle (M)]]'
  slots_of_30: 1
- item: '[[items/consumable/165-mp-point-300|MP Point (+300)]]'
  slots_of_30: 1
- item: '[[items/jewellery/271-socket-earring|Socket Earring]]'
  slots_of_30: 1
- item: '[[items/face/11-ashura-mask|Ashura Mask]]'
  slots_of_30: 0.2
- item: '[[items/hands/123-tamiya-gloves|Tamiya Gloves]]'
  slots_of_30: 0.2
- item: '[[items/weapon/406-bear-s-paw|Bear''s Paw]]'
  slots_of_30: 0.4
- item: '[[items/weapon/37-stone-hammer|Stone Hammer]]'
  slots_of_30: 0.4
- item: '[[items/weapon/167-trident|Trident]]'
  slots_of_30: 0.2
- item: '[[items/weapon/106-bull-sword|Bull Sword]]'
  slots_of_30: 0.2
- item: '[[items/weapon/264-bronze-launcher|Bronze Launcher]]'
  slots_of_30: 0.2
- item: '[[items/subweapon/67-basic-encyclopedia|Basic Encyclopedia]]'
  slots_of_30: 0.2
- item: '[[items/weapon/62-cutter-bow-gun|Cutter Bow Gun]]'
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
- '[[quests/3407-righteous-crusaders-in-preparation|Righteous Crusaders in Preparation]]'
spawns:
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5546.1
  y: 5142.8
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5546.1
  y: 5142.8
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5535.1
  y: 5188.5
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5535.1
  y: 5188.5
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5552
  y: 5122.9
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5552
  y: 5122.9
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5475.9
  y: 5177.8
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5481.5
  y: 5117.4
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5638.3
  y: 5089
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5638.3
  y: 5089
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5638.3
  y: 5089
  count: 2
  group: reinforcements
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5690.4
  y: 5085.7
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5690.4
  y: 5085.7
  count: 2
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5690.4
  y: 5085.7
  count: 2
  group: reinforcements
source:
  data: LIST_NPC.STB row 122, ITEM_DROP.STB row 155
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Krawfy Warrior

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
