---
kind: monster
id: 153
name: Doonga Origin
status: in-game
level: 68
hp: 30
attack: 254
hit: 161
defence: 200
resistance: 138
avoid: 102
attack_speed: 95
attack_range: 2.2
damage: physical
walk_speed: 230
run_speed: 560
xp: 48
drop_item_rate: 45
drop_money_rate: 12
zones: 1
drops:
- item: '[[items/material/187-black-animal-tail-fur|Black Animal Tail Fur]]'
  slots_of_30: 9
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/material/188-animal-tail-fur|Animal Tail Fur]]'
  slots_of_30: 7
- item: '[[items/jewellery/271-socket-earring|Socket Earring]]'
  slots_of_30: 1
- item: '[[items/consumable/423-doonga-captain|Doonga Captain]]'
  slots_of_30: 1
- item: '[[items/material/189-predator-claw|Predator Claw]]'
  slots_of_30: 1
- item: '[[items/material/162-blue-crystal|Blue Crystal]]'
  slots_of_30: 1
- item: '[[items/jewellery/160-amulet-earring|Amulet Earring]]'
  slots_of_30: 1
- item: '[[items/weapon/135-orc-axe|Orc Axe]]'
  slots_of_30: 0.2
- item: '[[items/weapon/264-bronze-launcher|Bronze Launcher]]'
  slots_of_30: 0.2
- item: '[[items/weapon/167-trident|Trident]]'
  slots_of_30: 0.4
- item: '[[items/weapon/208-maiya-bow|Maiya Bow]]'
  slots_of_30: 0.2
- item: '[[items/weapon/9-squid-sword|Squid Sword]]'
  slots_of_30: 0.4
- item: '[[items/weapon/63-quick-bow-gun|Quick Bow Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/107-two-handed-sword|Two-Handed Sword]]'
  slots_of_30: 0.2
- item: '[[items/subweapon/6-hoplon|Hoplon]]'
  slots_of_30: 0.2
- item: '[[items/weapon/207-iron-bow|Iron Bow]]'
  slots_of_30: 0.4
- item: '[[items/feet/33-trunket-boots|Trunket Boots]]'
  slots_of_30: 0.4
- item: '[[items/head/63-purple-hat-of-witch|Purple Hat of Witch]]'
  slots_of_30: 0.4
- item: '[[items/body/93-ranger-chest|Ranger Chest]]'
  slots_of_30: 0.4
- item: '[[items/hands/123-tamiya-gloves|Tamiya Gloves]]'
  slots_of_30: 0.4
quests:
- '[[quests/134-the-scheme|The Scheme]]'
spawns:
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5102.8
  y: 5105.8
  count: 2
  group: basic
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5142.5
  y: 5104.9
  count: 1
  group: basic
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5142.5
  y: 5104.9
  count: 2
  group: basic
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5263.3
  y: 4947.2
  count: 1
  group: basic
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5263.3
  y: 4947.2
  count: 1
  group: basic
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5263.3
  y: 4947.2
  count: 2
  group: basic
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5292
  y: 4930
  count: 2
  group: basic
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5292
  y: 4930
  count: 2
  group: basic
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5306.5
  y: 4917.4
  count: 1
  group: basic
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5306.5
  y: 4917.4
  count: 1
  group: basic
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5420.4
  y: 4801.4
  count: 1
  group: basic
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5420.4
  y: 4801.4
  count: 1
  group: basic
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5472.9
  y: 4831.9
  count: 1
  group: basic
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5472.9
  y: 4831.9
  count: 1
  group: basic
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5390.2
  y: 4783.7
  count: 1
  group: basic
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5390.2
  y: 4783.7
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 153, ITEM_DROP.STB row 172
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Doonga Origin

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
