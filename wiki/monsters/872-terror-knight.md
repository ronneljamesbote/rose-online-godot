---
kind: monster
id: 872
name: Terror Knight
status: in-game
level: 71
hp: 145550
hp_per_level: 2050
attack: 231
hit: 187
defence: 194
resistance: 87
avoid: 73
attack_speed: 101
attack_range: 3
damage: physical
walk_speed: 165
run_speed: 380
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/2371-terror-knight|Terror Knight]]'
source:
  data: LIST_NPC.STB row 872, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Terror Knight

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
