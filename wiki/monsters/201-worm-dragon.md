---
kind: monster
id: 201
name: Worm Dragon
status: in-game
level: 88
hp: 17160
hp_per_level: 195
attack: 419
hit: 256
defence: 341
resistance: 249
avoid: 110
attack_speed: 85
attack_range: 9
damage: magic
walk_speed: 300
run_speed: 450
xp: 516
drop_item_rate: 85
drop_money_rate: 1
zones: 1
drops:
- item: '[[items/consumable/173-stamina-100|Stamina (+100)]]'
  slots_of_30: 1
- item: '[[items/material/163-red-crystal|Red Crystal]]'
  slots_of_30: 3
- item: '[[items/material/200-dragon-scale|Dragon Scale]]'
  slots_of_30: 7
- item: '[[items/consumable/58-vital-jam-5|Vital Jam (+5)]]'
  slots_of_30: 2
- item: '[[items/material/197-nymph-powder|Nymph Powder]]'
  slots_of_30: 2
- item: '[[items/material/155-red-hearts|Red Hearts]]'
  slots_of_30: 1
- item: '[[items/consumable/310-defense-scroll-solo|Defense Scroll (Solo)]]'
  slots_of_30: 1
- item: '[[items/weapon/584-after-blade|After Blade]]'
  slots_of_30: 0.2
- item: '[[items/weapon/766-golden-staff|Golden Staff]]'
  slots_of_30: 0.2
- item: '[[items/weapon/506-shark-blade|Shark Blade]]'
  slots_of_30: 0.2
- item: '[[items/weapon/169-halberd|Halberd]]'
  slots_of_30: 0.2
- item: '[[items/weapon/826-crescent-knuckle|Crescent Knuckle]]'
  slots_of_30: 0.2
- item: '[[items/head/527-violet-beret|Violet Beret]]'
  slots_of_30: 0.4
- item: '[[items/feet/427-violet-boots|Violet Boots]]'
  slots_of_30: 0.4
- item: '[[items/hands/327-chrome-gauntlets|Chrome Gauntlets]]'
  slots_of_30: 0.4
- item: '[[items/body/627-semiya-vest|Semiya Vest]]'
  slots_of_30: 0.4
- item: '[[items/back/228-devil-wing|Devil Wing]]'
  slots_of_30: 0.4
- item: '[[items/feet/65-boots-of-shadow|Boots of Shadow]]'
  slots_of_30: 0.4
- item: '[[items/head/95-mariner-s-hat|Mariner''s Hat]]'
  slots_of_30: 0.4
- item: '[[items/body/125-merchant-vest|Merchant Vest]]'
  slots_of_30: 0.4
- item: '[[items/hands/35-mighty-gauntlets|Mighty Gauntlets]]'
  slots_of_30: 0.4
- item: '[[items/weapon/536-ice-hammer|Ice Hammer]]'
  slots_of_30: 0.4
- item: '[[items/weapon/734-mythril-launcher|Mythril Launcher]]'
  slots_of_30: 0.2
- item: '[[items/weapon/585-executioner|Executioner]]'
  slots_of_30: 0.2
- item: '[[items/weapon/538-morning-star|Morning Star]]'
  slots_of_30: 0.2
- item: '[[items/weapon/768-anima-staff|Anima Staff]]'
  slots_of_30: 0.2
- item: '[[items/weapon/507-viking-sword|Viking Sword]]'
  slots_of_30: 0.2
- item: '[[items/gem/341-emerald-1|Emerald 1]]'
  slots_of_30: 1
quests:
- '[[quests/156-monsters-in-the-desert|Monsters in the Desert]]'
spawns:
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5092.8
  y: 5389.7
  count: 1
  group: basic
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5102.8
  y: 5105.8
  count: 1
  group: reinforcements
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5083.4
  y: 5071.6
  count: 1
  group: reinforcements
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5113.8
  y: 5116.4
  count: 1
  group: basic
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5295.4
  y: 4947.9
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 201, ITEM_DROP.STB row 196
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Worm Dragon

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
