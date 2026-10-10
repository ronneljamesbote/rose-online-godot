---
kind: monster
id: 451
name: Seal Stone
status: in-game
level: 80
hp: 5840
hp_per_level: 73
attack: 290
hit: 204
defence: 322
resistance: 232
avoid: 67
attack_speed: 90
attack_range: 0.1
damage: magic
walk_speed: 0
run_speed: 0
xp: 68
drop_item_rate: 80
drop_money_rate: 92
zones: 1
drops:
- item: '[[items/weapon/267-mythril-launcher|Mythril Launcher]]'
  slots_of_30: 2
- item: '[[items/weapon/65-poison-bow-gun|Poison Bow Gun]]'
  slots_of_30: 1
- item: '[[items/weapon/10-shark-blade|Shark Blade]]'
  slots_of_30: 1
- item: '[[items/weapon/210-half-elf-bow|Half-Elf Bow]]'
  slots_of_30: 1
- item: '[[items/weapon/108-after-blade|After Blade]]'
  slots_of_30: 1
- item: '[[items/weapon/339-blizzard-wand|Blizzard Wand]]'
  slots_of_30: 1
- item: '[[items/weapon/408-crescent-knuckle|Crescent Knuckle]]'
  slots_of_30: 1
- item: '[[items/weapon/137-cross-axe|Cross Axe]]'
  slots_of_30: 1
- item: '[[items/weapon/39-ice-hammer|Ice Hammer]]'
  slots_of_30: 1
- item: '[[items/weapon/169-halberd|Halberd]]'
  slots_of_30: 1
- item: '[[items/weapon/309-anima-staff|Anima Staff]]'
  slots_of_30: 1
- item: '[[items/weapon/436-dual-ocean-swords|Dual Ocean Swords]]'
  slots_of_30: 1
- item: '[[items/weapon/239-beretta|Beretta]]'
  slots_of_30: 1
- item: '[[items/weapon/794-windphon-wand|Windphon Wand]]'
  slots_of_30: 1
- item: '[[items/weapon/823-bear-s-paw|Bear''s Paw]]'
  slots_of_30: 1
- item: '[[items/weapon/704-sorden-gun|Sorden Gun]]'
  slots_of_30: 1
- item: '[[items/weapon/853-twin-ghost-bat|Twin Ghost Bat]]'
  slots_of_30: 1
- item: '[[items/weapon/534-stone-hammer|Stone Hammer]]'
  slots_of_30: 1
- item: '[[items/weapon/643-trident|Trident]]'
  slots_of_30: 2
- item: '[[items/weapon/764-wisp-staff|Wisp Staff]]'
  slots_of_30: 1
- item: '[[items/weapon/732-bronze-launcher|Bronze Launcher]]'
  slots_of_30: 1
- item: '[[items/weapon/615-tower-axe|Tower Axe]]'
  slots_of_30: 1
quests:
- '[[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]'
spawns:
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5126
  y: 5375.8
  count: 1
  group: reinforcements
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5258.1
  y: 5202.1
  count: 1
  group: reinforcements
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5235.9
  y: 5170.1
  count: 1
  group: basic
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5125.4
  y: 5017.4
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 451, ITEM_DROP.STB row 401
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Seal Stone

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
