---
kind: monster
id: 161
name: Jewel Golem
status: in-game
level: 47
hp: 3478
hp_per_level: 74
attack: 208
hit: 164
defence: 168
resistance: 102
avoid: 57
attack_speed: 90
attack_range: 2.2
damage: physical
walk_speed: 250
run_speed: 550
xp: 131
drop_item_rate: 70
drop_money_rate: 1
zones: 2
drops:
- item: '[[items/material/201-steam-oil|Steam Oil]]'
  slots_of_30: 2
- item: '[[items/material/202-steam-heat|Steam Heat]]'
  slots_of_30: 2
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/material/203-steam-viper|Steam Viper]]'
  slots_of_30: 1
- item: '[[items/material/204-mana-metal|Mana Metal]]'
  slots_of_30: 2
- item: '[[items/material/205-mana-oil|Mana Oil]]'
  slots_of_30: 1
- item: '[[items/jewellery/251-socket-ring|Socket Ring]]'
  slots_of_30: 1
- item: '[[items/material/161-green-crystal|Green Crystal]]'
  slots_of_30: 3
- item: '[[items/weapon/262-basic-launcher|Basic Launcher]]'
  slots_of_30: 0.2
- item: '[[items/weapon/334-elven-wand|Elven Wand]]'
  slots_of_30: 0.2
- item: '[[items/weapon/432-long-sword-mace|Long Sword & Mace]]'
  slots_of_30: 0.2
- item: '[[items/weapon/61-simple-bow-gun|Simple Bow Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/165-bardiche|Bardiche]]'
  slots_of_30: 0.2
- item: '[[items/head/62-green-hat-of-witch|Green Hat of Witch]]'
  slots_of_30: 0.4
- item: '[[items/body/92-criker-chest|Criker Chest]]'
  slots_of_30: 0.4
- item: '[[items/hands/122-vibe-gloves|Vibe Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/32-iron-boots|Iron Boots]]'
  slots_of_30: 0.4
- item: '[[items/weapon/7-saber|Saber]]'
  slots_of_30: 0.4
- item: '[[items/weapon/105-surado|Surado]]'
  slots_of_30: 0.2
- item: '[[items/weapon/206-elf-bow|Elf Bow]]'
  slots_of_30: 0.2
- item: '[[items/weapon/235-iron-shotgun|Iron Shotgun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/305-white-staff|White Staff]]'
  slots_of_30: 0.2
- item: '[[items/weapon/405-wolf-s-paw|Wolf''s Paw]]'
  slots_of_30: 0.2
- item: '[[items/weapon/671-orc-bow|Orc Bow]]'
  slots_of_30: 0.2
- item: '[[items/feet/412-green-sandals-of-witch|Green Sandals of Witch]]'
  slots_of_30: 0.2
- item: '[[items/head/412-iron-helm|Iron Helm]]'
  slots_of_30: 0.2
- item: '[[items/hands/512-criker-gloves|Criker Gloves]]'
  slots_of_30: 0.2
- item: '[[items/head/712-vibe-turban|Vibe Turban]]'
  slots_of_30: 0.2
- item: '[[items/gem/301-garnet-1|Garnet 1]]'
  slots_of_30: 1
spawns:
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5205.4
  y: 5467.5
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5587
  y: 5194.9
  count: 1
  group: reinforcements
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5577.9
  y: 5243.5
  count: 1
  group: reinforcements
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5579.7
  y: 5269.8
  count: 1
  group: basic
- zone: '[[zones/25-anima-lake|Anima Lake]]'
  x: 5375.1
  y: 5092.9
  count: 1
  group: reinforcements
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5307.6
  y: 5271.9
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 161, ITEM_DROP.STB row 179
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Jewel Golem

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
