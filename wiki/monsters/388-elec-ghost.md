---
kind: monster
id: 388
name: Elec Ghost
status: in-game
level: 135
hp: 45
attack: 584
hit: 314
defence: 335
resistance: 527
avoid: 211
attack_speed: 110
attack_range: 15
damage: magic
walk_speed: 280
run_speed: 610
xp: 91
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
  x: 5324.1
  y: 5125.8
  count: 1
  group: basic
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5324.1
  y: 5125.8
  count: 1
  group: basic
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5324.1
  y: 5125.8
  count: 1
  group: basic
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5324.1
  y: 5125.8
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 388, ITEM_DROP.STB row 279
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Elec Ghost

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
