---
kind: monster
id: 922
name: Doonga Warrior
status: in-game
level: 72
hp: 2981
attack: 238
hit: 165
defence: 257
resistance: 108
avoid: 81
attack_speed: 95
attack_range: 3.5
damage: physical
walk_speed: 200
run_speed: 450
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2822
source:
  data: LIST_NPC.STB row 922, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Doonga Warrior

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
