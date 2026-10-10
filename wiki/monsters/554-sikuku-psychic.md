---
kind: monster
id: 554
name: Sikuku Psychic
status: in-game
level: 160
hp: 4960
hp_per_level: 31
attack: 772
hit: 474
defence: 422
resistance: 783
avoid: 258
attack_speed: 100
attack_range: 16
damage: magic
walk_speed: 220
run_speed: 490
xp: 191
drop_item_rate: 64
drop_money_rate: 25
zones: 1
drops:
- item: '[[items/material/282-sikuku-costume|Sikuku Costume]]'
  slots_of_30: 14
- item: '[[items/consumable/10-vital-water-s|Vital Water (S)]]'
  slots_of_30: 1
- item: '[[items/consumable/29-spiritual-water-s|Spiritual Water (S)]]'
  slots_of_30: 1
- item: '[[items/material/161-green-crystal|Green Crystal]]'
  slots_of_30: 1
- item: '[[items/material/162-blue-crystal|Blue Crystal]]'
  slots_of_30: 1
- item: '[[items/material/163-red-crystal|Red Crystal]]'
  slots_of_30: 1
- item: '[[items/material/164-white-crystal|White Crystal]]'
  slots_of_30: 1
- item: '[[items/jewellery/261-socket-necklace|Socket Necklace]]'
  slots_of_30: 1
- item: '[[items/jewellery/104-dazzling-necklace|Dazzling Necklace]]'
  slots_of_30: 1
- item: '[[items/weapon/50-final-hammer|Final Hammer]]'
  slots_of_30: 0.2
- item: '[[items/weapon/277-reverse-canon|Reverse Canon]]'
  slots_of_30: 0.2
- item: '[[items/weapon/319-tearing-staff|Tearing Staff]]'
  slots_of_30: 0.2
- item: '[[items/weapon/419-raven-knuckle|Raven Knuckle]]'
  slots_of_30: 0.2
- item: '[[items/weapon/348-ouroboros-wand|Ouroboros Wand]]'
  slots_of_30: 0.2
spawns:
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5382.4
  y: 5130.7
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6019.8
  y: 5079.1
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5418.5
  y: 4909.6
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5561.8
  y: 4844
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5826.6
  y: 4838.3
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 554, ITEM_DROP.STB row 473
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Sikuku Psychic

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
