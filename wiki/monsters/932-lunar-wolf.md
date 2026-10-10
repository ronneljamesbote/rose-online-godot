---
kind: monster
id: 932
name: Lunar Wolf
status: in-game
level: 87
hp: 271875
hp_per_level: 3125
attack: 353
hit: 236
defence: 266
resistance: 145
avoid: 116
attack_speed: 100
attack_range: 2.5
damage: physical
walk_speed: 230
run_speed: 570
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2832
source:
  data: LIST_NPC.STB row 932, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Lunar Wolf

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
