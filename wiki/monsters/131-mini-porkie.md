---
kind: monster
id: 131
name: Mini-Porkie
status: changed-from-irose
level: 19
hp: 475
hp_per_level: 25
attack: 46
hit: 80
defence: 61
resistance: 35
avoid: 37
attack_speed: 105
attack_range: 1.8
damage: physical
walk_speed: 180
run_speed: 500
xp: 26
drop_item_rate: 62
drop_money_rate: 20
zones: 1
drops:
- item: '[[items/material/178-sticky-liquid|Sticky Liquid]]'
  slots_of_30: 3
- item: '[[items/material/187-black-animal-tail-fur|Black Animal Tail Fur]]'
  slots_of_30: 6
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 8
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/161-mp-point-30|MP Point (+30)]]'
  slots_of_30: 1
- item: '[[items/consumable/417-porkie|Porkie]]'
  slots_of_30: 1
- item: '[[items/head/7-red-leather-beret|Red Leather Beret]]'
  slots_of_30: 0.6
- item: '[[items/weapon/401-knuckle|Knuckle]]'
  slots_of_30: 0.2
- item: '[[items/weapon/202-short-bow|Short Bow]]'
  slots_of_30: 0.2
- item: '[[items/weapon/101-haedong-sword|Haedong Sword]]'
  slots_of_30: 0.2
- item: '[[items/weapon/3-rapier|Rapier]]'
  slots_of_30: 0.2
- item: '[[items/weapon/162-javelin|Javelin]]'
  slots_of_30: 0.6
- item: '[[items/body/7-yellow-denim-look|Yellow Denim Look]]'
  slots_of_30: 0.4
- item: '[[items/hands/7-full-action-gloves|Full Action Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/7-islamic-shoes|Islamic Shoes]]'
  slots_of_30: 0.4
- item: '[[items/weapon/331-baobab-wand|Baobab Wand]]'
  slots_of_30: 0.2
- item: '[[items/weapon/32-monkey-wrench|Monkey Wrench]]'
  slots_of_30: 0.2
- item: '[[items/weapon/301-baobab-rod|Baobab Rod]]'
  slots_of_30: 0.2
- item: '[[items/subweapon/1-wooden-shield|Wooden Shield]]'
  slots_of_30: 0.2
spawns:
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5064.3
  y: 5520.9
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5065.4
  y: 5481.6
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5038.1
  y: 5508.6
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5141.1
  y: 5528.3
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5156.5
  y: 5457.9
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5210.4
  y: 5493.1
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5334
  y: 5529.5
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5119.9
  y: 5299.9
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5062.6
  y: 5377.9
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5050.4
  y: 5359.5
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5212.4
  y: 5432.6
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5252.2
  y: 5435.9
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5159.3
  y: 5370.1
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5140.1
  y: 5331.9
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5252.3
  y: 5384
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5309.5
  y: 5399.2
  count: 1
  group: basic
source:
  data: godot/data/monsters/131-mini-porkie.tres, drop table godot/data/drop_tables/157-mini-porkie.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Mini-Porkie

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].

## Changed from iROSE

The Zant city hub redesign makes [[zones/1-canyon-city-of-zant|Canyon City of Zant]] a safe city: this monster no longer spawns in or around Zant.
