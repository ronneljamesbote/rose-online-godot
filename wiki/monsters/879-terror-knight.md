---
kind: monster
id: 879
name: Terror Knight
status: in-game
level: 85
hp: 2400
attack: 284
hit: 218
defence: 222
resistance: 107
avoid: 89
attack_speed: 108
attack_range: 3
damage: physical
walk_speed: 200
run_speed: 450
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/2371-terror-knight|Terror Knight]]'
source:
  data: LIST_NPC.STB row 879, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Terror Knight

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
