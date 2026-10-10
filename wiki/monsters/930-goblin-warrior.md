---
kind: monster
id: 930
name: Goblin Warrior
status: in-game
level: 78
hp: 219726
hp_per_level: 2817
attack: 295
hit: 184
defence: 228
resistance: 101
avoid: 97
attack_speed: 100
attack_range: 3.5
damage: physical
walk_speed: 215
run_speed: 480
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- skill 2830
source:
  data: LIST_NPC.STB row 930, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Goblin Warrior

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
