---
kind: monster
id: 910
name: Beetle
status: in-game
level: 41
hp: 65723
hp_per_level: 1603
attack: 127
hit: 105
defence: 143
resistance: 63
avoid: 48
attack_speed: 90
attack_range: 2
damage: physical
walk_speed: 190
run_speed: 380
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2810
source:
  data: LIST_NPC.STB row 910, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Beetle

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
