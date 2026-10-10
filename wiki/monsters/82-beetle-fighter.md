---
kind: monster
id: 82
name: Beetle Fighter
status: in-game
level: 22
hp: 594
hp_per_level: 27
attack: 80
hit: 78
defence: 57
resistance: 43
avoid: 40
attack_speed: 100
attack_range: 2
damage: physical
walk_speed: 145
run_speed: 350
xp: 25
drop_item_rate: 62
drop_money_rate: 19
zones: 1
drops:
- item: '[[items/material/176-insect-leg|Insect Leg]]'
  slots_of_30: 6
- item: '[[items/material/172-thick-insect-shell|Thick Insect Shell]]'
  slots_of_30: 7
- item: '[[items/consumable/161-mp-point-30|MP Point (+30)]]'
  slots_of_30: 1
- item: '[[items/material/178-sticky-liquid|Sticky Liquid]]'
  slots_of_30: 2
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/2-health-vial-m|Health Vial (M)]]'
  slots_of_30: 1
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/410-beetle|Beetle]]'
  slots_of_30: 2
- item: '[[items/weapon/402-sword-knuckle|Sword Knuckle]]'
  slots_of_30: 0.2
- item: '[[items/weapon/162-javelin|Javelin]]'
  slots_of_30: 0.6
- item: '[[items/weapon/301-baobab-rod|Baobab Rod]]'
  slots_of_30: 0.2
- item: '[[items/weapon/331-baobab-wand|Baobab Wand]]'
  slots_of_30: 0.6
- item: '[[items/weapon/202-short-bow|Short Bow]]'
  slots_of_30: 0.6
- item: '[[items/feet/31-soldier-boots|Soldier Boots]]'
  slots_of_30: 0.2
- item: '[[items/head/61-spiritual-beret|Spiritual Beret]]'
  slots_of_30: 0.2
- item: '[[items/body/91-hunter-chest|Hunter Chest]]'
  slots_of_30: 0.2
- item: '[[items/hands/121-brown-gloves|Brown Gloves]]'
  slots_of_30: 0.2
- item: '[[items/subweapon/62-oriental-drum|Oriental Drum]]'
  slots_of_30: 0.2
- item: '[[items/weapon/3-rapier|Rapier]]'
  slots_of_30: 0.4
- item: '[[items/weapon/101-haedong-sword|Haedong Sword]]'
  slots_of_30: 0.4
quests:
- '[[quests/855-living-as-a-true-soldier|Living as a True Soldier]]'
spawns:
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5495.1
  y: 5245.4
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5495.1
  y: 5245.4
  count: 2
  group: reinforcements
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5386.9
  y: 4966.3
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5386.9
  y: 4966.3
  count: 2
  group: reinforcements
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5386.9
  y: 4966.3
  count: 2
  group: reinforcements
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5495.8
  y: 5076.3
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5495.8
  y: 5076.3
  count: 2
  group: reinforcements
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5495.8
  y: 5076.3
  count: 2
  group: reinforcements
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5249
  y: 4903.9
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5249
  y: 4903.9
  count: 2
  group: reinforcements
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5249
  y: 4903.9
  count: 2
  group: reinforcements
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5422.5
  y: 4941.1
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5422.5
  y: 4941.1
  count: 2
  group: reinforcements
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5422.5
  y: 4941.1
  count: 2
  group: reinforcements
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5436.3
  y: 4871.8
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5436.3
  y: 4871.8
  count: 2
  group: reinforcements
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5436.3
  y: 4871.8
  count: 2
  group: reinforcements
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5481.3
  y: 4881.9
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5481.3
  y: 4881.9
  count: 2
  group: reinforcements
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5481.3
  y: 4881.9
  count: 2
  group: reinforcements
source:
  data: LIST_NPC.STB row 82, ITEM_DROP.STB row 137
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Beetle Fighter

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
