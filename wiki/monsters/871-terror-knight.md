---
kind: monster
id: 871
name: Terror Knight
status: in-game
level: 69
hp: 2000
attack: 224
hit: 183
defence: 190
resistance: 84
avoid: 71
attack_speed: 100
attack_range: 3
damage: physical
walk_speed: 160
run_speed: 370
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/2371-terror-knight|Terror Knight]]'
source:
  data: LIST_NPC.STB row 871, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Terror Knight

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
