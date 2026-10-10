---
kind: monster
id: 52
name: Elder Smouly
status: in-game
level: 46
hp: 1380
hp_per_level: 30
attack: 173
hit: 114
defence: 113
resistance: 90
avoid: 71
attack_speed: 90
attack_range: 12.5
damage: magic
walk_speed: 230
run_speed: 326
xp: 35
drop_item_rate: 58
drop_money_rate: 12
zones: 1
drops:
- item: '[[items/material/182-thin-leaf|Thin Leaf]]'
  slots_of_30: 4
- item: '[[items/material/184-plant-seed|Plant Seed]]'
  slots_of_30: 3
- item: '[[items/material/190-bark|Bark]]'
  slots_of_30: 3
- item: '[[items/consumable/154-hp-point-300|HP Point (+300)]]'
  slots_of_30: 1
- item: '[[items/material/183-thick-leaf|Thick Leaf]]'
  slots_of_30: 4
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/22-mana-vial-m|Mana Vial (M)]]'
  slots_of_30: 1
- item: '[[items/consumable/164-mp-point-200|MP Point (+200)]]'
  slots_of_30: 1
- item: '[[items/jewellery/261-socket-necklace|Socket Necklace]]'
  slots_of_30: 1
- item: '[[items/consumable/307-mp-scroll-solo|MP Scroll (Solo)]]'
  slots_of_30: 2
- item: '[[items/weapon/304-mage-s-rod|Mage''s Rod]]'
  slots_of_30: 0.4
- item: '[[items/weapon/6-bushido|Bushido]]'
  slots_of_30: 0.2
- item: '[[items/weapon/432-long-sword-mace|Long Sword & Mace]]'
  slots_of_30: 0.2
- item: '[[items/weapon/164-scimitar|Scimitar]]'
  slots_of_30: 0.2
- item: '[[items/weapon/431-khukuri-long-sword|Khukuri & Long Sword]]'
  slots_of_30: 0.2
- item: '[[items/weapon/404-rake-hand|Rake Hand]]'
  slots_of_30: 0.6
- item: '[[items/body/32-iron-armor|Iron Armor]]'
  slots_of_30: 0.4
- item: '[[items/hands/62-green-gloves-of-witch|Green Gloves of Witch]]'
  slots_of_30: 0.4
- item: '[[items/feet/92-criker-boots|Criker Boots]]'
  slots_of_30: 0.4
- item: '[[items/head/122-vibe-turban|Vibe Turban]]'
  slots_of_30: 0.4
- item: '[[items/weapon/234-iron-rifle|Iron Rifle]]'
  slots_of_30: 0.2
- item: '[[items/weapon/35-onion-mace|Onion Mace]]'
  slots_of_30: 0.2
- item: '[[items/subweapon/3-round-shield|Round Shield]]'
  slots_of_30: 0.2
quests:
- '[[quests/5012-stockpiling-konara-branches|Stockpiling Konara Branches]]'
spawns:
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5268.9
  y: 5449.3
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5268.9
  y: 5449.3
  count: 1
  group: reinforcements
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5211.2
  y: 5453.7
  count: 1
  group: reinforcements
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5259
  y: 5399.4
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5259
  y: 5399.4
  count: 1
  group: reinforcements
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5430.2
  y: 5161.8
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5418.3
  y: 5185.6
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5465.4
  y: 5171.9
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5465.4
  y: 5171.9
  count: 2
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5470.2
  y: 5097.5
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5588
  y: 5014.1
  count: 2
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5529.5
  y: 5047.2
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5500.5
  y: 5072.3
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5614.2
  y: 5002.6
  count: 2
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5614.8
  y: 5044.5
  count: 2
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5658.3
  y: 5032.4
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5653.1
  y: 4985.1
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5611.3
  y: 4917.5
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 52, ITEM_DROP.STB row 122
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Elder Smouly

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
