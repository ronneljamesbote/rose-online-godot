---
kind: monster
id: 163
name: Master Golem
status: in-game
level: 84
hp: 15204
hp_per_level: 181
attack: 383
hit: 260
defence: 266
resistance: 216
avoid: 133
attack_speed: 85
attack_range: 2.5
damage: physical
walk_speed: 320
run_speed: 690
xp: 462
drop_item_rate: 80
drop_money_rate: 1
zones: 1
drops:
- item: '[[items/consumable/3-health-vial-l|Health Vial (L)]]'
  slots_of_30: 1
- item: '[[items/material/201-steam-oil|Steam Oil]]'
  slots_of_30: 2
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/material/202-steam-heat|Steam Heat]]'
  slots_of_30: 1
- item: '[[items/material/203-steam-viper|Steam Viper]]'
  slots_of_30: 1
- item: '[[items/consumable/29-spiritual-water-s|Spiritual Water (S)]]'
  slots_of_30: 1
- item: '[[items/material/204-mana-metal|Mana Metal]]'
  slots_of_30: 2
- item: '[[items/material/205-mana-oil|Mana Oil]]'
  slots_of_30: 2
- item: '[[items/material/206-mana-heat|Mana Heat]]'
  slots_of_30: 1
- item: '[[items/material/198-little-angel-feather|Little Angel Feather]]'
  slots_of_30: 3
- item: '[[items/jewellery/261-socket-necklace|Socket Necklace]]'
  slots_of_30: 1
- item: '[[items/jewellery/271-socket-earring|Socket Earring]]'
  slots_of_30: 1
- item: '[[items/consumable/318-advanced-accuracy-scroll-solo|Advanced Accuracy Scroll (Solo)]]'
  slots_of_30: 1
- item: '[[items/weapon/169-halberd|Halberd]]'
  slots_of_30: 0.2
- item: '[[items/weapon/65-poison-bow-gun|Poison Bow Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/11-viking-sword|Viking Sword]]'
  slots_of_30: 0.2
- item: '[[items/weapon/109-executioner|Executioner]]'
  slots_of_30: 0.2
- item: '[[items/weapon/309-anima-staff|Anima Staff]]'
  slots_of_30: 0.2
- item: '[[items/body/34-chrome-armor|Chrome Armor]]'
  slots_of_30: 0.2
- item: '[[items/hands/64-violet-gloves|Violet Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/124-semiya-shoes|Semiya Shoes]]'
  slots_of_30: 0.2
- item: '[[items/head/94-black-pirate-hat|Black Pirate Hat]]'
  slots_of_30: 0.2
- item: '[[items/weapon/855-dual-ocean-swords|Dual Ocean Swords]]'
  slots_of_30: 0.2
- item: '[[items/weapon/733-orc-launcher|Orc Launcher]]'
  slots_of_30: 0.4
- item: '[[items/weapon/585-executioner|Executioner]]'
  slots_of_30: 0.4
- item: '[[items/weapon/826-crescent-knuckle|Crescent Knuckle]]'
  slots_of_30: 0.4
- item: '[[items/weapon/766-golden-staff|Golden Staff]]'
  slots_of_30: 0.4
- item: '[[items/back/247-buffalo-backshield|Buffalo Backshield]]'
  slots_of_30: 0.4
- item: '[[items/weapon/506-shark-blade|Shark Blade]]'
  slots_of_30: 0.2
- item: '[[items/feet/625-semiya-shoes|Semiya Shoes]]'
  slots_of_30: 0.2
- item: '[[items/body/325-chrome-armor|Chrome Armor]]'
  slots_of_30: 0.2
- item: '[[items/hands/525-black-pirate-gloves|Black Pirate Gloves]]'
  slots_of_30: 0.2
- item: '[[items/head/725-semiya-goggles|Semiya Goggles]]'
  slots_of_30: 0.2
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
  x: 5083.4
  y: 5071.6
  count: 1
  group: reinforcements
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5263.3
  y: 4947.2
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 163, ITEM_DROP.STB row 181
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Master Golem

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
