---
kind: monster
id: 453
name: Seal Stone
status: in-game
level: 100
hp: 77
attack: 375
hit: 252
defence: 414
resistance: 305
avoid: 87
attack_speed: 90
attack_range: 0.1
damage: magic
walk_speed: 0
run_speed: 0
xp: 79
drop_item_rate: 80
drop_money_rate: 93
zones: 1
drops:
- item: '[[items/weapon/411-assassin-katar|Assassin Katar]]'
  slots_of_30: 1
- item: '[[items/weapon/42-blood-mace|Blood Mace]]'
  slots_of_30: 1
- item: '[[items/weapon/140-great-axe|Great Axe]]'
  slots_of_30: 1
- item: '[[items/weapon/341-windstorm-wand|Windstorm Wand]]'
  slots_of_30: 1
- item: '[[items/weapon/269-burning-launcher|Burning Launcher]]'
  slots_of_30: 1
- item: '[[items/weapon/111-claymore|Claymore]]'
  slots_of_30: 1
- item: '[[items/weapon/67-elf-bow-gun|Elf Bow Gun]]'
  slots_of_30: 1
- item: '[[items/weapon/13-katana|Katana]]'
  slots_of_30: 1
- item: '[[items/weapon/439-dual-katana|Dual Katana]]'
  slots_of_30: 1
- item: '[[items/weapon/172-chaos-spear|Chaos Spear]]'
  slots_of_30: 1
- item: '[[items/weapon/311-holy-staff|Holy Staff]]'
  slots_of_30: 1
- item: '[[items/weapon/241-justice-cannon|Justice Cannon]]'
  slots_of_30: 1
- item: '[[items/weapon/212-bow-of-east-archer|Bow of East Archer]]'
  slots_of_30: 1
- item: '[[items/weapon/831-assassin-katar|Assassin Katar]]'
  slots_of_30: 1
- item: '[[items/weapon/649-chaos-spear|Chaos Spear]]'
  slots_of_30: 1
- item: '[[items/weapon/564-dark-bow-gun|Dark Bow Gun]]'
  slots_of_30: 1
- item: '[[items/weapon/771-holy-staff|Holy Staff]]'
  slots_of_30: 1
- item: '[[items/weapon/735-cross-launcher|Cross Launcher]]'
  slots_of_30: 1
- item: '[[items/weapon/678-mythril-bow|Mythril Bow]]'
  slots_of_30: 1
- item: '[[items/weapon/541-blood-mace|Blood Mace]]'
  slots_of_30: 1
- item: '[[items/weapon/585-executioner|Executioner]]'
  slots_of_30: 1
- item: '[[items/weapon/511-katana|Katana]]'
  slots_of_30: 1
- item: '[[items/weapon/798-recovery-wand|Recovery Wand]]'
  slots_of_30: 1
quests:
- '[[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]'
spawns:
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5394.6
  y: 4831.1
  count: 1
  group: reinforcements
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5573.3
  y: 4697.9
  count: 1
  group: reinforcements
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5550.2
  y: 4721.3
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5575.6
  y: 4706.3
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5597.7
  y: 4718.9
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 453, ITEM_DROP.STB row 403
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Seal Stone

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
