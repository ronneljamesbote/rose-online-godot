---
kind: monster
id: 182
name: Clown
status: in-game
level: 49
hp: 28
attack: 126
hit: 127
defence: 140
resistance: 93
avoid: 75
attack_speed: 95
attack_range: 1.6
damage: physical
walk_speed: 165
run_speed: 530
xp: 35
drop_item_rate: 55
drop_money_rate: 23
zones: 1
drops:
- item: '[[items/material/187-black-animal-tail-fur|Black Animal Tail Fur]]'
  slots_of_30: 8
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 8
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/jewellery/251-socket-ring|Socket Ring]]'
  slots_of_30: 1
- item: '[[items/consumable/426-clown|Clown]]'
  slots_of_30: 1
- item: '[[items/consumable/307-mp-scroll-solo|MP Scroll (Solo)]]'
  slots_of_30: 1
- item: '[[items/feet/62-green-sandals-of-witch|Green Sandals of Witch]]'
  slots_of_30: 0.2
- item: '[[items/body/122-vibe-vest|Vibe Vest]]'
  slots_of_30: 0.2
- item: '[[items/weapon/234-iron-rifle|Iron Rifle]]'
  slots_of_30: 0.2
- item: '[[items/weapon/404-rake-hand|Rake Hand]]'
  slots_of_30: 0.6
- item: '[[items/weapon/304-mage-s-rod|Mage''s Rod]]'
  slots_of_30: 0.6
- item: '[[items/weapon/432-long-sword-mace|Long Sword & Mace]]'
  slots_of_30: 0.2
- item: '[[items/head/13-luxurious-venetian-hat|Luxurious Venetian Hat]]'
  slots_of_30: 0.2
- item: '[[items/body/13-fancy-blue|Fancy Blue]]'
  slots_of_30: 0.2
- item: '[[items/hands/13-luxurious-venetian-gloves|Luxurious Venetian Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/13-luxurious-marble-shoes|Luxurious Marble Shoes]]'
  slots_of_30: 0.2
- item: '[[items/weapon/35-onion-mace|Onion Mace]]'
  slots_of_30: 0.4
- item: '[[items/weapon/133-battle-axe|Battle Axe]]'
  slots_of_30: 0.4
- item: '[[items/subweapon/64-book-of-charm|Book of Charm]]'
  slots_of_30: 0.4
quests:
- '[[quests/5012-stockpiling-konara-branches|Stockpiling Konara Branches]]'
spawns:
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5182.9
  y: 5198.9
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5115.9
  y: 5059.5
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5119.8
  y: 5031.9
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5126.1
  y: 5072.2
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5401.7
  y: 4962.6
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5379.6
  y: 4966.9
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5329.8
  y: 5000
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5222.8
  y: 4944.4
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5276.9
  y: 4956.5
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5432.1
  y: 4953.3
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5397.1
  y: 4908.8
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5470.7
  y: 4898.9
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5641.8
  y: 4930.2
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5640.3
  y: 4881.5
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 182, ITEM_DROP.STB row 192
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Clown

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
