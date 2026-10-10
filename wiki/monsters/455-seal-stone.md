---
kind: monster
id: 455
name: Seal Stone
status: in-game
level: 120
hp: 82
attack: 465
hit: 304
defence: 514
resistance: 384
avoid: 109
attack_speed: 90
attack_range: 0.1
damage: magic
walk_speed: 0
run_speed: 0
xp: 98
drop_item_rate: 80
drop_money_rate: 94
zones: 1
drops:
- item: '[[items/weapon/272-particle-cannon|Particle Cannon]]'
  slots_of_30: 1
- item: '[[items/weapon/143-earthquake|Earthquake]]'
  slots_of_30: 1
- item: '[[items/weapon/114-heroic-sword|Heroic Sword]]'
  slots_of_30: 1
- item: '[[items/weapon/313-rune-staff|Rune Staff]]'
  slots_of_30: 1
- item: '[[items/weapon/16-lord-black-sun-sword|Lord Black Sun Sword]]'
  slots_of_30: 1
- item: '[[items/weapon/69-overgear-bow-gun|OverGear Bow Gun]]'
  slots_of_30: 1
- item: '[[items/weapon/344-bleak-wind|Bleak Wind]]'
  slots_of_30: 1
- item: '[[items/weapon/442-brilliant-dual-wield|Brilliant Dual Wield]]'
  slots_of_30: 1
- item: '[[items/weapon/414-stiletto|Stiletto]]'
  slots_of_30: 1
- item: '[[items/weapon/45-great-hammer|Great Hammer]]'
  slots_of_30: 1
- item: '[[items/weapon/243-piercing-gun|Piercing Gun]]'
  slots_of_30: 1
- item: weapon 182
  slots_of_30: 1
- item: '[[items/weapon/214-bow-of-wind-spirit|Bow of Wind Spirit]]'
  slots_of_30: 1
- item: '[[items/weapon/544-scorpion-club|Scorpion Club]]'
  slots_of_30: 1
- item: '[[items/weapon/566-anima-bow-gun|Anima Bow Gun]]'
  slots_of_30: 1
- item: '[[items/weapon/735-cross-launcher|Cross Launcher]]'
  slots_of_30: 1
- item: '[[items/weapon/774-dark-staff|Dark Staff]]'
  slots_of_30: 1
- item: '[[items/weapon/652-gungnir|Gungnir]]'
  slots_of_30: 1
- item: '[[items/weapon/588-flamberge|Flamberge]]'
  slots_of_30: 1
- item: '[[items/weapon/679-bow-of-east-archer|Bow of East Archer]]'
  slots_of_30: 1
- item: '[[items/weapon/709-justice-cannon|Justice Cannon]]'
  slots_of_30: 1
- item: '[[items/weapon/833-dragon-knuckle|Dragon Knuckle]]'
  slots_of_30: 1
- item: '[[items/weapon/858-dual-katana|Dual Katana]]'
  slots_of_30: 1
- item: '[[items/weapon/800-shadow-wand|Shadow Wand]]'
  slots_of_30: 1
quests:
- '[[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]'
spawns:
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5215.2
  y: 4570.1
  count: 1
  group: reinforcements
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5125.8
  y: 4463.6
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5306.3
  y: 4385
  count: 1
  group: reinforcements
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5307.8
  y: 4470.8
  count: 1
  group: reinforcements
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5395.2
  y: 4375.5
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5484.5
  y: 4384.5
  count: 1
  group: reinforcements
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5575.7
  y: 4385.1
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 455, ITEM_DROP.STB row 405
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Seal Stone

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
