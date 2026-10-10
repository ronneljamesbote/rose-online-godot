---
kind: monster
id: 512
name: Pincer Queen
status: in-game
level: 153
hp: 5814
hp_per_level: 38
attack: 655
hit: 440
defence: 570
resistance: 430
avoid: 147
attack_speed: 100
attack_range: 2.4
damage: physical
walk_speed: 250
run_speed: 550
xp: 132
drop_item_rate: 60
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
- item: '[[items/gem/303-garnet-3|Garnet 3]]'
  slots_of_30: 1
- item: '[[items/gem/313-ruby-3|Ruby 3]]'
  slots_of_30: 1
- item: '[[items/jewellery/91-glamour-necklace|Glamour Necklace]]'
  slots_of_30: 1
- item: '[[items/material/155-red-hearts|Red Hearts]]'
  slots_of_30: 1
- item: '[[items/weapon/248-shok-impact-gun|Shok Impact Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/318-twinkle-staff|Twinkle Staff]]'
  slots_of_30: 0.2
- item: '[[items/weapon/446-demise-dual-weapon|Demise Dual Weapon]]'
  slots_of_30: 0.2
- item: '[[items/weapon/418-hawk-knuckle|Hawk Knuckle]]'
  slots_of_30: 0.2
- item: '[[items/weapon/147-titan-axe|Titan Axe]]'
  slots_of_30: 0.2
spawns:
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5543.6
  y: 5302.5
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5449.7
  y: 5288.8
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5032.6
  y: 4543.4
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6210.7
  y: 5209.5
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
  x: 5627.5
  y: 4696.1
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6106.7
  y: 4753.7
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 512, ITEM_DROP.STB row 438
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Pincer Queen

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
