---
kind: monster
id: 77
name: Ruin Warden
status: in-game
level: 22
hp: 4224
hp_per_level: 192
attack: 70
hit: 96
defence: 101
resistance: 39
avoid: 15
attack_speed: 75
attack_range: 3
damage: physical
walk_speed: 160
run_speed: 420
xp: 216
drop_item_rate: 95
drop_money_rate: 90
zones: 1
drops:
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 2
- item: '[[items/consumable/161-mp-point-30|MP Point (+30)]]'
  slots_of_30: 2
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
- item: '[[items/material/175-insect-feeler|Insect Feeler]]'
  slots_of_30: 1
- item: '[[items/material/176-insect-leg|Insect Leg]]'
  slots_of_30: 1
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/material/178-sticky-liquid|Sticky Liquid]]'
  slots_of_30: 1
quests:
- '[[quests/2111-the-warden-below|The Warden Below]]'
spawns:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5218
  y: 5552
  count: 1
  group: basic
source:
  data: godot/data/monsters/77-ruin-warden.tres, drop table godot/data/drop_tables/488-ruin-warden.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Ruin Warden

The guardian of the sealed vault under the dig: a giant Pomic Soldier of carved stone. Only one stands in the vault; it is back 2 minutes after it falls. It carries the [[items/quest/707-lens-core|Lens Core]] for [[quests/2111-the-warden-below|The Warden Below]].

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].

## Changed from iROSE

A new elite monster of the Valley of Luxem Tower redesign. It uses the Pomic Soldier model and animations, scaled up, with its own stats and drop table.
