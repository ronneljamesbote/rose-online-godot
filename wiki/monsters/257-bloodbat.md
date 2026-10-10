---
kind: monster
id: 257
name: BloodBat
status: in-game
level: 50
hp: 1350
hp_per_level: 27
attack: 168
hit: 125
defence: 106
resistance: 111
avoid: 96
attack_speed: 100
attack_range: 2
damage: physical
walk_speed: 480
run_speed: 548
xp: 32
drop_item_rate: 52
drop_money_rate: 1
zones: 2
drops:
- item: '[[items/material/186-red-animal-tail-fur|Red Animal Tail Fur]]'
  slots_of_30: 8
- item: '[[items/material/188-animal-tail-fur|Animal Tail Fur]]'
  slots_of_30: 7
- item: '[[items/consumable/163-mp-point-100|MP Point (+100)]]'
  slots_of_30: 1
- item: '[[items/consumable/2-health-vial-m|Health Vial (M)]]'
  slots_of_30: 1
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/154-hp-point-300|HP Point (+300)]]'
  slots_of_30: 1
- item: '[[items/material/197-nymph-powder|Nymph Powder]]'
  slots_of_30: 2
- item: '[[items/jewellery/261-socket-necklace|Socket Necklace]]'
  slots_of_30: 1
- item: '[[items/jewellery/82-light-necklace|Light Necklace]]'
  slots_of_30: 1
- item: '[[items/head/33-trunket-helm|Trunket Helm]]'
  slots_of_30: 0.2
- item: '[[items/body/123-tamiya-vest|Tamiya Vest]]'
  slots_of_30: 0.2
- item: '[[items/weapon/165-bardiche|Bardiche]]'
  slots_of_30: 0.2
- item: '[[items/weapon/61-simple-bow-gun|Simple Bow Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/404-rake-hand|Rake Hand]]'
  slots_of_30: 0.2
- item: '[[items/weapon/35-onion-mace|Onion Mace]]'
  slots_of_30: 0.2
- item: '[[items/head/13-luxurious-venetian-hat|Luxurious Venetian Hat]]'
  slots_of_30: 0.2
- item: '[[items/hands/13-luxurious-venetian-gloves|Luxurious Venetian Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/13-luxurious-marble-shoes|Luxurious Marble Shoes]]'
  slots_of_30: 0.2
- item: '[[items/body/13-fancy-blue|Fancy Blue]]'
  slots_of_30: 0.2
- item: '[[items/weapon/7-saber|Saber]]'
  slots_of_30: 0.4
- item: '[[items/weapon/105-surado|Surado]]'
  slots_of_30: 0.4
- item: '[[items/weapon/206-elf-bow|Elf Bow]]'
  slots_of_30: 0.4
- item: '[[items/weapon/235-iron-shotgun|Iron Shotgun]]'
  slots_of_30: 0.4
- item: '[[items/subweapon/5-aspis|Aspis]]'
  slots_of_30: 0.4
spawns:
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5080.6
  y: 5281.5
  count: 1
  group: basic
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5424.4
  y: 5435
  count: 1
  group: basic
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5383
  y: 5152.8
  count: 2
  group: basic
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5378.2
  y: 5133.6
  count: 1
  group: basic
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5385.4
  y: 5170.7
  count: 2
  group: basic
- zone: '[[zones/32-goblin-cave-b2|Goblin Cave (B2)]]'
  x: 5376.5
  y: 5505
  count: 2
  group: basic
- zone: '[[zones/32-goblin-cave-b2|Goblin Cave (B2)]]'
  x: 5523.7
  y: 5411.5
  count: 1
  group: basic
- zone: '[[zones/32-goblin-cave-b2|Goblin Cave (B2)]]'
  x: 5251
  y: 5232.3
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 257, ITEM_DROP.STB row 222
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# BloodBat

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
