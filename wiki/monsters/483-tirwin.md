---
kind: monster
id: 483
name: Tirwin
status: in-game
level: 102
hp: 60
attack: 447
hit: 304
defence: 380
resistance: 225
avoid: 111
attack_speed: 107
attack_range: 2.3
damage: physical
walk_speed: 247
run_speed: 694
xp: 115
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
  x: 5394.6
  y: 4831.1
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5394.6
  y: 4831.1
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5574.5
  y: 4825.4
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5035.3
  y: 4744.4
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5213.1
  y: 4651.4
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5573.3
  y: 4697.9
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5573.3
  y: 4697.9
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5573.3
  y: 4697.9
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5550.2
  y: 4721.3
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5550.2
  y: 4721.3
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5550.2
  y: 4721.3
  count: 3
  group: reinforcements
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5575.6
  y: 4706.3
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5575.6
  y: 4706.3
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5597.7
  y: 4718.9
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5597.7
  y: 4718.9
  count: 3
  group: reinforcements
source:
  data: LIST_NPC.STB row 483, ITEM_DROP.STB row 412
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Tirwin

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
