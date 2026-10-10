---
kind: monster
id: 456
name: Seal Stone
status: in-game
level: 128
hp: 10624
hp_per_level: 83
attack: 502
hit: 325
defence: 556
resistance: 417
avoid: 118
attack_speed: 90
attack_range: 0.1
damage: magic
walk_speed: 0
run_speed: 0
xp: 103
drop_item_rate: 80
drop_money_rate: 94
zones: 1
drops:
- item: weapon 120
  slots_of_30: 1
- item: '[[items/weapon/45-great-hammer|Great Hammer]]'
  slots_of_30: 1
- item: '[[items/weapon/16-lord-black-sun-sword|Lord Black Sun Sword]]'
  slots_of_30: 1
- item: '[[items/weapon/69-overgear-bow-gun|OverGear Bow Gun]]'
  slots_of_30: 1
- item: '[[items/weapon/344-bleak-wind|Bleak Wind]]'
  slots_of_30: 1
- item: '[[items/weapon/143-earthquake|Earthquake]]'
  slots_of_30: 1
- item: '[[items/weapon/414-stiletto|Stiletto]]'
  slots_of_30: 1
- item: '[[items/weapon/175-dragon-spear|Dragon Spear]]'
  slots_of_30: 1
- item: '[[items/weapon/215-dual-stick-bow|Dual Stick Bow]]'
  slots_of_30: 1
- item: '[[items/weapon/244-remington|Remington]]'
  slots_of_30: 1
- item: '[[items/weapon/442-brilliant-dual-wield|Brilliant Dual Wield]]'
  slots_of_30: 1
- item: '[[items/weapon/272-particle-cannon|Particle Cannon]]'
  slots_of_30: 1
- item: '[[items/weapon/314-prayer-staff|Prayer Staff]]'
  slots_of_30: 1
- item: '[[items/weapon/860-dual-flare-swords|Dual Flare Swords]]'
  slots_of_30: 1
- item: '[[items/weapon/589-dragon-sword|Dragon Sword]]'
  slots_of_30: 1
- item: '[[items/weapon/567-overgear-bow-gun|OverGear Bow Gun]]'
  slots_of_30: 1
- item: '[[items/weapon/775-rune-staff|Rune Staff]]'
  slots_of_30: 1
- item: '[[items/weapon/653-oriental-spear|Oriental Spear]]'
  slots_of_30: 1
- item: '[[items/weapon/545-battle-mace|Battle Mace]]'
  slots_of_30: 1
- item: '[[items/weapon/516-lord-black-sun-sword|Lord Black Sun Sword]]'
  slots_of_30: 1
- item: '[[items/weapon/738-particle-cannon|Particle Cannon]]'
  slots_of_30: 1
- item: '[[items/weapon/681-dual-stick-bow|Dual Stick Bow]]'
  slots_of_30: 1
- item: '[[items/weapon/621-earthquake|Earthquake]]'
  slots_of_30: 1
- item: '[[items/weapon/802-bleak-wind|Bleak Wind]]'
  slots_of_30: 1
- item: '[[items/weapon/711-remington|Remington]]'
  slots_of_30: 1
quests:
- '[[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]'
spawns:
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5575
  y: 4476.6
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5575.7
  y: 4385.1
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5665.1
  y: 4381.9
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5665.1
  y: 4381.9
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5393.7
  y: 4209
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5304.4
  y: 4209.2
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5574.8
  y: 4293.4
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5304.8
  y: 4115.2
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5485.4
  y: 4115
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 456, ITEM_DROP.STB row 406
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Seal Stone

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
