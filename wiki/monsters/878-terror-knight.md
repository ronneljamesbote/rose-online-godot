---
kind: monster
id: 878
name: Terror Knight
status: in-game
level: 83
hp: 195050
hp_per_level: 2350
attack: 276
hit: 213
defence: 218
resistance: 104
avoid: 86
attack_speed: 107
attack_range: 3
damage: physical
walk_speed: 195
run_speed: 440
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/2371-terror-knight|Terror Knight]]'
source:
  data: LIST_NPC.STB row 878, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Terror Knight

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
