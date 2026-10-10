---
kind: monster
id: 452
name: Seal Stone
status: in-game
level: 88
hp: 6600
hp_per_level: 75
attack: 323
hit: 223
defence: 358
resistance: 260
avoid: 75
attack_speed: 90
attack_range: 0.1
damage: magic
walk_speed: 0
run_speed: 0
xp: 72
drop_item_rate: 80
drop_money_rate: 92
zones: 1
drops:
- item: '[[items/weapon/239-beretta|Beretta]]'
  slots_of_30: 1
- item: '[[items/weapon/267-mythril-launcher|Mythril Launcher]]'
  slots_of_30: 2
- item: '[[items/weapon/339-blizzard-wand|Blizzard Wand]]'
  slots_of_30: 1
- item: '[[items/weapon/438-blood-sword-axe|Blood Sword & Axe]]'
  slots_of_30: 1
- item: '[[items/weapon/410-shaft-claw|Shaft Claw]]'
  slots_of_30: 1
- item: '[[items/weapon/210-half-elf-bow|Half-Elf Bow]]'
  slots_of_30: 1
- item: '[[items/weapon/65-poison-bow-gun|Poison Bow Gun]]'
  slots_of_30: 1
- item: '[[items/weapon/170-lightning-spear|Lightning Spear]]'
  slots_of_30: 1
- item: '[[items/weapon/11-viking-sword|Viking Sword]]'
  slots_of_30: 1
- item: '[[items/weapon/310-land-staff|Land Staff]]'
  slots_of_30: 1
- item: '[[items/weapon/110-bastard-sword|Bastard Sword]]'
  slots_of_30: 1
- item: '[[items/weapon/138-silver-axe|Silver Axe]]'
  slots_of_30: 1
- item: '[[items/weapon/40-morning-star|Morning Star]]'
  slots_of_30: 1
- item: '[[items/weapon/706-beretta|Beretta]]'
  slots_of_30: 1
- item: '[[items/weapon/585-executioner|Executioner]]'
  slots_of_30: 1
- item: '[[items/weapon/856-viking-sword-axe|Viking Sword & Axe]]'
  slots_of_30: 1
- item: '[[items/weapon/537-morning-star|Morning Star]]'
  slots_of_30: 2
- item: '[[items/weapon/564-dark-bow-gun|Dark Bow Gun]]'
  slots_of_30: 1
- item: '[[items/weapon/796-blizzard-wand|Blizzard Wand]]'
  slots_of_30: 1
- item: '[[items/weapon/507-viking-sword|Viking Sword]]'
  slots_of_30: 1
- item: '[[items/weapon/827-dual-patar|Dual Patar]]'
  slots_of_30: 1
- item: '[[items/weapon/676-half-elf-bow|Half-Elf Bow]]'
  slots_of_30: 1
quests:
- '[[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]'
spawns:
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5575.4
  y: 5370.6
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5575.4
  y: 5370.6
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5258.2
  y: 5192.3
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5237.3
  y: 5216.9
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5282.6
  y: 5172.7
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5280.1
  y: 5217.3
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5575
  y: 5196.2
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5395.9
  y: 5019.4
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 452, ITEM_DROP.STB row 402
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Seal Stone

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
