---
kind: monster
id: 258
name: Elder BloodBat
status: in-game
level: 60
hp: 32
attack: 231
hit: 137
defence: 150
resistance: 122
avoid: 90
attack_speed: 110
attack_range: 2
damage: physical
walk_speed: 500
run_speed: 570
xp: 38
drop_item_rate: 48
drop_money_rate: 1
zones: 1
drops:
- item: '[[items/material/186-red-animal-tail-fur|Red Animal Tail Fur]]'
  slots_of_30: 5
- item: '[[items/material/188-animal-tail-fur|Animal Tail Fur]]'
  slots_of_30: 7
- item: '[[items/consumable/154-hp-point-300|HP Point (+300)]]'
  slots_of_30: 1
- item: '[[items/consumable/172-stamina-75|Stamina (+75)]]'
  slots_of_30: 1
- item: '[[items/consumable/3-health-vial-l|Health Vial (L)]]'
  slots_of_30: 1
- item: '[[items/consumable/164-mp-point-200|MP Point (+200)]]'
  slots_of_30: 1
- item: '[[items/material/198-little-angel-feather|Little Angel Feather]]'
  slots_of_30: 2
- item: '[[items/material/199-little-devil-feather|Little Devil Feather]]'
  slots_of_30: 2
- item: '[[items/jewellery/271-socket-earring|Socket Earring]]'
  slots_of_30: 1
- item: '[[items/jewellery/83-textual-necklace|Textual Necklace]]'
  slots_of_30: 1
- item: '[[items/body/63-purple-vest-of-witch|Purple Vest of Witch]]'
  slots_of_30: 0.4
- item: '[[items/head/93-ranger-hat|Ranger Hat]]'
  slots_of_30: 0.2
- item: '[[items/weapon/62-cutter-bow-gun|Cutter Bow Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/335-silence-wand|Silence Wand]]'
  slots_of_30: 0.2
- item: '[[items/weapon/134-tomahawk|Tomahawk]]'
  slots_of_30: 0.2
- item: '[[items/weapon/405-wolf-s-paw|Wolf''s Paw]]'
  slots_of_30: 0.2
- item: '[[items/head/33-trunket-helm|Trunket Helm]]'
  slots_of_30: 0.2
- item: '[[items/hands/93-ranger-gloves|Ranger Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/123-tamiya-shoes|Tamiya Shoes]]'
  slots_of_30: 0.2
- item: '[[items/weapon/8-elven-sword|Elven Sword]]'
  slots_of_30: 0.4
- item: '[[items/weapon/207-iron-bow|Iron Bow]]'
  slots_of_30: 0.4
- item: '[[items/weapon/37-stone-hammer|Stone Hammer]]'
  slots_of_30: 0.4
- item: '[[items/weapon/135-orc-axe|Orc Axe]]'
  slots_of_30: 0.4
- item: '[[items/back/223-butterfly-wing|Butterfly Wing]]'
  slots_of_30: 0.4
spawns:
- zone: '[[zones/32-goblin-cave-b2|Goblin Cave (B2)]]'
  x: 5109.6
  y: 5499.5
  count: 2
  group: basic
- zone: '[[zones/32-goblin-cave-b2|Goblin Cave (B2)]]'
  x: 5406.2
  y: 5512.5
  count: 1
  group: basic
- zone: '[[zones/32-goblin-cave-b2|Goblin Cave (B2)]]'
  x: 5405.8
  y: 5479.5
  count: 1
  group: basic
- zone: '[[zones/32-goblin-cave-b2|Goblin Cave (B2)]]'
  x: 5395.6
  y: 5533.7
  count: 1
  group: basic
- zone: '[[zones/32-goblin-cave-b2|Goblin Cave (B2)]]'
  x: 5117
  y: 5135.6
  count: 2
  group: basic
- zone: '[[zones/32-goblin-cave-b2|Goblin Cave (B2)]]'
  x: 5091.4
  y: 5133.8
  count: 2
  group: basic
- zone: '[[zones/32-goblin-cave-b2|Goblin Cave (B2)]]'
  x: 5121.5
  y: 5159.4
  count: 1
  group: basic
- zone: '[[zones/32-goblin-cave-b2|Goblin Cave (B2)]]'
  x: 5155.2
  y: 5152.9
  count: 1
  group: basic
- zone: '[[zones/32-goblin-cave-b2|Goblin Cave (B2)]]'
  x: 5155.2
  y: 5152.9
  count: 4
  group: reinforcements
- zone: '[[zones/32-goblin-cave-b2|Goblin Cave (B2)]]'
  x: 5434.8
  y: 5228.8
  count: 1
  group: basic
- zone: '[[zones/32-goblin-cave-b2|Goblin Cave (B2)]]'
  x: 5142.8
  y: 5118.9
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 258, ITEM_DROP.STB row 223
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Elder BloodBat

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
