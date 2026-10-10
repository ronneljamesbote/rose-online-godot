---
kind: monster
id: 325
name: Rune Stone
status: in-game
level: 79
hp: 4582
hp_per_level: 58
attack: 322
hit: 217
defence: 236
resistance: 385
avoid: 87
attack_speed: 100
attack_range: 0.1
damage: physical
walk_speed: 0
run_speed: 0
xp: 0
drop_item_rate: 99
drop_money_rate: 0
zones: 4
spawns:
- zone: '[[zones/51-magic-city-of-the-eucar|Magic City of the Eucar]]'
  x: 5259.2
  y: 5346.8
  count: 1
  group: basic
- zone: '[[zones/51-magic-city-of-the-eucar|Magic City of the Eucar]]'
  x: 5503.5
  y: 5322
  count: 1
  group: basic
- zone: '[[zones/51-magic-city-of-the-eucar|Magic City of the Eucar]]'
  x: 5056.3
  y: 4927.7
  count: 1
  group: basic
- zone: '[[zones/52-mana-snowfields|Mana Snowfields]]'
  x: 6120.2
  y: 5258.1
  count: 1
  group: basic
- zone: '[[zones/52-mana-snowfields|Mana Snowfields]]'
  x: 5468
  y: 5069
  count: 1
  group: basic
- zone: '[[zones/52-mana-snowfields|Mana Snowfields]]'
  x: 5373.4
  y: 4480.3
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5449.9
  y: 5133.8
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 6066
  y: 5195.5
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5107.6
  y: 5038.8
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5842.7
  y: 4980.9
  count: 1
  group: basic
- zone: '[[zones/53-arumic-valley|Arumic Valley]]'
  x: 5555.9
  y: 4806.3
  count: 1
  group: basic
- zone: '[[zones/54-crystal-snowfields|Crystal Snowfields]]'
  x: 5229.4
  y: 5109.7
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 325, ITEM_DROP.STB row 0
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Rune Stone

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
