---
kind: monster
id: 344
name: Yeti Captain
status: in-game
level: 105
hp: 68
attack: 460
hit: 278
defence: 383
resistance: 181
avoid: 168
attack_speed: 120
attack_range: 2.5
damage: physical
walk_speed: 235
run_speed: 580
xp: 151
drop_item_rate: 70
drop_money_rate: 0
zones: 1
drops:
- item: '[[items/material/193-frozen-animal-fur|Frozen Animal Fur]]'
  slots_of_30: 7
- item: '[[items/material/262-yeti-charm|Yeti Charm]]'
  slots_of_30: 6
- item: '[[items/material/114-platinum-thread|Platinum Thread]]'
  slots_of_30: 1
- item: '[[items/material/115-precious-platinum-thread|Precious Platinum Thread]]'
  slots_of_30: 1
- item: '[[items/material/144-ice-key-fragment|Ice Key Fragment]]'
  slots_of_30: 4
- item: '[[items/body/95-mariner-s-look|Mariner’s Look]]'
  slots_of_30: 0.6
- item: '[[items/hands/125-merchant-gloves|Merchant Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/35-mighty-boots|Mighty Boots]]'
  slots_of_30: 0.2
- item: '[[items/head/65-hat-of-shadow|Hat of Shadow]]'
  slots_of_30: 0.6
- item: '[[items/hands/35-mighty-gauntlets|Mighty Gauntlets]]'
  slots_of_30: 0.6
- item: '[[items/feet/125-merchant-boots|Merchant Boots]]'
  slots_of_30: 0.4
- item: '[[items/body/125-merchant-vest|Merchant Vest]]'
  slots_of_30: 0.4
- item: '[[items/weapon/437-viking-sword-axe|Viking Sword & Axe]]'
  slots_of_30: 0.4
- item: '[[items/weapon/67-elf-bow-gun|Elf Bow Gun]]'
  slots_of_30: 0.4
- item: '[[items/weapon/340-recovery-wand|Recovery Wand]]'
  slots_of_30: 0.4
- item: '[[items/weapon/311-holy-staff|Holy Staff]]'
  slots_of_30: 0.4
- item: '[[items/weapon/511-katana|Katana]]'
  slots_of_30: 0.4
spawns:
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5168.8
  y: 5333
  count: 0
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5168.8
  y: 5333
  count: 0
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5595.3
  y: 5315.5
  count: 1
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
  x: 5108
  y: 5154.1
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5250.2
  y: 5027.2
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5205.8
  y: 4986.6
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5310.6
  y: 5040.5
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5236.9
  y: 4949.5
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5263.1
  y: 4844.9
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5201.5
  y: 4894
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5150
  y: 4874.3
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5190.1
  y: 4874.3
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5333.2
  y: 4821.2
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5214
  y: 4794.5
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5166.6
  y: 4797.5
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 344, ITEM_DROP.STB row 363
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Yeti Captain

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
