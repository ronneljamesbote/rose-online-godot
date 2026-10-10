---
kind: monster
id: 385
name: Lunar Keeper
status: in-game
level: 127
hp: 44
attack: 543
hit: 294
defence: 309
resistance: 489
avoid: 197
attack_speed: 105
attack_range: 14
damage: magic
walk_speed: 250
run_speed: 550
xp: 83
drop_item_rate: 32
drop_money_rate: 57
zones: 2
spawns:
- zone: '[[zones/54-crystal-snowfields|Crystal Snowfields]]'
  x: 5048.5
  y: 5025.4
  count: 1
  group: basic
- zone: '[[zones/54-crystal-snowfields|Crystal Snowfields]]'
  x: 5542.9
  y: 4500.2
  count: 1
  group: reinforcements
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5618.6
  y: 5047.7
  count: 1
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5785.3
  y: 4982.2
  count: 1
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5492.5
  y: 4934.7
  count: 1
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5492.5
  y: 4934.7
  count: 1
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5539.6
  y: 4931.7
  count: 1
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5539.6
  y: 4931.7
  count: 1
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5539.6
  y: 4931.7
  count: 1
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5517.3
  y: 4891.4
  count: 1
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5517.3
  y: 4891.4
  count: 1
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5517.4
  y: 4868.5
  count: 1
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5517.4
  y: 4868.5
  count: 1
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5517.4
  y: 4868.5
  count: 1
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5771.5
  y: 4928.5
  count: 1
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5784.4
  y: 4950
  count: 1
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5410.9
  y: 4510
  count: 3
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5704.9
  y: 4547.7
  count: 1
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5704.9
  y: 4547.7
  count: 1
  group: reinforcements
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5747.3
  y: 4459.5
  count: 1
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5747.3
  y: 4459.5
  count: 1
  group: reinforcements
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5659
  y: 4466.7
  count: 1
  group: basic
- zone: '[[zones/55-freezing-plateau|Freezing Plateau]]'
  x: 5659
  y: 4466.7
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 385, ITEM_DROP.STB row 0
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Lunar Keeper

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
