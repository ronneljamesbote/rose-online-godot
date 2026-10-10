---
kind: monster
id: 166
name: Master Stone Golem
status: in-game
level: 80
hp: 14240
hp_per_level: 178
attack: 380
hit: 237
defence: 307
resistance: 223
avoid: 99
attack_speed: 100
attack_range: 2.5
damage: physical
walk_speed: 220
run_speed: 490
xp: 458
drop_item_rate: 65
drop_money_rate: 1
zones: 1
drops:
- item: '[[items/material/201-steam-oil|Steam Oil]]'
  slots_of_30: 2
- item: '[[items/material/202-steam-heat|Steam Heat]]'
  slots_of_30: 1
- item: '[[items/material/203-steam-viper|Steam Viper]]'
  slots_of_30: 2
- item: '[[items/consumable/173-stamina-100|Stamina (+100)]]'
  slots_of_30: 1
- item: '[[items/consumable/32-spiritual-water-xl|Spiritual Water (XL)]]'
  slots_of_30: 1
- item: '[[items/material/204-mana-metal|Mana Metal]]'
  slots_of_30: 2
- item: '[[items/material/205-mana-oil|Mana Oil]]'
  slots_of_30: 4
- item: '[[items/material/206-mana-heat|Mana Heat]]'
  slots_of_30: 2
- item: '[[items/jewellery/251-socket-ring|Socket Ring]]'
  slots_of_30: 1
- item: '[[items/consumable/315-advanced-dexterity-scroll-solo|Advanced Dexterity Scroll (Solo)]]'
  slots_of_30: 1
- item: '[[items/head/34-chrome-helm|Chrome Helm]]'
  slots_of_30: 0.2
- item: '[[items/body/64-violet-dress|Violet Dress]]'
  slots_of_30: 0.2
- item: '[[items/hands/94-black-pirate-gloves|Black Pirate Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/124-semiya-shoes|Semiya Shoes]]'
  slots_of_30: 0.2
- item: '[[items/weapon/308-golden-staff|Golden Staff]]'
  slots_of_30: 0.2
- item: '[[items/weapon/408-crescent-knuckle|Crescent Knuckle]]'
  slots_of_30: 0.4
- item: '[[items/weapon/39-ice-hammer|Ice Hammer]]'
  slots_of_30: 0.4
- item: '[[items/weapon/137-cross-axe|Cross Axe]]'
  slots_of_30: 0.4
- item: '[[items/weapon/266-orc-launcher|Orc Launcher]]'
  slots_of_30: 0.4
- item: '[[items/weapon/338-thunder-wand|Thunder Wand]]'
  slots_of_30: 0.4
- item: '[[items/weapon/436-dual-ocean-swords|Dual Ocean Swords]]'
  slots_of_30: 0.2
- item: '[[items/weapon/169-halberd|Halberd]]'
  slots_of_30: 0.2
- item: '[[items/weapon/65-poison-bow-gun|Poison Bow Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/238-showdown-gun|Showdown Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/10-shark-blade|Shark Blade]]'
  slots_of_30: 0.2
- item: '[[items/weapon/536-ice-hammer|Ice Hammer]]'
  slots_of_30: 0.2
- item: '[[items/head/424-chrome-helm|Chrome Helm]]'
  slots_of_30: 0.2
- item: '[[items/body/424-violet-dress|Violet Dress]]'
  slots_of_30: 0.2
- item: '[[items/hands/624-semiya-gloves|Semiya Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/524-black-pirate-boots|Black Pirate Boots]]'
  slots_of_30: 0.2
quests:
- '[[quests/153-kay-s-request|Kay''s Request]]'
spawns:
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5066.3
  y: 5293.9
  count: 1
  group: reinforcements
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5091.1
  y: 5350.2
  count: 1
  group: reinforcements
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5277.8
  y: 5357.5
  count: 1
  group: reinforcements
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5102.8
  y: 5105.8
  count: 1
  group: reinforcements
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5083.4
  y: 5071.6
  count: 1
  group: basic
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5142.5
  y: 5104.9
  count: 1
  group: reinforcements
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5565.2
  y: 5041.4
  count: 1
  group: reinforcements
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5292
  y: 4930
  count: 1
  group: reinforcements
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5306.5
  y: 4917.4
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 166, ITEM_DROP.STB row 184
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Master Stone Golem

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
