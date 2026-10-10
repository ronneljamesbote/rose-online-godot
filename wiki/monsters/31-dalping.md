---
kind: monster
id: 31
name: Dalping
status: in-game
level: 21
hp: 525
hp_per_level: 25
attack: 73
hit: 83
defence: 65
resistance: 39
avoid: 39
attack_speed: 90
attack_range: 16
damage: physical
walk_speed: 200
run_speed: 290
xp: 26
drop_item_rate: 60
drop_money_rate: 7
zones: 1
drops:
- item: '[[items/material/176-insect-leg|Insect Leg]]'
  slots_of_30: 5
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/104-kiwi|Kiwi]]'
  slots_of_30: 1
- item: '[[items/material/175-insect-feeler|Insect Feeler]]'
  slots_of_30: 8
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/161-mp-point-30|MP Point (+30)]]'
  slots_of_30: 1
- item: '[[items/consumable/405-dalping|Dalping]]'
  slots_of_30: 2
- item: '[[items/weapon/131-woodman-axe|Woodman Axe]]'
  slots_of_30: 0.2
- item: '[[items/weapon/3-rapier|Rapier]]'
  slots_of_30: 0.2
- item: '[[items/weapon/231-bubble-gun|Bubble Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/32-monkey-wrench|Monkey Wrench]]'
  slots_of_30: 0.8
- item: '[[items/weapon/331-baobab-wand|Baobab Wand]]'
  slots_of_30: 0.2
- item: '[[items/head/31-soldier-helm|Soldier Helm]]'
  slots_of_30: 0.4
- item: '[[items/body/61-spiritual-vest|Spiritual Vest]]'
  slots_of_30: 0.4
- item: '[[items/hands/91-hunter-gloves|Hunter Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/121-brown-shoes|Brown Shoes]]'
  slots_of_30: 0.4
- item: '[[items/weapon/401-knuckle|Knuckle]]'
  slots_of_30: 0.2
- item: '[[items/weapon/202-short-bow|Short Bow]]'
  slots_of_30: 0.2
- item: '[[items/weapon/301-baobab-rod|Baobab Rod]]'
  slots_of_30: 0.2
- item: '[[items/subweapon/61-book-of-standards|Book of Standards]]'
  slots_of_30: 0.2
quests:
- '[[quests/1007-an-eye-for-the-marketplace|An Eye for the Marketplace]]'
spawns:
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5078.1
  y: 5456.1
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5480.3
  y: 5469.1
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5490.4
  y: 5493.3
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5474.4
  y: 5415.6
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5479.9
  y: 5384.7
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5443.2
  y: 5223.3
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5440.1
  y: 5035.2
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5440.1
  y: 5035.2
  count: 2
  group: reinforcements
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5477.7
  y: 4990.3
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5477.7
  y: 4990.3
  count: 2
  group: reinforcements
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5449.8
  y: 5011
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5449.8
  y: 5011
  count: 2
  group: reinforcements
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5491.1
  y: 4960.3
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5491.1
  y: 4960.3
  count: 2
  group: reinforcements
source:
  data: LIST_NPC.STB row 31, ITEM_DROP.STB row 112
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Dalping

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
