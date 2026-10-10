---
kind: monster
id: 501
name: Warbiz
status: in-game
level: 140
hp: 27
attack: 660
hit: 402
defence: 452
resistance: 284
avoid: 209
attack_speed: 100
attack_range: 2.3
damage: physical
walk_speed: 300
run_speed: 650
xp: 113
drop_item_rate: 30
drop_money_rate: 50
zones: 1
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
- item: '[[items/jewellery/251-socket-ring|Socket Ring]]'
  slots_of_30: 1
- item: '[[items/jewellery/8-glass-ring|Glass Ring]]'
  slots_of_30: 1
- item: '[[items/weapon/18-righteous-sword|Righteous Sword]]'
  slots_of_30: 0.2
- item: '[[items/weapon/116-giant-sword|Giant Sword]]'
  slots_of_30: 0.2
- item: '[[items/weapon/217-lynx-bow|Lynx Bow]]'
  slots_of_30: 0.2
- item: '[[items/weapon/246-bastard-gun|Bastard Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/316-ikaness-staff|Ikaness Staff]]'
  slots_of_30: 0.2
- item: '[[items/weapon/416-phantasma-knuckle|Phantasma Knuckle]]'
  slots_of_30: 0.4
- item: '[[items/weapon/47-labor-hammer|Labor Hammer]]'
  slots_of_30: 0.4
- item: '[[items/weapon/145-collapse-axe|Collapse Axe]]'
  slots_of_30: 0.4
- item: '[[items/weapon/274-ferrell-cannon|Ferrell Cannon]]'
  slots_of_30: 0.4
- item: '[[items/weapon/346-twister-wand|Twister Wand]]'
  slots_of_30: 0.4
spawns:
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5650.2
  y: 5280.6
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5135.2
  y: 5125.8
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5750.7
  y: 5278.3
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5730.4
  y: 5153
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5713.1
  y: 5120.6
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5684.2
  y: 5123.5
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5795.9
  y: 5131.9
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5815.1
  y: 5099.2
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5051
  y: 4856.4
  count: 2
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5110
  y: 4939.8
  count: 3
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5241.2
  y: 4852.4
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5371.2
  y: 4869.6
  count: 2
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5505.4
  y: 4871.7
  count: 2
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5569.1
  y: 4868.2
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5627.8
  y: 4923.8
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5827.9
  y: 4816.1
  count: 2
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5778.3
  y: 4831.6
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5014.4
  y: 4650.1
  count: 2
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5714.8
  y: 4648.9
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5847.7
  y: 4774.1
  count: 2
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5086
  y: 4557.8
  count: 1
  group: basic
- zone: '[[zones/61-refuge-xita|Refuge Xita]]'
  x: 5202.5
  y: 4630.8
  count: 3
  group: basic
source:
  data: LIST_NPC.STB row 501, ITEM_DROP.STB row 432
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Warbiz

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
