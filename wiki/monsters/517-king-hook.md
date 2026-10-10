---
kind: monster
id: 517
name: King Hook
status: in-game
level: 155
hp: 5890
hp_per_level: 38
attack: 773
hit: 446
defence: 428
resistance: 569
avoid: 263
attack_speed: 125
attack_range: 2.4
damage: physical
walk_speed: 300
run_speed: 650
xp: 138
drop_item_rate: 47
drop_money_rate: 25
zones: 2
drops:
- item: '[[items/material/280-transperant-wing|Transperant Wing]]'
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
- item: '[[items/gem/323-sapphire-3|Sapphire 3]]'
  slots_of_30: 1
- item: '[[items/gem/333-topaz-3|Topaz 3]]'
  slots_of_30: 1
- item: '[[items/jewellery/20-lizard-ring|Lizard Ring]]'
  slots_of_30: 1
- item: '[[items/material/155-red-hearts|Red Hearts]]'
  slots_of_30: 1
- item: '[[items/weapon/446-demise-dual-weapon|Demise Dual Weapon]]'
  slots_of_30: 0.2
- item: '[[items/weapon/418-hawk-knuckle|Hawk Knuckle]]'
  slots_of_30: 0.2
- item: '[[items/weapon/318-twinkle-staff|Twinkle Staff]]'
  slots_of_30: 0.2
- item: '[[items/weapon/248-shok-impact-gun|Shok Impact Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/219-zephyr-long-bow|Zephyr Long Bow]]'
  slots_of_30: 0.2
spawns:
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5449.7
  y: 5288.8
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5821.3
  y: 4733.2
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5176.5
  y: 4623.1
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6163
  y: 5185.3
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6345.2
  y: 5176.3
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6296.2
  y: 5111.1
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6512
  y: 5019.6
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6438
  y: 5106.5
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6181.4
  y: 4908.4
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6250.4
  y: 4611.5
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 517, ITEM_DROP.STB row 442
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# King Hook

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
