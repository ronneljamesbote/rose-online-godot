---
kind: monster
id: 475
name: Tirwin
status: in-game
level: 118
hp: 5900
hp_per_level: 50
attack: 487
hit: 336
defence: 280
resistance: 501
avoid: 181
attack_speed: 89
attack_range: 12
damage: magic
walk_speed: 234
run_speed: 545
xp: 112
drop_item_rate: 49
drop_money_rate: 30
zones: 1
drops:
- item: '[[items/material/125-light-stone|Light Stone]]'
  slots_of_30: 1
- item: '[[items/material/129-jilsian|Jilsian]]'
  slots_of_30: 1
- item: '[[items/material/130-lusian|Lusian]]'
  slots_of_30: 1
- item: '[[items/material/82-lisent-cu|Lisent (Cu)]]'
  slots_of_30: 3
- item: '[[items/material/67-hime|Hime]]'
  slots_of_30: 2
- item: '[[items/material/81-lisent-fe|Lisent (Fe)]]'
  slots_of_30: 1
- item: '[[items/material/83-lisent-pb|Lisent (Pb)]]'
  slots_of_30: 3
- item: '[[items/material/66-iricer|Iricer]]'
  slots_of_30: 1
- item: '[[items/material/84-lisent-al|Lisent (Al)]]'
  slots_of_30: 2
- item: '[[items/material/68-low-enthiric|Low Enthiric]]'
  slots_of_30: 1
- item: '[[items/material/85-lisent-hg|Lisent (Hg)]]'
  slots_of_30: 1
- item: '[[items/material/69-enthiric|Enthiric]]'
  slots_of_30: 1
- item: '[[items/material/86-lisent-na|Lisent (Na)]]'
  slots_of_30: 1
- item: '[[items/material/163-red-crystal|Red Crystal]]'
  slots_of_30: 2
- item: '[[items/consumable/310-defense-scroll-solo|Defense Scroll (Solo)]]'
  slots_of_30: 1
- item: '[[items/consumable/318-advanced-accuracy-scroll-solo|Advanced Accuracy Scroll (Solo)]]'
  slots_of_30: 1
- item: '[[items/weapon/175-dragon-spear|Dragon Spear]]'
  slots_of_30: 0.2
- item: '[[items/weapon/344-bleak-wind|Bleak Wind]]'
  slots_of_30: 0.2
- item: '[[items/weapon/272-particle-cannon|Particle Cannon]]'
  slots_of_30: 0.2
- item: '[[items/weapon/442-brilliant-dual-wield|Brilliant Dual Wield]]'
  slots_of_30: 0.2
- item: '[[items/weapon/215-dual-stick-bow|Dual Stick Bow]]'
  slots_of_30: 0.6
- item: '[[items/body/125-merchant-vest|Merchant Vest]]'
  slots_of_30: 0.2
- item: '[[items/hands/95-mariner-s-gloves|Mariner''s Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/35-mighty-boots|Mighty Boots]]'
  slots_of_30: 0.2
- item: '[[items/head/65-hat-of-shadow|Hat of Shadow]]'
  slots_of_30: 0.2
- item: '[[items/body/95-mariner-s-look|Mariner’s Look]]'
  slots_of_30: 0.2
- item: '[[items/weapon/16-lord-black-sun-sword|Lord Black Sun Sword]]'
  slots_of_30: 0.8
- item: '[[items/weapon/271-corsair-cannon|Corsair Cannon]]'
  slots_of_30: 0.4
- item: '[[items/weapon/343-shadow-wand|Shadow Wand]]'
  slots_of_30: 0.4
spawns:
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5215.2
  y: 4570.1
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5124.5
  y: 4383.3
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5125.8
  y: 4463.6
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5306.3
  y: 4385
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5306.3
  y: 4385
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5307.8
  y: 4470.8
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5307.8
  y: 4470.8
  count: 2
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5395.2
  y: 4375.5
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5484.5
  y: 4384.5
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5484.5
  y: 4384.5
  count: 2
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5665.1
  y: 4381.9
  count: 2
  group: reinforcements
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5129
  y: 4289.5
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5129
  y: 4289.5
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5302.3
  y: 4290.4
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5385.7
  y: 4295.4
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 475, ITEM_DROP.STB row 413
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Tirwin

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
