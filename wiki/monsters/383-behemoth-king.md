---
kind: monster
id: 383
name: Behemoth King
status: in-game
level: 142
hp: 168412
hp_per_level: 1186
attack: 757
hit: 466
defence: 621
resistance: 501
avoid: 246
attack_speed: 110
attack_range: 2.5
damage: physical
walk_speed: 300
run_speed: 710
xp: 5005
drop_item_rate: 95
drop_money_rate: 40
zones: 1
drops:
- item: '[[items/weapon/42-blood-mace|Blood Mace]]'
  slots_of_30: 1
- item: '[[items/weapon/66-dark-bow-gun|Dark Bow Gun]]'
  slots_of_30: 1
- item: '[[items/weapon/111-claymore|Claymore]]'
  slots_of_30: 1
- item: '[[items/weapon/438-blood-sword-axe|Blood Sword & Axe]]'
  slots_of_30: 1
- item: '[[items/hands/434-gloves-of-shadow|Gloves of Shadow]]'
  slots_of_30: 1
- item: '[[items/body/334-mighty-armor|Mighty Armor]]'
  slots_of_30: 1
- item: '[[items/head/733-merchant-hat|Merchant Hat]]'
  slots_of_30: 1
- item: '[[items/feet/534-mariner-s-boots|Mariner''s Boots]]'
  slots_of_30: 1
- item: '[[items/weapon/514-ice-sword|Ice Sword]]'
  slots_of_30: 1
- item: '[[items/weapon/544-scorpion-club|Scorpion Club]]'
  slots_of_30: 1
- item: '[[items/weapon/566-anima-bow-gun|Anima Bow Gun]]'
  slots_of_30: 1
- item: '[[items/weapon/735-cross-launcher|Cross Launcher]]'
  slots_of_30: 1
- item: '[[items/weapon/774-dark-staff|Dark Staff]]'
  slots_of_30: 1
- item: '[[items/weapon/902-gladius|Gladius]]'
  slots_of_30: 1
- item: '[[items/weapon/910-spike-club|Spike Club]]'
  slots_of_30: 1
- item: '[[items/weapon/984-chakram|Chakram]]'
  slots_of_30: 1
- item: '[[items/weapon/903-firangi|Firangi]]'
  slots_of_30: 1
- item: '[[items/weapon/985-shining-finger|Shining Finger]]'
  slots_of_30: 1
- item: '[[items/weapon/961-faust|Faust]]'
  slots_of_30: 1
- item: '[[items/weapon/942-bow-of-sagittarius|Bow of Sagittarius]]'
  slots_of_30: 1
- item: '[[items/weapon/954-abyss-rifle|Abyss Rifle]]'
  slots_of_30: 1
- item: '[[items/weapon/934-halbert|Halbert]]'
  slots_of_30: 1
- item: '[[items/weapon/969-chronicle-staff|Chronicle Staff]]'
  slots_of_30: 1
- item: '[[items/weapon/928-dark-buster|Dark Buster]]'
  slots_of_30: 1
- item: '[[items/weapon/977-crystal-wand|Crystal Wand]]'
  slots_of_30: 1
- item: '[[items/weapon/993-firangi-firangi|Firangi-Firangi]]'
  slots_of_30: 1
- item: '[[items/material/231-castle-gear-engine-schematic|Castle Gear Engine Schematic]]'
  slots_of_30: 1
spawns:
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5332.8
  y: 5126.4
  count: 1
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5788.4
  y: 4946.1
  count: 0
  group: basic
source:
  data: LIST_NPC.STB row 383, ITEM_DROP.STB row 386
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Behemoth King

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
