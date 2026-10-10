---
kind: monster
id: 489
name: Tirwin
status: in-game
level: 158
hp: 70
attack: 746
hit: 480
defence: 654
resistance: 413
avoid: 189
attack_speed: 111
attack_range: 2.3
damage: physical
walk_speed: 283
run_speed: 766
xp: 179
drop_item_rate: 51
drop_money_rate: 30
zones: 1
drops:
- item: '[[items/material/83-lisent-pb|Lisent (Pb)]]'
  slots_of_30: 3
- item: '[[items/material/82-lisent-cu|Lisent (Cu)]]'
  slots_of_30: 1
- item: '[[items/material/67-hime|Hime]]'
  slots_of_30: 1
- item: '[[items/material/84-lisent-al|Lisent (Al)]]'
  slots_of_30: 3
- item: '[[items/material/68-low-enthiric|Low Enthiric]]'
  slots_of_30: 1
- item: '[[items/material/85-lisent-hg|Lisent (Hg)]]'
  slots_of_30: 2
- item: '[[items/material/87-lisent-cr|Lisent (Cr)]]'
  slots_of_30: 1
- item: '[[items/material/86-lisent-na|Lisent (Na)]]'
  slots_of_30: 2
- item: '[[items/material/69-enthiric|Enthiric]]'
  slots_of_30: 1
- item: '[[items/material/88-lisent-au|Lisent (Au)]]'
  slots_of_30: 1
spawns:
- zone: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
  x: 5033
  y: 4561
  count: 3
  group: reinforcements
source:
  data: LIST_NPC.STB row 489, ITEM_DROP.STB row 414
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Tirwin

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
