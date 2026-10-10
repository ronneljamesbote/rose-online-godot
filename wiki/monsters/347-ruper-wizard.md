---
kind: monster
id: 347
name: Ruper Wizard
status: in-game
level: 102
hp: 41
attack: 420
hit: 234
defence: 244
resistance: 391
avoid: 155
attack_speed: 95
attack_range: 15
damage: magic
walk_speed: 210
run_speed: 530
xp: 82
drop_item_rate: 85
drop_money_rate: 0
zones: 1
drops:
- item: '[[items/material/193-frozen-animal-fur|Frozen Animal Fur]]'
  slots_of_30: 6
- item: '[[items/material/263-ruper-crest|Ruper Crest]]'
  slots_of_30: 7
- item: '[[items/material/128-hersian|Hersian]]'
  slots_of_30: 1
- item: '[[items/material/123-earth-stone|Earth Stone]]'
  slots_of_30: 1
- item: '[[items/material/143-ocarina-pieces|Ocarina Pieces]]'
  slots_of_30: 3
- item: '[[items/material/142-coin-of-eucar-tribe|Coin of Eucar Tribe]]'
  slots_of_30: 1
- item: '[[items/hands/65-gloves-of-shadow|Gloves of Shadow]]'
  slots_of_30: 0.2
- item: '[[items/feet/95-mariner-s-boots|Mariner''s Boots]]'
  slots_of_30: 0.2
- item: '[[items/body/35-mighty-armor|Mighty Armor]]'
  slots_of_30: 0.2
- item: '[[items/head/125-merchant-hat|Merchant Hat]]'
  slots_of_30: 0.2
- item: '[[items/head/35-mighty-helm|Mighty Helm]]'
  slots_of_30: 0.2
- item: '[[items/weapon/111-claymore|Claymore]]'
  slots_of_30: 0.2
- item: '[[items/weapon/438-blood-sword-axe|Blood Sword & Axe]]'
  slots_of_30: 0.2
- item: '[[items/weapon/410-shaft-claw|Shaft Claw]]'
  slots_of_30: 0.2
- item: '[[items/weapon/13-katana|Katana]]'
  slots_of_30: 0.2
- item: '[[items/weapon/171-shining-spear|Shining Spear]]'
  slots_of_30: 0.2
- item: '[[items/weapon/212-bow-of-east-archer|Bow of East Archer]]'
  slots_of_30: 0.4
- item: '[[items/weapon/311-holy-staff|Holy Staff]]'
  slots_of_30: 0.4
- item: '[[items/subweapon/70-mana-stone|Mana Stone]]'
  slots_of_30: 0.4
- item: '[[items/subweapon/71-rune-stone|Rune Stone]]'
  slots_of_30: 0.4
- item: '[[items/subweapon/72-cubic-symbol|Cubic Symbol]]'
  slots_of_30: 0.4
spawns:
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
  x: 5483.2
  y: 5320.5
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5630.1
  y: 5343
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5604.3
  y: 5332.5
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
  x: 5539.9
  y: 5170.4
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5714.3
  y: 5202.1
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5613.3
  y: 5140.7
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5566.6
  y: 5016
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5719.6
  y: 4997.3
  count: 2
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5743
  y: 5054.2
  count: 2
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 6069.3
  y: 5065.1
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 6069.3
  y: 5065.1
  count: 1
  group: reinforcements
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5540.7
  y: 4947.8
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5540.7
  y: 4947.8
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5555
  y: 4869.5
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5555
  y: 4869.5
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 347, ITEM_DROP.STB row 365
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Ruper Wizard

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
