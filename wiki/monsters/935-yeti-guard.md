---
kind: monster
id: 935
name: Yeti Guard
status: in-game
level: 97
hp: 4385
attack: 382
hit: 252
defence: 383
resistance: 164
avoid: 130
attack_speed: 100
attack_range: 3
damage: physical
walk_speed: 225
run_speed: 560
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2835
source:
  data: LIST_NPC.STB row 935, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Yeti Guard

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
