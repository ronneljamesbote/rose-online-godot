---
kind: monster
id: 256
name: Needle Bat
status: in-game
level: 46
hp: 27
attack: 165
hit: 122
defence: 132
resistance: 87
avoid: 71
attack_speed: 95
attack_range: 1.8
damage: physical
walk_speed: 450
run_speed: 515
xp: 31
drop_item_rate: 55
drop_money_rate: 1
zones: 1
drops:
- item: '[[items/material/188-animal-tail-fur|Animal Tail Fur]]'
  slots_of_30: 5
- item: '[[items/material/187-black-animal-tail-fur|Black Animal Tail Fur]]'
  slots_of_30: 8
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/164-mp-point-200|MP Point (+200)]]'
  slots_of_30: 1
- item: '[[items/consumable/1-health-vial-s|Health Vial (S)]]'
  slots_of_30: 1
- item: '[[items/consumable/153-hp-point-200|HP Point (+200)]]'
  slots_of_30: 1
- item: '[[items/jewellery/251-socket-ring|Socket Ring]]'
  slots_of_30: 1
- item: '[[items/jewellery/81-shining-necklace|Shining Necklace]]'
  slots_of_30: 1
- item: '[[items/weapon/261-wooden-launcher|Wooden Launcher]]'
  slots_of_30: 0.2
- item: '[[items/weapon/6-bushido|Bushido]]'
  slots_of_30: 0.2
- item: '[[items/weapon/133-battle-axe|Battle Axe]]'
  slots_of_30: 0.2
- item: '[[items/weapon/262-basic-launcher|Basic Launcher]]'
  slots_of_30: 0.2
- item: '[[items/weapon/334-elven-wand|Elven Wand]]'
  slots_of_30: 0.4
- item: '[[items/weapon/432-long-sword-mace|Long Sword & Mace]]'
  slots_of_30: 0.6
- item: '[[items/weapon/61-simple-bow-gun|Simple Bow Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/165-bardiche|Bardiche]]'
  slots_of_30: 0.2
- item: '[[items/subweapon/4-buckler|Buckler]]'
  slots_of_30: 0.2
- item: '[[items/head/13-luxurious-venetian-hat|Luxurious Venetian Hat]]'
  slots_of_30: 0.4
- item: '[[items/hands/13-luxurious-venetian-gloves|Luxurious Venetian Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/13-luxurious-marble-shoes|Luxurious Marble Shoes]]'
  slots_of_30: 0.4
- item: '[[items/body/13-fancy-blue|Fancy Blue]]'
  slots_of_30: 0.4
spawns:
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5061.9
  y: 5282.6
  count: 1
  group: basic
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5061.9
  y: 5282.6
  count: 1
  group: basic
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5080.6
  y: 5281.5
  count: 1
  group: basic
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5069.4
  y: 5307.6
  count: 1
  group: basic
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5378.2
  y: 5133.6
  count: 2
  group: basic
source:
  data: LIST_NPC.STB row 256, ITEM_DROP.STB row 221
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Needle Bat

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
