---
kind: monster
id: 924
name: Jewel Golem
status: in-game
level: 62
hp: 160766
hp_per_level: 2593
attack: 222
hit: 179
defence: 202
resistance: 87
avoid: 79
attack_speed: 80
attack_range: 2.5
damage: physical
walk_speed: 250
run_speed: 500
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2824
source:
  data: LIST_NPC.STB row 924, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Jewel Golem

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
