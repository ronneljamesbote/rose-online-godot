---
kind: monster
id: 261
name: Goblin Jar
status: in-game
level: 45
hp: 1485
hp_per_level: 33
attack: 146
hit: 131
defence: 169
resistance: 96
avoid: 36
attack_speed: 90
attack_range: 8
damage: physical
walk_speed: 150
run_speed: 350
xp: 25
drop_item_rate: 52
drop_money_rate: 60
zones: 1
drops:
- item: '[[items/material/201-steam-oil|Steam Oil]]'
  slots_of_30: 4
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/4-health-bottle-s|Health Bottle (S)]]'
  slots_of_30: 1
- item: '[[items/consumable/163-mp-point-100|MP Point (+100)]]'
  slots_of_30: 1
- item: '[[items/material/202-steam-heat|Steam Heat]]'
  slots_of_30: 3
- item: '[[items/consumable/153-hp-point-200|HP Point (+200)]]'
  slots_of_30: 1
- item: '[[items/material/203-steam-viper|Steam Viper]]'
  slots_of_30: 1
- item: '[[items/material/204-mana-metal|Mana Metal]]'
  slots_of_30: 3
- item: '[[items/jewellery/271-socket-earring|Socket Earring]]'
  slots_of_30: 1
- item: '[[items/jewellery/84-fine-necklace|Fine Necklace]]'
  slots_of_30: 1
- item: '[[items/weapon/7-saber|Saber]]'
  slots_of_30: 0.2
- item: '[[items/weapon/61-simple-bow-gun|Simple Bow Gun]]'
  slots_of_30: 0.4
- item: '[[items/weapon/432-long-sword-mace|Long Sword & Mace]]'
  slots_of_30: 0.4
- item: '[[items/weapon/262-basic-launcher|Basic Launcher]]'
  slots_of_30: 0.2
- item: '[[items/weapon/35-onion-mace|Onion Mace]]'
  slots_of_30: 0.2
- item: '[[items/weapon/334-elven-wand|Elven Wand]]'
  slots_of_30: 0.2
- item: '[[items/weapon/165-bardiche|Bardiche]]'
  slots_of_30: 0.2
- item: '[[items/subweapon/64-book-of-charm|Book of Charm]]'
  slots_of_30: 0.2
- item: '[[items/weapon/133-battle-axe|Battle Axe]]'
  slots_of_30: 0.4
- item: '[[items/head/32-iron-helm|Iron Helm]]'
  slots_of_30: 0.4
- item: '[[items/body/62-green-vest-of-witch|Green Vest of Witch]]'
  slots_of_30: 0.4
- item: '[[items/hands/92-criker-gloves|Criker Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/122-vibe-shoes|Vibe Shoes]]'
  slots_of_30: 0.4
quests:
- '[[quests/1981-zeppastone-wind-gem|Zeppastone Wind Gem]]'
spawns:
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5061.9
  y: 5282.6
  count: 3
  group: reinforcements
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5061.9
  y: 5282.6
  count: 4
  group: reinforcements
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5080.6
  y: 5281.5
  count: 3
  group: reinforcements
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5069.4
  y: 5307.6
  count: 3
  group: reinforcements
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5424.4
  y: 5435
  count: 1
  group: basic
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5424.4
  y: 5435
  count: 3
  group: reinforcements
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5424.4
  y: 5435
  count: 5
  group: reinforcements
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5383
  y: 5152.8
  count: 5
  group: reinforcements
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5378.2
  y: 5133.6
  count: 5
  group: reinforcements
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5385.4
  y: 5170.7
  count: 5
  group: reinforcements
- zone: '[[zones/31-goblin-cave-b1|Goblin Cave (B1)]]'
  x: 5162.2
  y: 5057.7
  count: 4
  group: reinforcements
source:
  data: LIST_NPC.STB row 261, ITEM_DROP.STB row 224
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Goblin Jar

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
