---
kind: monster
id: 918
name: Grunter
status: in-game
level: 56
hp: 1992
attack: 194
hit: 155
defence: 167
resistance: 60
avoid: 64
attack_speed: 100
attack_range: 3.5
damage: physical
walk_speed: 185
run_speed: 420
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2818
source:
  data: LIST_NPC.STB row 918, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Grunter

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
