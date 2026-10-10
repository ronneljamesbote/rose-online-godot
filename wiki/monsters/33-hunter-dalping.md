---
kind: monster
id: 33
name: Hunter Dalping
status: in-game
level: 24
hp: 20
attack: 91
hit: 105
defence: 65
resistance: 28
avoid: 48
attack_speed: 80
attack_range: 18
damage: physical
walk_speed: 220
run_speed: 314
xp: 25
drop_item_rate: 60
drop_money_rate: 9
zones: 1
drops:
- item: '[[items/material/176-insect-leg|Insect Leg]]'
  slots_of_30: 8
- item: '[[items/material/175-insect-feeler|Insect Feeler]]'
  slots_of_30: 9
- item: '[[items/consumable/21-mana-vial-s|Mana Vial (S)]]'
  slots_of_30: 1
- item: '[[items/weapon/332-mage-s-wand|Mage''s Wand]]'
  slots_of_30: 0.2
- item: '[[items/weapon/203-long-bow|Long Bow]]'
  slots_of_30: 0.2
- item: '[[items/weapon/331-baobab-wand|Baobab Wand]]'
  slots_of_30: 0.6
- item: '[[items/weapon/162-javelin|Javelin]]'
  slots_of_30: 0.8
- item: '[[items/head/8-rodeo-hat|Rodeo Hat]]'
  slots_of_30: 0.4
- item: '[[items/body/8-blue-denim-look|Blue Denim Look]]'
  slots_of_30: 0.4
- item: '[[items/hands/8-gray-gloves|Gray Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/8-field-walkers|Field Walkers]]'
  slots_of_30: 0.4
- item: '[[items/weapon/301-baobab-rod|Baobab Rod]]'
  slots_of_30: 0.2
- item: '[[items/weapon/101-haedong-sword|Haedong Sword]]'
  slots_of_30: 0.2
- item: '[[items/subweapon/61-book-of-standards|Book of Standards]]'
  slots_of_30: 0.2
spawns:
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5067
  y: 5461.4
  count: 1
  group: basic
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5258.9
  y: 5504.8
  count: 1
  group: reinforcements
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5217.7
  y: 5487.5
  count: 1
  group: reinforcements
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5124.3
  y: 5474.9
  count: 1
  group: reinforcements
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5480.3
  y: 5469.1
  count: 2
  group: reinforcements
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5116.9
  y: 5427
  count: 1
  group: reinforcements
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5147.9
  y: 5413
  count: 1
  group: reinforcements
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5205.9
  y: 5426.8
  count: 1
  group: reinforcements
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5182.9
  y: 5399.6
  count: 1
  group: reinforcements
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5231.4
  y: 5409.7
  count: 1
  group: reinforcements
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5479.9
  y: 5384.7
  count: 2
  group: reinforcements
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5495.1
  y: 5245.4
  count: 1
  group: reinforcements
- zone: '[[zones/23-breezy-hills|Breezy Hills]]'
  x: 5443.2
  y: 5223.3
  count: 2
  group: reinforcements
source:
  data: LIST_NPC.STB row 33, ITEM_DROP.STB row 114
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Hunter Dalping

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
