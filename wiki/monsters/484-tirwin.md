---
kind: monster
id: 484
name: Tirwin
status: in-game
level: 108
hp: 61
attack: 477
hit: 322
defence: 407
resistance: 243
avoid: 119
attack_speed: 108
attack_range: 2.3
damage: physical
walk_speed: 253
run_speed: 706
xp: 122
drop_item_rate: 49
drop_money_rate: 30
zones: 1
drops:
- item: '[[items/material/118-etil|Etil]]'
  slots_of_30: 1
- item: '[[items/material/124-fire-stone|Fire Stone]]'
  slots_of_30: 1
- item: '[[items/material/115-precious-platinum-thread|Precious Platinum Thread]]'
  slots_of_30: 1
- item: '[[items/material/80-rainbow-powder|Rainbow Powder]]'
  slots_of_30: 3
- item: '[[items/material/81-lisent-fe|Lisent (Fe)]]'
  slots_of_30: 3
- item: '[[items/material/66-iricer|Iricer]]'
  slots_of_30: 2
- item: '[[items/material/82-lisent-cu|Lisent (Cu)]]'
  slots_of_30: 3
- item: '[[items/material/83-lisent-pb|Lisent (Pb)]]'
  slots_of_30: 2
- item: '[[items/material/67-hime|Hime]]'
  slots_of_30: 1
- item: '[[items/material/84-lisent-al|Lisent (Al)]]'
  slots_of_30: 1
- item: '[[items/material/68-low-enthiric|Low Enthiric]]'
  slots_of_30: 1
- item: '[[items/material/162-blue-crystal|Blue Crystal]]'
  slots_of_30: 2
- item: '[[items/consumable/309-strength-scroll-solo|Strength Scroll (Solo)]]'
  slots_of_30: 1
- item: '[[items/consumable/317-advanced-defense-scroll-solo|Advanced Defense Scroll (Solo)]]'
  slots_of_30: 1
- item: '[[items/weapon/564-dark-bow-gun|Dark Bow Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/507-viking-sword|Viking Sword]]'
  slots_of_30: 0.2
- item: '[[items/weapon/537-morning-star|Morning Star]]'
  slots_of_30: 0.2
- item: '[[items/weapon/827-dual-patar|Dual Patar]]'
  slots_of_30: 0.2
- item: '[[items/weapon/212-bow-of-east-archer|Bow of East Archer]]'
  slots_of_30: 0.2
- item: '[[items/head/16-redfield-green-hat|Redfield Green Hat]]'
  slots_of_30: 0.2
- item: '[[items/hands/16-redfield-green-gloves|Redfield Green Gloves]]'
  slots_of_30: 0.2
- item: '[[items/body/125-merchant-vest|Merchant Vest]]'
  slots_of_30: 0.2
- item: '[[items/hands/95-mariner-s-gloves|Mariner''s Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/35-mighty-boots|Mighty Boots]]'
  slots_of_30: 0.2
- item: '[[items/weapon/269-burning-launcher|Burning Launcher]]'
  slots_of_30: 0.4
- item: '[[items/weapon/341-windstorm-wand|Windstorm Wand]]'
  slots_of_30: 0.4
- item: '[[items/weapon/111-claymore|Claymore]]'
  slots_of_30: 0.4
- item: '[[items/weapon/140-great-axe|Great Axe]]'
  slots_of_30: 0.4
- item: '[[items/weapon/172-chaos-spear|Chaos Spear]]'
  slots_of_30: 0.4
spawns:
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5214.8
  y: 4828
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5214.8
  y: 4828
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5214.8
  y: 4828
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5035.3
  y: 4744.4
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5035.3
  y: 4744.4
  count: 3
  group: reinforcements
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5213.1
  y: 4651.4
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5213.1
  y: 4651.4
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5599.8
  y: 4675.5
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5599.8
  y: 4675.5
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5553.9
  y: 4677.6
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5553.9
  y: 4677.6
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5553.9
  y: 4677.6
  count: 3
  group: reinforcements
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5033
  y: 4561
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5033
  y: 4561
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5573.4
  y: 4577.8
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 484, ITEM_DROP.STB row 412
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Tirwin

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
