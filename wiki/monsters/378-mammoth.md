---
kind: monster
id: 378
name: Mammoth
status: in-game
level: 115
hp: 40135
hp_per_level: 349
attack: 565
hit: 351
defence: 458
resistance: 345
avoid: 195
attack_speed: 100
attack_range: 2.5
damage: physical
walk_speed: 260
run_speed: 630
xp: 1087
drop_item_rate: 78
drop_money_rate: 0
zones: 3
drops:
- item: '[[items/material/195-broken-horn|Broken Horn]]'
  slots_of_30: 5
- item: '[[items/material/267-huge-horn|Huge Horn]]'
  slots_of_30: 8
- item: '[[items/material/144-ice-key-fragment|Ice Key Fragment]]'
  slots_of_30: 3
- item: '[[items/material/143-ocarina-pieces|Ocarina Pieces]]'
  slots_of_30: 1
- item: '[[items/material/163-red-crystal|Red Crystal]]'
  slots_of_30: 2
- item: '[[items/material/154-pink-hearts|Pink Hearts]]'
  slots_of_30: 1
- item: '[[items/gem/301-garnet-1|Garnet 1]]'
  slots_of_30: 1
- item: '[[items/weapon/270-shadow-launcher|Shadow Launcher]]'
  slots_of_30: 0.2
- item: '[[items/weapon/243-piercing-gun|Piercing Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/173-gungnir|Gungnir]]'
  slots_of_30: 0.2
- item: '[[items/weapon/15-flare-sword|Flare Sword]]'
  slots_of_30: 0.2
- item: '[[items/weapon/43-scorpion-club|Scorpion Club]]'
  slots_of_30: 0.2
- item: '[[items/weapon/68-anima-bow-gun|Anima Bow Gun]]'
  slots_of_30: 0.4
- item: '[[items/weapon/313-rune-staff|Rune Staff]]'
  slots_of_30: 0.4
- item: '[[items/weapon/343-shadow-wand|Shadow Wand]]'
  slots_of_30: 0.4
- item: '[[items/weapon/413-sudden-raid-dagger|Sudden Raid Dagger]]'
  slots_of_30: 0.4
- item: '[[items/weapon/441-dual-flare-swords|Dual Flare Swords]]'
  slots_of_30: 0.4
- item: '[[items/head/96-jaguar-hat|Jaguar Hat]]'
  slots_of_30: 0.4
- item: '[[items/feet/66-velvet-boots-of-witch|Velvet Boots of Witch]]'
  slots_of_30: 0.4
- item: '[[items/body/126-rich-merchant-vest|Rich Merchant Vest]]'
  slots_of_30: 0.4
- item: '[[items/hands/36-kurash-gauntlets|Kurash Gauntlets]]'
  slots_of_30: 0.4
- item: '[[items/hands/66-velvet-gloves-of-witch|Velvet Gloves of Witch]]'
  slots_of_30: 0.4
spawns:
- zone: '[[zones/51-magic-city-of-the-eucar|Magic City of the Eucar]]'
  x: 5116.8
  y: 5224.4
  count: 1
  group: basic
- zone: '[[zones/51-magic-city-of-the-eucar|Magic City of the Eucar]]'
  x: 5383
  y: 5257.3
  count: 1
  group: reinforcements
- zone: '[[zones/51-magic-city-of-the-eucar|Magic City of the Eucar]]'
  x: 5532.5
  y: 5214.5
  count: 1
  group: reinforcements
- zone: '[[zones/51-magic-city-of-the-eucar|Magic City of the Eucar]]'
  x: 5599.2
  y: 4910.7
  count: 1
  group: basic
- zone: '[[zones/51-magic-city-of-the-eucar|Magic City of the Eucar]]'
  x: 5061.6
  y: 4759.1
  count: 1
  group: basic
- zone: '[[zones/51-magic-city-of-the-eucar|Magic City of the Eucar]]'
  x: 5386.8
  y: 4764.1
  count: 1
  group: basic
- zone: '[[zones/51-magic-city-of-the-eucar|Magic City of the Eucar]]'
  x: 5302.6
  y: 4765.5
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5101.2
  y: 5185.9
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5988.4
  y: 5248.6
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5186.8
  y: 5036
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5226.4
  y: 4877.4
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5245.6
  y: 4797.7
  count: 1
  group: basic
- zone: '[[zones/54-crystal-snowfields|Crystal Snowfields]]'
  x: 5108.6
  y: 4560.8
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 378, ITEM_DROP.STB row 378
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Mammoth

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
