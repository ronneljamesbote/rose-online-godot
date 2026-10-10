---
kind: monster
id: 32
name: Ranger Dalping
status: in-game
level: 22
hp: 506
hp_per_level: 23
attack: 70
hit: 81
defence: 46
resistance: 50
avoid: 53
attack_speed: 90
attack_range: 18
damage: physical
walk_speed: 210
run_speed: 302
xp: 24
drop_item_rate: 62
drop_money_rate: 7
zones: 1
drops:
- item: '[[items/material/176-insect-leg|Insect Leg]]'
  slots_of_30: 6
- item: '[[items/material/175-insect-feeler|Insect Feeler]]'
  slots_of_30: 8
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/161-mp-point-30|MP Point (+30)]]'
  slots_of_30: 1
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/face/7-jolly-front-mask|Jolly Front Mask]]'
  slots_of_30: 0.2
- item: '[[items/weapon/231-bubble-gun|Bubble Gun]]'
  slots_of_30: 0.6
- item: '[[items/weapon/3-rapier|Rapier]]'
  slots_of_30: 0.2
- item: '[[items/weapon/401-knuckle|Knuckle]]'
  slots_of_30: 0.2
- item: '[[items/weapon/331-baobab-wand|Baobab Wand]]'
  slots_of_30: 0.8
- item: '[[items/body/31-soldier-armor|Soldier Armor]]'
  slots_of_30: 0.4
- item: '[[items/hands/61-spiritual-gloves|Spiritual Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/91-hunter-boots|Hunter Boots]]'
  slots_of_30: 0.4
- item: '[[items/head/121-brown-turban|Brown Turban]]'
  slots_of_30: 0.4
- item: '[[items/weapon/32-monkey-wrench|Monkey Wrench]]'
  slots_of_30: 0.2
- item: '[[items/subweapon/61-book-of-standards|Book of Standards]]'
  slots_of_30: 0.2
quests:
- '[[quests/5003-dalping-hunt-fetch-quest|Dalping Hunt (Fetch Quest)]]'
spawns:
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5258.9
  y: 5504.8
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5217.7
  y: 5487.5
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5433.3
  y: 5483.9
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5343.9
  y: 5483.6
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5293.8
  y: 5465.8
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5387
  y: 5494.3
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5374.8
  y: 5469.2
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5423.3
  y: 5440.8
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5480.3
  y: 5469.1
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5465.5
  y: 5440.1
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5465.5
  y: 5440.1
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5490.4
  y: 5493.3
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5490.4
  y: 5493.3
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5474.4
  y: 5415.6
  count: 2
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5474.4
  y: 5415.6
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5479.9
  y: 5384.7
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5495.1
  y: 5245.4
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5495.1
  y: 5245.4
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5443.2
  y: 5223.3
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 32, ITEM_DROP.STB row 113
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Ranger Dalping

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
