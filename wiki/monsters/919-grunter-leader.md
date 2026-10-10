---
kind: monster
id: 919
name: Grunter Leader
status: in-game
level: 65
hp: 177840
hp_per_level: 2736
attack: 234
hit: 186
defence: 213
resistance: 91
avoid: 83
attack_speed: 110
attack_range: 5
damage: physical
walk_speed: 220
run_speed: 490
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2819
source:
  data: LIST_NPC.STB row 919, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Grunter Leader

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
