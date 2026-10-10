---
kind: monster
id: 123
name: Krawfy Captain
status: in-game
level: 68
hp: 3196
hp_per_level: 47
attack: 292
hit: 183
defence: 220
resistance: 144
avoid: 83
attack_speed: 105
attack_range: 2.5
damage: physical
walk_speed: 240
run_speed: 530
xp: 82
drop_item_rate: 63
drop_money_rate: 20
zones: 1
drops:
- item: '[[items/material/192-animal-backbone|Animal Backbone]]'
  slots_of_30: 6
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 7
- item: '[[items/consumable/172-stamina-75|Stamina (+75)]]'
  slots_of_30: 1
- item: '[[items/material/178-sticky-liquid|Sticky Liquid]]'
  slots_of_30: 1
- item: '[[items/consumable/25-mana-bottle-m|Mana Bottle (M)]]'
  slots_of_30: 1
- item: '[[items/consumable/164-mp-point-200|MP Point (+200)]]'
  slots_of_30: 1
- item: '[[items/consumable/154-hp-point-300|HP Point (+300)]]'
  slots_of_30: 1
- item: '[[items/jewellery/261-socket-necklace|Socket Necklace]]'
  slots_of_30: 1
- item: '[[items/consumable/416-krawfy|Krawfy]]'
  slots_of_30: 1
- item: '[[items/material/189-predator-claw|Predator Claw]]'
  slots_of_30: 1
- item: '[[items/body/93-ranger-chest|Ranger Chest]]'
  slots_of_30: 0.2
- item: '[[items/weapon/236-sorden-gun|Sorden Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/135-orc-axe|Orc Axe]]'
  slots_of_30: 0.2
- item: '[[items/weapon/264-bronze-launcher|Bronze Launcher]]'
  slots_of_30: 0.2
- item: '[[items/weapon/9-squid-sword|Squid Sword]]'
  slots_of_30: 0.6
- item: '[[items/weapon/8-elven-sword|Elven Sword]]'
  slots_of_30: 0.2
- item: '[[items/hands/33-trunket-gloves|Trunket Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/63-purple-sandals-of-witch|Purple Sandals of Witch]]'
  slots_of_30: 0.2
- item: '[[items/head/93-ranger-hat|Ranger Hat]]'
  slots_of_30: 0.2
- item: '[[items/body/123-tamiya-vest|Tamiya Vest]]'
  slots_of_30: 0.2
- item: '[[items/weapon/107-two-handed-sword|Two-Handed Sword]]'
  slots_of_30: 0.4
- item: '[[items/weapon/208-maiya-bow|Maiya Bow]]'
  slots_of_30: 0.4
- item: '[[items/weapon/237-edge-gun|Edge Gun]]'
  slots_of_30: 0.4
- item: '[[items/subweapon/7-kite-shield|Kite Shield]]'
  slots_of_30: 0.4
- item: '[[items/weapon/64-crossbow-gun|Crossbow Gun]]'
  slots_of_30: 0.2
- item: '[[items/head/419-trunket-helm|Trunket Helm]]'
  slots_of_30: 0.2
- item: '[[items/body/419-purple-vest-of-witch|Purple Vest of Witch]]'
  slots_of_30: 0.2
- item: '[[items/hands/519-ranger-gloves|Ranger Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/619-tamiya-shoes|Tamiya Shoes]]'
  slots_of_30: 0.2
spawns:
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5546.1
  y: 5142.8
  count: 1
  group: reinforcements
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5546.1
  y: 5142.8
  count: 2
  group: reinforcements
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5535.1
  y: 5188.5
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5535.1
  y: 5188.5
  count: 2
  group: reinforcements
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5552
  y: 5122.9
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5475.9
  y: 5177.8
  count: 2
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5481.5
  y: 5117.4
  count: 2
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
  group: reinforcements
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5690.4
  y: 5085.7
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5690.4
  y: 5085.7
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 123, ITEM_DROP.STB row 156
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Krawfy Captain

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
