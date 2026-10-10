---
kind: monster
id: 287
name: Grandmaster Goblin
status: in-game
level: 105
hp: 21735
hp_per_level: 207
attack: 526
hit: 316
defence: 417
resistance: 309
avoid: 134
attack_speed: 115
attack_range: 2.5
damage: physical
walk_speed: 340
run_speed: 790
xp: 740
drop_item_rate: 88
drop_money_rate: 15
zones: 1
drops:
- item: '[[items/material/192-animal-backbone|Animal Backbone]]'
  slots_of_30: 4
- item: '[[items/consumable/173-stamina-100|Stamina (+100)]]'
  slots_of_30: 1
- item: '[[items/material/164-white-crystal|White Crystal]]'
  slots_of_30: 3
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 4
- item: '[[items/consumable/165-mp-point-300|MP Point (+300)]]'
  slots_of_30: 1
- item: '[[items/consumable/155-hp-point-500|HP Point (+500)]]'
  slots_of_30: 1
- item: '[[items/consumable/58-vital-jam-5|Vital Jam (+5)]]'
  slots_of_30: 2
- item: '[[items/gem/361-diamond-1|Diamond 1]]'
  slots_of_30: 1
- item: '[[items/weapon/169-halberd|Halberd]]'
  slots_of_30: 0.4
- item: '[[items/weapon/269-burning-launcher|Burning Launcher]]'
  slots_of_30: 0.4
- item: '[[items/weapon/341-windstorm-wand|Windstorm Wand]]'
  slots_of_30: 0.4
- item: '[[items/weapon/439-dual-katana|Dual Katana]]'
  slots_of_30: 0.4
- item: '[[items/weapon/172-chaos-spear|Chaos Spear]]'
  slots_of_30: 0.4
- item: '[[items/body/34-chrome-armor|Chrome Armor]]'
  slots_of_30: 0.4
- item: '[[items/hands/64-violet-gloves|Violet Gloves]]'
  slots_of_30: 0.4
- item: '[[items/head/94-black-pirate-hat|Black Pirate Hat]]'
  slots_of_30: 0.4
- item: '[[items/feet/124-semiya-shoes|Semiya Shoes]]'
  slots_of_30: 0.4
- item: '[[items/weapon/513-ice-sword|Ice Sword]]'
  slots_of_30: 0.4
- item: '[[items/material/151-black-hearts|Black Hearts]]'
  slots_of_30: 0.6
- item: '[[items/material/153-blue-hearts|Blue Hearts]]'
  slots_of_30: 0.6
- item: '[[items/material/155-red-hearts|Red Hearts]]'
  slots_of_30: 0.6
- item: '[[items/material/157-white-hearts|White Hearts]]'
  slots_of_30: 0.6
- item: '[[items/material/154-pink-hearts|Pink Hearts]]'
  slots_of_30: 0.6
- item: '[[items/weapon/709-justice-cannon|Justice Cannon]]'
  slots_of_30: 0.8
- item: '[[items/head/433-mighty-helm|Mighty Helm]]'
  slots_of_30: 0.8
- item: '[[items/body/334-mighty-armor|Mighty Armor]]'
  slots_of_30: 0.8
- item: '[[items/feet/434-boots-of-shadow|Boots of Shadow]]'
  slots_of_30: 0.8
- item: '[[items/hands/534-mariner-s-gloves|Mariner''s Gloves]]'
  slots_of_30: 0.8
quests:
- '[[quests/158-leaders-in-the-abyss|Leaders in the Abyss]]'
spawns:
- zone: '[[zones/33-goblin-cave-b3|Goblin Cave (B3)]]'
  x: 5512.1
  y: 4949.1
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 287, ITEM_DROP.STB row 241
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Grandmaster Goblin

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
