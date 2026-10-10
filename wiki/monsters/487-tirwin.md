---
kind: monster
id: 487
name: Tirwin
status: in-game
level: 138
hp: 9246
hp_per_level: 67
attack: 635
hit: 414
defence: 550
resistance: 342
avoid: 160
attack_speed: 111
attack_range: 2.3
damage: physical
walk_speed: 271
run_speed: 742
xp: 125
drop_item_rate: 51
drop_money_rate: 30
zones: 1
drops:
- item: '[[items/consumable/182-clan-point-2|Clan Point (+2)]]'
  slots_of_30: 1
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/183-clan-point-3|Clan Point (+3)]]'
  slots_of_30: 1
- item: '[[items/consumable/152-hp-point-100|HP Point (+100)]]'
  slots_of_30: 1
- item: '[[items/material/80-rainbow-powder|Rainbow Powder]]'
  slots_of_30: 3
- item: '[[items/material/81-lisent-fe|Lisent (Fe)]]'
  slots_of_30: 3
- item: '[[items/material/66-iricer|Iricer]]'
  slots_of_30: 2
- item: '[[items/material/82-lisent-cu|Lisent (Cu)]]'
  slots_of_30: 3
- item: '[[items/material/83-lisent-pb|Lisent (Pb)]]'
  slots_of_30: 2
- item: '[[items/material/67-hime|Hime]]'
  slots_of_30: 1
- item: '[[items/material/84-lisent-al|Lisent (Al)]]'
  slots_of_30: 1
- item: '[[items/material/68-low-enthiric|Low Enthiric]]'
  slots_of_30: 1
- item: '[[items/consumable/184-clan-point-5|Clan Point (+5)]]'
  slots_of_30: 1
spawns:
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5319.5
  y: 5352.5
  count: 1
  group: basic
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5317.9
  y: 5123.7
  count: 1
  group: basic
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5545.1
  y: 5129.7
  count: 1
  group: basic
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5545.1
  y: 5129.7
  count: 1
  group: basic
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5320.6
  y: 4904
  count: 1
  group: basic
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5320.6
  y: 4904
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 487, ITEM_DROP.STB row 280
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Tirwin

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
