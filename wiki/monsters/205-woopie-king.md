---
kind: monster
id: 205
name: Woopie King
status: in-game
level: 25
hp: 74
attack: 110
hit: 121
defence: 99
resistance: 56
avoid: 33
attack_speed: 85
attack_range: 2.5
damage: physical
walk_speed: 230
run_speed: 550
xp: 130
drop_item_rate: 70
drop_money_rate: 20
zones: 1
drops:
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 6
- item: '[[items/material/187-black-animal-tail-fur|Black Animal Tail Fur]]'
  slots_of_30: 8
- item: '[[items/consumable/23-mana-vial-l|Mana Vial (L)]]'
  slots_of_30: 1
- item: '[[items/material/161-green-crystal|Green Crystal]]'
  slots_of_30: 2
- item: '[[items/weapon/4-khukuri|Khukuri]]'
  slots_of_30: 0.6
- item: '[[items/weapon/203-long-bow|Long Bow]]'
  slots_of_30: 0.2
- item: '[[items/weapon/302-lemmings-rod|Lemmings Rod]]'
  slots_of_30: 0.2
- item: '[[items/weapon/33-buffoon-mace|Buffoon Mace]]'
  slots_of_30: 0.2
- item: '[[items/weapon/332-mage-s-wand|Mage''s Wand]]'
  slots_of_30: 0.2
- item: '[[items/weapon/402-sword-knuckle|Sword Knuckle]]'
  slots_of_30: 0.4
- item: '[[items/weapon/162-javelin|Javelin]]'
  slots_of_30: 0.4
- item: '[[items/weapon/232-air-gun|Air Gun]]'
  slots_of_30: 0.4
- item: '[[items/weapon/163-round-spear|Round Spear]]'
  slots_of_30: 0.6
- item: '[[items/head/31-soldier-helm|Soldier Helm]]'
  slots_of_30: 0.2
- item: '[[items/hands/121-brown-gloves|Brown Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/61-spiritual-shoes|Spiritual Shoes]]'
  slots_of_30: 0.2
- item: '[[items/hands/61-spiritual-gloves|Spiritual Gloves]]'
  slots_of_30: 0.2
- item: '[[items/head/406-soldier-helm|Soldier Helm]]'
  slots_of_30: 0.4
- item: '[[items/body/503-hunter-chest|Hunter Chest]]'
  slots_of_30: 0.4
- item: '[[items/hands/603-brown-gloves|Brown Gloves]]'
  slots_of_30: 0.4
- item: '[[items/body/210-brown-denim-look|Brown Denim Look]]'
  slots_of_30: 0.4
- item: '[[items/hands/210-hill-gray-gloves|Hill Gray Gloves]]'
  slots_of_30: 0.4
quests:
- '[[quests/113-hourglass-of-purification|Hourglass of Purification]]'
spawns:
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5441.3
  y: 5448.9
  count: 1
  group: reinforcements
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5332
  y: 5339.6
  count: 1
  group: reinforcements
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5382.9
  y: 5244.3
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 205, ITEM_DROP.STB row 200
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Woopie King

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
