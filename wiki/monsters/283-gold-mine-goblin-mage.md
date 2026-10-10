---
kind: monster
id: 283
name: Gold Mine Goblin Mage
status: in-game
level: 76
hp: 48
attack: 330
hit: 200
defence: 249
resistance: 164
avoid: 93
attack_speed: 100
attack_range: 10
damage: magic
walk_speed: 225
run_speed: 530
xp: 63
drop_item_rate: 58
drop_money_rate: 10
zones: 1
drops:
- item: '[[items/material/192-animal-backbone|Animal Backbone]]'
  slots_of_30: 7
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 6
- item: '[[items/consumable/155-hp-point-500|HP Point (+500)]]'
  slots_of_30: 1
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/165-mp-point-300|MP Point (+300)]]'
  slots_of_30: 1
- item: '[[items/material/199-little-devil-feather|Little Devil Feather]]'
  slots_of_30: 2
- item: '[[items/jewellery/251-socket-ring|Socket Ring]]'
  slots_of_30: 1
- item: '[[items/jewellery/163-talisman-earring|Talisman Earring]]'
  slots_of_30: 1
- item: '[[items/head/94-black-pirate-hat|Black Pirate Hat]]'
  slots_of_30: 0.2
- item: '[[items/body/124-semiya-vest|Semiya Vest]]'
  slots_of_30: 0.2
- item: '[[items/weapon/337-flame-wand|Flame Wand]]'
  slots_of_30: 0.4
- item: '[[items/weapon/136-tower-axe|Tower Axe]]'
  slots_of_30: 0.2
- item: '[[items/weapon/307-shadow-staff|Shadow Staff]]'
  slots_of_30: 0.2
- item: '[[items/weapon/435-saber-elven-sword|Saber & Elven Sword]]'
  slots_of_30: 0.2
- item: '[[items/weapon/64-crossbow-gun|Crossbow Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/10-shark-blade|Shark Blade]]'
  slots_of_30: 0.2
- item: '[[items/subweapon/67-basic-encyclopedia|Basic Encyclopedia]]'
  slots_of_30: 0.2
- item: '[[items/weapon/107-two-handed-sword|Two-Handed Sword]]'
  slots_of_30: 0.4
- item: '[[items/body/34-chrome-armor|Chrome Armor]]'
  slots_of_30: 0.4
- item: '[[items/hands/64-violet-gloves|Violet Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/94-black-pirate-boots|Black Pirate Boots]]'
  slots_of_30: 0.4
- item: '[[items/head/124-semiya-goggles|Semiya Goggles]]'
  slots_of_30: 0.4
spawns:
- zone: '[[zones/32-goblin-cave-b2|Goblin Cave (B2)]]'
  x: 5119.3
  y: 5529.1
  count: 1
  group: reinforcements
- zone: '[[zones/32-goblin-cave-b2|Goblin Cave (B2)]]'
  x: 5148.6
  y: 5501.6
  count: 1
  group: reinforcements
- zone: '[[zones/32-goblin-cave-b2|Goblin Cave (B2)]]'
  x: 5376.5
  y: 5505
  count: 1
  group: reinforcements
- zone: '[[zones/32-goblin-cave-b2|Goblin Cave (B2)]]'
  x: 5395.6
  y: 5533.7
  count: 1
  group: reinforcements
- zone: '[[zones/32-goblin-cave-b2|Goblin Cave (B2)]]'
  x: 5091.4
  y: 5133.8
  count: 1
  group: reinforcements
- zone: '[[zones/32-goblin-cave-b2|Goblin Cave (B2)]]'
  x: 5121.5
  y: 5159.4
  count: 1
  group: reinforcements
- zone: '[[zones/32-goblin-cave-b2|Goblin Cave (B2)]]'
  x: 5142.8
  y: 5118.9
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 283, ITEM_DROP.STB row 238
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Gold Mine Goblin Mage

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
