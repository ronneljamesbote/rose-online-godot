---
kind: monster
id: 387
name: Elec Ghost
status: in-game
level: 125
hp: 43
attack: 533
hit: 289
defence: 303
resistance: 480
avoid: 194
attack_speed: 110
attack_range: 15
damage: magic
walk_speed: 280
run_speed: 610
xp: 85
drop_item_rate: 50
drop_money_rate: 20
zones: 1
drops:
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/182-clan-point-2|Clan Point (+2)]]'
  slots_of_30: 2
- item: '[[items/consumable/152-hp-point-100|HP Point (+100)]]'
  slots_of_30: 1
- item: '[[items/material/79-transparent-powder|Transparent Powder]]'
  slots_of_30: 3
- item: '[[items/material/80-rainbow-powder|Rainbow Powder]]'
  slots_of_30: 3
- item: '[[items/material/65-high-ether|High Ether]]'
  slots_of_30: 2
- item: '[[items/material/81-lisent-fe|Lisent (Fe)]]'
  slots_of_30: 3
- item: '[[items/material/78-golden-powder|Golden Powder]]'
  slots_of_30: 1
- item: '[[items/material/66-iricer|Iricer]]'
  slots_of_30: 1
- item: '[[items/material/82-lisent-cu|Lisent (Cu)]]'
  slots_of_30: 2
- item: '[[items/material/67-hime|Hime]]'
  slots_of_30: 1
- item: '[[items/consumable/183-clan-point-3|Clan Point (+3)]]'
  slots_of_30: 1
spawns:
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5319.5
  y: 5352.5
  count: 1
  group: basic
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5324.1
  y: 5125.8
  count: 1
  group: basic
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5545.1
  y: 5129.7
  count: 2
  group: reinforcements
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5546.6
  y: 5126.5
  count: 1
  group: basic
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5540.7
  y: 5127.1
  count: 1
  group: basic
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5540.7
  y: 5127.1
  count: 1
  group: basic
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5540.7
  y: 5127.1
  count: 1
  group: basic
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5320.6
  y: 4904
  count: 2
  group: reinforcements
source:
  data: LIST_NPC.STB row 387, ITEM_DROP.STB row 279
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Elec Ghost

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
