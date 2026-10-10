---
kind: monster
id: 454
name: Seal Stone
status: in-game
level: 108
hp: 8532
hp_per_level: 79
attack: 410
hit: 273
defence: 453
resistance: 336
avoid: 95
attack_speed: 90
attack_range: 0.1
damage: magic
walk_speed: 0
run_speed: 0
xp: 90
drop_item_rate: 80
drop_money_rate: 93
zones: 1
drops:
- item: '[[items/weapon/312-dark-staff|Dark Staff]]'
  slots_of_30: 1
- item: '[[items/weapon/242-rp-911|RP-911]]'
  slots_of_30: 1
- item: '[[items/weapon/412-dragon-knuckle|Dragon Knuckle]]'
  slots_of_30: 1
- item: '[[items/weapon/173-gungnir|Gungnir]]'
  slots_of_30: 1
- item: '[[items/weapon/440-ice-flare-swords|Ice & Flare Swords]]'
  slots_of_30: 1
- item: '[[items/weapon/14-ice-sword|Ice Sword]]'
  slots_of_30: 1
- item: '[[items/weapon/342-holy-wand|Holy Wand]]'
  slots_of_30: 1
- item: '[[items/weapon/270-shadow-launcher|Shadow Launcher]]'
  slots_of_30: 1
- item: '[[items/weapon/141-ice-axe|Ice Axe]]'
  slots_of_30: 1
- item: '[[items/weapon/68-anima-bow-gun|Anima Bow Gun]]'
  slots_of_30: 1
- item: '[[items/weapon/43-scorpion-club|Scorpion Club]]'
  slots_of_30: 1
- item: '[[items/weapon/112-flamberge|Flamberge]]'
  slots_of_30: 1
- item: '[[items/weapon/213-cupid-bow|Cupid Bow]]'
  slots_of_30: 1
- item: '[[items/weapon/832-assassin-katar|Assassin Katar]]'
  slots_of_30: 1
- item: '[[items/weapon/773-dark-staff|Dark Staff]]'
  slots_of_30: 1
- item: '[[items/weapon/735-cross-launcher|Cross Launcher]]'
  slots_of_30: 1
- item: '[[items/weapon/512-katana|Katana]]'
  slots_of_30: 1
- item: '[[items/weapon/542-blood-mace|Blood Mace]]'
  slots_of_30: 1
- item: '[[items/weapon/565-elf-bow-gun|Elf Bow Gun]]'
  slots_of_30: 1
- item: '[[items/weapon/586-claymore|Claymore]]'
  slots_of_30: 1
- item: '[[items/weapon/619-great-axe|Great Axe]]'
  slots_of_30: 1
- item: '[[items/weapon/650-chaos-spear|Chaos Spear]]'
  slots_of_30: 1
- item: '[[items/weapon/679-bow-of-east-archer|Bow of East Archer]]'
  slots_of_30: 1
- item: '[[items/weapon/709-justice-cannon|Justice Cannon]]'
  slots_of_30: 1
quests:
- '[[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]'
spawns:
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5214.8
  y: 4828
  count: 1
  group: reinforcements
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5035.3
  y: 4744.4
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5213.1
  y: 4651.4
  count: 1
  group: reinforcements
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5599.8
  y: 4675.5
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5553.9
  y: 4677.6
  count: 0
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5033
  y: 4561
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 454, ITEM_DROP.STB row 404
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Seal Stone

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
