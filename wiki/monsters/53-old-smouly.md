---
kind: monster
id: 53
name: Old Smouly
status: in-game
level: 48
hp: 1680
hp_per_level: 35
attack: 190
hit: 140
defence: 147
resistance: 70
avoid: 79
attack_speed: 100
attack_range: 13
damage: magic
walk_speed: 240
run_speed: 338
xp: 49
drop_item_rate: 62
drop_money_rate: 15
zones: 1
drops:
- item: '[[items/material/184-plant-seed|Plant Seed]]'
  slots_of_30: 3
- item: '[[items/material/182-thin-leaf|Thin Leaf]]'
  slots_of_30: 4
- item: '[[items/consumable/172-stamina-75|Stamina (+75)]]'
  slots_of_30: 1
- item: '[[items/consumable/154-hp-point-300|HP Point (+300)]]'
  slots_of_30: 1
- item: '[[items/consumable/23-mana-vial-l|Mana Vial (L)]]'
  slots_of_30: 1
- item: '[[items/material/190-bark|Bark]]'
  slots_of_30: 4
- item: '[[items/material/183-thick-leaf|Thick Leaf]]'
  slots_of_30: 4
- item: '[[items/consumable/164-mp-point-200|MP Point (+200)]]'
  slots_of_30: 1
- item: '[[items/jewellery/271-socket-earring|Socket Earring]]'
  slots_of_30: 1
- item: '[[items/jewellery/92-charming-necklace|Charming Necklace]]'
  slots_of_30: 1
- item: '[[items/consumable/308-dexterity-scroll-solo|Dexterity Scroll (Solo)]]'
  slots_of_30: 1
- item: '[[items/weapon/261-wooden-launcher|Wooden Launcher]]'
  slots_of_30: 0.2
- item: '[[items/body/32-iron-armor|Iron Armor]]'
  slots_of_30: 0.2
- item: '[[items/weapon/304-mage-s-rod|Mage''s Rod]]'
  slots_of_30: 0.2
- item: '[[items/weapon/6-bushido|Bushido]]'
  slots_of_30: 0.4
- item: '[[items/weapon/35-onion-mace|Onion Mace]]'
  slots_of_30: 0.2
- item: '[[items/hands/32-iron-gauntlets|Iron Gauntlets]]'
  slots_of_30: 0.2
- item: '[[items/feet/62-green-sandals-of-witch|Green Sandals of Witch]]'
  slots_of_30: 0.2
- item: '[[items/head/92-criker-hat|Criker Hat]]'
  slots_of_30: 0.2
- item: '[[items/body/122-vibe-vest|Vibe Vest]]'
  slots_of_30: 0.2
- item: '[[items/weapon/133-battle-axe|Battle Axe]]'
  slots_of_30: 0.4
- item: '[[items/weapon/262-basic-launcher|Basic Launcher]]'
  slots_of_30: 0.4
- item: '[[items/weapon/334-elven-wand|Elven Wand]]'
  slots_of_30: 0.4
- item: '[[items/weapon/432-long-sword-mace|Long Sword & Mace]]'
  slots_of_30: 0.4
- item: '[[items/subweapon/63-book-of-concentration|Book of Concentration]]'
  slots_of_30: 0.4
quests:
- '[[quests/5012-stockpiling-konara-branches|Stockpiling Konara Branches]]'
spawns:
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
  group: reinforcements
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5430.2
  y: 5161.8
  count: 1
  group: reinforcements
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5470.2
  y: 5097.5
  count: 1
  group: reinforcements
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5588
  y: 5014.1
  count: 2
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5500.5
  y: 5072.3
  count: 1
  group: reinforcements
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
  x: 5611.3
  y: 4917.5
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 53, ITEM_DROP.STB row 123
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Old Smouly

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
