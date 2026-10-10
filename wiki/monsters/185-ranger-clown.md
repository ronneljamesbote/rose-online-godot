---
kind: monster
id: 185
name: Ranger Clown
status: in-game
level: 48
hp: 26
attack: 161
hit: 122
defence: 101
resistance: 106
avoid: 93
attack_speed: 102
attack_range: 25
damage: physical
walk_speed: 175
run_speed: 550
xp: 39
drop_item_rate: 53
drop_money_rate: 22
zones: 1
drops:
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 3
- item: '[[items/material/187-black-animal-tail-fur|Black Animal Tail Fur]]'
  slots_of_30: 8
- item: '[[items/consumable/154-hp-point-300|HP Point (+300)]]'
  slots_of_30: 1
- item: '[[items/consumable/164-mp-point-200|MP Point (+200)]]'
  slots_of_30: 1
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/jewellery/251-socket-ring|Socket Ring]]'
  slots_of_30: 1
- item: '[[items/consumable/301-purify-scroll-solo|Purify Scroll (Solo)]]'
  slots_of_30: 1
- item: '[[items/jewellery/9-pointed-ring|Pointed Ring]]'
  slots_of_30: 1
- item: '[[items/weapon/234-iron-rifle|Iron Rifle]]'
  slots_of_30: 0.4
- item: '[[items/weapon/262-basic-launcher|Basic Launcher]]'
  slots_of_30: 0.2
- item: '[[items/weapon/205-white-wing-bow|White Wing Bow]]'
  slots_of_30: 0.2
- item: '[[items/weapon/6-bushido|Bushido]]'
  slots_of_30: 0.2
- item: '[[items/weapon/165-bardiche|Bardiche]]'
  slots_of_30: 0.6
- item: '[[items/head/13-luxurious-venetian-hat|Luxurious Venetian Hat]]'
  slots_of_30: 0.2
- item: '[[items/body/13-fancy-blue|Fancy Blue]]'
  slots_of_30: 0.2
- item: '[[items/hands/13-luxurious-venetian-gloves|Luxurious Venetian Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/13-luxurious-marble-shoes|Luxurious Marble Shoes]]'
  slots_of_30: 0.2
- item: '[[items/weapon/61-simple-bow-gun|Simple Bow Gun]]'
  slots_of_30: 0.4
- item: '[[items/weapon/432-long-sword-mace|Long Sword & Mace]]'
  slots_of_30: 0.4
- item: '[[items/weapon/334-elven-wand|Elven Wand]]'
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
  x: 5245.7
  y: 5061.8
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5245.7
  y: 5061.8
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
  count: 2
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5379.6
  y: 4966.9
  count: 2
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5329.8
  y: 5000
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5298.5
  y: 5010.8
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5298.5
  y: 5010.8
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5306.1
  y: 5081.6
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5306.1
  y: 5081.6
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5614.2
  y: 5002.6
  count: 2
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
  count: 2
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5397.1
  y: 4908.8
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5470.7
  y: 4898.9
  count: 2
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
  data: LIST_NPC.STB row 185, ITEM_DROP.STB row 194
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Ranger Clown

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
