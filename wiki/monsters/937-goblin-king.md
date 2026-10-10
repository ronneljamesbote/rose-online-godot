---
kind: monster
id: 937
name: Goblin King
status: in-game
level: 104
hp: 7461
attack: 454
hit: 275
defence: 375
resistance: 160
avoid: 133
attack_speed: 100
attack_range: 3
damage: physical
walk_speed: 310
run_speed: 730
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2837
source:
  data: LIST_NPC.STB row 937, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Goblin King

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
