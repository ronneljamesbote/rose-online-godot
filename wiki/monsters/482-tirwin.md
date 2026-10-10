---
kind: monster
id: 482
name: Tirwin
status: in-game
level: 88
hp: 5016
hp_per_level: 57
attack: 378
hit: 264
defence: 319
resistance: 184
avoid: 94
attack_speed: 106
attack_range: 2.3
damage: physical
walk_speed: 241
run_speed: 682
xp: 101
drop_item_rate: 48
drop_money_rate: 30
zones: 1
drops:
- item: '[[items/material/119-petil|Petil]]'
  slots_of_30: 1
- item: '[[items/material/120-setil|Setil]]'
  slots_of_30: 1
- item: '[[items/material/114-platinum-thread|Platinum Thread]]'
  slots_of_30: 1
- item: '[[items/material/79-transparent-powder|Transparent Powder]]'
  slots_of_30: 3
- item: '[[items/material/80-rainbow-powder|Rainbow Powder]]'
  slots_of_30: 3
- item: '[[items/material/65-high-ether|High Ether]]'
  slots_of_30: 2
- item: '[[items/material/81-lisent-fe|Lisent (Fe)]]'
  slots_of_30: 3
- item: '[[items/material/78-golden-powder|Golden Powder]]'
  slots_of_30: 1
- item: '[[items/material/66-iricer|Iricer]]'
  slots_of_30: 1
- item: '[[items/material/82-lisent-cu|Lisent (Cu)]]'
  slots_of_30: 2
- item: '[[items/material/67-hime|Hime]]'
  slots_of_30: 1
- item: '[[items/material/161-green-crystal|Green Crystal]]'
  slots_of_30: 2
- item: '[[items/consumable/308-dexterity-scroll-solo|Dexterity Scroll (Solo)]]'
  slots_of_30: 1
- item: '[[items/consumable/316-advanced-strength-scroll-solo|Advanced Strength Scroll (Solo)]]'
  slots_of_30: 1
- item: '[[items/weapon/704-sorden-gun|Sorden Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/615-tower-axe|Tower Axe]]'
  slots_of_30: 0.2
- item: '[[items/weapon/534-stone-hammer|Stone Hammer]]'
  slots_of_30: 0.2
- item: '[[items/weapon/503-elven-sword|Elven Sword]]'
  slots_of_30: 0.2
- item: '[[items/weapon/11-viking-sword|Viking Sword]]'
  slots_of_30: 0.2
- item: '[[items/head/16-redfield-green-hat|Redfield Green Hat]]'
  slots_of_30: 0.2
- item: '[[items/body/16-redfield-green-suit|Redfield Green Suit]]'
  slots_of_30: 0.4
- item: '[[items/hands/16-redfield-green-gloves|Redfield Green Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/16-redfield-green-shoes|Redfield Green Shoes]]'
  slots_of_30: 0.2
- item: '[[items/weapon/40-morning-star|Morning Star]]'
  slots_of_30: 0.4
- item: '[[items/weapon/65-poison-bow-gun|Poison Bow Gun]]'
  slots_of_30: 0.4
- item: '[[items/weapon/109-executioner|Executioner]]'
  slots_of_30: 0.4
- item: '[[items/weapon/138-silver-axe|Silver Axe]]'
  slots_of_30: 0.4
- item: '[[items/weapon/210-half-elf-bow|Half-Elf Bow]]'
  slots_of_30: 0.4
spawns:
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5395.3
  y: 5363.9
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5575.4
  y: 5370.6
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5258.1
  y: 5202.1
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5235.9
  y: 5170.1
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5258.2
  y: 5192.3
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5237.3
  y: 5216.9
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5282.6
  y: 5172.7
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5282.6
  y: 5172.7
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5395.1
  y: 5188
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5280.1
  y: 5217.3
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5280.1
  y: 5217.3
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5575
  y: 5196.2
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5395.9
  y: 5019.4
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5395.9
  y: 5019.4
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5597.7
  y: 4718.9
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 482, ITEM_DROP.STB row 411
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Tirwin

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
