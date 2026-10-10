---
kind: monster
id: 867
name: Mercenary Hunter
status: in-game
level: 70
hp: 134190
hp_per_level: 1917
attack: 230
hit: 214
defence: 186
resistance: 88
avoid: 104
attack_speed: 112
attack_range: 18.5
damage: physical
walk_speed: 260
run_speed: 570
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/2361-hire-hunter|Hire Hunter]]'
source:
  data: LIST_NPC.STB row 867, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Mercenary Hunter

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
