---
kind: monster
id: 121
name: Krawfy
status: in-game
level: 64
hp: 1920
hp_per_level: 30
attack: 237
hit: 154
defence: 187
resistance: 128
avoid: 96
attack_speed: 95
attack_range: 2.3
damage: physical
walk_speed: 220
run_speed: 550
xp: 47
drop_item_rate: 55
drop_money_rate: 15
zones: 1
drops:
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 9
- item: '[[items/consumable/154-hp-point-300|HP Point (+300)]]'
  slots_of_30: 1
- item: '[[items/material/192-animal-backbone|Animal Backbone]]'
  slots_of_30: 6
- item: '[[items/consumable/164-mp-point-200|MP Point (+200)]]'
  slots_of_30: 1
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/jewellery/261-socket-necklace|Socket Necklace]]'
  slots_of_30: 1
- item: '[[items/consumable/416-krawfy|Krawfy]]'
  slots_of_30: 1
- item: '[[items/head/33-trunket-helm|Trunket Helm]]'
  slots_of_30: 0.6
- item: '[[items/body/63-purple-vest-of-witch|Purple Vest of Witch]]'
  slots_of_30: 0.6
- item: '[[items/weapon/434-twin-ghost-bat|Twin Ghost Bat]]'
  slots_of_30: 0.2
- item: '[[items/weapon/167-trident|Trident]]'
  slots_of_30: 0.2
- item: '[[items/weapon/264-bronze-launcher|Bronze Launcher]]'
  slots_of_30: 0.2
- item: '[[items/weapon/8-elven-sword|Elven Sword]]'
  slots_of_30: 0.2
- item: '[[items/weapon/207-iron-bow|Iron Bow]]'
  slots_of_30: 0.2
- item: '[[items/weapon/306-wisp-staff|Wisp Staff]]'
  slots_of_30: 0.2
- item: '[[items/weapon/37-stone-hammer|Stone Hammer]]'
  slots_of_30: 0.2
- item: '[[items/subweapon/7-kite-shield|Kite Shield]]'
  slots_of_30: 0.2
- item: '[[items/weapon/166-iron-spear|Iron Spear]]'
  slots_of_30: 0.4
- item: '[[items/hands/93-ranger-gloves|Ranger Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/123-tamiya-shoes|Tamiya Shoes]]'
  slots_of_30: 0.4
quests:
- '[[quests/3007-the-arumics-request|The Arumics'' Request]]'
spawns:
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
  x: 5690.4
  y: 5085.7
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5690.4
  y: 5085.7
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 121, ITEM_DROP.STB row 154
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Krawfy

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
