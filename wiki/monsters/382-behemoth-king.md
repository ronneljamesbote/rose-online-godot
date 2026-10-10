---
kind: monster
id: 382
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
drop_money_rate: 0
zones: 1
drops:
- item: '[[items/material/195-broken-horn|Broken Horn]]'
  slots_of_30: 1
- item: '[[items/material/270-behemoth-tendon|Behemoth Tendon]]'
  slots_of_30: 1
- item: '[[items/material/151-black-hearts|Black Hearts]]'
  slots_of_30: 1
- item: '[[items/material/152-green-hearts|Green Hearts]]'
  slots_of_30: 1
- item: '[[items/material/153-blue-hearts|Blue Hearts]]'
  slots_of_30: 1
- item: '[[items/material/154-pink-hearts|Pink Hearts]]'
  slots_of_30: 1
- item: '[[items/material/155-red-hearts|Red Hearts]]'
  slots_of_30: 1
- item: '[[items/material/156-golden-hearts|Golden Hearts]]'
  slots_of_30: 1
- item: '[[items/material/157-white-hearts|White Hearts]]'
  slots_of_30: 1
- item: '[[items/weapon/861-brilliant-dual-wield|Brilliant Dual Wield]]'
  slots_of_30: 1
- item: '[[items/weapon/836-stiletto|Stiletto]]'
  slots_of_30: 1
- item: '[[items/weapon/802-bleak-wind|Bleak Wind]]'
  slots_of_30: 1
- item: '[[items/weapon/776-prayer-staff|Prayer Staff]]'
  slots_of_30: 1
- item: '[[items/weapon/738-particle-cannon|Particle Cannon]]'
  slots_of_30: 1
- item: '[[items/weapon/681-dual-stick-bow|Dual Stick Bow]]'
  slots_of_30: 1
- item: '[[items/weapon/655-dragon-spear|Dragon Spear]]'
  slots_of_30: 1
- item: '[[items/weapon/621-earthquake|Earthquake]]'
  slots_of_30: 1
- item: '[[items/weapon/590-great-sword|Great Sword]]'
  slots_of_30: 2
- item: '[[items/weapon/568-overgear-bow-gun|OverGear Bow Gun]]'
  slots_of_30: 1
- item: '[[items/weapon/548-forest-hammer|Forest Hammer]]'
  slots_of_30: 2
- item: '[[items/weapon/517-lord-black-sun-sword|Lord Black Sun Sword]]'
  slots_of_30: 1
- item: '[[items/weapon/518-flamboyant|Flamboyant]]'
  slots_of_30: 1
- item: '[[items/weapon/623-bloody-axe|Bloody Axe]]'
  slots_of_30: 1
- item: '[[items/weapon/656-dagan-spear|Dagan Spear]]'
  slots_of_30: 1
- item: '[[items/weapon/682-mediator-bow|Mediator Bow]]'
  slots_of_30: 1
- item: '[[items/weapon/714-brave-gun|Brave Gun]]'
  slots_of_30: 1
- item: '[[items/weapon/778-cypress-pole|Cypress Pole]]'
  slots_of_30: 1
spawns:
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5329.1
  y: 5129.7
  count: 1
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5422.4
  y: 4935.9
  count: 1
  group: reinforcements
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5784.4
  y: 4950
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 382, ITEM_DROP.STB row 381
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Behemoth King

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
