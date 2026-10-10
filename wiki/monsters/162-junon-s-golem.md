---
kind: monster
id: 162
name: Junon's Golem
status: in-game
level: 73
hp: 79
attack: 334
hit: 221
defence: 262
resistance: 167
avoid: 89
attack_speed: 90
attack_range: 2.3
damage: physical
walk_speed: 290
run_speed: 630
xp: 162
drop_item_rate: 72
drop_money_rate: 1
zones: 1
drops:
- item: '[[items/consumable/2-health-vial-m|Health Vial (M)]]'
  slots_of_30: 1
- item: '[[items/material/201-steam-oil|Steam Oil]]'
  slots_of_30: 2
- item: '[[items/material/202-steam-heat|Steam Heat]]'
  slots_of_30: 2
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/material/203-steam-viper|Steam Viper]]'
  slots_of_30: 1
- item: '[[items/material/204-mana-metal|Mana Metal]]'
  slots_of_30: 1
- item: '[[items/material/205-mana-oil|Mana Oil]]'
  slots_of_30: 3
- item: '[[items/material/206-mana-heat|Mana Heat]]'
  slots_of_30: 3
- item: '[[items/jewellery/261-socket-necklace|Socket Necklace]]'
  slots_of_30: 1
- item: '[[items/jewellery/16-critical-ring|Critical Ring]]'
  slots_of_30: 1
- item: '[[items/consumable/317-advanced-defense-scroll-solo|Advanced Defense Scroll (Solo)]]'
  slots_of_30: 1
- item: '[[items/weapon/435-saber-elven-sword|Saber & Elven Sword]]'
  slots_of_30: 0.6
- item: '[[items/weapon/168-death-spear|Death Spear]]'
  slots_of_30: 0.6
- item: '[[items/weapon/64-crossbow-gun|Crossbow Gun]]'
  slots_of_30: 0.6
- item: '[[items/weapon/38-skeleton-hammer|Skeleton Hammer]]'
  slots_of_30: 0.4
- item: '[[items/weapon/136-tower-axe|Tower Axe]]'
  slots_of_30: 0.2
- item: '[[items/body/94-black-pirate-armor|Black Pirate Armor]]'
  slots_of_30: 0.2
- item: '[[items/hands/124-semiya-gloves|Semiya Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/34-chrome-boots|Chrome Boots]]'
  slots_of_30: 0.2
- item: '[[items/head/64-violet-beret|Violet Beret]]'
  slots_of_30: 0.2
- item: '[[items/weapon/10-shark-blade|Shark Blade]]'
  slots_of_30: 0.4
- item: '[[items/back/245-lion-backshield|Lion Backshield]]'
  slots_of_30: 0.4
- item: '[[items/weapon/538-morning-star|Morning Star]]'
  slots_of_30: 0.2
- item: '[[items/hands/320-trunket-gloves|Trunket Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/320-trunket-boots|Trunket Boots]]'
  slots_of_30: 0.2
- item: '[[items/head/620-ranger-hat|Ranger Hat]]'
  slots_of_30: 0.2
- item: '[[items/body/620-tamiya-vest|Tamiya Vest]]'
  slots_of_30: 0.2
spawns:
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5275.7
  y: 5473.6
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5144.6
  y: 5323.6
  count: 1
  group: reinforcements
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5293.8
  y: 5321.6
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5205.3
  y: 5270.5
  count: 1
  group: reinforcements
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5535.1
  y: 5188.5
  count: 1
  group: reinforcements
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5552
  y: 5122.9
  count: 1
  group: reinforcements
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5475.9
  y: 5177.8
  count: 1
  group: reinforcements
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5481.5
  y: 5117.4
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 162, ITEM_DROP.STB row 180
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Junon's Golem

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
