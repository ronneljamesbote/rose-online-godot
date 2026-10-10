---
kind: monster
id: 905
name: Dalping
status: in-game
level: 42
hp: 1343
attack: 131
hit: 122
defence: 133
resistance: 52
avoid: 66
attack_speed: 80
attack_range: 12
damage: physical
walk_speed: 150
run_speed: 200
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2805
source:
  data: LIST_NPC.STB row 905, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Dalping

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
