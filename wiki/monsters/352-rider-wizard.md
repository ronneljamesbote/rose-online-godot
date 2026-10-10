---
kind: monster
id: 352
name: Rider Wizard
status: in-game
level: 104
hp: 4888
hp_per_level: 47
attack: 429
hit: 239
defence: 251
resistance: 400
avoid: 158
attack_speed: 100
attack_range: 15
damage: magic
walk_speed: 275
run_speed: 640
xp: 106
drop_item_rate: 58
drop_money_rate: 0
zones: 1
drops:
- item: '[[items/material/193-frozen-animal-fur|Frozen Animal Fur]]'
  slots_of_30: 4
- item: '[[items/material/262-yeti-charm|Yeti Charm]]'
  slots_of_30: 8
- item: '[[items/material/123-earth-stone|Earth Stone]]'
  slots_of_30: 1
- item: '[[items/material/124-fire-stone|Fire Stone]]'
  slots_of_30: 1
- item: '[[items/material/144-ice-key-fragment|Ice Key Fragment]]'
  slots_of_30: 3
- item: '[[items/material/143-ocarina-pieces|Ocarina Pieces]]'
  slots_of_30: 1
- item: '[[items/consumable/436-yeti-rider|Yeti Rider]]'
  slots_of_30: 2
- item: '[[items/head/35-mighty-helm|Mighty Helm]]'
  slots_of_30: 0.2
- item: '[[items/body/65-coat-of-shadow|Coat of Shadow]]'
  slots_of_30: 0.2
- item: '[[items/hands/95-mariner-s-gloves|Mariner''s Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/125-merchant-boots|Merchant Boots]]'
  slots_of_30: 0.2
- item: '[[items/head/65-hat-of-shadow|Hat of Shadow]]'
  slots_of_30: 0.2
- item: '[[items/weapon/13-katana|Katana]]'
  slots_of_30: 0.2
- item: '[[items/weapon/43-scorpion-club|Scorpion Club]]'
  slots_of_30: 0.2
- item: '[[items/weapon/411-assassin-katar|Assassin Katar]]'
  slots_of_30: 0.2
- item: '[[items/weapon/311-holy-staff|Holy Staff]]'
  slots_of_30: 0.2
- item: '[[items/weapon/341-windstorm-wand|Windstorm Wand]]'
  slots_of_30: 0.2
- item: '[[items/head/125-merchant-hat|Merchant Hat]]'
  slots_of_30: 0.4
- item: '[[items/body/95-mariner-s-look|Mariner’s Look]]'
  slots_of_30: 0.4
- item: '[[items/body/125-merchant-vest|Merchant Vest]]'
  slots_of_30: 0.4
- item: '[[items/hands/35-mighty-gauntlets|Mighty Gauntlets]]'
  slots_of_30: 0.4
- item: '[[items/feet/65-boots-of-shadow|Boots of Shadow]]'
  slots_of_30: 0.4
spawns:
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
  x: 5988.4
  y: 5248.6
  count: 1
  group: reinforcements
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
  x: 5214
  y: 4794.5
  count: 1
  group: reinforcements
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5166.6
  y: 4797.5
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 352, ITEM_DROP.STB row 368
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Rider Wizard

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
