---
kind: monster
id: 488
name: Tirwin
status: in-game
level: 148
hp: 10212
hp_per_level: 69
attack: 690
hit: 447
defence: 601
resistance: 377
avoid: 174
attack_speed: 112
attack_range: 2.3
damage: physical
walk_speed: 277
run_speed: 754
xp: 135
drop_item_rate: 51
drop_money_rate: 30
zones: 1
drops:
- item: '[[items/consumable/153-hp-point-200|HP Point (+200)]]'
  slots_of_30: 2
- item: '[[items/consumable/152-hp-point-100|HP Point (+100)]]'
  slots_of_30: 1
- item: '[[items/consumable/154-hp-point-300|HP Point (+300)]]'
  slots_of_30: 1
- item: '[[items/material/82-lisent-cu|Lisent (Cu)]]'
  slots_of_30: 3
- item: '[[items/material/67-hime|Hime]]'
  slots_of_30: 2
- item: '[[items/material/81-lisent-fe|Lisent (Fe)]]'
  slots_of_30: 1
- item: '[[items/material/83-lisent-pb|Lisent (Pb)]]'
  slots_of_30: 3
- item: '[[items/material/66-iricer|Iricer]]'
  slots_of_30: 1
- item: '[[items/material/84-lisent-al|Lisent (Al)]]'
  slots_of_30: 2
- item: '[[items/material/68-low-enthiric|Low Enthiric]]'
  slots_of_30: 1
- item: '[[items/material/85-lisent-hg|Lisent (Hg)]]'
  slots_of_30: 1
- item: '[[items/material/69-enthiric|Enthiric]]'
  slots_of_30: 1
- item: '[[items/material/86-lisent-na|Lisent (Na)]]'
  slots_of_30: 1
- item: '[[items/consumable/185-clan-point-7|Clan Point (+7)]]'
  slots_of_30: 1
spawns:
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5319.5
  y: 5352.5
  count: 2
  group: reinforcements
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5317.9
  y: 5123.7
  count: 1
  group: basic
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5317.9
  y: 5123.7
  count: 2
  group: reinforcements
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5320.6
  y: 4904
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 488, ITEM_DROP.STB row 281
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Tirwin

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
