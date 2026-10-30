---
kind: monster
id: 76
name: Sandscar
status: in-game
level: 21
hp: 2268
hp_per_level: 108
attack: 98
hit: 85
defence: 63
resistance: 37
avoid: 39
attack_speed: 90
attack_range: 3.2
damage: physical
walk_speed: 160
run_speed: 380
xp: 144
drop_item_rate: 95
drop_money_rate: 90
zones: 1
drops:
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 2
- item: '[[items/consumable/161-mp-point-30|MP Point (+30)]]'
  slots_of_30: 1
- item: '[[items/material/175-insect-feeler|Insect Feeler]]'
  slots_of_30: 1
- item: '[[items/material/176-insect-leg|Insect Leg]]'
  slots_of_30: 1
- item: '[[items/head/121-brown-turban|Brown Turban]]'
  slots_of_30: 1
- item: '[[items/body/31-soldier-armor|Soldier Armor]]'
  slots_of_30: 1
- item: '[[items/weapon/202-short-bow|Short Bow]]'
  slots_of_30: 2
- item: '[[items/weapon/401-knuckle|Knuckle]]'
  slots_of_30: 1
- item: '[[items/weapon/231-bubble-gun|Bubble Gun]]'
  slots_of_30: 1
- item: '[[items/weapon/162-javelin|Javelin]]'
  slots_of_30: 2
- item: '[[items/head/31-soldier-helm|Soldier Helm]]'
  slots_of_30: 1
- item: '[[items/body/61-spiritual-vest|Spiritual Vest]]'
  slots_of_30: 1
- item: '[[items/hands/91-hunter-gloves|Hunter Gloves]]'
  slots_of_30: 1
- item: '[[items/feet/121-brown-shoes|Brown Shoes]]'
  slots_of_30: 1
- item: '[[items/weapon/101-haedong-sword|Haedong Sword]]'
  slots_of_30: 1
- item: '[[items/weapon/3-rapier|Rapier]]'
  slots_of_30: 2
- item: '[[items/weapon/32-monkey-wrench|Monkey Wrench]]'
  slots_of_30: 2
- item: '[[items/subweapon/62-oriental-drum|Oriental Drum]]'
  slots_of_30: 1
- item: '[[items/weapon/4-khukuri|Khukuri]]'
  slots_of_30: 1
- item: '[[items/weapon/102-nameless-sword|Nameless Sword]]'
  slots_of_30: 1
- item: '[[items/jewellery/82-light-necklace|Light Necklace]]'
  slots_of_30: 2
- item: '[[items/consumable/409-pomic-soldier|Pomic Soldier]]'
  slots_of_30: 1
- item: '[[items/material/178-sticky-liquid|Sticky Liquid]]'
  slots_of_30: 1
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
spawns:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5415
  y: 5555
  count: 1
  group: basic
source:
  data: godot/data/monsters/76-sandscar.tres, drop table godot/data/drop_tables/487-sandscar.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Sandscar

A Pomic Fighter grown huge on mana from the failing tower. It roams alone on the high ground north of the ruins, west of the tower mesa, and comes back about every 3 minutes after it falls.

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].

## Changed from iROSE

A new elite monster of the Valley of Luxem Tower redesign. It uses the Pomic Fighter model and animations, scaled up, with its own stats and drop table.
