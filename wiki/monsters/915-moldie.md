---
kind: monster
id: 915
name: Moldie
status: in-game
level: 53
hp: 2109
attack: 168
hit: 127
defence: 184
resistance: 80
avoid: 61
attack_speed: 100
attack_range: 2.5
damage: physical
walk_speed: 180
run_speed: 410
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2815
source:
  data: LIST_NPC.STB row 915, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Moldie

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
