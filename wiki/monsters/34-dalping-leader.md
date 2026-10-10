---
kind: monster
id: 34
name: Dalping Leader
status: in-game
level: 26
hp: 34
attack: 100
hit: 102
defence: 86
resistance: 36
avoid: 49
attack_speed: 100
attack_range: 20
damage: physical
walk_speed: 240
run_speed: 338
xp: 49
drop_item_rate: 62
drop_money_rate: 12
zones: 1
drops:
- item: '[[items/material/175-insect-feeler|Insect Feeler]]'
  slots_of_30: 9
- item: '[[items/material/176-insect-leg|Insect Leg]]'
  slots_of_30: 7
- item: '[[items/consumable/21-mana-vial-s|Mana Vial (S)]]'
  slots_of_30: 1
- item: '[[items/consumable/405-dalping|Dalping]]'
  slots_of_30: 2
- item: '[[items/material/178-sticky-liquid|Sticky Liquid]]'
  slots_of_30: 1
- item: '[[items/weapon/163-round-spear|Round Spear]]'
  slots_of_30: 0.2
- item: '[[items/weapon/131-woodman-axe|Woodman Axe]]'
  slots_of_30: 0.2
- item: '[[items/weapon/102-nameless-sword|Nameless Sword]]'
  slots_of_30: 0.2
- item: '[[items/weapon/331-baobab-wand|Baobab Wand]]'
  slots_of_30: 0.2
- item: '[[items/weapon/232-air-gun|Air Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/4-khukuri|Khukuri]]'
  slots_of_30: 0.6
- item: '[[items/feet/31-soldier-boots|Soldier Boots]]'
  slots_of_30: 0.2
- item: '[[items/head/61-spiritual-beret|Spiritual Beret]]'
  slots_of_30: 0.2
- item: '[[items/body/91-hunter-chest|Hunter Chest]]'
  slots_of_30: 0.2
- item: '[[items/hands/121-brown-gloves|Brown Gloves]]'
  slots_of_30: 0.2
- item: '[[items/weapon/162-javelin|Javelin]]'
  slots_of_30: 0.4
- item: '[[items/weapon/302-lemmings-rod|Lemmings Rod]]'
  slots_of_30: 0.4
- item: '[[items/weapon/203-long-bow|Long Bow]]'
  slots_of_30: 0.4
- item: '[[items/subweapon/2-tarz|Tarz]]'
  slots_of_30: 0.4
spawns:
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5078.1
  y: 5456.1
  count: 1
  group: reinforcements
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5067
  y: 5461.4
  count: 1
  group: reinforcements
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5490.4
  y: 5493.3
  count: 1
  group: reinforcements
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5474.4
  y: 5415.6
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 34, ITEM_DROP.STB row 115
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Dalping Leader

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
