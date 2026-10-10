---
kind: monster
id: 521
name: Nepenthes
status: in-game
level: 147
hp: 5439
hp_per_level: 37
attack: 718
hit: 460
defence: 446
resistance: 417
avoid: 271
attack_speed: 100
attack_range: 2.2
damage: physical
walk_speed: 100
run_speed: 120
xp: 119
drop_item_rate: 35
drop_money_rate: 35
zones: 2
drops:
- item: '[[items/material/279-killer-leaf|Killer Leaf]]'
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
- item: '[[items/jewellery/100-lizard-necklace|Lizard Necklace]]'
  slots_of_30: 1
- item: '[[items/material/151-black-hearts|Black Hearts]]'
  slots_of_30: 1
- item: '[[items/weapon/218-bifenith-bow|Bifenith Bow]]'
  slots_of_30: 0.2
- item: '[[items/weapon/247-steam-shocker|Steam Shocker]]'
  slots_of_30: 0.2
- item: '[[items/weapon/317-oracle-staff|Oracle Staff]]'
  slots_of_30: 0.6
- item: '[[items/weapon/417-bloody-hook-knuckle|Bloody Hook Knuckle]]'
  slots_of_30: 0.2
- item: '[[items/weapon/48-nirvana-hammer|Nirvana Hammer]]'
  slots_of_30: 0.2
- item: '[[items/weapon/146-adamantium-axe|Adamantium Axe]]'
  slots_of_30: 0.4
- item: '[[items/weapon/71-mythril-bow-gun|Mythril Bow Gun]]'
  slots_of_30: 0.4
- item: '[[items/weapon/445-glorious-dual-wield|Glorious Dual Wield]]'
  slots_of_30: 0.4
- item: '[[items/weapon/178-nimrodpiece|Nimrodpiece]]'
  slots_of_30: 0.4
spawns:
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5094.8
  y: 5290.1
  count: 2
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5137.3
  y: 5292.7
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5731.8
  y: 5347
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5846.6
  y: 5280.7
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5702.5
  y: 5148.5
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5713.1
  y: 5120.6
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5655
  y: 5222.9
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6173
  y: 5223.1
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6206.1
  y: 5121.6
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6235.4
  y: 5232.8
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6200.4
  y: 5175
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6129.9
  y: 5226.5
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5198.9
  y: 5036.1
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6512
  y: 5019.6
  count: 2
  group: reinforcements
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5136.3
  y: 4953.9
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5186
  y: 4660
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 5534.3
  y: 4703.8
  count: 1
  group: basic
- zone: '[[zones/62-shady-jungle|Shady Jungle]]'
  x: 6495.7
  y: 4703.3
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 521, ITEM_DROP.STB row 445
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Nepenthes

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
