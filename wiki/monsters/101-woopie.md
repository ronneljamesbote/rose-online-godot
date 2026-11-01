---
kind: monster
id: 101
name: Woopie
status: changed-from-irose
level: 16
hp: 400
hp_per_level: 25
attack: 56
hit: 75
defence: 54
resistance: 31
avoid: 33
attack_speed: 90
attack_range: 2.5
damage: physical
walk_speed: 150
run_speed: 350
xp: 22
drop_item_rate: 60
drop_money_rate: 20
zones: 1
drops:
- item: '[[items/material/185-white-animal-tail-fur|White Animal Tail Fur]]'
  slots_of_30: 7
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/material/186-red-animal-tail-fur|Red Animal Tail Fur]]'
  slots_of_30: 7
- item: '[[items/consumable/161-mp-point-30|MP Point (+30)]]'
  slots_of_30: 1
- item: '[[items/consumable/21-mana-vial-s|Mana Vial (S)]]'
  slots_of_30: 1
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/414-woopy|Woopy]]'
  slots_of_30: 2
- item: '[[items/weapon/3-rapier|Rapier]]'
  slots_of_30: 0.4
- item: '[[items/weapon/32-monkey-wrench|Monkey Wrench]]'
  slots_of_30: 0.2
- item: '[[items/hands/202-action-gloves|Action Gloves]]'
  slots_of_30: 0.2
- item: '[[items/body/202-natalie-look|Natalie Look]]'
  slots_of_30: 0.2
- item: '[[items/feet/202-shoes-of-iguje|Shoes of Iguje]]'
  slots_of_30: 0.2
- item: '[[items/weapon/301-baobab-rod|Baobab Rod]]'
  slots_of_30: 0.4
- item: '[[items/head/6-red-beret|Red Beret]]'
  slots_of_30: 0.4
- item: '[[items/body/6-natalie-look|Natalie Look]]'
  slots_of_30: 0.4
- item: '[[items/hands/6-action-gloves|Action Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/6-shoes-of-iguje|Shoes of Iguje]]'
  slots_of_30: 0.4
- item: '[[items/weapon/101-haedong-sword|Haedong Sword]]'
  slots_of_30: 0.2
- item: '[[items/weapon/202-short-bow|Short Bow]]'
  slots_of_30: 0.2
- item: '[[items/weapon/231-bubble-gun|Bubble Gun]]'
  slots_of_30: 0.2
- item: '[[items/subweapon/61-book-of-standards|Book of Standards]]'
  slots_of_30: 0.2
spawns:
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5430.0
  y: 5090.0
  count: 3
  group: basic
source:
  data: godot/data/monsters/101-woopie.tres, drop table godot/data/drop_tables/147-woopie.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Woopie

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].

## Changed from iROSE

The Zant city hub redesign takes the Woopies out of [[zones/1-canyon-city-of-zant|Canyon City of Zant]]'s fields, apart from three that guard the [[monsters/205-woopie-king|Woopie King]] in Woopie Hollow (5430, 5090); they come back 30 seconds after they fall.
