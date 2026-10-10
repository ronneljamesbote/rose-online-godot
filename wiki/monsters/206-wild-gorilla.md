---
kind: monster
id: 206
name: Wild Gorilla
status: in-game
level: 43
hp: 1505
hp_per_level: 35
attack: 169
hit: 131
defence: 132
resistance: 62
avoid: 72
attack_speed: 90
attack_range: 2
damage: physical
walk_speed: 200
run_speed: 550
xp: 49
drop_item_rate: 60
drop_money_rate: 1
zones: 1
drops:
- item: '[[items/material/187-black-animal-tail-fur|Black Animal Tail Fur]]'
  slots_of_30: 5
- item: '[[items/material/188-animal-tail-fur|Animal Tail Fur]]'
  slots_of_30: 5
- item: '[[items/consumable/163-mp-point-100|MP Point (+100)]]'
  slots_of_30: 1
- item: '[[items/consumable/153-hp-point-200|HP Point (+200)]]'
  slots_of_30: 1
- item: '[[items/consumable/24-mana-bottle-s|Mana Bottle (S)]]'
  slots_of_30: 1
- item: '[[items/material/162-blue-crystal|Blue Crystal]]'
  slots_of_30: 3
- item: '[[items/consumable/307-mp-scroll-solo|MP Scroll (Solo)]]'
  slots_of_30: 1
- item: '[[items/weapon/304-mage-s-rod|Mage''s Rod]]'
  slots_of_30: 0.6
- item: '[[items/weapon/262-basic-launcher|Basic Launcher]]'
  slots_of_30: 0.4
- item: '[[items/weapon/164-scimitar|Scimitar]]'
  slots_of_30: 0.2
- item: '[[items/head/62-green-hat-of-witch|Green Hat of Witch]]'
  slots_of_30: 0.2
- item: '[[items/body/92-criker-chest|Criker Chest]]'
  slots_of_30: 0.2
- item: '[[items/head/32-iron-helm|Iron Helm]]'
  slots_of_30: 0.4
- item: '[[items/body/62-green-vest-of-witch|Green Vest of Witch]]'
  slots_of_30: 0.4
- item: '[[items/hands/92-criker-gloves|Criker Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/122-vibe-shoes|Vibe Shoes]]'
  slots_of_30: 0.4
- item: '[[items/weapon/432-long-sword-mace|Long Sword & Mace]]'
  slots_of_30: 0.2
- item: '[[items/weapon/334-elven-wand|Elven Wand]]'
  slots_of_30: 0.2
- item: '[[items/weapon/133-battle-axe|Battle Axe]]'
  slots_of_30: 0.2
- item: '[[items/subweapon/4-buckler|Buckler]]'
  slots_of_30: 0.2
- item: '[[items/weapon/531-ogre-mace|Ogre Mace]]'
  slots_of_30: 0.2
- item: '[[items/weapon/761-animal-rod|Animal Rod]]'
  slots_of_30: 0.2
- item: '[[items/weapon/501-long-sword|Long Sword]]'
  slots_of_30: 0.2
- item: '[[items/weapon/671-orc-bow|Orc Bow]]'
  slots_of_30: 0.2
- item: '[[items/weapon/701-gloria-gun|Gloria Gun]]'
  slots_of_30: 0.2
spawns:
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5156.5
  y: 5457.9
  count: 1
  group: reinforcements
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5547.8
  y: 5259.6
  count: 1
  group: reinforcements
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5158.6
  y: 4933.8
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 206, ITEM_DROP.STB row 201
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Wild Gorilla

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
