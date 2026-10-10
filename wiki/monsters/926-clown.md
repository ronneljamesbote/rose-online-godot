---
kind: monster
id: 926
name: Clown
status: in-game
level: 59
hp: 2304
attack: 195
hit: 143
defence: 189
resistance: 75
avoid: 72
attack_speed: 100
attack_range: 3.5
damage: physical
walk_speed: 260
run_speed: 550
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2826
source:
  data: LIST_NPC.STB row 926, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Clown

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
