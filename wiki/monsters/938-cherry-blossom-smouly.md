---
kind: monster
id: 938
name: Cherry Blossom Smouly
status: in-game
level: 70
hp: 132020
hp_per_level: 1886
attack: 295
hit: 215
defence: 191
resistance: 90
avoid: 87
attack_speed: 100
attack_range: 14
damage: physical
walk_speed: 300
run_speed: 600
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2838
source:
  data: LIST_NPC.STB row 938, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Cherry Blossom Smouly

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
