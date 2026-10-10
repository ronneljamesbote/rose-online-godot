---
kind: monster
id: 183
name: Fighter Clown
status: in-game
level: 52
hp: 31
attack: 137
hit: 124
defence: 128
resistance: 104
avoid: 79
attack_speed: 100
attack_range: 2.3
damage: physical
walk_speed: 170
run_speed: 450
xp: 35
drop_item_rate: 56
drop_money_rate: 28
zones: 1
drops:
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 6
- item: '[[items/material/187-black-animal-tail-fur|Black Animal Tail Fur]]'
  slots_of_30: 6
- item: '[[items/consumable/163-mp-point-100|MP Point (+100)]]'
  slots_of_30: 1
- item: '[[items/consumable/154-hp-point-300|HP Point (+300)]]'
  slots_of_30: 1
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/jewellery/271-socket-earring|Socket Earring]]'
  slots_of_30: 1
- item: '[[items/material/189-predator-claw|Predator Claw]]'
  slots_of_30: 1
- item: '[[items/consumable/309-strength-scroll-solo|Strength Scroll (Solo)]]'
  slots_of_30: 1
- item: '[[items/jewellery/7-amulet-ring|Amulet Ring]]'
  slots_of_30: 1
- item: '[[items/weapon/405-wolf-s-paw|Wolf''s Paw]]'
  slots_of_30: 0.2
- item: '[[items/weapon/61-simple-bow-gun|Simple Bow Gun]]'
  slots_of_30: 0.4
- item: '[[items/weapon/165-bardiche|Bardiche]]'
  slots_of_30: 0.2
- item: '[[items/weapon/334-elven-wand|Elven Wand]]'
  slots_of_30: 0.2
- item: '[[items/weapon/35-onion-mace|Onion Mace]]'
  slots_of_30: 0.2
- item: '[[items/head/13-luxurious-venetian-hat|Luxurious Venetian Hat]]'
  slots_of_30: 0.2
- item: '[[items/body/13-fancy-blue|Fancy Blue]]'
  slots_of_30: 0.2
- item: '[[items/hands/13-luxurious-venetian-gloves|Luxurious Venetian Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/13-luxurious-marble-shoes|Luxurious Marble Shoes]]'
  slots_of_30: 0.2
- item: '[[items/weapon/7-saber|Saber]]'
  slots_of_30: 0.2
- item: '[[items/weapon/105-surado|Surado]]'
  slots_of_30: 0.2
- item: '[[items/weapon/206-elf-bow|Elf Bow]]'
  slots_of_30: 0.2
- item: '[[items/weapon/235-iron-shotgun|Iron Shotgun]]'
  slots_of_30: 0.2
- item: '[[items/subweapon/5-aspis|Aspis]]'
  slots_of_30: 0.2
- item: '[[items/weapon/36-ghost-bat|Ghost Bat]]'
  slots_of_30: 0.2
- item: '[[items/weapon/134-tomahawk|Tomahawk]]'
  slots_of_30: 0.2
- item: '[[items/weapon/263-hard-launcher|Hard Launcher]]'
  slots_of_30: 0.2
- item: '[[items/weapon/792-elven-wand|Elven Wand]]'
  slots_of_30: 0.2
- item: '[[items/weapon/502-saber|Saber]]'
  slots_of_30: 0.2
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
  x: 5432.1
  y: 4953.3
  count: 2
  group: reinforcements
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
  data: LIST_NPC.STB row 183, ITEM_DROP.STB row 193
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Fighter Clown

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
