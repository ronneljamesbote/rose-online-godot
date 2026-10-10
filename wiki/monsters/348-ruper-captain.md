---
kind: monster
id: 348
name: Ruper Captain
status: in-game
level: 106
hp: 7208
hp_per_level: 68
attack: 477
hit: 289
defence: 399
resistance: 244
avoid: 179
attack_speed: 110
attack_range: 16
damage: magic
walk_speed: 215
run_speed: 540
xp: 158
drop_item_rate: 68
drop_money_rate: 0
zones: 1
drops:
- item: '[[items/material/263-ruper-crest|Ruper Crest]]'
  slots_of_30: 7
- item: '[[items/material/193-frozen-animal-fur|Frozen Animal Fur]]'
  slots_of_30: 6
- item: '[[items/material/123-earth-stone|Earth Stone]]'
  slots_of_30: 1
- item: '[[items/material/124-fire-stone|Fire Stone]]'
  slots_of_30: 1
- item: '[[items/material/142-coin-of-eucar-tribe|Coin of Eucar Tribe]]'
  slots_of_30: 3
- item: '[[items/material/143-ocarina-pieces|Ocarina Pieces]]'
  slots_of_30: 1
- item: '[[items/weapon/140-great-axe|Great Axe]]'
  slots_of_30: 0.2
- item: '[[items/weapon/171-shining-spear|Shining Spear]]'
  slots_of_30: 0.2
- item: '[[items/weapon/268-cross-launcher|Cross Launcher]]'
  slots_of_30: 0.2
- item: '[[items/weapon/13-katana|Katana]]'
  slots_of_30: 0.2
- item: '[[items/weapon/269-burning-launcher|Burning Launcher]]'
  slots_of_30: 0.2
- item: '[[items/weapon/241-justice-cannon|Justice Cannon]]'
  slots_of_30: 0.4
- item: '[[items/weapon/173-gungnir|Gungnir]]'
  slots_of_30: 0.4
- item: '[[items/weapon/112-flamberge|Flamberge]]'
  slots_of_30: 0.4
- item: '[[items/weapon/68-anima-bow-gun|Anima Bow Gun]]'
  slots_of_30: 0.4
- item: '[[items/weapon/43-scorpion-club|Scorpion Club]]'
  slots_of_30: 0.4
- item: '[[items/feet/65-boots-of-shadow|Boots of Shadow]]'
  slots_of_30: 0.4
- item: '[[items/body/125-merchant-vest|Merchant Vest]]'
  slots_of_30: 0.4
- item: '[[items/head/95-mariner-s-hat|Mariner''s Hat]]'
  slots_of_30: 0.4
- item: '[[items/hands/35-mighty-gauntlets|Mighty Gauntlets]]'
  slots_of_30: 0.4
- item: '[[items/hands/125-merchant-gloves|Merchant Gloves]]'
  slots_of_30: 0.4
spawns:
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5168.8
  y: 5333
  count: 0
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5576.1
  y: 5356.7
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5630.1
  y: 5343
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5108.4
  y: 5211.3
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5108.4
  y: 5211.3
  count: 2
  group: reinforcements
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5108
  y: 5154.1
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5108
  y: 5154.1
  count: 2
  group: reinforcements
source:
  data: LIST_NPC.STB row 348, ITEM_DROP.STB row 366
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Ruper Captain

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
