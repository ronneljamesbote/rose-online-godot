---
kind: monster
id: 353
name: Rider Captain
status: in-game
level: 108
hp: 10368
hp_per_level: 96
attack: 487
hit: 295
defence: 375
resistance: 249
avoid: 138
attack_speed: 120
attack_range: 2.5
damage: physical
walk_speed: 290
run_speed: 670
xp: 223
drop_item_rate: 69
drop_money_rate: 0
zones: 1
drops:
- item: '[[items/material/193-frozen-animal-fur|Frozen Animal Fur]]'
  slots_of_30: 7
- item: '[[items/material/262-yeti-charm|Yeti Charm]]'
  slots_of_30: 6
- item: '[[items/material/128-hersian|Hersian]]'
  slots_of_30: 1
- item: '[[items/material/130-lusian|Lusian]]'
  slots_of_30: 1
- item: '[[items/material/142-coin-of-eucar-tribe|Coin of Eucar Tribe]]'
  slots_of_30: 1
- item: '[[items/material/143-ocarina-pieces|Ocarina Pieces]]'
  slots_of_30: 1
- item: '[[items/material/144-ice-key-fragment|Ice Key Fragment]]'
  slots_of_30: 2
- item: '[[items/weapon/14-ice-sword|Ice Sword]]'
  slots_of_30: 0.2
- item: '[[items/weapon/440-ice-flare-swords|Ice & Flare Swords]]'
  slots_of_30: 0.2
- item: '[[items/weapon/113-dragon-sword|Dragon Sword]]'
  slots_of_30: 0.2
- item: '[[items/weapon/43-scorpion-club|Scorpion Club]]'
  slots_of_30: 0.2
- item: '[[items/weapon/68-anima-bow-gun|Anima Bow Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/112-flamberge|Flamberge]]'
  slots_of_30: 0.2
- item: '[[items/weapon/141-ice-axe|Ice Axe]]'
  slots_of_30: 0.2
- item: '[[items/weapon/173-gungnir|Gungnir]]'
  slots_of_30: 0.2
- item: '[[items/weapon/214-bow-of-wind-spirit|Bow of Wind Spirit]]'
  slots_of_30: 0.2
- item: '[[items/weapon/270-shadow-launcher|Shadow Launcher]]'
  slots_of_30: 0.2
- item: '[[items/hands/65-gloves-of-shadow|Gloves of Shadow]]'
  slots_of_30: 0.8
- item: '[[items/feet/95-mariner-s-boots|Mariner''s Boots]]'
  slots_of_30: 0.4
- item: '[[items/body/35-mighty-armor|Mighty Armor]]'
  slots_of_30: 0.4
- item: '[[items/head/125-merchant-hat|Merchant Hat]]'
  slots_of_30: 0.4
spawns:
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5168.8
  y: 5333
  count: 0
  group: reinforcements
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5630.1
  y: 5343
  count: 1
  group: reinforcements
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5630.1
  y: 5343
  count: 1
  group: reinforcements
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5250.2
  y: 5027.2
  count: 2
  group: reinforcements
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5205.8
  y: 4986.6
  count: 2
  group: reinforcements
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5310.6
  y: 5040.5
  count: 2
  group: reinforcements
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5236.9
  y: 4949.5
  count: 2
  group: reinforcements
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5263.1
  y: 4844.9
  count: 2
  group: reinforcements
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5201.5
  y: 4894
  count: 2
  group: reinforcements
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5214
  y: 4794.5
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5214
  y: 4794.5
  count: 2
  group: reinforcements
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5166.6
  y: 4797.5
  count: 2
  group: reinforcements
source:
  data: LIST_NPC.STB row 353, ITEM_DROP.STB row 369
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Rider Captain

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
