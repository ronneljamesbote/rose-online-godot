---
kind: monster
id: 904
name: Pumpkin
status: in-game
level: 30
hp: 27540
hp_per_level: 918
attack: 90
hit: 86
defence: 80
resistance: 41
avoid: 83
attack_speed: 90
attack_range: 2
damage: physical
walk_speed: 200
run_speed: 260
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2804
source:
  data: LIST_NPC.STB row 904, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Pumpkin

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
