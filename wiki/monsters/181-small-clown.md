---
kind: monster
id: 181
name: Small Clown
status: in-game
level: 47
hp: 1316
hp_per_level: 28
attack: 131
hit: 142
defence: 146
resistance: 64
avoid: 48
attack_speed: 115
attack_range: 1.6
damage: physical
walk_speed: 180
run_speed: 580
xp: 33
drop_item_rate: 53
drop_money_rate: 20
zones: 1
drops:
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 7
- item: '[[items/material/187-black-animal-tail-fur|Black Animal Tail Fur]]'
  slots_of_30: 9
- item: '[[items/material/178-sticky-liquid|Sticky Liquid]]'
  slots_of_30: 1
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/jewellery/271-socket-earring|Socket Earring]]'
  slots_of_30: 1
- item: '[[items/consumable/426-clown|Clown]]'
  slots_of_30: 1
- item: '[[items/jewellery/6-smooth-ring|Smooth Ring]]'
  slots_of_30: 1
- item: '[[items/weapon/6-bushido|Bushido]]'
  slots_of_30: 0.4
- item: '[[items/weapon/133-battle-axe|Battle Axe]]'
  slots_of_30: 0.2
- item: '[[items/weapon/132-small-axe|Small Axe]]'
  slots_of_30: 0.2
- item: '[[items/weapon/333-sorcerer-s-wand|Sorcerer''s Wand]]'
  slots_of_30: 0.2
- item: '[[items/weapon/164-scimitar|Scimitar]]'
  slots_of_30: 0.2
- item: '[[items/weapon/104-cutter-edge|Cutter Edge]]'
  slots_of_30: 0.2
- item: '[[items/weapon/205-white-wing-bow|White Wing Bow]]'
  slots_of_30: 0.2
- item: '[[items/weapon/234-iron-rifle|Iron Rifle]]'
  slots_of_30: 0.2
- item: '[[items/subweapon/4-buckler|Buckler]]'
  slots_of_30: 0.2
- item: '[[items/weapon/334-elven-wand|Elven Wand]]'
  slots_of_30: 0.4
- item: '[[items/body/32-iron-armor|Iron Armor]]'
  slots_of_30: 0.4
- item: '[[items/hands/62-green-gloves-of-witch|Green Gloves of Witch]]'
  slots_of_30: 0.4
- item: '[[items/feet/92-criker-boots|Criker Boots]]'
  slots_of_30: 0.4
- item: '[[items/head/122-vibe-turban|Vibe Turban]]'
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
  count: 2
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5329.8
  y: 5000
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
  data: LIST_NPC.STB row 181, ITEM_DROP.STB row 191
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Small Clown

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
